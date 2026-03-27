"""Tests for the OILSHIP CLI risk module."""

from __future__ import annotations

from oilship_cli.risk import (
    Anomaly,
    BASELINE_SCORE,
    compute,
    is_sailable,
    smooth,
    tier_for,
    toll_multiplier_bps,
)


def make(kind: str, severity: str, message: str = "test") -> Anomaly:
    return Anomaly(kind=kind, severity=severity, message=message)


def test_baseline_only():
    r = compute("mayan", [])
    assert r.score == BASELINE_SCORE
    assert r.tier == "tier_1"
    assert r.factors[0].name == "baseline"


def test_low_severity_increases_modestly():
    r = compute("mayan", [make("OracleDrift", "low")])
    assert r.score > BASELINE_SCORE
    assert r.score < 40


def test_critical_event_pushes_into_higher_tier():
    r = compute("mayan", [make("AdminKeyRotation", "critical")])
    assert r.score > 50

