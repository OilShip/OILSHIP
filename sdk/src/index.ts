// Public re-exports for the OILSHIP TypeScript SDK.

export * from "./types.js";
export * from "./errors.js";
export * from "./utils.js";
export * from "./risk.js";
export { OilshipClient, bridgeIdFromSymbol } from "./client.js";
export { Bridges, KNOWN_BRIDGES, QUARANTINED } from "./bridges.js";
export { Router } from "./routing.js";
export type { RouteCandidate, RouteRequest } from "./routing.js";
export { Policies } from "./policy.js";
export type {
  OpenPolicyRequest,
  SettlePolicyRequest,
  ClaimPolicyRequest,
} from "./policy.js";
export { WreckFund } from "./wreckFund.js";
export type { DepositRequest, WreckHealth } from "./wreckFund.js";
export { Escort } from "./escort.js";
export type { EscortOpenInput, EscortQuoteInput, PreparedOpen } from "./escort.js";
export { QuoteBuilder } from "./quote.js";

export const VERSION = "0.1.0";
export const DEFAULT_TOLL_BPS = 10;
