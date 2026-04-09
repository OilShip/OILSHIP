# OILSHIP — Architecture

> **Website** · [oilship.xyz](https://oilship.xyz/) &nbsp;·&nbsp; **X** · [@Oilship2026](https://x.com/Oilship2026) &nbsp;·&nbsp; **Repo** · [OilShip/OILSHIP](https://github.com/OilShip/OILSHIP)

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

## 3. The watch engine

The watch engine is a Rust binary (`watch/`) that polls every
configured bridge on a schedule and computes a fresh risk score from
its observations.

Adapter implementations live in `bridges.rs`. Each one knows how to
read its own bridge's on-chain footprint.

Anomaly extractors live in `signals.rs`. Risk scoring lives in
`score.rs`. The score is bounded above by 100 and capped per category
so a single extractor cannot dominate.

| Tier         | Score |
|---|---|
| Tier 1       | 0–30  |
| Tier 2       | 31–55 |
| Tier 3       | 56–80 |
| Quarantined  | 81+   |

## 4. The TypeScript SDK

`@oilship/sdk` is the binding library that frontends and integrating
dApps use. Public surface area:

- `OilshipClient`     — RPC adapter with high-level reads
- `Bridges`           — registry helpers
- `Router`            — picks the cheapest safe route
- `Policies`          — opens / settles / claims policies
- `WreckFund`         — reserve health and capacity checks
- `Escort`            — top-level facade
- `decoder.ts`        — binary decoder for every account type
- `risk.ts`           — local replica of the watch engine's scoring
- `errors.ts`         — typed error hierarchy

## 5. The Python CLI

`oilship-cli` is the operator and shareholder control surface.

```text
oilship status         — RPC + program + fleet snapshot
oilship fleet          — bridge table
oilship quote 1.5      — escort quote for 1.5 SOL
oilship open  1.5      — prepare an open-policy tx
oilship fund           — wreck fund + treasury P&L
oilship policy list    — policies for a beneficiary
oilship threat simulate — local risk simulator
oilship threat smooth   — EWMA-smooth a series of scores
oilship config show     — print the active config
oilship config save     — persist the active config
```

## 6. Cashflow

```
toll = bpsOf(cargo, 10) * risk_multiplier(score)
 ├── 60 % → wreck_fund   (grows the coverage cap)
 ├── 30 % → $OIL buyback (returned to holders)
 └── 10 % → operations
```

## 7. Failure modes & responses

| Failure | What the protocol does |
|---|---|
| Bridge TVL drops 25 % in 24 h | Watch engine raises score, possibly into Tier 3. |
| Bridge admin key rotates | Score climbs by 30 + severity factor. |
| Bridge is exploited | Watch engine quarantines the bridge. |
| Wreck fund reserve ratio drops below 15 % | Program rejects new policies. |
| Watch engine is offline | Routing still works against the last known score. |

## 8. Threat model

- The on-chain program is the trust root.
- The watch engine is **untrusted by users**. The worst it can do is
  raise a false alarm.
- The wreck fund is held in a PDA. There is no human key that can
  unilaterally drain it.

## 9. Glossary

- **Strait** — the bridge layer between Solana and another chain.
- **Tanker** — a single open policy.
- **Convoy** — a batch of policies opened in the same window.
- **Wreck Fund** — the on-chain insurance pool.
- **Wreck** — a payout event triggered by a quarantine.
- **Pirate** — anyone running an exploit against a bridge.
- **Class** — vessel class derived from cargo size.
