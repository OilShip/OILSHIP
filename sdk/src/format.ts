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

function effectiveBps(cargo: Lamports, toll: Lamports): string {
  if (cargo === 0n) return "0.00%";
  const bps = Number((toll * 10_000n) / cargo);
  return fmtBps(bps);
}

export function formatNumber(n: bigint, locale = "en-US"): string {
  return n.toLocaleString(locale);
}

export function formatDuration(slots: bigint): string {
  const SLOT_MS = 400n;
  const totalMs = slots * SLOT_MS;
  const seconds = Number(totalMs / 1000n);
  const m = Math.floor(seconds / 60) % 60;
  const h = Math.floor(seconds / 3600) % 24;
  const d = Math.floor(seconds / 86400);
  const parts = [];
  if (d > 0) parts.push(`${d}d`);
  if (h > 0) parts.push(`${h}h`);
  if (m > 0) parts.push(`${m}m`);
  if (parts.length === 0) parts.push(`${seconds}s`);
  return parts.join(" ");
}

export function table(rows: Array<Record<string, string | number>>): string {
  if (rows.length === 0) return "";
  const headers = Object.keys(rows[0]);
  const widths = headers.map((h) =>
    Math.max(h.length, ...rows.map((r) => String(r[h]).length))
  );
  const fmt = (cells: string[]) =>
    cells.map((c, i) => c.padEnd(widths[i])).join("  ");
  const lines = [fmt(headers), fmt(widths.map((w) => "-".repeat(w)))];
  for (const r of rows) {
    lines.push(fmt(headers.map((h) => String(r[h]))));
  }
  return lines.join("\n");
}
