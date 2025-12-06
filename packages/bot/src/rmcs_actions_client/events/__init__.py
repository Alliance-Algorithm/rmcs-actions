"""
Protocol event emitters.

This module contains utilities for emitting protocol-defined events.
"""

from __future__ import annotations

from rmcs_actions_client.events.heartbeat import send_heartbeat

__all__ = [
    "send_heartbeat",
]
