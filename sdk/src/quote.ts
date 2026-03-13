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

    const preferred = req.preferredBridge?.toLowerCase();
    candidates.sort((a, b) => {
      const aPref = preferred && a.symbol.toLowerCase() === preferred ? -1 : 0;
      const bPref = preferred && b.symbol.toLowerCase() === preferred ? -1 : 0;
      if (aPref !== bPref) return aPref - bPref;
      return a.riskScore - b.riskScore;
    });

    const winner = candidates[0];
    const baseToll = bpsOf(req.cargo, req.baseTollBps);
    const adjusted = applyRiskMultiplier(baseToll, winner.riskScore);

    return {
      bridge: bridgeIdFromSymbol(winner.symbol, pubkey(PLACEHOLDER_PROGRAM)),
      cargo: req.cargo,
      baseToll,
      riskAdjustedToll: adjusted,
      tier: tierFromScore(winner.riskScore),
      riskScore: winner.riskScore,
      vesselClass: classFromCargo(req.cargo),
      validUntilSlot: 0n,
    };
  }

  static breakdown(quote: EscortQuote): { label: string; value: string }[] {
    return [
      { label: "bridge",        value: quote.bridge.symbol },
      { label: "cargo",         value: `${quote.cargo} lamports` },
      { label: "base toll",     value: `${quote.baseToll} lamports` },
      { label: "risk-adjusted", value: `${quote.riskAdjustedToll} lamports` },
      { label: "tier",          value: quote.tier },
      { label: "risk score",    value: `${quote.riskScore} / 100` },
      { label: "vessel class",  value: quote.vesselClass },
    ];
  }
}
