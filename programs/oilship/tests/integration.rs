//! Pure-rust unit tests for the OILSHIP on-chain program.

use oilship::constants::*;
use oilship::math::*;
use oilship::state::*;

#[test]
fn vessel_class_boundaries() {
    assert!(matches!(VesselClass::from_cargo(0), VesselClass::Coaster));
    assert!(matches!(VesselClass::from_cargo(LAMPORTS_PER_SOL - 1), VesselClass::Coaster));
    assert!(matches!(VesselClass::from_cargo(LAMPORTS_PER_SOL), VesselClass::Tanker));
    assert!(matches!(VesselClass::from_cargo(50 * LAMPORTS_PER_SOL), VesselClass::Capesize));
    assert!(matches!(VesselClass::from_cargo(250 * LAMPORTS_PER_SOL), VesselClass::DarkFleet));
}

#[test]
fn policy_state_round_trip() {
    for v in 0..=4u8 {
        let s = PolicyState::from_u8(v);
        assert_eq!(v, s as u8);
    }
}

#[test]
fn apply_bps_zero_value() {
    assert_eq!(apply_bps(0, 100).unwrap(), 0);
}

#[test]
fn apply_bps_full() {
    assert_eq!(apply_bps(123_456, 10_000).unwrap(), 123_456);
}

#[test]
fn apply_bps_round_half() {
    assert_eq!(apply_bps_round(1, 5_000).unwrap(), 1);
    assert_eq!(apply_bps_round(100, 250).unwrap(), 3);
}

#[test]
fn split_toll_invalid() {
    let r = split_toll(1_000, 5_000, 4_000, 500);
    assert!(r.is_err());
}

#[test]
fn split_toll_valid() {
    let (f, b, o) = split_toll(1_000, 6_000, 3_000, 1_000).unwrap();
    assert_eq!(f + b + o, 1_000);
}

#[test]
fn risk_curve_buckets() {
    assert_eq!(risk_multiplier_bps(0), 9_500);
    assert_eq!(risk_multiplier_bps(20), 9_500);
    assert_eq!(risk_multiplier_bps(21), 10_000);
    assert_eq!(risk_multiplier_bps(40), 10_000);
    assert_eq!(risk_multiplier_bps(41), 11_500);
    assert_eq!(risk_multiplier_bps(60), 11_500);
    assert_eq!(risk_multiplier_bps(61), 13_500);
    assert_eq!(risk_multiplier_bps(80), 13_500);
    assert_eq!(risk_multiplier_bps(81), 19_000);
    assert_eq!(risk_multiplier_bps(100), 19_000);
}

#[test]
fn reserve_ratio_no_open() {
    assert_eq!(reserve_ratio_bps(1_000, 0), BPS_DENOM as u16);
}

#[test]
fn reserve_ratio_full_cover() {
    assert_eq!(reserve_ratio_bps(1_000, 1_000), BPS_DENOM as u16);
}

#[test]
fn reserve_ratio_half() {
    let r = reserve_ratio_bps(500, 1_000);
    assert_eq!(r, 5_000);
}

#[test]
fn slots_to_days_round() {
    assert_eq!(slots_to_days(216_000), 1);
    assert_eq!(slots_to_days(216_000 * 2), 2);
    assert_eq!(slots_to_days(108_000), 0);
}

#[test]
fn days_to_slots_round() {
    assert_eq!(days_to_slots(0), 0);
    assert_eq!(days_to_slots(1), 216_000);
    assert_eq!(days_to_slots(2), 432_000);
}

#[test]
fn check_capacity_ok() {
    assert!(check_capacity(1_000, 500).is_ok());
    assert!(check_capacity(1_000, 1_000).is_ok());
}

#[test]
fn check_capacity_err() {
    assert!(check_capacity(500, 1_000).is_err());
}

#[test]
fn safe_add_overflow() {
    assert!(safe_add(u64::MAX, 1).is_err());
}

#[test]
fn safe_sub_underflow() {
    assert!(safe_sub(1, 2).is_err());
}
