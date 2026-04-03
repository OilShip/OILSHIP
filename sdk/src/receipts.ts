// Receipt builder — turns an executed escort into a serialisable record
// suitable for the user's transaction history.

import {
  EscortQuote,
  Lamports,
  Pubkey,
  Tier,
  VesselClass,
  pubkey,
} from "./types.js";
import { fmtSol, fmtBps } from "./utils.js";

export interface Receipt {
  version: number;
  generatedAt: number;
  policy: Pubkey;
  beneficiary: Pubkey;
  bridge: string;
  cargo: Lamports;
  baseToll: Lamports;
  riskAdjustedToll: Lamports;
  tier: Tier;
  riskScore: number;
  vesselClass: VesselClass;
  txSignature: string;
}

export interface ReceiptInput {
  policy: Pubkey;
  beneficiary: Pubkey;
  quote: EscortQuote;
  txSignature: string;
}

export function buildReceipt(input: ReceiptInput): Receipt {
  return {
    version: 1,
    generatedAt: Math.floor(Date.now() / 1000),
    policy: input.policy,
    beneficiary: input.beneficiary,
    bridge: input.quote.bridge.symbol,
    cargo: input.quote.cargo,
    baseToll: input.quote.baseToll,
    riskAdjustedToll: input.quote.riskAdjustedToll,
    tier: input.quote.tier,
    riskScore: input.quote.riskScore,
    vesselClass: input.quote.vesselClass,
    txSignature: input.txSignature,
  };
}

export function serialise(receipt: Receipt): string {
  return JSON.stringify(receipt, (_k, v) => (typeof v === "bigint" ? v.toString() : v), 2);
}

export function deserialise(raw: string): Receipt {
  const parsed = JSON.parse(raw);
  return {
    ...parsed,
    cargo: BigInt(parsed.cargo),
    baseToll: BigInt(parsed.baseToll),
    riskAdjustedToll: BigInt(parsed.riskAdjustedToll),
    policy: pubkey(parsed.policy),
    beneficiary: pubkey(parsed.beneficiary),
  };
}

export function renderReceipt(r: Receipt): string {
  return [
    `OILSHIP escort receipt`,
    `  policy        ${r.policy}`,
    `  beneficiary   ${r.beneficiary}`,
    `  bridge        ${r.bridge}`,
    `  cargo         ${fmtSol(r.cargo)}`,
    `  base toll     ${fmtSol(r.baseToll)}`,
    `  risk-adjusted ${fmtSol(r.riskAdjustedToll)}`,
    `  tier          ${r.tier}`,
    `  risk          ${r.riskScore} / 100`,
    `  class         ${r.vesselClass}`,
    `  signature     ${r.txSignature}`,
  ].join("\n");
}

export function effectiveBpsFromReceipt(r: Receipt): string {
  if (r.cargo === 0n) return "0.00%";
  return fmtBps(Number((r.riskAdjustedToll * 10_000n) / r.cargo));
}
