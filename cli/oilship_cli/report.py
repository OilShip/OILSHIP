"""Report generation — turns the engine output into shareable text."""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Iterable

from .risk import RiskAssessment, tier_for


@dataclass
class ReportRow:
    bridge: str
    score: int
    tier: str
    delta: int = 0
    note: str = ""

