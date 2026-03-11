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

export const FIXTURE_WRECK_FUND: WreckFundView = {
  authority: PK,
  balance: 0n,
  openCoverage: 0n,
  lifetimeDeposits: 0n,
  lifetimePayouts: 0n,
  payoutCount: 0n,
};

export function fixturePolicy(beneficiary: Pubkey, cargo: bigint = solToLamports(1)): Policy {
  return {
    pubkey: PK,
    beneficiary,
    bridge: PK,
    cargo,
    tollPaid: (cargo * 10n) / 10_000n,
    riskAtOpen: 22,
    vesselClass: cargo < 50_000_000_000n ? "tanker" : "capesize",
    openedSlot: 0n,
    matureSlot: 0n,
    expiresSlot: 0n,
    state: "active",
  };
}

function bridge(symbol: string, name: string, riskScore: number, quarantined = false): Bridge {
  return {
    symbol,
    name,
    operator: PK,
    riskScore,
    tier: tierFromScore(riskScore),
    routable: !quarantined,
    quarantined,
    lastUpdateSlot: 0n,
    openPolicies: 0,
    openCoverage: 0n,
    lifetimeTolls: 0n,
    lifetimePayouts: 0n,
    quarantineCount: quarantined ? 1 : 0,
  };
}

export const FIXTURE_ANOMALY_SCENARIO = {
  bridge: "mayan",
  anomalies: [
    { kind: "TvlDrop",          severity: "medium",   message: "tvl down 13%",            source: "tvl-drop", capturedAt: 0 },
    { kind: "OracleDrift",      severity: "low",      message: "drift 35 bps",            source: "oracle",   capturedAt: 0 },
    { kind: "AdminKeyRotation", severity: "high",     message: "admin key moved",         source: "key",      capturedAt: 0 },
  ],
};

export const FIXTURE_INTEGRATION_SLOT_RANGE = {
  startSlot: 250_000_000n,
  endSlot:   250_000_500n,
};
