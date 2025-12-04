from __future__ import annotations

from devtools import debug


def test_ip():
    from services.ip import fetch_ip_details

    ip_details = fetch_ip_details()
    assert ip_details is not None
    assert isinstance(ip_details.detail, dict)

    debug(ip_details)
