// Small utilities used across the SDK.

import { Pubkey, pubkey, Lamports, Tier } from "./types.js";

const ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const ALPHABET_MAP: Record<string, number> = {};
for (let i = 0; i < ALPHABET.length; i++) {
  ALPHABET_MAP[ALPHABET.charAt(i)] = i;
}

export function base58Encode(buf: Uint8Array): string {
  if (buf.length === 0) return "";
  const digits: number[] = [0];
  for (let i = 0; i < buf.length; i++) {
    let carry = buf[i];
    for (let j = 0; j < digits.length; j++) {
      carry += digits[j] << 8;
      digits[j] = carry % 58;
      carry = (carry / 58) | 0;
    }
    while (carry) {
      digits.push(carry % 58);
      carry = (carry / 58) | 0;
    }
  }
  let zeros = 0;
  while (zeros < buf.length && buf[zeros] === 0) zeros++;
  let str = "";
  for (let i = 0; i < zeros; i++) str += ALPHABET.charAt(0);
  for (let i = digits.length - 1; i >= 0; i--) str += ALPHABET.charAt(digits[i]);
  return str;
}

export function base58Decode(str: string): Uint8Array {
  if (str.length === 0) return new Uint8Array();
  const bytes: number[] = [0];
  for (let i = 0; i < str.length; i++) {
    const c = str.charAt(i);
    if (!(c in ALPHABET_MAP)) throw new Error(`bad base58 char: ${c}`);
    let carry = ALPHABET_MAP[c];
    for (let j = 0; j < bytes.length; j++) {
      carry += bytes[j] * 58;
      bytes[j] = carry & 0xff;
      carry >>= 8;
    }
    while (carry) {
      bytes.push(carry & 0xff);
      carry >>= 8;
    }
  }
  let zeros = 0;
  while (zeros < str.length && str.charAt(zeros) === ALPHABET.charAt(0)) zeros++;
  for (let i = 0; i < zeros; i++) bytes.push(0);
  return Uint8Array.from(bytes.reverse());
}

export type Hasher = (input: Uint8Array) => Uint8Array;

export function joinSeeds(seeds: Uint8Array[]): Uint8Array {
  let total = 0;
  for (const s of seeds) total += s.length;
  const out = new Uint8Array(total);
  let off = 0;
  for (const s of seeds) {
    out.set(s, off);
    off += s.length;
  }
  return out;
}

export const PDA_MARKER = new TextEncoder().encode("ProgramDerivedAddress");

export function bpsOf(value: Lamports, bps: number): Lamports {
  if (bps < 0 || bps > 10_000) throw new RangeError(`bps out of range: ${bps}`);
  return (value * BigInt(bps)) / 10_000n;
}
