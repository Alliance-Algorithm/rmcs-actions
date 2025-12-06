"""
Unified service module for managing protocol interactions.

This module provides:
- A generic event loop manager keyed by session ID
- Registration and execution of instruction handlers
- Bridging between the message adaptor and user-defined handlers
- Support for session-scoped async interactions
"""

from __future__ import annotations

import json
import time
import uuid

from collections.abc import Awaitable
from collections.abc import Callable
from typing import TYPE_CHECKING
from typing import Any

from rmcs_actions_client.lib.adaptor import MessageAdaptor
from rmcs_actions_client.lib.eventloop import ThreadSafeEventLoop
from rmcs_actions_client.log import GL


if TYPE_CHECKING:
    from rmcs_actions_client.lib.adaptor import Message
    from rmcs_actions_client.lib.session import SessionContext


# Type alias for instruction handlers
InstructionHandler = Callable[
    [dict[str, Any], Any],
    Awaitable[dict[str, Any] | None],
]

# Type alias for event handlers
EventHandler = Callable[[dict[str, Any]], Awaitable[None]]

# Global event loop (singleton)
_event_loop: ThreadSafeEventLoop | None = None

# Registry of instruction handlers
_instruction_handlers: dict[str, InstructionHandler] = {}

# Registry of event handlers
_event_handlers: dict[str, EventHandler] = {}


def get_loop() -> ThreadSafeEventLoop:
    """
    Get or create the global event loop.

    Returns:
        The global ThreadSafeEventLoop instance
    """
    global _event_loop

    if _event_loop is None:
        _event_loop = ThreadSafeEventLoop(name="EventLoop")
        _event_loop.start()
        GL.info("Event loop started")

    return _event_loop


def register_instruction(instruction: str, handler: InstructionHandler) -> None:
    """
    Register an instruction handler.

    Args:
        instruction: Instruction endpoint name
        handler: Async handler function
    """
    _instruction_handlers[instruction] = handler
    GL.info(f"Registered instruction handler: {instruction}")


def unregister_instruction(instruction: str) -> None:
    """
    Unregister an instruction handler.

    Args:
        instruction: Instruction endpoint name
    """
    if instruction in _instruction_handlers:
        del _instruction_handlers[instruction]
        GL.info(f"Unregistered instruction handler: {instruction}")


def register_event(event: str, handler: EventHandler) -> None:
    """
    Register an event handler.

    Args:
        event: Event name
        handler: Async handler function
    """
    _event_handlers[event] = handler
    GL.info(f"Registered event handler: {event}")


def unregister_event(event: str) -> None:
    """
    Unregister an event handler.

    Args:
        event: Event name
    """
    if event in _event_handlers:
        del _event_handlers[event]
        GL.info(f"Unregistered event handler: {event}")


async def handle_instruction_message(
    message: Message,
    session_ctx: SessionContext | None = None,
) -> dict[str, Any] | None:
    """
    Bridge instruction messages from the adaptor to registered handlers.

    Args:
        message: The instruction message
        session_ctx: Optional session context for multi-step interactions

    Returns:
        Response payload dict, or None if no response should be sent
    """
    instruction = message.payload.get("instruction")
    if not instruction:
        GL.warning("Instruction message missing 'instruction' field")
        return None

    handler = _instruction_handlers.get(instruction)

    if handler is None:
        GL.warning(f"No handler registered for instruction: {instruction}")
        return None

    payload = message.payload.get("message", {})

    try:
        result = await handler(payload, session_ctx)
        GL.debug(f"Instruction '{instruction}' processed successfully")
        return result
    except Exception as exc:
        GL.error(f"Error processing instruction '{instruction}': {exc}", exc_info=True)
        raise


async def handle_event_message(message: Message) -> None:
    """
    Bridge event messages from the adaptor to registered handlers.

    Args:
        message: The event message
    """
    event_name = message.payload.get("event")
    if not event_name:
        GL.warning("Event message missing 'event' field")
        return

    handler = _event_handlers.get(event_name)

    if handler is None:
        GL.debug(f"No handler registered for event: {event_name}")
        return

    payload = message.payload.get("detail", {})

    try:
        await handler(payload)
        GL.debug(f"Event '{event_name}' processed successfully")
    except Exception as exc:
        GL.error(f"Error processing event '{event_name}': {exc}", exc_info=True)


async def send_event(
    websocket: Any,
    event: str,
    detail: dict[str, Any],
    *,
    session_id: str | None = None,
) -> None:
    """
    Send an outbound event according to the protocol specification.

    Args:
        websocket: WebSocket connection
        event: Event name
        detail: Event detail payload
        session_id: Optional session ID (auto-generated if not provided)
    """
    envelope = MessageAdaptor.build_event_envelope(event, detail, session_id=session_id)
    await websocket.send(envelope)
    GL.debug(f"Sent event '{event}' with session {json.loads(envelope).get('session_id')}")


def submit_event(
    websocket: Any,
    event: str,
    detail: dict[str, Any],
    *,
    session_id: str | None = None,
) -> None:
    """
    Submit an event from a sync context.

    Args:
        websocket: WebSocket connection
        event: Event name
        detail: Event detail payload
        session_id: Optional session ID
    """
    loop = get_loop()
    future = loop.submit(send_event(websocket, event, detail, session_id=session_id))

    try:
        future.result(timeout=5.0)
    except Exception as exc:
        GL.error(f"Failed to send event '{event}': {exc}", exc_info=True)


async def send_instruction(
    websocket: Any,
    instruction: str,
    message: dict[str, Any],
    *,
    session_id: str | None = None,
) -> None:
    """
    Send an outbound instruction according to the protocol specification.

    Args:
        websocket: WebSocket connection
        instruction: Instruction endpoint name
        message: Instruction message payload
        session_id: Optional session ID (auto-generated if not provided)
    """
    if session_id is None:
        session_id = str(uuid.uuid4())

    envelope = {
        "session_id": session_id,
        "local_timestamp": time.time(),
        "payload": {
            "type": "instruction",
            "instruction": instruction,
            "message": message,
        },
    }

    await websocket.send(json.dumps(envelope))
    GL.debug(f"Sent instruction '{instruction}' with session {session_id}")


def submit_instruction(
    websocket: Any,
    instruction: str,
    message: dict[str, Any],
    *,
    session_id: str | None = None,
) -> None:
    """
    Submit an instruction from a sync context.

    Args:
        websocket: WebSocket connection
        instruction: Instruction endpoint name
        message: Instruction message payload
        session_id: Optional session ID
    """
    loop = get_loop()
    future = loop.submit(
        send_instruction(websocket, instruction, message, session_id=session_id)
    )

    try:
        future.result(timeout=5.0)
    except Exception as exc:
        GL.error(f"Failed to send instruction '{instruction}': {exc}", exc_info=True)


def submit_task(coro: Any) -> Any:
    """
    Submit a coroutine to the event loop from a sync context.

    Args:
        coro: The coroutine to execute

    Returns:
        A Future object for result polling
    """
    loop = get_loop()
    return loop.submit(coro)


def shutdown() -> None:
    """Shutdown the global event loop."""
    global _event_loop

    if _event_loop is not None:
        _event_loop.stop()
        _event_loop = None
        GL.info("Event loop stopped")
