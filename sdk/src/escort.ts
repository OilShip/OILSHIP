// Escort — the highest-level public API of the SDK.

import {
  Bridge,
  EscortQuote,
  Lamports,
  Pubkey,
  classFromCargo,
} from "./types.js";
import { OilshipClient } from "./client.js";
import { Bridges } from "./bridges.js";
import { Router, RouteCandidate } from "./routing.js";
import { Policies, OpenPolicyRequest } from "./policy.js";
import { WreckFund } from "./wreckFund.js";
import { ValidationError, QuarantinedError } from "./errors.js";
import { bpsOf, fmtSol, fmtBps } from "./utils.js";

export interface EscortOpenInput {
  beneficiary: Pubkey;
  cargo: Lamports;
  lifetimeHours: number;
  preferredBridge?: string;
  excludeBridges?: string[];
  maxRiskScore?: number;
}

export interface EscortQuoteInput {
  cargo: Lamports;
  preferredBridge?: string;
  excludeBridges?: string[];
  maxRiskScore?: number;
}

export interface PreparedOpen {
  request: OpenPolicyRequest;
  quote: EscortQuote;
  route: RouteCandidate;
  derivedPolicyAccount: Pubkey;
}
