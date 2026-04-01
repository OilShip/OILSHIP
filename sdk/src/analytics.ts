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
