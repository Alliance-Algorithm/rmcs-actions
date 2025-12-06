"""
Session context manager for maintaining request-response interactions within a session.

This module provides a session abstraction that allows handlers to send outbound
messages and wait for corresponding responses, simulating a bidirectional
communication channel within a single protocol session.
"""

from __future__ import annotations

import asyncio
import json
import time

from typing import Any

from pydantic import BaseModel

from rmcs_actions_client.log import GL


class Message(BaseModel):
    """A message received within a session context."""

    type: str
    payload: dict[str, Any]
    local_timestamp: float | None = None


class SessionRecvQueue:
    """
    A queue-like interface that handlers can iterate over to receive messages
    within a session (similar to websocket iteration).
    """

    def __init__(self) -> None:
        self._queue: asyncio.Queue[Message] = asyncio.Queue()
        self._closed = False

    async def recv(self) -> Message:
        """Receive a message from the session queue."""
        if self._closed and self._queue.empty():
            raise EOFError("Session closed") from None

        try:
            return await self._queue.get()
        except asyncio.CancelledError as exc:
            raise EOFError("Session cancelled") from exc

    def __aiter__(self) -> SessionRecvQueue:
        """Support async for iteration."""
        return self

    async def __anext__(self) -> Message:
        """Get next message in async iteration."""
        try:
            return await self.recv()
        except EOFError as exc:
            raise StopAsyncIteration from exc

    async def _put(self, message: Message) -> None:
        """Internal: enqueue a message."""
        if not self._closed:
            await self._queue.put(message)

    def _close(self) -> None:
        """Internal: mark queue as closed."""
        self._closed = True

    @property
    def closed(self) -> bool:
        """Check if the session is closed."""
        return self._closed


class SessionContext:
    """
    Represents the bidirectional context of a single protocol session.

    Handlers can await on recv() to get response messages, or iterate over
    the queue like a websocket. The session is tied to a session_id and bridges
    between the adaptor's response dispatch and the handler's receive loop.
    """

    def __init__(self, session_id: str, send_fn: Any) -> None:
        """
        Initialize a session context.

        Args:
            session_id: Unique session identifier
            send_fn: Async callable to send outbound messages on the websocket
                    (signature: send_fn(payload: dict, instruction: str | None))
        """
        self.session_id = session_id
        self._send_fn = send_fn
        self._recv_queue = SessionRecvQueue()

    async def send(
        self,
        instruction: str | None = None,
        payload: dict[str, Any] | None = None,
    ) -> None:
        """
        Send a message within this session.

        Args:
            instruction: Optional instruction name to include in the message
            payload: Message payload dict
        """
        if payload is None:
            payload = {}

        message_payload = {
            "type": "instruction",
            "instruction": instruction,
            "message": payload,
        }

        # Call the adaptor's send function to emit via websocket
        await self._send_fn(self.session_id, message_payload)

    async def recv(self) -> Message:
        """
        Receive a message within this session (blocks until available).

        Returns:
            A Message object with type, payload, and timestamp
        """
        return await self._recv_queue.recv()

    def __aiter__(self) -> SessionRecvQueue:
        """Support async for iteration over received messages."""
        return self._recv_queue

    async def __aenter__(self) -> SessionContext:
        """Context manager entry."""
        return self

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Context manager exit; closes the session."""
        self.close()

    def close(self) -> None:
        """Mark the session as closed."""
        self._recv_queue._close()

    def _deliver_message(self, msg: Message) -> None:
        """Internal: deliver a message to the handler's receive queue."""
        try:
            asyncio.get_running_loop()
            task = asyncio.create_task(self._recv_queue._put(msg))
            # Keep a weak reference to avoid garbage collection warnings
            task.add_done_callback(lambda _: None)
        except RuntimeError:
            pass


class SessionManager:
    """
    Manages active sessions and routes response messages to their contexts.

    Each instruction handler receives a SessionContext that it can use to await
    responses tied to the same session_id.
    """

    def __init__(self, websocket: Any) -> None:
        """
        Initialize the session manager.

        Args:
            websocket: The websocket connection object
        """
        self._websocket = websocket
        self._sessions: dict[str, SessionContext] = {}

    async def create_session(self, session_id: str) -> SessionContext:
        """
        Create or retrieve a session context.

        Args:
            session_id: Unique session identifier

        Returns:
            A SessionContext for this session_id
        """
        if session_id not in self._sessions:
            self._sessions[session_id] = SessionContext(
                session_id=session_id,
                send_fn=self._send_within_session,
            )

        return self._sessions[session_id]

    def get_session(self, session_id: str) -> SessionContext | None:
        """
        Retrieve an active session context.

        Args:
            session_id: Session identifier

        Returns:
            SessionContext if active, else None
        """
        return self._sessions.get(session_id)

    def close_session(self, session_id: str) -> None:
        """
        Close a session and remove it from the registry.

        Args:
            session_id: Session to close
        """
        if session_id in self._sessions:
            self._sessions[session_id].close()
            del self._sessions[session_id]

    async def _send_within_session(
        self,
        session_id: str,
        message_payload: dict[str, Any],
    ) -> None:
        """
        Internal: send a message within a session via the websocket.

        Args:
            session_id: Session identifier
            message_payload: Payload dict (instruction | response | event)
        """
        envelope = {
            "session_id": session_id,
            "local_timestamp": time.time(),
            "payload": message_payload,
        }

        try:
            await self._websocket.send(json.dumps(envelope))
        except Exception as exc:
            GL.error(f"Failed to send within session {session_id}: {exc}", exc_info=True)

    def deliver_response(self, session_id: str, payload: dict[str, Any]) -> None:
        """
        Route an incoming response to its session.

        Args:
            session_id: Session identifier
            payload: Response payload dict
        """
        session = self.get_session(session_id)
        if session:
            msg = Message(
                type=payload.get("type", "response"),
                payload=payload.get("message", {}),
                local_timestamp=None,
            )
            session._deliver_message(msg)

    def deliver_event(self, session_id: str, payload: dict[str, Any]) -> None:
        """
        Route an incoming event to its session.

        Args:
            session_id: Session identifier
            payload: Event payload dict
        """
        session = self.get_session(session_id)
        if session:
            msg = Message(
                type=payload.get("type", "event"),
                payload=payload.get("detail", {}),
                local_timestamp=payload.get("local_timestamp"),
            )
            session._deliver_message(msg)
