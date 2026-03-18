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

test("applyRiskMultiplier preserves order", () => {
  const baseToll = 10_000n;
  const low = applyRiskMultiplier(baseToll, 10);
  const high = applyRiskMultiplier(baseToll, 90);
  assert.ok(low < baseToll);
  assert.ok(high > baseToll);
});

test("tierFromScore is monotonic", () => {
  let lastIdx = -1;
  const order = ["tier_1", "tier_2", "tier_3", "quarantined"] as const;
  for (let s = 0; s <= 100; s++) {
    const tier = tierFromScore(s);
    const idx = order.indexOf(tier);
    assert.ok(idx >= lastIdx, `tier index dropped at ${s}`);
    lastIdx = idx;
  }
});

test("hoursToSlots and daysToSlots agree", () => {
  assert.equal(hoursToSlots(24), daysToSlots(1));
  assert.equal(hoursToSlots(48), daysToSlots(2));
});

test("fmt helpers do not throw", () => {
  fmtSol(1_500_000_000n);
  fmtBps(125);
  fmtPubkey("11111111111111111111111111111111" as ReturnType<typeof pubkey>);
});

test("computeRisk baseline", () => {
  const r = computeRisk({ symbol: "x", pubkey: pubkey("11111111111111111111111111111111") }, []);
  assert.equal(r.score, 18);
  assert.equal(r.tier, "tier_1");
});

test("computeRisk critical event", () => {
  const id = { symbol: "x", pubkey: pubkey("11111111111111111111111111111111") };
  const r = computeRisk(id, [
    { kind: "AdminKeyRotation", severity: "critical", message: "key moved", capturedAt: 0, source: "test" },
  ]);
  assert.ok(r.score > 18);
});

test("smoothScores trends to last value", () => {
  const s = smoothScores([10, 20, 30, 40], 0.5);
  assert.ok(s > 10 && s <= 40);
});

test("isSailable rejects quarantined", () => {
  assert.equal(isSailable(20), true);
  assert.equal(isSailable(99), false);
});

test("tollMultiplierBps matches utils", () => {
  for (const score of [0, 30, 50, 70, 95]) {
    assert.equal(tollMultiplierBps(score), riskMultiplierBps(score));
  }
});

test("QuoteBuilder picks lowest risk", () => {
  const bridges: Bridge[] = [
    fakeBridge("a", 70),
    fakeBridge("b", 12),
    fakeBridge("c", 40),
  ];
  const q = QuoteBuilder.build({ cargo: solToLamports(1), baseTollBps: 10 }, bridges);
  assert.equal(q.bridge.symbol, "b");
});

test("QuoteBuilder skips quarantined", () => {
  const bridges: Bridge[] = [
    fakeBridge("a", 5, true),
    fakeBridge("b", 30),
  ];
  const q = QuoteBuilder.build({ cargo: solToLamports(1), baseTollBps: 10 }, bridges);
  assert.equal(q.bridge.symbol, "b");
});

test("QuoteBuilder respects preferredBridge", () => {
  const bridges: Bridge[] = [
    fakeBridge("a", 5),
    fakeBridge("b", 30),
  ];
  const q = QuoteBuilder.build(
    { cargo: solToLamports(1), baseTollBps: 10, preferredBridge: "b" },
    bridges,
  );
  assert.equal(q.bridge.symbol, "b");
});

test("QuoteBuilder rejects bad cargo", () => {
  assert.throws(
    () => QuoteBuilder.build({ cargo: 0n, baseTollBps: 10 }, [fakeBridge("a", 1)]),
    ValidationError,
  );
});

test("QuoteBuilder rejects no eligible bridges", () => {
  assert.throws(
    () =>
      QuoteBuilder.build(
        { cargo: solToLamports(1), baseTollBps: 10 },
        [fakeBridge("a", 5, true)],
      ),
    ValidationError,
  );
});
