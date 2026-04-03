// Offline simulator — runs the OILSHIP economics through synthetic
// inputs. Used for fee previews, what-if analysis and tests.

import {
  Bridge,
  EscortQuote,
  Lamports,
  classFromCargo,
  pubkey,
} from "./types.js";
import { ValidationError } from "./errors.js";
import { applyRiskMultiplier, bpsOf, tierFromScore } from "./utils.js";
import { bridgeIdFromSymbol } from "./client.js";
import { computeRisk } from "./risk.js";

export interface ScenarioBridge {
  symbol: string;
  riskScore: number;
  quarantined?: boolean;
}

export interface Scenario {
  cargo: Lamports;
  baseTollBps: number;
  bridges: ScenarioBridge[];
  preferredBridge?: string;
}

export interface ScenarioResult {
  quote: EscortQuote;
  totalReserveImpact: Lamports;
  estimatedFundShare: Lamports;
  estimatedBuybackShare: Lamports;
  estimatedOpsShare: Lamports;
  vesselClass: string;
}

const FUND_SPLIT_BPS = 6_000;
const BUYBACK_SPLIT_BPS = 3_000;
const OPS_SPLIT_BPS = 1_000;
const PLACEHOLDER_PROGRAM = "11111111111111111111111111111111";

export class Simulator {
  static run(scenario: Scenario): ScenarioResult {
    if (scenario.cargo <= 0n) throw new ValidationError("cargo must be > 0");
    if (scenario.bridges.length === 0) throw new ValidationError("bridges must not be empty");
    const eligible = scenario.bridges.filter((b) => !b.quarantined && b.riskScore <= 80);
    if (eligible.length === 0) throw new ValidationError("no eligible bridges");

    const preferred = scenario.preferredBridge?.toLowerCase();
    eligible.sort((a, b) => {
      const aPref = preferred && a.symbol.toLowerCase() === preferred ? -1 : 0;
      const bPref = preferred && b.symbol.toLowerCase() === preferred ? -1 : 0;
      if (aPref !== bPref) return aPref - bPref;
      return a.riskScore - b.riskScore;
    });

    const winner = eligible[0];
    const baseToll = bpsOf(scenario.cargo, scenario.baseTollBps);
    const adjustedToll = applyRiskMultiplier(baseToll, winner.riskScore);

    const fundShare = bpsOf(adjustedToll, FUND_SPLIT_BPS);
    const buybackShare = bpsOf(adjustedToll, BUYBACK_SPLIT_BPS);
    const opsShare = bpsOf(adjustedToll, OPS_SPLIT_BPS);

    const quote: EscortQuote = {
      bridge: bridgeIdFromSymbol(winner.symbol, pubkey(PLACEHOLDER_PROGRAM)),
      cargo: scenario.cargo,
      baseToll,
      riskAdjustedToll: adjustedToll,
      tier: tierFromScore(winner.riskScore),
      riskScore: winner.riskScore,
      vesselClass: classFromCargo(scenario.cargo),
      validUntilSlot: 0n,
    };

    return {
      quote,
      totalReserveImpact: scenario.cargo,
      estimatedFundShare: fundShare,
      estimatedBuybackShare: buybackShare,
      estimatedOpsShare: opsShare,
      vesselClass: classFromCargo(scenario.cargo),
    };
  }

  static stressGrid(cargos: Lamports[], scores: number[]): ScenarioResult[] {
    const out: ScenarioResult[] = [];
    for (const cargo of cargos) {
      for (const score of scores) {
        const r = Simulator.run({
          cargo,
          baseTollBps: 10,
          bridges: [{ symbol: "synthetic", riskScore: score }],
        });
        out.push(r);
      }
    }
    return out;
  }

  static toCsv(results: ScenarioResult[]): string {
    const header = "bridge,cargo,base_toll,adjusted_toll,fund_share,buyback_share,ops_share,risk,tier,class";
    const lines = [header];
    for (const r of results) {
      const q = r.quote;
      lines.push(
        [
          q.bridge.symbol,
          q.cargo.toString(),
          q.baseToll.toString(),
          q.riskAdjustedToll.toString(),
          r.estimatedFundShare.toString(),
          r.estimatedBuybackShare.toString(),
          r.estimatedOpsShare.toString(),
          q.riskScore,
          q.tier,
          r.vesselClass,
        ].join(","),
      );
    }
    return lines.join("\n");
  }
}

export function fakeBridges(): Bridge[] {
  const PK = pubkey("11111111111111111111111111111111");
  const make = (symbol: string, score: number): Bridge => ({
    symbol,
    name: symbol,
    operator: PK,
    riskScore: score,
    tier: tierFromScore(score),
    routable: true,
    quarantined: false,
    lastUpdateSlot: 0n,
    openPolicies: 0,
    openCoverage: 0n,
    lifetimeTolls: 0n,
    lifetimePayouts: 0n,
    quarantineCount: 0,
  });
  return [
    make("mayan", 12),
    make("debridge", 22),
    make("wormhole", 48),
    make("allbridge", 71),
  ];
}
