// Shared types for the OILSHIP SDK.

export type Pubkey = string & { readonly __brand: "Pubkey" };

export function pubkey(s: string): Pubkey {
  if (!/^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(s)) {
    throw new Error(`invalid base58 pubkey: ${s}`);
  }
  return s as Pubkey;
}

export type Lamports = bigint;

export const LAMPORTS_PER_SOL: Lamports = 1_000_000_000n;

export type Tier = "tier_1" | "tier_2" | "tier_3" | "quarantined";

export type PolicyState = "pending" | "active" | "settled" | "claimed" | "expired";

export type VesselClass = "coaster" | "tanker" | "capesize" | "dark_fleet";

export interface BridgeId {
  symbol: string;
  pubkey: Pubkey;
}

export interface Bridge {
  symbol: string;
  name: string;
  operator: Pubkey;
  riskScore: number;
  tier: Tier;
  routable: boolean;
  quarantined: boolean;
  lastUpdateSlot: bigint;
  openPolicies: number;
  openCoverage: Lamports;
  lifetimeTolls: Lamports;
  lifetimePayouts: Lamports;
  quarantineCount: number;
}

export interface Policy {
  pubkey: Pubkey;
  beneficiary: Pubkey;
  bridge: Pubkey;
  cargo: Lamports;
  tollPaid: Lamports;
  riskAtOpen: number;
  vesselClass: VesselClass;
  openedSlot: bigint;
  matureSlot: bigint;
  expiresSlot: bigint;
  state: PolicyState;
}

export interface WreckFundView {
  authority: Pubkey;
  balance: Lamports;
  openCoverage: Lamports;
  lifetimeDeposits: Lamports;
  lifetimePayouts: Lamports;
  payoutCount: bigint;
}

export interface TreasuryView {
  authority: Pubkey;
  balance: Lamports;
  lifetimeIn: Lamports;
  lifetimeOut: Lamports;
}

export interface GlobalConfigView {
  admin: Pubkey;
  oilMint: Pubkey;
  treasury: Pubkey;
  wreckFund: Pubkey;
  tollBps: number;
  fundSplitBps: number;
  buybackSplitBps: number;
  opsSplitBps: number;
  bridgesRegistered: number;
  policiesOpened: bigint;
  policiesSettled: bigint;
  wreckClaimsPaid: bigint;
  lifetimeTolls: Lamports;
  lifetimePayouts: Lamports;
  paused: boolean;
}

export interface EscortQuote {
  bridge: BridgeId;
  cargo: Lamports;
  baseToll: Lamports;
  riskAdjustedToll: Lamports;
  tier: Tier;
  riskScore: number;
  vesselClass: VesselClass;
  validUntilSlot: bigint;
}

export interface OpenPolicyResult {
  signature: string;
  policy: Pubkey;
  toll: Lamports;
}

export interface TxResult {
  signature: string;
}

export interface Anomaly {
  kind: string;
  severity: "info" | "low" | "medium" | "high" | "critical";
  message: string;
  capturedAt: number;
  source: string;
}

export interface RiskAssessment {
  bridge: BridgeId;
  score: number;
  tier: Tier;
  computedAt: number;
  factors: { name: string; contribution: number; note: string }[];
}

export interface ClientOptions {
  rpcUrl: string;
  programId: Pubkey;
  commitment?: "processed" | "confirmed" | "finalized";
  fetcher?: typeof fetch;
}

export function solToLamports(sol: number): Lamports {
  if (sol < 0) throw new RangeError("sol must be non-negative");
  return BigInt(Math.round(sol * 1e9));
}

export function lamportsToSol(l: Lamports): number {
  return Number(l) / 1e9;
}

export function classFromCargo(cargo: Lamports): VesselClass {
  if (cargo < LAMPORTS_PER_SOL) return "coaster";
  if (cargo < 50n * LAMPORTS_PER_SOL) return "tanker";
  if (cargo < 250n * LAMPORTS_PER_SOL) return "capesize";
  return "dark_fleet";
}
