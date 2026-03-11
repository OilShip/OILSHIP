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

export function formatPolicy(p: Policy): string {
  return [
    `policy ${fmtPubkey(p.pubkey)}`,
    `  beneficiary ${fmtPubkey(p.beneficiary)}`,
    `  bridge      ${fmtPubkey(p.bridge)}`,
    `  cargo       ${fmtSol(p.cargo)}`,
    `  toll paid   ${fmtSol(p.tollPaid)}`,
    `  risk @ open ${p.riskAtOpen}`,
    `  class       ${p.vesselClass}`,
    `  state       ${p.state}`,
    `  matures     slot ${p.matureSlot}`,
    `  expires     slot ${p.expiresSlot}`,
  ].join("\n");
}

export function formatWreckFund(v: WreckFundView): string {
  return [
    `Wreck Fund`,
    `  authority      ${fmtPubkey(v.authority)}`,
    `  reserve        ${fmtSol(v.balance)}`,
    `  open coverage  ${fmtSol(v.openCoverage)}`,
    `  lifetime in    ${fmtSol(v.lifetimeDeposits)}`,
    `  lifetime out   ${fmtSol(v.lifetimePayouts)}`,
    `  payouts        ${v.payoutCount}`,
  ].join("\n");
}

export function formatQuote(q: EscortQuote): string {
  return [
    `OILSHIP escort quote`,
    `  bridge        ${q.bridge.symbol}`,
    `  cargo         ${fmtSol(q.cargo)}`,
    `  base toll     ${fmtSol(q.baseToll)}`,
    `  risk-adjusted ${fmtSol(q.riskAdjustedToll)}`,
    `  effective bps ${effectiveBps(q.cargo, q.riskAdjustedToll)}`,
    `  tier          ${TIER_LABEL[q.tier]}`,
    `  risk score    ${q.riskScore} / 100`,
    `  vessel class  ${q.vesselClass}`,
  ].join("\n");
}
