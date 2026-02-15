//! On-chain accounts owned by the OILSHIP program.

use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub oil_mint: Pubkey,
    pub treasury: Pubkey,
    pub wreck_fund: Pubkey,
    pub toll_bps: u16,
    pub fund_split_bps: u16,
    pub buyback_split_bps: u16,
    pub ops_split_bps: u16,
    pub bridges_registered: u16,
    pub policies_opened: u64,
    pub policies_settled: u64,
    pub wreck_claims_paid: u64,
    pub lifetime_tolls: u64,
    pub lifetime_payouts: u64,
    pub paused: bool,
    pub bump: u8,
    pub reserved: [u8; 64],
}

impl GlobalConfig {
    pub const LEN: usize = DISCRIMINATOR_LEN
        + 32 + 32 + 32 + 32 + 2 + 2 + 2 + 2 + 2 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 64;
}

#[account]
#[derive(Default, Debug)]
pub struct Bridge {
    pub symbol: [u8; MAX_SYMBOL_LEN],
    pub name: [u8; MAX_NAME_LEN],
    pub operator: Pubkey,
    pub risk_score: u8,
    pub tier: u8,
    pub routable: bool,
    pub quarantined: bool,
    pub last_update_slot: u64,
    pub open_policies: u32,
    pub open_coverage: u64,
    pub throughput_slot: u64,
    pub throughput_count: u32,
    pub lifetime_tolls: u64,
    pub lifetime_payouts: u64,
    pub quarantine_count: u16,
    pub bump: u8,
    pub reserved: [u8; 32],
}

impl Bridge {
    pub const LEN: usize = DISCRIMINATOR_LEN
        + MAX_SYMBOL_LEN + MAX_NAME_LEN + 32 + 1 + 1 + 1 + 1 + 8 + 4 + 8 + 8 + 4 + 8 + 8 + 2 + 1 + 32;

    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&c| c == 0).unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..end]).unwrap_or("")
    }

    pub fn symbol_str(&self) -> &str {
        let end = self.symbol.iter().position(|&c| c == 0).unwrap_or(self.symbol.len());
        core::str::from_utf8(&self.symbol[..end]).unwrap_or("")
    }
}

#[account]
#[derive(Default, Debug)]
pub struct Policy {
    pub beneficiary: Pubkey,
    pub bridge: Pubkey,
    pub convoy: Pubkey,
    pub cargo: u64,
    pub toll_paid: u64,
    pub risk_at_open: u8,
    pub class: u8,
    pub opened_slot: u64,
    pub mature_slot: u64,
    pub expires_slot: u64,
    pub state: u8,
    pub bump: u8,
    pub reserved: [u8; 16],
}

impl Policy {
    pub const LEN: usize = DISCRIMINATOR_LEN
        + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 8 + 8 + 8 + 1 + 1 + 16;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyState {
    Pending = 0,
    Active = 1,
    Settled = 2,
    Claimed = 3,
    Expired = 4,
}

impl Default for PolicyState {
    fn default() -> Self { Self::Pending }
}

impl PolicyState {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Pending,
            1 => Self::Active,
            2 => Self::Settled,
            3 => Self::Claimed,
            4 => Self::Expired,
            _ => Self::Pending,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VesselClass {
    Coaster = 0,
    Tanker = 1,
    Capesize = 2,
    DarkFleet = 3,
}

impl Default for VesselClass {
    fn default() -> Self { Self::Coaster }
}

impl VesselClass {
    pub fn from_cargo(cargo: u64) -> Self {
        const ONE_SOL: u64 = LAMPORTS_PER_SOL;
        if cargo < ONE_SOL { Self::Coaster }
        else if cargo < 50 * ONE_SOL { Self::Tanker }
        else if cargo < 250 * ONE_SOL { Self::Capesize }
        else { Self::DarkFleet }
    }
}

#[account]
#[derive(Default, Debug)]
pub struct WreckFund {
    pub authority: Pubkey,
    pub balance: u64,
    pub open_coverage: u64,
    pub lifetime_deposits: u64,
    pub lifetime_payouts: u64,
    pub payout_count: u64,
    pub bump: u8,
    pub reserved: [u8; 32],
}

impl WreckFund {
    pub const LEN: usize = DISCRIMINATOR_LEN + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 32;
}

#[account]
#[derive(Default, Debug)]
pub struct Treasury {
    pub authority: Pubkey,
    pub balance: u64,
    pub lifetime_in: u64,
    pub lifetime_out: u64,
    pub bump: u8,
    pub reserved: [u8; 32],
}

impl Treasury {
    pub const LEN: usize = DISCRIMINATOR_LEN + 32 + 8 + 8 + 8 + 1 + 32;
}

#[account]
#[derive(Default, Debug)]
pub struct Convoy {
    pub bridge: Pubkey,
    pub opened_slot: u64,
    pub closed_slot: u64,
    pub policy_count: u32,
    pub total_cargo: u64,
    pub total_toll: u64,
    pub bump: u8,
    pub reserved: [u8; 16],
}

impl Convoy {
    pub const LEN: usize = DISCRIMINATOR_LEN + 32 + 8 + 8 + 4 + 8 + 8 + 1 + 16;
}

pub fn copy_into<const N: usize>(src: &str) -> [u8; N] {
    let mut dst = [0u8; N];
    let bytes = src.as_bytes();
    let len = core::cmp::min(bytes.len(), N);
    dst[..len].copy_from_slice(&bytes[..len]);
    dst
}
