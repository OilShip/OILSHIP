// Lightweight analytics — derive aggregate stats from a list of
// fixtures or live state.

import { Bridge, Lamports, Tier } from "./types.js";

export interface FleetSummary {
  totalBridges: number;
  byTier: Record<Tier, number>;
  averageRisk: number;
  totalOpenCoverage: bigint;
  totalLifetimeTolls: bigint;
  routableCount: number;
  quarantinedCount: number;
}

export function summarise(bridges: Bridge[]): FleetSummary {
  const byTier: Record<Tier, number> = {
    tier_1: 0,
    tier_2: 0,
    tier_3: 0,
    quarantined: 0,
  };
  let riskTotal = 0;
  let totalOpen: bigint = 0n;
  let totalTolls: bigint = 0n;
  let routable = 0;
  let quarantined = 0;
  for (const b of bridges) {
    byTier[b.tier]++;
    riskTotal += b.riskScore;
    totalOpen += b.openCoverage;
    totalTolls += b.lifetimeTolls;
    if (b.routable) routable++;
    if (b.quarantined) quarantined++;
  }
  return {
    totalBridges: bridges.length,
    byTier,
    averageRisk: bridges.length === 0 ? 0 : riskTotal / bridges.length,
    totalOpenCoverage: totalOpen,
    totalLifetimeTolls: totalTolls,
    routableCount: routable,
    quarantinedCount: quarantined,
  };
}

export function topByCoverage(bridges: Bridge[], n = 3): Bridge[] {
  return [...bridges]
    .sort((a, b) => (a.openCoverage > b.openCoverage ? -1 : 1))
    .slice(0, n);
}

export function safestBridge(bridges: Bridge[]): Bridge | null {
  const sailable = bridges.filter((b) => !b.quarantined && b.routable);
  if (sailable.length === 0) return null;
  return sailable.sort((a, b) => a.riskScore - b.riskScore)[0];
}

export function diversityScore(bridges: Bridge[]): number {
  const total = bridges.reduce((acc, b) => acc + b.openCoverage, 0n);
  if (total === 0n || bridges.length <= 1) return 0;
  let hhi = 0;
  for (const b of bridges) {
    const share = Number((b.openCoverage * 10_000n) / total) / 10_000;
    hhi += share * share;
  }
  // 1 - HHI gives a 0..1 diversity index.
  return Math.max(0, Math.min(1, 1 - hhi));
}
