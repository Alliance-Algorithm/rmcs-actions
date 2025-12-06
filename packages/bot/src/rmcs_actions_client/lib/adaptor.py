"""
WebSocket message adaptor that routes protocol-compliant messages.

The adaptor understands the envelope described in ``docs/protocol.md``:

{
    "session_id": <unique session id>,
    "local_timestamp": <UTC timestamp>,
    "payload": {
        "type": "instruction" | "response" | "event",
        ...
    }
}
"""

from __future__ import annotations

import json
import time
import uuid

from collections.abc import Awaitable
from collections.abc import Callable
from dataclasses import dataclass
from enum import Enum
from typing import Any

from rmcs_actions_client.lib.session import SessionManager
from rmcs_actions_client.log import GL


class MessageType(str, Enum):
    """Protocol message types."""

    INSTRUCTION = "instruction"
    RESPONSE = "response"
    EVENT = "event"


@dataclass
class Message:
    """Protocol envelope with parsed payload."""

    session_id: str | None
    local_timestamp: float | None
    payload: dict[str, Any]
    type: MessageType


InstructionHandler = Callable[
    [Message, Any],
    Awaitable[dict[str, Any] | None],
]
EventHandler = Callable[[Message], Awaitable[None]]
ResponseHandler = Callable[[Message], Awaitable[None]]


class MessageAdaptor:
    """Dispatch protocol messages to registered handlers."""

    def __init__(self) -> None:
        self._instruction_handlers: dict[str, InstructionHandler] = {}
        self._event_handlers: dict[str, EventHandler] = {}
        self._response_handlers: dict[str, ResponseHandler] = {}
        self._running = False
        self._websocket: Any | None = None
        self._session_manager: SessionManager | None = None

    def register_instruction_handler(self, instruction: str, handler: InstructionHandler) -> None:
        self._instruction_handlers[instruction] = handler
        GL.debug(f"Registered instruction handler: {instruction}")

    def register_event_handler(self, event: str, handler: EventHandler) -> None:
        self._event_handlers[event] = handler
        GL.debug(f"Registered event handler: {event}")

    def register_response_handler(self, session_id: str, handler: ResponseHandler) -> None:
        self._response_handlers[session_id] = handler
        GL.debug(f"Registered response handler for session: {session_id}")

    def unregister_instruction_handler(self, instruction: str) -> None:
        self._instruction_handlers.pop(instruction, None)

    def unregister_event_handler(self, event: str) -> None:
        self._event_handlers.pop(event, None)

    def unregister_response_handler(self, session_id: str) -> None:
        self._response_handlers.pop(session_id, None)

    async def dispatch(self, raw_message: str | bytes) -> None:
        """
        Dispatch a raw WebSocket message to appropriate handlers.
        
        Args:
            raw_message: Raw message from WebSocket (JSON string or bytes)
        """
        try:
            # Parse message
            if isinstance(raw_message, bytes):
                raw_message = raw_message.decode("utf-8")
            
            data = json.loads(raw_message)
            message = self._parse_message(data)

            if message.type == MessageType.INSTRUCTION:
                await self._dispatch_instruction(message)
            elif message.type == MessageType.EVENT:
                await self._dispatch_event(message)
            elif message.type == MessageType.RESPONSE:
                await self._dispatch_response(message)
            else:
                GL.warning(f"Unknown message type: {message.type}")

        except json.JSONDecodeError as e:
            GL.error(f"Failed to decode JSON message: {e}")
        except Exception as e:
            GL.error(f"Error dispatching message: {e}", exc_info=True)

    def _parse_message(self, data: dict[str, Any]) -> Message:
        payload = data.get("payload", {})
        raw_type = payload.get("type", MessageType.EVENT.value)
        try:
            msg_type = MessageType(raw_type)
        except ValueError:
            msg_type = MessageType.EVENT
            GL.warning(f"Unsupported message type '{raw_type}', defaulting to EVENT")

        return Message(
            session_id=data.get("session_id"),
            local_timestamp=data.get("local_timestamp"),
            payload=payload,
            type=msg_type,
        )

    async def _dispatch_instruction(self, message: Message) -> None:
        instruction = message.payload.get("instruction")
        if not instruction:
            GL.warning("Instruction message missing 'instruction' field")
            return

        handler = self._instruction_handlers.get(instruction)
        if handler is None:
            GL.debug(f"No handler registered for instruction: {instruction}")
            return

        try:
            session_ctx = None
            if self._session_manager and message.session_id:
                session_ctx = await self._session_manager.create_session(
                    message.session_id
                )
            response_payload = await handler(message, session_ctx)
            if response_payload is not None:
                await self._send_response(message.session_id, response_payload)
        except Exception as exc:
            GL.error(
                f"Error in instruction handler '{instruction}': {exc}",
                exc_info=True,
            )
        finally:
            if session_ctx:
                self._session_manager.close_session(message.session_id)

    async def _dispatch_event(self, message: Message) -> None:
        event_name = message.payload.get("event")
        if not event_name:
            GL.warning("Event message missing 'event' field")
            return

        handler = self._event_handlers.get(event_name)
        if handler is None:
            GL.debug(f"No handler registered for event: {event_name}")
            return

        try:
            await handler(message)
        except Exception as exc:
            GL.error(f"Error in event handler '{event_name}': {exc}", exc_info=True)

    async def _dispatch_response(self, message: Message) -> None:
        if not message.session_id:
            GL.debug("Response message missing session_id")
            return

        if self._session_manager:
            self._session_manager.deliver_response(message.session_id, message.payload)

        handler = self._response_handlers.get(message.session_id)
        if handler is None:
            GL.debug(f"No handler registered for response session: {message.session_id}")
            return

        try:
            await handler(message)
        except Exception as exc:
            GL.error(
                f"Error in response handler for session '{message.session_id}': {exc}",
                exc_info=True,
            )

    async def start_listening(self, websocket: Any) -> None:
        """
        Start listening for messages from a WebSocket connection.

        Args:
            websocket: WebSocket connection object
        """
        self._running = True
        self._websocket = websocket
        if not self._session_manager:
            self._session_manager = SessionManager(websocket)
        GL.info("Message adaptor started listening")

        try:
            async for message in websocket:
                if not self._running:
                    break
                await self.dispatch(message)
        except Exception as e:
            GL.error(f"Error in message listening loop: {e}", exc_info=True)
        finally:
            self._running = False
            self._websocket = None
            GL.info("Message adaptor stopped listening")

    def stop(self) -> None:
        """Stop listening for messages."""
        self._running = False

    @property
    def is_running(self) -> bool:
        """Check if the adaptor is currently listening."""
        return self._running

    async def _send_response(self, session_id: str | None, message: dict[str, Any]) -> None:
        if self._websocket is None or session_id is None:
            return

        envelope = {
            "session_id": session_id,
            "local_timestamp": time.time(),
            "payload": {
                "type": MessageType.RESPONSE.value,
                "message": message,
            },
        }

        try:
            await self._websocket.send(json.dumps(envelope))
            GL.debug(f"Sent response for session {session_id}")
        except Exception as exc:
            GL.error(f"Failed to send response for session {session_id}: {exc}", exc_info=True)

    @staticmethod
    def build_event_envelope(event: str, detail: dict[str, Any], *, session_id: str | None = None) -> str:
        payload = {
            "type": MessageType.EVENT.value,
            "event": event,
            "detail": detail,
        }

        envelope = {
            "session_id": session_id or str(uuid.uuid4()),
            "local_timestamp": time.time(),
            "payload": payload,
        }

        return json.dumps(envelope)
