// Local replica of the watch engine's risk model.

import { Anomaly, RiskAssessment, BridgeId, Tier } from "./types.js";
import { tierFromScore } from "./utils.js";

const BASELINE_SCORE = 18;

const ANOMALY_WEIGHTS: Record<string, number> = {
  TvlDrop: 25,
  AdminKeyRotation: 30,
  SignerCollusion: 35,
  OracleDrift: 12,
  UnusualWithdrawal: 18,
  PauseFlagSet: 8,
  ContractUpgrade: 22,
  GuardianOffline: 14,
  PoolImbalance: 10,
  SuspiciousMemo: 6,
};

const SEVERITY_FACTOR: Record<string, number> = {
  info: 0.25,
  low: 0.5,
  medium: 1.0,
  high: 1.6,
  critical: 2.4,
};

const PER_CATEGORY_CAP = 40;

export function computeRisk(bridge: BridgeId, anomalies: Anomaly[]): RiskAssessment {
  let total = BASELINE_SCORE;
  const factors: { name: string; contribution: number; note: string }[] = [
    { name: "baseline", contribution: BASELINE_SCORE, note: "every bridge starts at 18" },
  ];
  const caps: Record<string, number> = {};

  for (const a of anomalies) {
    const weight = ANOMALY_WEIGHTS[a.kind] ?? 5;
    const factor = SEVERITY_FACTOR[a.severity] ?? 1.0;
    const raw = Math.round(weight * factor);
    const used = Math.min(raw, PER_CATEGORY_CAP - (caps[a.kind] ?? 0));
    if (used <= 0) continue;
    caps[a.kind] = (caps[a.kind] ?? 0) + used;
    total += used;
    factors.push({ name: a.kind, contribution: used, note: a.message });
  }

  const score = Math.min(100, total);
  return {
    bridge,
    score,
    tier: tierFromScore(score),
    computedAt: Math.floor(Date.now() / 1000),
    factors,
  };
}

export function smoothScores(history: number[], alpha = 0.4): number {
  if (history.length === 0) return 0;
  let acc = history[0];
  for (let i = 1; i < history.length; i++) {
    acc = alpha * history[i] + (1 - alpha) * acc;
  }
  return Math.round(Math.min(100, Math.max(0, acc)));
}

export function tollMultiplierBps(score: number): number {
  if (score <= 20) return 9_500;
  if (score <= 40) return 10_000;
  if (score <= 60) return 11_500;
  if (score <= 80) return 13_500;
  return 19_000;
}

export function isSailable(score: number): boolean {
  return tierFromScore(score) !== "quarantined";
}

export function tierLabel(tier: Tier): string {
  switch (tier) {
    case "tier_1": return "TIER 1 — clear";
    case "tier_2": return "TIER 2 — watch";
    case "tier_3": return "TIER 3 — elevated";
    case "quarantined": return "QUARANTINED";
  }
}
