// PDA derivation helpers.
//
// The OILSHIP program uses fixed seed prefixes for every account it
// owns. This module exposes pure functions that build the seed buffer
// for each PDA so a consumer can later run `findProgramAddress` from
// whichever Solana SDK they prefer.

import { Pubkey, Lamports } from "./types.js";
import { joinSeeds, base58Decode } from "./utils.js";

const ENC = new TextEncoder();

export const SEED_CONFIG = ENC.encode("oilship.config");
export const SEED_TREASURY = ENC.encode("oilship.treasury");
export const SEED_WRECK_FUND = ENC.encode("oilship.wreck");
export const SEED_BRIDGE = ENC.encode("oilship.bridge");
export const SEED_POLICY = ENC.encode("oilship.policy");
export const SEED_VAULT = ENC.encode("oilship.vault");
export const SEED_CONVOY = ENC.encode("oilship.convoy");

export function configSeeds(): Uint8Array[] {
  return [SEED_CONFIG];
}

export function treasurySeeds(): Uint8Array[] {
  return [SEED_TREASURY];
}

export function wreckFundSeeds(): Uint8Array[] {
  return [SEED_WRECK_FUND];
}

export function bridgeSeeds(symbol: string): Uint8Array[] {
  return [SEED_BRIDGE, ENC.encode(symbol)];
}

export function policySeeds(beneficiary: Pubkey, bridge: Pubkey, seed: bigint): Uint8Array[] {
  return [
    SEED_POLICY,
    base58Decode(beneficiary as unknown as string),
    base58Decode(bridge as unknown as string),
    u64Le(seed),
  ];
}

export function convoySeeds(bridge: Pubkey, seed: bigint): Uint8Array[] {
  return [SEED_CONVOY, base58Decode(bridge as unknown as string), u64Le(seed)];
}

export function vaultSeeds(owner: Pubkey): Uint8Array[] {
  return [SEED_VAULT, base58Decode(owner as unknown as string)];
}

function u64Le(v: bigint): Uint8Array {
  if (v < 0n) throw new RangeError("seed must be >= 0");
  const buf = new Uint8Array(8);
  let n = v;
  for (let i = 0; i < 8; i++) {
    buf[i] = Number(n & 0xffn);
    n >>= 8n;
  }
  return buf;
}

export function joinedConfigSeed(): Uint8Array {
  return joinSeeds(configSeeds());
}
