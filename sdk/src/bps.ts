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

export function bpsToFraction(bps: number): number {
  return bps / BPS_DENOM;
}

export function fractionToBps(f: number): number {
  return Math.round(f * BPS_DENOM);
}

export function splitInto(amount: Lamports, parts: number[]): Lamports[] {
  const totalBps = parts.reduce((a, b) => a + b, 0);
  if (totalBps !== BPS_DENOM) {
    throw new RangeError(`split parts must sum to ${BPS_DENOM}, got ${totalBps}`);
  }
  const out: Lamports[] = [];
  let used: Lamports = 0n;
  for (let i = 0; i < parts.length - 1; i++) {
    const piece = applyBps(amount, parts[i]);
    out.push(piece);
    used += piece;
  }
  out.push(amount - used);
  return out;
}

export function sumBps(values: Lamports[]): Lamports {
  let total = 0n;
  for (const v of values) total += v;
  return total;
}

export function maxLamports(values: Lamports[]): Lamports {
  let m: Lamports = 0n;
  for (const v of values) {
    if (v > m) m = v;
  }
  return m;
}

export function minLamports(values: Lamports[]): Lamports {
  if (values.length === 0) return 0n;
  let m = values[0];
  for (const v of values) {
    if (v < m) m = v;
  }
  return m;
}
