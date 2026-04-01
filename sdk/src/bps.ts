// BPS arithmetic helpers — pure functions, no side effects.

import { Lamports } from "./types.js";

export const BPS_DENOM = 10_000;

export function applyBps(value: Lamports, bps: number): Lamports {
  if (bps < 0) throw new RangeError("bps must be >= 0");
  if (bps > BPS_DENOM) throw new RangeError("bps must be <= 10_000");
  return (value * BigInt(bps)) / BigInt(BPS_DENOM);
}

export function applyBpsRound(value: Lamports, bps: number): Lamports {
  const numerator = value * BigInt(bps);
  const half = BigInt(BPS_DENOM) / 2n;
  return (numerator + half) / BigInt(BPS_DENOM);
}

export function bpsFromRatio(numer: Lamports, denom: Lamports): number {
  if (denom === 0n) return 0;
  const bps = Number((numer * BigInt(BPS_DENOM)) / denom);
  return Math.min(BPS_DENOM, Math.max(0, bps));
}

export function lerpBps(a: number, b: number, t: number): number {
  if (t < 0) t = 0;
  if (t > 1) t = 1;
  return Math.round(a + (b - a) * t);
}

export function clampBps(b: number): number {
  if (b < 0) return 0;
  if (b > BPS_DENOM) return BPS_DENOM;
  return b;
}
