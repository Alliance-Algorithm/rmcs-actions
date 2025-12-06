"""
WebSocket client for robot daemon.

This module implements the main client connection logic per protocol.md.
"""

from __future__ import annotations

import asyncio
import contextlib

from typing import TYPE_CHECKING
from typing import Any

import websockets.asyncio.client

from rmcs_actions_client.events.heartbeat import send_heartbeat
from rmcs_actions_client.identification import request_robot_id
from rmcs_actions_client.lib.adaptor import MessageAdaptor
from rmcs_actions_client.log import GL
from rmcs_actions_client.service import get_loop
from rmcs_actions_client.service import handle_instruction_message
from rmcs_actions_client.service import register_instruction
from rmcs_actions_client.service import shutdown
from rmcs_actions_client.storage import RobotStorage


if TYPE_CHECKING:
    from rmcs_actions_client.config import Configuration


class RobotClient:
    """WebSocket client for robot daemon."""

    def __init__(self, config: Configuration) -> None:
        """
        Initialize robot client.

        Args:
            config: Application configuration
        """
        self.config = config
        self.storage = RobotStorage(config.storage.directory)
        self.robot_id: str | None = None
        self.websocket: Any = None
        self.adaptor: MessageAdaptor | None = None
        self.heartbeat_task: asyncio.Task | None = None
        self._shutdown = False

    async def identify(self) -> str:
        """
        Identify robot with server and obtain robot ID.

        Args:
        Returns:
            Robot ID

        Raises:
            Exception: If identification fails
        """
        # Try to load cached robot ID
        cached_id = self.storage.get_robot_id()
        if cached_id:
            GL.info(f"Using cached robot ID: {cached_id}")
            return cached_id

        # No cached ID, request from server via whoami
        GL.info("No cached robot ID, requesting from server via /ident/whoami")

        try:
            robot_id = await request_robot_id(self.config.server.http)
            GL.info(f"Server assigned robot ID: {robot_id}")

            self.storage.save_robot_id(robot_id)
            return robot_id

        except Exception as exc:
            GL.error(f"Failed to identify with server: {exc}", exc_info=True)
            # Fail fast per requirement: do not fallback
            raise

    def _register_handlers(self) -> None:
        """Register all protocol instruction handlers."""
        from rmcs_actions_client.handlers import fetch_network_handler
        from rmcs_actions_client.handlers import sync_robot_id_handler

        GL.info("Registering instruction handlers")

        # Wrap handlers to inject storage dependency
        async def wrapped_sync_robot_id(payload: dict, session_ctx: Any) -> dict | None:
            return await sync_robot_id_handler(payload, session_ctx, self.storage)

        register_instruction("fetch_network", fetch_network_handler)
        register_instruction("sync_robot_id", wrapped_sync_robot_id)

        GL.info("Registered 2 instruction handlers")

    async def _heartbeat_loop(self) -> None:
        """Background task to send periodic heartbeat events."""
        GL.info("Starting heartbeat loop")

        try:
            while not self._shutdown:
                try:
                    if self.websocket:
                        await send_heartbeat(self.websocket)
                except Exception as exc:
                    GL.warning(f"Heartbeat failed: {exc}")

                # Wait 30 seconds between heartbeats
                await asyncio.sleep(30)

        except asyncio.CancelledError:
            GL.info("Heartbeat loop cancelled")
        except Exception as exc:
            GL.error(f"Heartbeat loop error: {exc}", exc_info=True)

    async def connect(self) -> None:
        """
        Connect to server WebSocket.

        Raises:
            Exception: If connection fails
        """
        # Identify and get robot ID
        self.robot_id = await self.identify()

        # Build WebSocket URL
        ws_url = f"{self.config.server.websocket}/ws/{self.robot_id}"
        GL.info(f"Connecting to WebSocket: {ws_url}")

        # Initialize event loop
        loop = get_loop()
        loop.start()

        # Register handlers
        self._register_handlers()

        # Create adaptor
        self.adaptor = MessageAdaptor()
        self.adaptor.register_instruction_handler("fetch_network", handle_instruction_message)
        self.adaptor.register_instruction_handler("sync_robot_id", handle_instruction_message)

        # Connect to WebSocket
        try:
            async with websockets.asyncio.client.connect(
                ws_url,
                logger=GL,
                ping_interval=20,
                ping_timeout=10,
            ) as websocket:
                self.websocket = websocket
                GL.info("WebSocket connection established")

                # Send initial heartbeat
                await send_heartbeat(websocket)

                # Start heartbeat loop
                self.heartbeat_task = asyncio.create_task(self._heartbeat_loop())

                # Start listening for messages
                try:
                    await self.adaptor.start_listening(websocket)
                except KeyboardInterrupt:
                    GL.info("Received keyboard interrupt, shutting down")
                except Exception as exc:
                    GL.error(f"Connection error: {exc}", exc_info=True)
                    raise

        except websockets.exceptions.WebSocketException as exc:
            GL.error(f"WebSocket error: {exc}", exc_info=True)
            raise
        except Exception as exc:
            GL.error(f"Connection failed: {exc}", exc_info=True)
            raise
        finally:
            await self.disconnect()

    async def disconnect(self) -> None:
        """Clean up and disconnect."""
        GL.info("Disconnecting...")

        self._shutdown = True

        # Cancel heartbeat task
        if self.heartbeat_task:
            self.heartbeat_task.cancel()
            with contextlib.suppress(asyncio.CancelledError):
                await self.heartbeat_task

        # Stop adaptor
        if self.adaptor:
            self.adaptor.stop()

        # Shutdown event loop
        shutdown()

        self.websocket = None
        GL.info("Disconnected")

    async def run(self) -> None:
        """
        Run client with reconnection logic.
        """
        retry_delay = 5

        while not self._shutdown:
            try:
                await self.connect()
            except KeyboardInterrupt:
                GL.info("Shutting down on keyboard interrupt")
                break
            except Exception as exc:
                GL.error(f"Connection lost: {exc}")
                GL.info(f"Reconnecting in {retry_delay} seconds...")
                await asyncio.sleep(retry_delay)
                retry_delay = min(retry_delay * 2, 60)  # Exponential backoff, max 60s
