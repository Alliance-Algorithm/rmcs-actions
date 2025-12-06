"""
Network-related instruction handlers.

Implements handlers for network information retrieval.
"""

from __future__ import annotations

import socket

from typing import TYPE_CHECKING
from typing import Any

import psutil

from rmcs_actions_client.log import GL

if TYPE_CHECKING:
    from rmcs_actions_client.lib.session import SessionContext


async def fetch_network_handler(
    payload: dict,
    session_ctx: SessionContext | None,
) -> dict[str, Any]:
    """
    Handle fetch_network instruction.

    Collects and returns network interface information.

    Args:
        payload: Instruction payload (empty for this handler)
        session_ctx: Session context (unused for this handler)

    Returns:
        Dict mapping NIC names to their detailed information
    """
    GL.info("Fetching network information")

    result: dict[str, Any] = {}

    try:
        # Get network interface stats
        net_stats = psutil.net_if_stats()
        # Get network IO counters
        net_io = psutil.net_io_counters(pernic=True)
        # Get network addresses
        net_addrs = psutil.net_if_addrs()

        for nic_name in net_addrs:
            # Skip loopback and virtual interfaces in some cases
            # (but include them for completeness per protocol)
            nic_info: dict[str, Any] = {
                "nic": nic_name,
                "stats": _get_interface_stats(nic_name, net_stats),
                "io": _get_interface_io(nic_name, net_io),
                "addrs": _get_interface_addresses(nic_name, net_addrs),
            }

            result[nic_name] = nic_info

        GL.info(f"Successfully fetched info for {len(result)} network interfaces")
        return result

    except Exception as exc:
        GL.error(f"Failed to fetch network info: {exc}", exc_info=True)
        return {"error": str(exc)}


def _get_interface_stats(
    nic_name: str,
    net_stats: dict[str, Any],
) -> dict[str, Any]:
    """
    Get interface statistics.

    Args:
        nic_name: Network interface name
        net_stats: psutil net_if_stats dict

    Returns:
        Stats dict with speed, up status, mtu, duplex
    """
    if nic_name not in net_stats:
        return {
            "speed": 0.0,
            "up": False,
            "mtu": None,
            "duplex": None,
        }

    stats = net_stats[nic_name]

    # Map psutil duplex constants to protocol strings
    duplex_map = {
        psutil.NIC_DUPLEX_FULL: "full",
        psutil.NIC_DUPLEX_HALF: "half",
        psutil.NIC_DUPLEX_UNKNOWN: "unknown",
    }

    return {
        "speed": float(stats.speed) if stats.speed else 0.0,
        "up": stats.isup,
        "mtu": stats.mtu if hasattr(stats, "mtu") else None,
        "duplex": duplex_map.get(stats.duplex, "unknown") if hasattr(stats, "duplex") else None,
    }


def _get_interface_io(
    nic_name: str,
    net_io: dict[str, Any],
) -> dict[str, float]:
    """
    Get interface IO counters.

    Args:
        nic_name: Network interface name
        net_io: psutil net_io_counters dict

    Returns:
        IO dict with incoming/outgoing bytes and errors
    """
    if nic_name not in net_io:
        return {
            "incoming": 0.0,
            "outgoing": 0.0,
            "incoming_errs": 0.0,
            "outgoing_errs": 0.0,
            "incoming_drops": 0.0,
            "outgoing_drops": 0.0,
        }

    io = net_io[nic_name]

    return {
        "incoming": float(io.bytes_recv),
        "outgoing": float(io.bytes_sent),
        "incoming_errs": float(io.errin),
        "outgoing_errs": float(io.errout),
        "incoming_drops": float(io.dropin),
        "outgoing_drops": float(io.dropout),
    }


def _get_interface_addresses(
    nic_name: str,
    net_addrs: dict[str, list[Any]],
) -> dict[str, Any]:
    """
    Get interface addresses.

    Args:
        nic_name: Network interface name
        net_addrs: psutil net_if_addrs dict

    Returns:
        Addresses dict with ipv4, ipv6, mac, and unknown entries
    """
    addrs: dict[str, Any] = {
        "ipv4": None,
        "ipv6": None,
        "mac": None,
        "unknown": [],
    }

    if nic_name not in net_addrs:
        return addrs

    for addr in net_addrs[nic_name]:
        addr_info = {
            "address": addr.address,
            "netmask": addr.netmask,
            "broadcast": addr.broadcast,
            "family": addr.family,
        }

        if addr.family == socket.AF_INET:
            # IPv4
            if addrs["ipv4"] is None:
                addrs["ipv4"] = addr_info
        elif addr.family == socket.AF_INET6:
            # IPv6
            if addrs["ipv6"] is None:
                addrs["ipv6"] = addr_info
        elif addr.family == psutil.AF_LINK:
            # MAC address
            if addrs["mac"] is None:
                addrs["mac"] = addr_info
        else:
            # Unknown family
            addrs["unknown"].append(addr_info)

    return addrs
