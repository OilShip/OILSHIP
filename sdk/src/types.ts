// Shared types for the OILSHIP SDK.

export type Pubkey = string & { readonly __brand: "Pubkey" };

export function pubkey(s: string): Pubkey {
  if (!/^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(s)) {
    throw new Error(`invalid base58 pubkey: ${s}`);
  }
  return s as Pubkey;
}

export type Lamports = bigint;

export const LAMPORTS_PER_SOL: Lamports = 1_000_000_000n;

export type Tier = "tier_1" | "tier_2" | "tier_3" | "quarantined";

export type PolicyState = "pending" | "active" | "settled" | "claimed" | "expired";

export type VesselClass = "coaster" | "tanker" | "capesize" | "dark_fleet";
