//! Anchor events emitted by the OILSHIP program.

use anchor_lang::prelude::*;

#[event]
pub struct ProgramInitialized {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub wreck_fund: Pubkey,
    pub toll_bps: u16,
    pub timestamp: i64,
}

#[event]
pub struct BridgeRegistered {
    pub bridge: Pubkey,
    pub symbol: String,
    pub operator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct RiskUpdated {
    pub bridge: Pubkey,
    pub previous_score: u8,
    pub new_score: u8,
    pub previous_tier: u8,
    pub new_tier: u8,
    pub slot: u64,
    pub timestamp: i64,
}

#[event]
pub struct PolicyOpened {
    pub policy: Pubkey,
    pub beneficiary: Pubkey,
    pub bridge: Pubkey,
    pub cargo: u64,
    pub toll_paid: u64,
    pub class: u8,
    pub risk_at_open: u8,
    pub mature_slot: u64,
    pub expires_slot: u64,
    pub timestamp: i64,
}

#[event]
pub struct PolicySettled {
    pub policy: Pubkey,
    pub beneficiary: Pubkey,
    pub bridge: Pubkey,
    pub cargo: u64,
    pub timestamp: i64,
}

#[event]
pub struct WreckPayout {
    pub policy: Pubkey,
    pub beneficiary: Pubkey,
    pub bridge: Pubkey,
    pub principal_paid: u64,
    pub fund_balance_after: u64,
    pub timestamp: i64,
}

#[event]
pub struct BridgeQuarantined {
    pub bridge: Pubkey,
    pub final_score: u8,
    pub open_policies_at_quarantine: u32,
    pub coverage_at_quarantine: u64,
    pub timestamp: i64,
}
