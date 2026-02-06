//! Reusable runtime guards.

use crate::constants::*;
use crate::errors::OilshipError;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn check_bridge_routable(bridge: &Bridge) -> Result<()> {
    if bridge.quarantined {
        return err!(OilshipError::BridgeQuarantined);
    }
    if !bridge.routable {
        return err!(OilshipError::BridgeQuarantined);
    }
    Ok(())
}

pub fn check_cargo_range(cargo: u64) -> Result<()> {
    if cargo < MIN_POLICY_CARGO_LAMPORTS {
        return err!(OilshipError::CargoTooSmall);
    }
    if cargo > MAX_POLICY_CARGO_LAMPORTS {
        return err!(OilshipError::CargoTooLarge);
    }
    Ok(())
}

pub fn check_lifetime_range(slots: u64) -> Result<()> {
    if slots < MIN_POLICY_SLOTS {
        return err!(OilshipError::PolicyTooShort);
    }
    if slots > MAX_POLICY_SLOTS {
        return err!(OilshipError::PolicyTooLong);
    }
    Ok(())
}

pub fn check_not_paused(cfg: &GlobalConfig) -> Result<()> {
    if cfg.paused {
        return err!(OilshipError::Paused);
    }
    Ok(())
}

pub fn check_pda(actual: Pubkey, expected: Pubkey) -> Result<()> {
    if actual != expected {
        return err!(OilshipError::PdaMismatch);
    }
    Ok(())
}

pub fn check_policy_active(policy: &Policy) -> Result<()> {
    let st = PolicyState::from_u8(policy.state);
    if st != PolicyState::Active {
        return err!(OilshipError::PolicyAlreadySettled);
    }
    Ok(())
}

pub fn check_mature(now: u64, mature: u64) -> Result<()> {
    if now < mature {
        return err!(OilshipError::PolicyNotMature);
    }
    Ok(())
}

pub fn check_not_expired(now: u64, expires: u64) -> Result<()> {
    if now > expires {
        return err!(OilshipError::PolicyExpired);
    }
    Ok(())
}

pub fn check_throughput(bridge: &Bridge, now_slot: u64) -> Result<()> {
    if bridge.throughput_slot == now_slot
        && bridge.throughput_count >= MAX_POLICIES_PER_BRIDGE_PER_BLOCK
    {
        return err!(OilshipError::ThroughputExceeded);
    }
    Ok(())
}

pub fn check_reserve_ratio(reserve: u64, current_open: u64, delta: u64) -> Result<()> {
    let new_open = current_open
        .checked_add(delta)
        .ok_or(OilshipError::MathOverflow)?;
    if new_open == 0 {
        return Ok(());
    }
    let ratio = (reserve as u128 * BPS_DENOM as u128) / new_open as u128;
    if (ratio as u64) < MIN_RESERVE_RATIO_BPS as u64 {
        return err!(OilshipError::ReserveRatioBreach);
    }
    Ok(())
}

pub fn check_str_len(s: &str, max: usize) -> Result<()> {
    if s.is_empty() || s.len() > max {
        return err!(OilshipError::InvalidBridgeName);
    }
    Ok(())
}

pub fn check_admin(cfg: &GlobalConfig, signer: Pubkey) -> Result<()> {
    if cfg.admin != signer {
        return err!(OilshipError::NotAdmin);
    }
    Ok(())
}

pub fn check_operator(bridge: &Bridge, signer: Pubkey) -> Result<()> {
    if bridge.operator != signer {
        return err!(OilshipError::NotBridgeOperator);
    }
    Ok(())
}
