// Policy management.

import { Lamports, Policy, Pubkey, VesselClass, classFromCargo } from "./types.js";
import { OilshipClient } from "./client.js";
import { ValidationError } from "./errors.js";
import { bpsOf, daysToSlots, hoursToSlots } from "./utils.js";

export interface OpenPolicyRequest {
  beneficiary: Pubkey;
  bridgeSymbol: string;
  cargo: Lamports;
  lifetimeHours: number;
  baseTollBps: number;
  seed?: bigint;
}

export interface SettlePolicyRequest {
  beneficiary: Pubkey;
  policy: Pubkey;
  bridgeSymbol: string;
}

export interface ClaimPolicyRequest extends SettlePolicyRequest {}

export class Policies {
  constructor(private readonly client: OilshipClient) {}

  validate(req: OpenPolicyRequest): void {
    if (req.cargo <= 0n) throw new ValidationError("cargo must be > 0");
    if (req.lifetimeHours < 1) throw new ValidationError("lifetime must be at least 1 hour");
    if (req.lifetimeHours > 48) throw new ValidationError("lifetime cannot exceed 48 hours");
    if (req.baseTollBps < 1 || req.baseTollBps > 100) {
      throw new ValidationError("baseTollBps out of range");
    }
  }

  estimateToll(cargo: Lamports, baseTollBps: number, riskMult: number): Lamports {
    const base = bpsOf(cargo, baseTollBps);
    return (base * BigInt(riskMult)) / 10_000n;
  }

  vesselFor(cargo: Lamports): VesselClass {
    return classFromCargo(cargo);
  }

  toLifetimeSlots(hours: number): bigint {
    return hoursToSlots(hours);
  }

  toExpirySlots(days: number): bigint {
    return daysToSlots(days);
  }

  async fetch(addr: Pubkey): Promise<Policy | null> {
    return this.client.fetchPolicy(addr);
  }

  async isMature(addr: Pubkey): Promise<boolean> {
    const p = await this.fetch(addr);
    if (!p) return false;
    const slot = await this.client.getSlot();
    return slot >= p.matureSlot && slot <= p.expiresSlot;
  }

  async isExpired(addr: Pubkey): Promise<boolean> {
    const p = await this.fetch(addr);
    if (!p) return false;
    const slot = await this.client.getSlot();
    return slot > p.expiresSlot;
  }
}
