# OILSHIP — Architecture

This document describes the four components that ship in this
repository, the contracts between them and the on-chain data they own.

## 1. Components

```
                  ┌──────────────┐
                  │   Watch      │
                  │   engine     │  rust + tokio
                  └──────┬───────┘
              risk update │
                          ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   SDK        │───▶│   Anchor     │◀───│   CLI        │
│ typescript   │    │   program    │    │   python     │
└──────────────┘    └──────────────┘    └──────────────┘
```

The on-chain program is the single source of truth. Every other
component reads it and most of them write to it.

## 2. The on-chain program

The Anchor program owns five account types:

| Account        | Owner   | What it holds |
|---|---|---|
| `GlobalConfig` | program | admin, treasury & wreck fund pubkeys, toll schedule, pause flag |
| `Bridge`       | program | per-bridge metadata + risk score + open coverage |
| `Policy`       | program | per-transit lifecycle state, cargo, expiry |
| `WreckFund`    | program | reserve balance, open coverage, lifetime payouts |
| `Treasury`     | program | operational balance, lifetime in/out |

Every state-changing instruction is one of:

- `initialize(params)`               — bootstrap the protocol
- `register_bridge(params)`         — admin adds a bridge
- `update_risk(score)`              — operator pushes a new risk score
- `open_policy(params)`             — user opens a transit policy
- `settle_policy()`                  — anyone settles a clean transit
- `claim_payout()`                   — user claims a wreck payout
- `deposit_fund(params)`            — anyone tops up the wreck fund
- `set_paused(bool)`                 — admin pauses the protocol
- `lift_quarantine()`                — admin lifts a bridge quarantine
- `open_convoy(params)`             — open a convoy account
