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
