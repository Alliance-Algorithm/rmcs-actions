"""
Robot identification utilities.

This module handles robot identification with the server.
"""

from __future__ import annotations

import json
import uuid
from getpass import getuser
from typing import Any

import aiohttp
import psutil

from rmcs_actions_client.log import GL


def get_mac_address() -> str:
    """
    Get primary MAC address of this machine.

    Returns:
        MAC address as string
    """
    try:
        # Get all network interfaces
        addrs = psutil.net_if_addrs()

        # Prefer non-loopback interfaces
        for interface_name, interface_addrs in addrs.items():
            if interface_name.startswith(("lo", "docker", "veth")):
                continue

            for addr in interface_addrs:
                if addr.family == psutil.AF_LINK:  # MAC address
                    mac = addr.address
                    if mac and mac != "00:00:00:00:00:00":
                        GL.debug(f"Found MAC address on {interface_name}: {mac}")
                        return mac

        # Fallback: use any MAC address
        for interface_name, interface_addrs in addrs.items():
            for addr in interface_addrs:
                if addr.family == psutil.AF_LINK:
                    mac = addr.address
                    if mac and mac != "00:00:00:00:00:00":
                        GL.warning(f"Using fallback MAC address from {interface_name}: {mac}")
                        return mac

        # Ultimate fallback: generate a pseudo-MAC
        fallback_mac = str(uuid.getnode())
        GL.warning(f"No valid MAC found, using node UUID: {fallback_mac}")
        return fallback_mac

    except Exception as exc:
        GL.error(f"Failed to get MAC address: {exc}", exc_info=True)
        # Generate a random identifier
        return str(uuid.uuid4())


def build_identification_payload() -> dict[str, Any]:
    """
    Build identification payload for whoami request.

    Returns:
        Dict containing identification information
    """
    mac_address = get_mac_address()

    return {
        "mac_address": mac_address,
        "hostname": psutil.os.uname().nodename if hasattr(psutil.os, "uname") else "unknown",
    }


def _normalize_base_url(server_url: str) -> str:
    """Convert ws/wss URLs to http/https for REST calls."""
    if server_url.startswith("ws://"):
        return "http://" + server_url[len("ws://") :]
    if server_url.startswith("wss://"):
        return "https://" + server_url[len("wss://") :]
    return server_url


async def request_robot_id(server_url: str) -> str:
    """
    Request robot_id from /ident/whoami as defined in protocol.md.

    Args:
        server_url: Base server URL (ws://, wss://, http://, or https://)

    Returns:
        Robot ID string

    Raises:
        Exception if the request fails or the response is invalid
    """

    base_http = _normalize_base_url(server_url).rstrip("/")
    url = f"{base_http}/ident/whoami"

    payload = {
        "username": getuser(),
        "mac": get_mac_address(),
    }

    GL.info(f"Requesting robot ID via POST {url}")

    try:
        timeout = aiohttp.ClientTimeout(total=10)
        async with aiohttp.ClientSession(timeout=timeout) as session:
            async with session.post(url, json=payload) as resp:
                if resp.status != 200:
                    raise RuntimeError(f"whoami returned status {resp.status}")

                try:
                    parsed = await resp.json()
                except Exception as exc:  # pragma: no cover - defensive
                    raise RuntimeError("Invalid JSON from whoami") from exc

                robot_id = parsed.get("robot_id")
                if not robot_id:
                    raise RuntimeError("Missing 'robot_id' in whoami response")

                return robot_id

    except Exception as exc:  # pragma: no cover - network errors
        GL.error(f"Failed to request robot ID: {exc}", exc_info=True)
        raise
