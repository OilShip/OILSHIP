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
