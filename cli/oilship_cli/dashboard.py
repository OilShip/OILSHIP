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
