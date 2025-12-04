from __future__ import annotations

from rmcs_actions_client.log import GL


async def heartbeat_task() -> None:
    """A simple heartbeat task that logs a heartbeat message."""
    GL.info("Heartbeat: Daemon is alive.")
    