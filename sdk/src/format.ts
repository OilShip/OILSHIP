// Display formatters used by the CLI and any consuming dApp.

import { Bridge, EscortQuote, Lamports, Policy, Tier, WreckFundView } from "./types.js";
import { fmtSol, fmtBps, fmtPubkey } from "./utils.js";

const TIER_LABEL: Record<Tier, string> = {
  tier_1: "TIER 1 — clear",
  tier_2: "TIER 2 — watch",
  tier_3: "TIER 3 — elevated",
  quarantined: "QUARANTINED",
};

export function formatBridge(b: Bridge): string {
  return [
    `${b.symbol.padEnd(12)} ${TIER_LABEL[b.tier].padEnd(20)}`,
    `  risk          ${b.riskScore} / 100`,
    `  routable      ${b.routable ? "yes" : "no"}`,
    `  open policies ${b.openPolicies}`,
    `  open coverage ${fmtSol(b.openCoverage)}`,
    `  lifetime toll ${fmtSol(b.lifetimeTolls)}`,
  ].join("\n");
}
