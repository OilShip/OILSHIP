// Smoke tests for the OILSHIP SDK.

import { test } from "node:test";
import assert from "node:assert/strict";

import {
  solToLamports,
  lamportsToSol,
  classFromCargo,
  LAMPORTS_PER_SOL,
} from "../src/types.js";
import {
  bpsOf,
  riskMultiplierBps,
  applyRiskMultiplier,
  tierFromScore,
  fmtSol,
  fmtBps,
  fmtPubkey,
  hoursToSlots,
  daysToSlots,
} from "../src/utils.js";
import { computeRisk, smoothScores, isSailable, tollMultiplierBps } from "../src/risk.js";
import { QuoteBuilder } from "../src/quote.js";
import { ValidationError } from "../src/errors.js";
import { pubkey } from "../src/types.js";
import type { Bridge } from "../src/types.js";

function fakeBridge(symbol: string, riskScore: number, quarantined = false): Bridge {
  return {
    symbol,
    name: symbol,
    operator: pubkey("11111111111111111111111111111111"),
    riskScore,
    tier: tierFromScore(riskScore),
    routable: !quarantined,
    quarantined,
    lastUpdateSlot: 0n,
    openPolicies: 0,
    openCoverage: 0n,
    lifetimeTolls: 0n,
    lifetimePayouts: 0n,
    quarantineCount: 0,
  };
}

test("solToLamports round trip", () => {
  assert.equal(solToLamports(1.5), 1_500_000_000n);
  assert.equal(lamportsToSol(LAMPORTS_PER_SOL), 1);
});

test("classFromCargo boundaries", () => {
  assert.equal(classFromCargo(0n), "coaster");
  assert.equal(classFromCargo(LAMPORTS_PER_SOL), "tanker");
  assert.equal(classFromCargo(50n * LAMPORTS_PER_SOL), "capesize");
  assert.equal(classFromCargo(250n * LAMPORTS_PER_SOL), "dark_fleet");
});

test("bpsOf returns expected fraction", () => {
  assert.equal(bpsOf(10_000n, 250), 250n);
  assert.equal(bpsOf(0n, 250), 0n);
});

test("riskMultiplierBps tiers", () => {
  assert.equal(riskMultiplierBps(0), 9_500);
  assert.equal(riskMultiplierBps(20), 9_500);
  assert.equal(riskMultiplierBps(21), 10_000);
  assert.equal(riskMultiplierBps(60), 11_500);
  assert.equal(riskMultiplierBps(81), 19_000);
});
