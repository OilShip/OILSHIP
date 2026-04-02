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
