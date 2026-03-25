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


@dataclass
class DailyReport:
    generated_at: str
    rows: list[ReportRow] = field(default_factory=list)
    headline: str = ""
    healthy_count: int = 0
    elevated_count: int = 0
    quarantined_count: int = 0


def build_daily_report(assessments: Iterable[RiskAssessment]) -> DailyReport:
    rows: list[ReportRow] = []
    healthy = elevated = quarantined = 0
    for a in assessments:
        if a.tier in ("tier_1", "tier_2"):
            healthy += 1
        elif a.tier == "tier_3":
            elevated += 1
        else:
            quarantined += 1
        rows.append(ReportRow(bridge=a.bridge, score=a.score, tier=a.tier))
    headline = (
        "calm seas across the strait"
        if quarantined == 0 and elevated == 0
        else "watch the strait"
    )
    if quarantined > 0:
        headline = "warning: bridges quarantined"
    return DailyReport(
        generated_at=datetime.now(timezone.utc).isoformat(timespec="seconds"),
        rows=rows,
        headline=headline,
        healthy_count=healthy,
        elevated_count=elevated,
        quarantined_count=quarantined,
    )


def render_text(report: DailyReport) -> str:
    out = [
        f"OILSHIP daily report — {report.generated_at}",
        f"  headline    : {report.headline}",
        f"  healthy     : {report.healthy_count}",
        f"  elevated    : {report.elevated_count}",
        f"  quarantined : {report.quarantined_count}",
        "",
        "  bridge          score   tier",
        "  ----------------------------",
    ]
    for r in report.rows:
        out.append(f"  {r.bridge:<14} {r.score:>5}   {r.tier}")
    return "\n".join(out)

