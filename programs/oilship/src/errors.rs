//! Typed errors raised by the OILSHIP on-chain program.

use anchor_lang::prelude::*;

#[error_code]
pub enum OilshipError {
    #[msg("Configuration account is already initialized.")]
    AlreadyInitialized,
    #[msg("Caller is not the protocol admin.")]
    NotAdmin,
    #[msg("Caller is not the bridge operator.")]
    NotBridgeOperator,
    #[msg("Bridge identifier is empty or too long.")]
    InvalidBridgeName,
    #[msg("Bridge symbol must be between 1 and 12 characters.")]
    InvalidBridgeSymbol,
    #[msg("Bridge registry is full; no more bridges can be added.")]
    BridgeRegistryFull,
    #[msg("Requested bridge is not registered with the protocol.")]
    BridgeNotFound,
    #[msg("Bridge is currently quarantined and cannot accept new policies.")]
    BridgeQuarantined,
    #[msg("Risk score is outside of the legal range (0..=100).")]
    InvalidRiskScore,
    #[msg("Toll is above the protocol-wide ceiling.")]
    TollTooHigh,
    #[msg("Toll splits do not sum to the basis-point denominator.")]
    InvalidSplit,
    #[msg("Cargo is below the minimum policy size.")]
    CargoTooSmall,
    #[msg("Cargo exceeds the maximum policy size.")]
    CargoTooLarge,
    #[msg("Policy lifetime is below the protocol minimum.")]
    PolicyTooShort,
    #[msg("Policy lifetime exceeds the protocol maximum.")]
    PolicyTooLong,
    #[msg("Policy is already settled.")]
    PolicyAlreadySettled,
    #[msg("Policy is not yet eligible to settle.")]
    PolicyNotMature,
    #[msg("Policy has expired and can no longer be settled.")]
    PolicyExpired,
    #[msg("Policy beneficiary mismatch.")]
    BeneficiaryMismatch,
    #[msg("Wreck fund has insufficient reserve to cover this policy.")]
    InsufficientReserve,
    #[msg("Reserve ratio would drop below the safety threshold.")]
    ReserveRatioBreach,
    #[msg("Convoy is full and cannot accept additional policies.")]
    ConvoyFull,
    #[msg("Convoy window has already closed.")]
    ConvoyWindowClosed,
    #[msg("Numeric overflow in fee calculation.")]
    MathOverflow,
    #[msg("Numeric underflow in fee calculation.")]
    MathUnderflow,
    #[msg("Division by zero.")]
    DivisionByZero,
    #[msg("Operation paused by the protocol admin.")]
    Paused,
    #[msg("Account discriminator mismatch.")]
    AccountMismatch,
    #[msg("Provided pubkey does not match the expected derivation.")]
    PdaMismatch,
    #[msg("Bridge operator already registered.")]
    OperatorAlreadyRegistered,
    #[msg("Coverage cap exceeded for this bridge.")]
    CoverageCapExceeded,
    #[msg("Wreck claim cannot be filed against a healthy bridge.")]
    BridgeStillHealthy,
    #[msg("Claim window has closed.")]
    ClaimWindowClosed,
    #[msg("Wreck Fund payout already disbursed for this policy.")]
    AlreadyClaimed,
    #[msg("Per-block bridge throughput exceeded.")]
    ThroughputExceeded,
}
