// Quote builder — the deterministic side of the SDK.

import {
  Bridge,
  EscortQuote,
  Lamports,
  classFromCargo,
} from "./types.js";
import { ValidationError } from "./errors.js";
import { bpsOf, tierFromScore, applyRiskMultiplier } from "./utils.js";
import { bridgeIdFromSymbol } from "./client.js";
import { pubkey } from "./types.js";

export interface QuoteRequest {
  cargo: Lamports;
  baseTollBps: number;
  preferredBridge?: string;
  excludeBridges?: string[];
  maxRiskScore?: number;
}

const PLACEHOLDER_PROGRAM = "11111111111111111111111111111111";

export class QuoteBuilder {
  static build(req: QuoteRequest, bridges: Bridge[]): EscortQuote {
    if (req.cargo <= 0n) throw new ValidationError("cargo must be > 0");
    if (req.baseTollBps < 1 || req.baseTollBps > 100) {
      throw new ValidationError("base toll out of range");
    }
    const exclude = new Set((req.excludeBridges ?? []).map((s) => s.toLowerCase()));
    const maxRisk = req.maxRiskScore ?? 80;

    const candidates = bridges.filter((b) => {
      if (b.quarantined || !b.routable) return false;
      if (b.riskScore > maxRisk) return false;
      if (exclude.has(b.symbol.toLowerCase())) return false;
      return true;
    });

    if (candidates.length === 0) {
      throw new ValidationError("no eligible bridges");
    }
