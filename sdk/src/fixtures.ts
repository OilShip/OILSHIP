// Test fixtures and seed data.

import { Bridge, Policy, Pubkey, WreckFundView, GlobalConfigView, pubkey, solToLamports } from "./types.js";
import { tierFromScore } from "./utils.js";

const PK = pubkey("11111111111111111111111111111111");

export const FIXTURE_BRIDGES: Bridge[] = [
  bridge("mayan",     "Mayan Finance",   12),
  bridge("debridge",  "deBridge",        22),
  bridge("wormhole",  "Wormhole Portal", 48),
  bridge("allbridge", "Allbridge Core",  71),
];

export const FIXTURE_QUARANTINED: Bridge[] = [
  bridge("orbit", "Orbit Bridge", 96, true),
];

export const FIXTURE_GLOBAL_CONFIG: GlobalConfigView = {
  admin: PK,
  oilMint: PK,
  treasury: PK,
  wreckFund: PK,
  tollBps: 10,
  fundSplitBps: 6_000,
  buybackSplitBps: 3_000,
  opsSplitBps: 1_000,
  bridgesRegistered: FIXTURE_BRIDGES.length,
  policiesOpened: 0n,
  policiesSettled: 0n,
  wreckClaimsPaid: 0n,
  lifetimeTolls: 0n,
  lifetimePayouts: 0n,
  paused: false,
};
