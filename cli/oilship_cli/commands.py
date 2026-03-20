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


def cmd_open(cfg: CliConfig, sol: float, bridge: str | None, hours: int) -> None:
    console = make_console(cfg.color)
    if sol <= 0:
        console.print("[red]cargo must be > 0[/red]")
        raise SystemExit(2)
    if hours < 1 or hours > 48:
        console.print("[red]lifetime must be between 1 and 48 hours[/red]")
        raise SystemExit(2)
    bridge = bridge or cfg.default_bridge
    cargo = sol_to_lamports(sol)
    console.print(f"prepared open: cargo={fmt_sol(cargo)} bridge={bridge} lifetime={hours}h")
    console.print("[yellow]signing requires a wallet adapter — see SDK 'prepareOpen' for the binding.[/yellow]")


def cmd_policy_list(cfg: CliConfig, beneficiary: str | None) -> None:
    console = make_console(cfg.color)
    if not beneficiary:
        console.print("[red]--beneficiary required[/red]")
        raise SystemExit(2)
    with OilshipRpc(cfg.rpc_url) as rpc:
        accounts = rpc.get_program_accounts(cfg.program_id)
    console.print(f"found {len(accounts)} program-owned accounts; filtering not yet wired")


def cmd_fund(cfg: CliConfig) -> None:
    console = make_console(cfg.color)
    view = {
        "fund_balance": 0,
        "open_coverage": 0,
        "reserve_ratio_bps": 10_000,
        "lifetime_tolls": 0,
        "lifetime_payouts": 0,
        "wreck_claims_paid": 0,
        "bridges_registered": 4,
    }
    console.print(render_pnl(view))


def cmd_threat_simulate(cfg: CliConfig, file: Path) -> None:
    console = make_console(cfg.color)
    if not file.exists():
        console.print(f"[red]anomaly file not found: {file}[/red]")
        raise SystemExit(2)
    raw = json.loads(file.read_text(encoding="utf-8"))
    bridge = raw.get("bridge", "unknown")
    anomalies = [Anomaly(**a) for a in raw.get("anomalies", [])]
    assessment = compute_risk(bridge, anomalies)
    console.print(
        Panel.fit(
            "\n".join(
                [
                    f"bridge   : {assessment.bridge}",
                    f"score    : {assessment.score} / 100",
                    f"tier     : {assessment.tier}",
                    f"sailable : {is_sailable(assessment.score)}",
                ]
            ),
            title="risk simulation",
            border_style="cyan",
        )
    )
    console.print(render_anomalies([asdict(a) for a in anomalies]))


def cmd_threat_smooth(cfg: CliConfig, scores: list[int]) -> None:
    console = make_console(cfg.color)
    if not scores:
        console.print("[red]provide at least one score[/red]")
        raise SystemExit(2)
    smoothed = smooth(scores)
    console.print(f"input    : {scores}")
    console.print(f"smoothed : {smoothed}")
    console.print(f"tier     : {tier_for(smoothed)}")


def cmd_config_show(cfg: CliConfig) -> None:
    console = make_console(cfg.color)
    console.print_json(data=cfg.as_dict())


def cmd_config_save(cfg: CliConfig) -> None:
    console = make_console(cfg.color)
    cfg.save()
    console.print(f"[green]saved[/green]")
