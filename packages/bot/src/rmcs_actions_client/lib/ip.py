from __future__ import annotations

import socket

from datetime import datetime
from enum import StrEnum
from enum import unique

import psutil

from pydantic import BaseModel
from pydantic import Field


class IpPack(BaseModel):
    local_time_stamp: datetime
    detail: dict[str, IpDetail]


class IpDetail(BaseModel):
    nic: str
    stats: IpStats | None = None
    io: IpIo | None = None
    addrs: IpAddrs | None = None


class IpStats(BaseModel):
    speed: float
    up: bool
    mtu: int | None = None
    duplex: str | None = None


@unique
class IpDuplex(StrEnum):
    FULL = "full"
    HALF = "half"
    UNKNOWN = "unknown"

    @classmethod
    def from_psutil(cls, duplex: int) -> IpDuplex:
        if duplex == psutil.NIC_DUPLEX_FULL:
            return cls.FULL
        elif duplex == psutil.NIC_DUPLEX_HALF:
            return cls.HALF
        else:
            return cls.UNKNOWN


class IpIo(BaseModel):
    incoming: float  # in bytes
    outgoing: float  # in bytes
    incoming_errs: float
    outgoing_errs: float
    incoming_drops: float
    outgoing_drops: float


class IpAddrs(BaseModel):
    ipv4: IpAddr | None = None
    ipv6: IpAddr | None = None
    mac: IpAddr | None = None

    unknown: list[IpAddr] = Field(default_factory=list)


class IpAddr(BaseModel):
    address: str
    netmask: str | None = None
    broadcast: str | None = None
    family: int | None = None

    @property
    def is_ipv4(self) -> bool:
        return self.family == socket.AF_INET

    @property
    def is_ipv6(self) -> bool:
        return self.family == socket.AF_INET6

    @property
    def is_mac(self) -> bool:
        return self.family == psutil.AF_LINK


def fetch_ip_details() -> IpPack:
    stats = psutil.net_if_stats()
    io_counters = psutil.net_io_counters(pernic=True)
    pack = IpPack(local_time_stamp=datetime.now(), detail={})

    for nic, addrs in psutil.net_if_addrs().items():
        detail = IpDetail(nic=nic)

        if nic in stats:
            st = stats[nic]
            detail.stats = IpStats(
                speed=st.speed, up=st.isup, mtu=st.mtu, duplex=IpDuplex.from_psutil(st.duplex)
            )

        if nic in io_counters:
            io = io_counters[nic]
            detail.io = IpIo(
                incoming=io.bytes_recv,
                outgoing=io.bytes_sent,
                incoming_errs=io.errin,
                outgoing_errs=io.errout,
                incoming_drops=io.dropin,
                outgoing_drops=io.dropout,
            )

        addr_detail = IpAddrs()
        for addr in addrs:
            addr_info = IpAddr(
                address=addr.address,
                netmask=addr.netmask,
                broadcast=addr.broadcast,
                family=addr.family,
            )
            if addr_info.is_ipv4:
                addr_detail.ipv4 = addr_info
            elif addr_info.is_ipv6:
                addr_detail.ipv6 = addr_info
            elif addr_info.is_mac:
                addr_detail.mac = addr_info
            else:
                addr_detail.unknown.append(addr_info)

        detail.addrs = addr_detail
        pack.detail[nic] = detail

    return pack
