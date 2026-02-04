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
