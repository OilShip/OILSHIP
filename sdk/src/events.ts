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

export interface WreckPayoutPayload {
  policy: Pubkey;
  beneficiary: Pubkey;
  bridge: Pubkey;
  principalPaid: bigint;
  fundBalanceAfter: bigint;
}

export class EventEmitter<T> {
  private readonly listeners: Set<(value: T) => void> = new Set();

  on(handler: (value: T) => void): () => void {
    this.listeners.add(handler);
    return () => this.listeners.delete(handler);
  }

  emit(value: T): void {
    for (const h of this.listeners) {
      try {
        h(value);
      } catch (err) {
        console.error("oilship event handler error:", err);
      }
    }
  }

  size(): number {
    return this.listeners.size;
  }
