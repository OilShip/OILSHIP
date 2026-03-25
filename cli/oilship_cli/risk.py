"""Local risk model — mirror of the watch engine."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Iterable


BASELINE_SCORE = 18
PER_CATEGORY_CAP = 40

ANOMALY_WEIGHTS: dict[str, int] = {
    "TvlDrop": 25,
    "AdminKeyRotation": 30,
    "SignerCollusion": 35,
    "OracleDrift": 12,
    "UnusualWithdrawal": 18,
    "PauseFlagSet": 8,
    "ContractUpgrade": 22,
    "GuardianOffline": 14,
    "PoolImbalance": 10,
    "SuspiciousMemo": 6,
}

SEVERITY_FACTOR: dict[str, float] = {
    "info": 0.25,
    "low": 0.5,
    "medium": 1.0,
    "high": 1.6,
    "critical": 2.4,
}


@dataclass
class Anomaly:
    kind: str
    severity: str
    message: str
    source: str = "manual"
    captured_at: int = 0


@dataclass
class RiskFactor:
    name: str
    contribution: int
    note: str


@dataclass
class RiskAssessment:
    bridge: str
    score: int
    tier: str
    factors: list[RiskFactor] = field(default_factory=list)


def tier_for(score: int) -> str:
    if score <= 30:
        return "tier_1"
    if score <= 55:
        return "tier_2"
    if score <= 80:
        return "tier_3"
    return "quarantined"
