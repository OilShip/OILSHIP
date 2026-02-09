//! Instruction handlers for the OILSHIP convoy escort program.

use crate::constants::*;
use crate::errors::OilshipError;
use crate::math::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeParams {
    pub toll_bps: u16,
    pub fund_split_bps: u16,
    pub buyback_split_bps: u16,
    pub ops_split_bps: u16,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = GlobalConfig::LEN, seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, GlobalConfig>,
    #[account(init, payer = admin, space = Treasury::LEN, seeds = [SEED_TREASURY], bump)]
    pub treasury: Account<'info, Treasury>,
    #[account(init, payer = admin, space = WreckFund::LEN, seeds = [SEED_WRECK_FUND], bump)]
    pub wreck_fund: Account<'info, WreckFund>,
    /// CHECK: arbitrary mint pubkey, validated off-chain.
    pub oil_mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    if params.toll_bps > MAX_TOLL_BPS {
        return err!(OilshipError::TollTooHigh);
    }
    let total = params.fund_split_bps as u32 + params.buyback_split_bps as u32 + params.ops_split_bps as u32;
    if total != BPS_DENOM as u32 {
        return err!(OilshipError::InvalidSplit);
    }
    let cfg = &mut ctx.accounts.config;
    if cfg.admin != Pubkey::default() {
        return err!(OilshipError::AlreadyInitialized);
    }
    cfg.admin = ctx.accounts.admin.key();
    cfg.oil_mint = ctx.accounts.oil_mint.key();
    cfg.treasury = ctx.accounts.treasury.key();
    cfg.wreck_fund = ctx.accounts.wreck_fund.key();
    cfg.toll_bps = params.toll_bps;
    cfg.fund_split_bps = params.fund_split_bps;
    cfg.buyback_split_bps = params.buyback_split_bps;
    cfg.ops_split_bps = params.ops_split_bps;
    cfg.bridges_registered = 0;
    cfg.policies_opened = 0;
    cfg.policies_settled = 0;
    cfg.wreck_claims_paid = 0;
    cfg.lifetime_tolls = 0;
    cfg.lifetime_payouts = 0;
    cfg.paused = false;
    cfg.bump = ctx.bumps.config;

    let treasury = &mut ctx.accounts.treasury;
    treasury.authority = ctx.accounts.admin.key();
    treasury.balance = 0;
    treasury.lifetime_in = 0;
    treasury.lifetime_out = 0;
    treasury.bump = ctx.bumps.treasury;

    let fund = &mut ctx.accounts.wreck_fund;
    fund.authority = ctx.accounts.admin.key();
    fund.balance = 0;
    fund.open_coverage = 0;
    fund.lifetime_deposits = 0;
    fund.lifetime_payouts = 0;
    fund.payout_count = 0;
    fund.bump = ctx.bumps.wreck_fund;

    msg!("oilship initialized: toll_bps={}", params.toll_bps);
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RegisterBridgeParams {
    pub symbol: String,
    pub name: String,
    pub operator: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: RegisterBridgeParams)]
pub struct RegisterBridge<'info> {
    #[account(mut, address = config.admin @ OilshipError::NotAdmin)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Account<'info, GlobalConfig>,
    #[account(init, payer = admin, space = Bridge::LEN, seeds = [SEED_BRIDGE, params.symbol.as_bytes()], bump)]
    pub bridge: Account<'info, Bridge>,
    pub system_program: Program<'info, System>,
}

pub fn register_bridge_handler(ctx: Context<RegisterBridge>, params: RegisterBridgeParams) -> Result<()> {
    if params.symbol.is_empty() || params.symbol.len() > MAX_SYMBOL_LEN {
        return err!(OilshipError::InvalidBridgeSymbol);
    }
    if params.name.is_empty() || params.name.len() > MAX_NAME_LEN {
        return err!(OilshipError::InvalidBridgeName);
    }
    if (ctx.accounts.config.bridges_registered as usize) >= MAX_REGISTERED_BRIDGES {
        return err!(OilshipError::BridgeRegistryFull);
    }
    let bridge = &mut ctx.accounts.bridge;
    bridge.symbol = copy_into::<MAX_SYMBOL_LEN>(&params.symbol);
    bridge.name = copy_into::<MAX_NAME_LEN>(&params.name);
    bridge.operator = params.operator;
    bridge.risk_score = 50;
    bridge.tier = 2;
    bridge.routable = true;
    bridge.quarantined = false;
    bridge.last_update_slot = Clock::get()?.slot;
    bridge.open_policies = 0;
    bridge.open_coverage = 0;
    bridge.throughput_slot = 0;
    bridge.throughput_count = 0;
    bridge.lifetime_tolls = 0;
    bridge.lifetime_payouts = 0;
    bridge.quarantine_count = 0;
    bridge.bump = ctx.bumps.bridge;
    ctx.accounts.config.bridges_registered = ctx.accounts.config.bridges_registered.checked_add(1).ok_or(OilshipError::MathOverflow)?;
    msg!("bridge registered: {}", params.symbol);
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UpdateRiskParams {
    pub score: u8,
}

#[derive(Accounts)]
pub struct UpdateRisk<'info> {
    pub operator: Signer<'info>,
    #[account(mut)]
    pub bridge: Account<'info, Bridge>,
}

