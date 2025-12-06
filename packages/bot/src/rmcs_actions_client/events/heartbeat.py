"""
Heartbeat event implementation.

This module implements the periodic heartbeat event per protocol.
"""

from __future__ import annotations

from typing import Any

from rmcs_actions_client.log import GL
from rmcs_actions_client.service import send_event


async def send_heartbeat(websocket: Any) -> None:
    """
    Send heartbeat event to server.

    Args:
        websocket: WebSocket connection
    """
    GL.debug("Sending heartbeat event")

    try:
        await send_event(
            websocket=websocket,
            event="heartbeat",
            detail={},
        )
    except Exception as exc:
        GL.error(f"Failed to send heartbeat: {exc}", exc_info=True)
        raise
