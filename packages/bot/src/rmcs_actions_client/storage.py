"""
Storage utilities for persistent robot state.

This module manages local storage of robot identification and state data.
"""

from __future__ import annotations

from pathlib import Path

from rmcs_actions_client.log import GL


class RobotStorage:
    """Manages robot persistent storage."""

    def __init__(self, storage_dir: Path) -> None:
        """
        Initialize robot storage.

        Args:
            storage_dir: Base directory for storage
        """
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.robot_id_file = self.storage_dir / "robot_id"

    def get_robot_id(self) -> str | None:
        """
        Get cached robot ID from storage.

        Returns:
            Robot ID string if exists, None otherwise
        """
        if not self.robot_id_file.exists():
            GL.info("No cached robot ID found")
            return None

        try:
            robot_id = self.robot_id_file.read_text(encoding="utf-8").strip()
            if robot_id:
                GL.info(f"Loaded robot ID from storage: {robot_id}")
                return robot_id
            return None
        except Exception as exc:
            GL.error(f"Failed to read robot ID: {exc}", exc_info=True)
            return None

    def save_robot_id(self, robot_id: str) -> None:
        """
        Save robot ID to storage.

        Args:
            robot_id: Robot ID to persist
        """
        try:
            self.robot_id_file.write_text(robot_id, encoding="utf-8")
            GL.info(f"Saved robot ID to storage: {robot_id}")
        except Exception as exc:
            GL.error(f"Failed to save robot ID: {exc}", exc_info=True)
            raise

    def clear_robot_id(self) -> None:
        """Remove cached robot ID."""
        if self.robot_id_file.exists():
            self.robot_id_file.unlink()
            GL.info("Cleared robot ID from storage")