pub fn update_risk_handler(ctx: Context<UpdateRisk>, params: UpdateRiskParams) -> Result<()> {
    if params.score > 100 {
        return err!(OilshipError::InvalidRiskScore);
    }
    let bridge = &mut ctx.accounts.bridge;
    if bridge.operator != ctx.accounts.operator.key() {
        return err!(OilshipError::NotBridgeOperator);
    }
    if bridge.quarantined {
        return err!(OilshipError::BridgeQuarantined);
    }
    bridge.risk_score = params.score;
    bridge.tier = match params.score {
        0..=RISK_TIER_1_MAX => 1,
        v if v <= RISK_TIER_2_MAX => 2,
        v if v <= RISK_TIER_3_MAX => 3,
        _ => 4,
    };
    bridge.routable = bridge.tier <= 3;
    bridge.last_update_slot = Clock::get()?.slot;
    if params.score >= RISK_QUARANTINE_MIN {
        bridge.quarantined = true;
        bridge.routable = false;
        bridge.quarantine_count = bridge.quarantine_count.saturating_add(1);
        msg!("bridge quarantined at score {}", params.score);
    }
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OpenPolicyParams {
    pub cargo: u64,
    pub lifetime_slots: u64,
    pub seed: u64,
}

#[derive(Accounts)]
#[instruction(params: OpenPolicyParams)]
pub struct OpenPolicy<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    #[account(seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub bridge: Account<'info, Bridge>,
    #[account(mut, seeds = [SEED_WRECK_FUND], bump = wreck_fund.bump)]
    pub wreck_fund: Account<'info, WreckFund>,
    #[account(mut, seeds = [SEED_TREASURY], bump = treasury.bump)]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init,
        payer = beneficiary,
        space = Policy::LEN,
        seeds = [SEED_POLICY, beneficiary.key().as_ref(), bridge.key().as_ref(), &params.seed.to_le_bytes()],
        bump,
    )]
    pub policy: Account<'info, Policy>,
    pub system_program: Program<'info, System>,
}

pub fn open_policy_handler(ctx: Context<OpenPolicy>, params: OpenPolicyParams) -> Result<()> {
    let cfg = &ctx.accounts.config;
    if cfg.paused {
        return err!(OilshipError::Paused);
    }
    if params.cargo < MIN_POLICY_CARGO_LAMPORTS {
        return err!(OilshipError::CargoTooSmall);
    }
    if params.cargo > MAX_POLICY_CARGO_LAMPORTS {
        return err!(OilshipError::CargoTooLarge);
    }
    if params.lifetime_slots < MIN_POLICY_SLOTS {
        return err!(OilshipError::PolicyTooShort);
    }
    if params.lifetime_slots > MAX_POLICY_SLOTS {
        return err!(OilshipError::PolicyTooLong);
    }
    let bridge = &mut ctx.accounts.bridge;
    if bridge.quarantined || !bridge.routable {
        return err!(OilshipError::BridgeQuarantined);
    }
    let now_slot = Clock::get()?.slot;
    if bridge.throughput_slot != now_slot {
        bridge.throughput_slot = now_slot;
        bridge.throughput_count = 0;
    }
    if bridge.throughput_count >= MAX_POLICIES_PER_BRIDGE_PER_BLOCK {
        return err!(OilshipError::ThroughputExceeded);
    }
    bridge.throughput_count = bridge.throughput_count.saturating_add(1);

    let base_toll = compute_toll(params.cargo, cfg.toll_bps)?;
    let toll_paid = apply_risk_multiplier(base_toll, bridge.risk_score)?;

    let fund = &mut ctx.accounts.wreck_fund;
    let new_open = safe_add(fund.open_coverage, params.cargo)?;
    let ratio = reserve_ratio_bps(fund.balance, new_open);
    if ratio < MIN_RESERVE_RATIO_BPS {
        return err!(OilshipError::ReserveRatioBreach);
    }

    let (fund_share, buyback_share, ops_share) = split_toll(
        toll_paid,
        cfg.fund_split_bps,
        cfg.buyback_split_bps,
        cfg.ops_split_bps,
    )?;

    let cpi_accounts = system_program::Transfer {
        from: ctx.accounts.beneficiary.to_account_info(),
        to: ctx.accounts.wreck_fund.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    system_program::transfer(cpi_ctx, fund_share)?;
