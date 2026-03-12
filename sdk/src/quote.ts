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
