"""
System-related instruction handlers.

Implements handlers for system management instructions like robot ID sync.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from rmcs_actions_client.log import GL

if TYPE_CHECKING:
    from rmcs_actions_client.lib.session import SessionContext
    from rmcs_actions_client.storage import RobotStorage


async def sync_robot_id_handler(
    payload: dict,
    session_ctx: SessionContext | None,
    storage: RobotStorage,
) -> dict | None:
    """
    Handle sync_robot_id instruction.

    Forces the robot to change its robot ID.

    Args:
        payload: Instruction payload containing {"id": "<new robot id>"}
        session_ctx: Session context (unused for this handler)
        storage: Robot storage instance

    Returns:
        None (no response required per protocol)
    """
    new_id = payload.get("id")

    if not new_id:
        GL.warning("sync_robot_id received without 'id' field")
        return None

    GL.info(f"Syncing robot ID to: {new_id}")

    try:
        storage.save_robot_id(new_id)
        GL.info(f"Robot ID successfully updated to: {new_id}")
    except Exception as exc:
        GL.error(f"Failed to save new robot ID: {exc}", exc_info=True)

    # Per protocol: NO RESPONSE
    return None
