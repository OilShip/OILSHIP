//! Fixed-point arithmetic helpers used throughout the OILSHIP program.

use crate::constants::BPS_DENOM;
use crate::errors::OilshipError;
use anchor_lang::prelude::*;

pub fn apply_bps(value: u64, bps: u16) -> Result<u64> {
    let value = value as u128;
    let bps = bps as u128;
    let denom = BPS_DENOM as u128;
    let product = value.checked_mul(bps).ok_or(OilshipError::MathOverflow)?;
    let result = product.checked_div(denom).ok_or(OilshipError::DivisionByZero)?;
    if result > u64::MAX as u128 {
        return err!(OilshipError::MathOverflow);
    }
    Ok(result as u64)
}

pub fn apply_bps_round(value: u64, bps: u16) -> Result<u64> {
    let value = value as u128;
    let bps = bps as u128;
    let denom = BPS_DENOM as u128;
    let product = value.checked_mul(bps).ok_or(OilshipError::MathOverflow)?;
    let half = denom / 2;
    let rounded = product
        .checked_add(half)
        .ok_or(OilshipError::MathOverflow)?
        .checked_div(denom)
        .ok_or(OilshipError::DivisionByZero)?;
    if rounded > u64::MAX as u128 {
        return err!(OilshipError::MathOverflow);
    }
    Ok(rounded as u64)
}

pub fn safe_add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b).ok_or_else(|| error!(OilshipError::MathOverflow))
}

pub fn safe_sub(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b).ok_or_else(|| error!(OilshipError::MathUnderflow))
}
