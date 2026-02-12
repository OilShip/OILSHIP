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

pub fn safe_div(numer: u64, denom: u64) -> Result<u64> {
    if denom == 0 {
        return err!(OilshipError::DivisionByZero);
    }
    Ok(numer / denom)
}

pub fn compute_toll(cargo: u64, toll_bps: u16) -> Result<u64> {
    apply_bps(cargo, toll_bps)
}

pub fn split_toll(toll: u64, fund_bps: u16, buyback_bps: u16, ops_bps: u16) -> Result<(u64, u64, u64)> {
    let total = fund_bps as u32 + buyback_bps as u32 + ops_bps as u32;
    if total != BPS_DENOM as u32 {
        return err!(OilshipError::InvalidSplit);
    }
    let buyback = apply_bps(toll, buyback_bps)?;
    let ops = apply_bps(toll, ops_bps)?;
    let fund = safe_sub(toll, safe_add(buyback, ops)?)?;
    Ok((fund, buyback, ops))
}

pub fn risk_multiplier_bps(score: u8) -> u16 {
    match score {
        0..=20 => 9_500,
        21..=40 => 10_000,
        41..=60 => 11_500,
        61..=80 => 13_500,
        _ => 19_000,
    }
}

pub fn apply_risk_multiplier(base_toll: u64, score: u8) -> Result<u64> {
    let mult = risk_multiplier_bps(score);
    apply_bps(base_toll, mult)
}
