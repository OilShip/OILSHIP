"""Tiny terminal dashboard renderer.

Pulls the latest fleet state and renders it as a single Rich panel
that re-paints on demand. Used by `oilship dashboard`.
"""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Iterable

from rich.console import Console
from rich.layout import Layout
from rich.panel import Panel
from rich.table import Table
from rich.text import Text


@dataclass
class DashboardRow:
    bridge: str
    score: int
    tier: str
    routable: bool
    open_policies: int
    open_coverage_sol: float
    last_update_iso: str


def make_layout() -> Layout:
    layout = Layout()
    layout.split_column(
        Layout(name="header", size=3),
        Layout(name="body"),
        Layout(name="footer", size=3),
    )
    return layout


def render_header() -> Panel:
    title = Text("OILSHIP STRAIT CONVOY", style="bold orange3")
    sub = Text(
        f"  bridge escort for solana  ·  {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')}",
        style="dim",
    )
    body = Text.assemble(title, "\n", sub)
    return Panel(body, border_style="orange3")


def render_footer(total_bridges: int, healthy: int, quarantined: int) -> Panel:
    text = Text.assemble(
        ("bridges: ", "dim"), (str(total_bridges), "bold white"),
        ("   healthy: ", "dim"), (str(healthy), "bold green"),
        ("   quarantined: ", "dim"), (str(quarantined), "bold red"),
    )
    return Panel(text, border_style="cyan")


def render_body(rows: Iterable[DashboardRow]) -> Panel:
    table = Table(expand=True, header_style="bold cyan")
    table.add_column("Bridge")
    table.add_column("Tier")
    table.add_column("Risk", justify="right")
    table.add_column("Open policies", justify="right")
    table.add_column("Open coverage", justify="right")
    table.add_column("Last update")
    for r in rows:
        tier_color = {
            "tier_1": "green",
            "tier_2": "cyan",
            "tier_3": "yellow",
            "quarantined": "red",
        }.get(r.tier, "white")
        table.add_row(
            r.bridge,
            Text(r.tier.upper().replace("_", " "), style=tier_color),
            f"{r.score}/100",
            str(r.open_policies),
            f"{r.open_coverage_sol:.2f} SOL",
            r.last_update_iso,
        )
    return Panel(table, border_style="cyan", title="fleet")


def render_dashboard(console: Console, rows: list[DashboardRow]) -> None:
    layout = make_layout()
    layout["header"].update(render_header())
    layout["body"].update(render_body(rows))
    healthy = sum(1 for r in rows if r.tier in ("tier_1", "tier_2"))
    quarantined = sum(1 for r in rows if r.tier == "quarantined")
    layout["footer"].update(render_footer(len(rows), healthy, quarantined))
    console.print(layout)


def stub_rows() -> list[DashboardRow]:
    """Pre-launch placeholder rows for the dashboard."""
    now = datetime.now(timezone.utc).strftime("%H:%M:%S")
    return [
        DashboardRow("mayan", 12, "tier_1", True, 0, 0.0, now),
        DashboardRow("debridge", 22, "tier_1", True, 0, 0.0, now),
        DashboardRow("wormhole", 48, "tier_2", True, 0, 0.0, now),
        DashboardRow("allbridge", 71, "tier_3", True, 0, 0.0, now),
    ]
