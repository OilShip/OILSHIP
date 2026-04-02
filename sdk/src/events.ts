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

  clear(): void {
    this.listeners.clear();
  }
}

export class EventStream {
  readonly riskUpdated = new EventEmitter<RiskUpdatedPayload>();
  readonly policyOpened = new EventEmitter<PolicyOpenedPayload>();
  readonly wreckPayout = new EventEmitter<WreckPayoutPayload>();
  readonly bridgeQuarantined = new EventEmitter<Pubkey>();

  dispatch(event: OilshipEvent): void {
    switch (event.kind) {
      case "RiskUpdated":
        this.riskUpdated.emit(event.payload as unknown as RiskUpdatedPayload);
        break;
      case "PolicyOpened":
        this.policyOpened.emit(event.payload as unknown as PolicyOpenedPayload);
        break;
      case "WreckPayout":
        this.wreckPayout.emit(event.payload as unknown as WreckPayoutPayload);
        break;
      case "BridgeQuarantined":
        this.bridgeQuarantined.emit(event.payload.bridge as Pubkey);
        break;
      default:
        break;
    }
  }
}

export function summarisePolicy(p: Policy): string {
  return `${p.vesselClass} carrying ${p.cargo} → ${p.bridge}`;
}

export function summariseBridge(b: Bridge): string {
  return `${b.symbol} (${b.tier}, risk ${b.riskScore})`;
}
