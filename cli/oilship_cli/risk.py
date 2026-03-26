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


def compute(bridge: str, anomalies: Iterable[Anomaly]) -> RiskAssessment:
    total = BASELINE_SCORE
    factors: list[RiskFactor] = [
        RiskFactor("baseline", BASELINE_SCORE, "every bridge starts at 18")
    ]
    caps: dict[str, int] = {}
    for a in anomalies:
        weight = ANOMALY_WEIGHTS.get(a.kind, 5)
        factor = SEVERITY_FACTOR.get(a.severity, 1.0)
        raw = round(weight * factor)
        cap_used = caps.get(a.kind, 0)
        allowed = max(0, PER_CATEGORY_CAP - cap_used)
        if allowed == 0:
            continue
        used = min(raw, allowed)
        caps[a.kind] = cap_used + used
        total += used
        factors.append(RiskFactor(a.kind, used, a.message))
    score = min(100, total)
    return RiskAssessment(
        bridge=bridge,
        score=score,
        tier=tier_for(score),
        factors=factors,
    )


def smooth(history: list[int], alpha: float = 0.4) -> int:
    if not history:
        return 0
    acc = float(history[0])
    for v in history[1:]:
        acc = alpha * v + (1 - alpha) * acc
    return max(0, min(100, round(acc)))


def toll_multiplier_bps(score: int) -> int:
    if score <= 20:
        return 9_500
    if score <= 40:
        return 10_000
    if score <= 60:
        return 11_500
    if score <= 80:
        return 13_500
    return 19_000


def is_sailable(score: int) -> bool:
    return tier_for(score) != "quarantined"
