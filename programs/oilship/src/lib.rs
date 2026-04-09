//! OILSHIP — Solana on-chain program for the Strait Convoy escort.

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod guards;
pub mod math;
pub mod state;
pub mod instructions;

pub use constants::*;
pub use errors::*;
pub use state::*;
pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod oilship {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize_handler(ctx, params)
    }

    pub fn register_bridge(ctx: Context<RegisterBridge>, params: RegisterBridgeParams) -> Result<()> {
        instructions::register_bridge_handler(ctx, params)
    }

    pub fn update_risk(ctx: Context<UpdateRisk>, params: UpdateRiskParams) -> Result<()> {
        instructions::update_risk_handler(ctx, params)
    }

    pub fn open_policy(ctx: Context<OpenPolicy>, params: OpenPolicyParams) -> Result<()> {
        instructions::open_policy_handler(ctx, params)
    }

    pub fn settle_policy(ctx: Context<SettlePolicy>) -> Result<()> {
        instructions::settle_policy_handler(ctx)
    }

    pub fn claim_payout(ctx: Context<ClaimPayout>) -> Result<()> {
        instructions::claim_payout_handler(ctx)
    }

    pub fn deposit_fund(ctx: Context<DepositFund>, params: DepositParams) -> Result<()> {
        instructions::deposit_fund_handler(ctx, params)
    }

    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        instructions::set_paused_handler(ctx, paused)
    }

    pub fn lift_quarantine(ctx: Context<LiftQuarantine>) -> Result<()> {
        instructions::lift_quarantine_handler(ctx)
    }

    pub fn open_convoy(ctx: Context<OpenConvoy>, params: OpenConvoyParams) -> Result<()> {
        instructions::open_convoy_handler(ctx, params)
    }
}
