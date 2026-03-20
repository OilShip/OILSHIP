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


def cmd_status(cfg: CliConfig) -> None:
    console = make_console(cfg.color)
    console.print(banner(), style="bold cyan")
    with OilshipRpc(cfg.rpc_url) as rpc:
        try:
            slot = rpc.get_slot()
            health = rpc.health()
        except Exception as exc:
            console.print(f"[red]rpc error:[/red] {exc}")
            raise SystemExit(2)
        console.print(
            Panel.fit(
                f"slot     : {slot}\nhealth   : {health}\nrpc      : {cfg.rpc_url}\nprogram  : {fmt_pubkey(cfg.program_id)}",
                title="oilship status",
                border_style="cyan",
            )
        )
        rows = _bridges_view(rpc, cfg)
        console.print(render_bridge_table(rows))


def cmd_fleet(cfg: CliConfig, json_output: bool) -> None:
    console = make_console(cfg.color)
    with OilshipRpc(cfg.rpc_url) as rpc:
        rows = _bridges_view(rpc, cfg)
    if json_output or cfg.json_output:
        print(json.dumps(rows, indent=2))
        return
    console.print(render_bridge_table(rows))


def cmd_quote(cfg: CliConfig, sol: float, bridge: str | None, max_risk: int | None) -> None:
    console = make_console(cfg.color)
    if sol <= 0:
        console.print("[red]cargo must be > 0[/red]")
        raise SystemExit(2)
    cargo = sol_to_lamports(sol)
    base_toll_bps = 10
    fake_score = 35
    mult = toll_multiplier_bps(fake_score)
    base_toll = (cargo * base_toll_bps) // 10_000
    risk_toll = (base_toll * mult) // 10_000
    bridge = bridge or cfg.default_bridge
    console.print(
        Panel.fit(
            "\n".join(
                [
                    f"bridge        : {bridge}",
                    f"cargo         : {fmt_sol(cargo)}",
                    f"base toll     : {fmt_sol(base_toll)}  ({base_toll_bps/100:.2f}%)",
                    f"risk-adjusted : {fmt_sol(risk_toll)}",
                    f"risk score    : {fake_score}/100",
                    f"tier          : {tier_for(fake_score)}",
                    f"max risk      : {max_risk if max_risk is not None else cfg.default_max_risk}",
                ]
            ),
            title="oilship quote",
            border_style="orange3",
        )
    )
