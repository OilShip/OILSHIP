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
