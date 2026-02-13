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
