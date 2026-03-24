"""Output formatting helpers — Rich tables and colour scheme."""

from __future__ import annotations

from typing import Iterable

from rich.console import Console
from rich.table import Table
from rich.text import Text


SEVERITY_COLORS = {
    "info": "white",
    "low": "cyan",
    "medium": "yellow",
    "high": "orange3",
    "critical": "red",
}

TIER_COLORS = {
    "tier_1": "green",
    "tier_2": "cyan",
    "tier_3": "yellow",
    "quarantined": "red",
}


def make_console(color: bool = True) -> Console:
    return Console(color_system="auto" if color else None, highlight=False)


def fmt_sol(lamports: int, decimals: int = 4) -> str:
    sol = lamports / 1e9
    return f"{sol:,.{decimals}f} SOL"


def fmt_pct(bps: int) -> str:
    return f"{bps/100:.2f}%"


def fmt_pubkey(pk: str, edge: int = 4) -> str:
    if not pk or len(pk) <= edge * 2 + 2:
        return pk
    return f"{pk[:edge]}…{pk[-edge:]}"


def fmt_score(score: int) -> Text:
    color = "green"
    if score > 80:
        color = "red"
    elif score > 55:
        color = "yellow"
    elif score > 30:
        color = "cyan"
    return Text(f"{score}/100", style=color)


def render_bridge_table(rows: Iterable[dict]) -> Table:
    table = Table(title="OILSHIP fleet status", header_style="bold cyan")
    table.add_column("Bridge", style="bold white")
    table.add_column("Tier")
    table.add_column("Risk")
    table.add_column("Open policies", justify="right")
    table.add_column("Open coverage", justify="right")
    table.add_column("Lifetime tolls", justify="right")
    table.add_column("Status")
    for r in rows:
        tier = r["tier"]
        tier_text = Text(tier.upper().replace("_", " "), style=TIER_COLORS.get(tier, "white"))
        status = "ROUTABLE" if r["routable"] else "BLOCKED"
        status_text = Text(status, style="green" if r["routable"] else "red")
        table.add_row(
            r["symbol"],
            tier_text,
            fmt_score(r["risk_score"]),
            str(r["open_policies"]),
            fmt_sol(r["open_coverage"]),
            fmt_sol(r["lifetime_tolls"]),
            status_text,
        )
    return table


def render_anomalies(anomalies: Iterable[dict]) -> Table:
    table = Table(title="anomalies", header_style="bold cyan")
    table.add_column("Severity")
    table.add_column("Kind")
    table.add_column("Source")
    table.add_column("Message")
    for a in anomalies:
        sev = a["severity"]
        sev_text = Text(sev.upper(), style=SEVERITY_COLORS.get(sev, "white"))
        table.add_row(sev_text, a["kind"], a["source"], a["message"])
    return table


def render_pnl(view: dict) -> Table:
    table = Table(title="OILSHIP P&L", header_style="bold cyan")
    table.add_column("Metric", style="bold white")
    table.add_column("Value", justify="right")
    table.add_row("Wreck Fund balance", fmt_sol(view["fund_balance"]))
    table.add_row("Open coverage", fmt_sol(view["open_coverage"]))
    table.add_row("Reserve ratio", f"{view['reserve_ratio_bps']/100:.2f}%")
    table.add_row("Lifetime tolls", fmt_sol(view["lifetime_tolls"]))
    table.add_row("Lifetime payouts", fmt_sol(view["lifetime_payouts"]))
    table.add_row("Wreck claims paid", str(view["wreck_claims_paid"]))
    table.add_row("Bridges registered", str(view["bridges_registered"]))
    return table


def banner() -> str:
    return r"""
    ____  ____ __    _____ __  ____ ____
   / __ \/  _// /   / ___// / / /  _/ __ \
  / / / // / / /    \__ \/ /_/ // // /_/ /
 / /_/ // / / /___ ___/ / __  // // ____/
 \____/___//_____//____/_/ /_/___/_/

 STRAIT CONVOY  ::  bridge escort for solana
"""
