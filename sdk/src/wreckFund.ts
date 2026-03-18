// WreckFund — read & write helpers for the OILSHIP insurance pool.

import { Lamports, WreckFundView } from "./types.js";
import { OilshipClient } from "./client.js";
import { ValidationError, CapacityError } from "./errors.js";

export interface DepositRequest {
  amount: Lamports;
}

export interface WreckHealth {
  reserve: Lamports;
  openCoverage: Lamports;
  reserveRatioBps: number;
  capacityRemaining: Lamports;
  payoutCount: bigint;
}

const MIN_RESERVE_RATIO_BPS = 1_500;

export class WreckFund {
  constructor(private readonly client: OilshipClient) {}

  async fetch(): Promise<WreckFundView | null> {
    return this.client.fetchWreckFund();
  }

  async health(): Promise<WreckHealth> {
    const v = await this.fetch();
    if (!v) {
      return {
        reserve: 0n,
        openCoverage: 0n,
        reserveRatioBps: 0,
        capacityRemaining: 0n,
        payoutCount: 0n,
      };
    }
    const ratio = v.openCoverage === 0n ? 10_000 : Number((v.balance * 10_000n) / v.openCoverage);
    const minReserve = (v.openCoverage * BigInt(MIN_RESERVE_RATIO_BPS)) / 10_000n;
    const remaining = v.balance > minReserve ? v.balance - minReserve : 0n;
    return {
      reserve: v.balance,
      openCoverage: v.openCoverage,
      reserveRatioBps: ratio,
      capacityRemaining: remaining,
      payoutCount: v.payoutCount,
    };
  }

  validateDeposit(req: DepositRequest): void {
    if (req.amount <= 0n) throw new ValidationError("deposit amount must be > 0");
  }

  async assertCanCover(cargo: Lamports): Promise<void> {
    const h = await this.health();
    if (cargo > h.capacityRemaining) {
      throw new CapacityError(cargo, h.capacityRemaining);
    }
  }

  async simulate(cargo: Lamports): Promise<{ newReserveRatioBps: number; blocked: boolean }> {
    const h = await this.health();
    const newCoverage = h.openCoverage + cargo;
    if (newCoverage === 0n) {
      return { newReserveRatioBps: 10_000, blocked: false };
    }
    const ratio = Number((h.reserve * 10_000n) / newCoverage);
    return {
      newReserveRatioBps: ratio,
      blocked: ratio < MIN_RESERVE_RATIO_BPS,
    };
  }
}
