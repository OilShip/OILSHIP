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


def test_per_category_cap():
    a = make("AdminKeyRotation", "critical")
    spam = [a] * 20
    r = compute("mayan", spam)
    assert r.score <= 58


def test_quarantine_threshold():
    a = make("AdminKeyRotation", "critical")
    b = make("SignerCollusion", "critical")
    c = make("TvlDrop", "critical")
    r = compute("mayan", [a] * 5 + [b] * 5 + [c] * 5)
    assert r.tier == "quarantined"
    assert r.score == 100


def test_tier_for_boundaries():
    assert tier_for(0) == "tier_1"
    assert tier_for(30) == "tier_1"
    assert tier_for(31) == "tier_2"
    assert tier_for(55) == "tier_2"
    assert tier_for(56) == "tier_3"
    assert tier_for(80) == "tier_3"
    assert tier_for(81) == "quarantined"
    assert tier_for(100) == "quarantined"


def test_smooth_empty():
    assert smooth([]) == 0


def test_smooth_single():
    assert smooth([42]) == 42


def test_smooth_trends_toward_last():
    s = smooth([10, 12, 14, 50])
    assert 14 < s <= 50


def test_smooth_clamps():
    s = smooth([200, 200, 200])
    assert s <= 100


def test_toll_multiplier_buckets():
    assert toll_multiplier_bps(10) == 9_500
    assert toll_multiplier_bps(30) == 10_000
    assert toll_multiplier_bps(50) == 11_500
    assert toll_multiplier_bps(70) == 13_500
    assert toll_multiplier_bps(95) == 19_000


def test_is_sailable():
    assert is_sailable(0)
    assert is_sailable(80)
    assert not is_sailable(81)
    assert not is_sailable(100)


def test_compute_factors_include_each_anomaly():
    anomalies = [make("OracleDrift", "low"), make("TvlDrop", "medium")]
    r = compute("mayan", anomalies)
    names = {f.name for f in r.factors}
    assert "baseline" in names
    assert "OracleDrift" in names
    assert "TvlDrop" in names


def test_unknown_kind_uses_fallback_weight():
    r = compute("mayan", [make("UnknownThing", "medium")])
    assert r.score >= BASELINE_SCORE


def test_severity_factor_scales_contribution():
    low = compute("x", [make("TvlDrop", "low")]).score
    high = compute("x", [make("TvlDrop", "high")]).score
    assert high > low
