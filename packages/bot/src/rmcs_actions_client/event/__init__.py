"""
Action module for proactively sending protocol events.

This module manages a dedicated event loop for sending outbound messages
that follow ``docs/protocol.md``.
"""

from __future__ import annotations

import json

from typing import Any

from rmcs_actions_client.lib.adaptor import MessageAdaptor
from rmcs_actions_client.lib.eventloop import ThreadSafeEventLoop
from rmcs_actions_client.log import GL


# Global action event loop
_action_loop: ThreadSafeEventLoop | None = None


def get_action_loop() -> ThreadSafeEventLoop:
    """
    Get or create the action event loop.
    
    Returns:
        The global action event loop instance
    """
    global _action_loop
    
    if _action_loop is None:
        _action_loop = ThreadSafeEventLoop(name="ActionLoop")
        _action_loop.start()
        GL.info("Action event loop started")
    
    return _action_loop


async def send_event(websocket: Any, event: str, detail: dict[str, Any], *, session_id: str | None = None) -> None:
    """Send an outbound event according to the protocol specification."""
    envelope = MessageAdaptor.build_event_envelope(event, detail, session_id=session_id)
    await websocket.send(envelope)
    GL.debug(f"Sent event '{event}' with session {json.loads(envelope).get('session_id')}")


def submit_event(websocket: Any, event: str, detail: dict[str, Any], *, session_id: str | None = None) -> None:
    """Submit an event from a sync context using the action loop."""
    loop = get_action_loop()
    future = loop.submit(send_event(websocket, event, detail, session_id=session_id))

    try:
        future.result(timeout=5.0)
    except Exception as exc:
        GL.error(f"Failed to send event '{event}': {exc}", exc_info=True)


def shutdown_action_loop() -> None:
    """Shutdown the action event loop."""
    global _action_loop
    
    if _action_loop is not None:
        _action_loop.stop()
        _action_loop = None
        GL.info("Action event loop stopped")
