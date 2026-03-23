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
