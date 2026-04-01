// Event subscription helpers for the OILSHIP SDK.

import { Bridge, Policy, Pubkey, Tier } from "./types.js";

export interface OilshipEvent {
  kind: EventKind;
  payload: Record<string, unknown>;
  slot: bigint;
  signature: string;
}

export type EventKind =
  | "ProgramInitialized"
  | "BridgeRegistered"
  | "RiskUpdated"
  | "PolicyOpened"
  | "PolicySettled"
  | "WreckPayout"
  | "BridgeQuarantined"
  | "QuarantineLifted"
  | "WreckFundDeposit"
  | "ConvoyOpened"
  | "PausedToggled"
  | "AdminTransferred"
  | "ConfigUpdated"
  | "ThroughputThrottled";

export interface RiskUpdatedPayload {
  bridge: Pubkey;
  previousScore: number;
  newScore: number;
  previousTier: Tier;
  newTier: Tier;
  slot: bigint;
}

export interface PolicyOpenedPayload {
  policy: Pubkey;
  beneficiary: Pubkey;
  bridge: Pubkey;
  cargo: bigint;
  tollPaid: bigint;
}
