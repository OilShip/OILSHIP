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

