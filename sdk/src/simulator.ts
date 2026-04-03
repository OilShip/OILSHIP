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
