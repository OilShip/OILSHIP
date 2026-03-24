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
