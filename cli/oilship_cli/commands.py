"""All CLI sub-commands."""

from __future__ import annotations

import json
from dataclasses import asdict
from pathlib import Path
from typing import Any

from rich.panel import Panel

from .client import OilshipRpc, sol_to_lamports
from .config import CliConfig
from .format import (
    banner,
    fmt_pubkey,
    fmt_sol,
    make_console,
    render_anomalies,
    render_bridge_table,
    render_pnl,
)
from .risk import (
    Anomaly,
    compute as compute_risk,
    is_sailable,
    smooth,
    tier_for,
    toll_multiplier_bps,
)


def _bridges_view(rpc: OilshipRpc, _cfg: CliConfig) -> list[dict[str, Any]]:
    catalogue = [
        ("mayan", "Mayan Finance"),
        ("debridge", "deBridge"),
        ("wormhole", "Wormhole Portal"),
        ("allbridge", "Allbridge Core"),
    ]
    rows: list[dict[str, Any]] = []
    for symbol, _name in catalogue:
        rows.append(
            {
                "symbol": symbol,
                "tier": "tier_2",
                "risk_score": 35,
                "open_policies": 0,
                "open_coverage": 0,
                "lifetime_tolls": 0,
                "routable": True,
            }
        )
    return rows
