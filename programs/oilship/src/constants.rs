//! On-chain constants for the OILSHIP convoy escort program.
//!
//! Every limit, seed and basis-point figure that the program enforces lives
//! in this file. Keeping the values here means the rest of the codebase only
//! has to reason about *names*, not magic numbers.

use anchor_lang::prelude::*;

/// Program-derived address seeds.
pub const SEED_CONFIG: &[u8] = b"oilship.config";
pub const SEED_TREASURY: &[u8] = b"oilship.treasury";
pub const SEED_WRECK_FUND: &[u8] = b"oilship.wreck";
pub const SEED_BRIDGE: &[u8] = b"oilship.bridge";
pub const SEED_POLICY: &[u8] = b"oilship.policy";
pub const SEED_VAULT: &[u8] = b"oilship.vault";
pub const SEED_CONVOY: &[u8] = b"oilship.convoy";

/// Basis points denominator (100% == 10_000).
pub const BPS_DENOM: u64 = 10_000;

/// Default toll the protocol takes from each escorted transit.
/// 10 bps == 0.10% of the cargo value.
pub const DEFAULT_TOLL_BPS: u16 = 10;

/// Maximum toll the admin can set. Anything above this is an obvious griefing
/// attempt and should be rejected outright.
pub const MAX_TOLL_BPS: u16 = 100;

/// Default split of collected tolls.
/// All three values must sum to BPS_DENOM (10_000).
pub const DEFAULT_FUND_SPLIT_BPS: u16 = 6_000; // 60% -> wreck fund
pub const DEFAULT_BUYBACK_SPLIT_BPS: u16 = 3_000; // 30% -> $OIL buyback
pub const DEFAULT_OPS_SPLIT_BPS: u16 = 1_000; // 10% -> operations

/// Risk thresholds. The monitoring engine writes risk scores into the
/// Bridge account; routing decisions are taken by comparing the score
/// against these thresholds.
pub const RISK_TIER_1_MAX: u8 = 30;
pub const RISK_TIER_2_MAX: u8 = 55;
pub const RISK_TIER_3_MAX: u8 = 80;
pub const RISK_QUARANTINE_MIN: u8 = 81;

/// Maximum number of bridges the registry will hold at once.
pub const MAX_REGISTERED_BRIDGES: usize = 64;

/// Maximum length of human-readable identifiers anchored on chain.
pub const MAX_NAME_LEN: usize = 48;
pub const MAX_SYMBOL_LEN: usize = 12;
pub const MAX_URL_LEN: usize = 96;

/// A policy can never cover more cargo than this hard cap, regardless of
/// available wreck fund balance. Single-policy concentration risk.
pub const MAX_POLICY_CARGO_LAMPORTS: u64 = 250_000 * LAMPORTS_PER_SOL;

/// Lower bound on a policy. Anything smaller is not worth the rent.
pub const MIN_POLICY_CARGO_LAMPORTS: u64 = LAMPORTS_PER_SOL / 100; // 0.01 SOL

/// Lamports in a SOL, hard-coded so we never depend on a foreign module
/// at the BPF layer.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
