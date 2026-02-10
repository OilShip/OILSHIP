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

declare_id!("11111111111111111111111111111111");

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
