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
