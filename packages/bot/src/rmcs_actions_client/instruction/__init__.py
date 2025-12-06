"""
Service module for receiving and processing protocol instructions.

This module manages the service event loop and bridges instruction messages
to user-defined handlers.
"""

from __future__ import annotations

from collections.abc import Awaitable
from collections.abc import Callable
from typing import TYPE_CHECKING
from typing import Any


if TYPE_CHECKING:
    from rmcs_actions_client.lib.adaptor import Message

from rmcs_actions_client.lib.eventloop import ThreadSafeEventLoop
from rmcs_actions_client.log import GL


# Type alias for instruction handlers
InstructionHandler = Callable[[dict[str, Any]], Awaitable[dict[str, Any] | None]]

# Global service event loop
_service_loop: ThreadSafeEventLoop | None = None

# Registry of instruction handlers
_instruction_handlers: dict[str, InstructionHandler] = {}


def get_service_loop() -> ThreadSafeEventLoop:
    """
    Get or create the service event loop.
    
    Returns:
        The global service event loop instance
    """
    global _service_loop
    
    if _service_loop is None:
        _service_loop = ThreadSafeEventLoop(name="ServiceLoop")
        _service_loop.start()
        GL.info("Service event loop started")
    
    return _service_loop


def register_instruction(instruction: str, handler: InstructionHandler) -> None:
    """Register an instruction handler keyed by instruction endpoint."""
    _instruction_handlers[instruction] = handler
    GL.info(f"Registered instruction handler: {instruction}")


def unregister_instruction(instruction: str) -> None:
    """Unregister an instruction handler."""
    if instruction in _instruction_handlers:
        del _instruction_handlers[instruction]
        GL.info(f"Unregistered instruction handler: {instruction}")


async def handle_instruction_message(message: Message) -> dict[str, Any] | None:
    """Bridge instruction messages from the adaptor to registered handlers."""
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
        result = await handler(payload)
        GL.debug(f"Instruction '{instruction}' processed successfully")
        return result
    except Exception as exc:
        GL.error(f"Error processing instruction '{instruction}': {exc}", exc_info=True)
        raise


def submit_instruction_task(instruction: str, data: dict[str, Any]) -> None:
    """Run an instruction handler from a sync context using the service loop."""
    handler = _instruction_handlers.get(instruction)

    if handler is None:
        GL.warning(f"No handler registered for instruction: {instruction}")
        return

    loop = get_service_loop()

    try:
        future = loop.submit(handler(data))
        future.result(timeout=30.0)
    except Exception as exc:
        GL.error(f"Failed to process instruction '{instruction}': {exc}", exc_info=True)


def shutdown_service_loop() -> None:
    """Shutdown the service event loop."""
    global _service_loop
    
    if _service_loop is not None:
        _service_loop.stop()
        _service_loop = None
        GL.info("Service event loop stopped")
