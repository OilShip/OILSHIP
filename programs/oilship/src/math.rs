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

pub fn reserve_ratio_bps(reserve: u64, open_coverage: u64) -> u16 {
    if open_coverage == 0 {
        return BPS_DENOM as u16;
    }
    let ratio = (reserve as u128 * BPS_DENOM as u128) / open_coverage as u128;
    if ratio >= u16::MAX as u128 {
        u16::MAX
    } else {
        ratio as u16
    }
}

pub fn slots_to_days(slots: u64) -> u64 { slots / 216_000 }
pub fn days_to_slots(days: u64) -> u64 { days.saturating_mul(216_000) }

pub fn check_capacity(reserve: u64, requested: u64) -> Result<()> {
    if requested > reserve {
        return err!(OilshipError::InsufficientReserve);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apply_bps_basic() {
        assert_eq!(apply_bps(1_000, 250).unwrap(), 25);
        assert_eq!(apply_bps(1_000, 10_000).unwrap(), 1_000);
        assert_eq!(apply_bps(0, 10_000).unwrap(), 0);
    }
    #[test]
    fn split_toll_sums_back() {
        let (fund, buyback, ops) = split_toll(1_000, 6_000, 3_000, 1_000).unwrap();
        assert_eq!(fund + buyback + ops, 1_000);
        assert_eq!(buyback, 300);
        assert_eq!(ops, 100);
        assert_eq!(fund, 600);
    }
    #[test]
    fn risk_curve_monotonic() {
        let mut last = 0u16;
        for score in [0u8, 21, 41, 61, 90] {
            let m = risk_multiplier_bps(score);
            assert!(m >= last);
            last = m;
        }
    }
}
