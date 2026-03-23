"""Export helpers — turn engine output into shareable artefacts."""

from __future__ import annotations

import csv
import io
import json
from dataclasses import asdict
from pathlib import Path
from typing import Iterable

from .risk import RiskAssessment


def to_json(items: Iterable[RiskAssessment]) -> str:
    payload = [asdict(it) for it in items]
    return json.dumps(payload, indent=2, default=str)


def to_csv(items: Iterable[RiskAssessment]) -> str:
    buffer = io.StringIO()
    writer = csv.writer(buffer)
    writer.writerow(["bridge", "score", "tier", "factors"])
    for r in items:
        factor_summary = ";".join(f"{f.name}={f.contribution}" for f in r.factors)
        writer.writerow([r.bridge, r.score, r.tier, factor_summary])
    return buffer.getvalue()


def to_markdown(items: Iterable[RiskAssessment]) -> str:
    rows = list(items)
    lines = ["# OILSHIP risk export", ""]
    lines.append("| bridge | score | tier | factors |")
    lines.append("|---|---:|---|---|")
    for r in rows:
        factor_summary = ", ".join(f"{f.name} ({f.contribution})" for f in r.factors)
        lines.append(f"| `{r.bridge}` | {r.score} | {r.tier} | {factor_summary} |")
    return "\n".join(lines)


def write_file(content: str, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def export_all(items: list[RiskAssessment], directory: Path) -> dict[str, Path]:
    out = {
        "json": directory / "oilship.json",
        "csv": directory / "oilship.csv",
        "md": directory / "oilship.md",
    }
    write_file(to_json(items), out["json"])
    write_file(to_csv(items), out["csv"])
    write_file(to_markdown(items), out["md"])
    return out


def round_trip_json(items: list[RiskAssessment]) -> list[dict]:
    raw = to_json(items)
    return json.loads(raw)


def histogram(items: list[RiskAssessment], bins: int = 5) -> dict[str, int]:
    counts: dict[str, int] = {}
    if not items:
        return counts
    width = max(1, 100 // bins)
    for r in items:
        bucket_lo = (r.score // width) * width
        bucket_hi = bucket_lo + width - 1
        key = f"{bucket_lo:>3}-{bucket_hi:<3}"
        counts[key] = counts.get(key, 0) + 1
    return dict(sorted(counts.items()))
