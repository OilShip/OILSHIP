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
