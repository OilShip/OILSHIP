<p align="center">
  <img src="./assets/banner.png" alt="OILSHIP"/>
</p>

<p align="center">
  <a href="https://oilship.xyz/"><img src="https://img.shields.io/badge/website-oilship.xyz-ff5b1f?style=for-the-badge" alt="website"/></a>
  <a href="https://x.com/Oilship2026"><img src="https://img.shields.io/badge/x-@Oilship2026-1c1c1f?style=for-the-badge&logo=x" alt="x"/></a>
  <a href="https://github.com/OilShip/OILSHIP"><img src="https://img.shields.io/badge/github-OilShip%2FOILSHIP-ff5b1f?style=for-the-badge&logo=github" alt="github"/></a>
  <a href="https://github.com/OilShip/OILSHIP/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/OilShip/OILSHIP/ci.yml?branch=main&style=for-the-badge&label=ci" alt="ci"/></a>
  <a href="./LICENSE"><img src="https://img.shields.io/github/license/OilShip/OILSHIP?style=for-the-badge" alt="license"/></a>
</p>

<p align="center">
  <a href="https://oilship.xyz/"><img src="https://img.shields.io/badge/status-pre--launch-1c1c1f?style=for-the-badge" alt="status"/></a>
  <a href="https://solana.com"><img src="https://img.shields.io/badge/chain-solana-ff5b1f?style=for-the-badge" alt="chain"/></a>
  <a href="https://www.anchor-lang.com/"><img src="https://img.shields.io/badge/program-anchor-1c1c1f?style=for-the-badge" alt="program"/></a>
  <a href="./sdk"><img src="https://img.shields.io/badge/sdk-typescript-ff5b1f?style=for-the-badge" alt="sdk"/></a>
  <a href="./watch"><img src="https://img.shields.io/badge/watch-rust-1c1c1f?style=for-the-badge" alt="watch"/></a>
  <a href="https://oilship.xyz/"><img src="https://img.shields.io/badge/ticker-%24OIL-ff5b1f?style=for-the-badge" alt="ticker"/></a>
</p>

<p align="center">
  <a href="https://oilship.xyz/">Website</a>
  &nbsp;·&nbsp;
  <a href="https://oilship.xyz/docs/">Docs</a>
  &nbsp;·&nbsp;
  <a href="https://x.com/Oilship2026">X</a>
  &nbsp;·&nbsp;
  <a href="https://github.com/OilShip/OILSHIP">GitHub</a>
</p>

# OILSHIP

On-chain bridge insurance for Solana. Anchor program + Rust risk engine + TypeScript SDK.

Since 2021, cross-chain bridges have lost more than **$2.8 billion** to exploits (Wormhole, Ronin, Nomad, Multichain, Orbit). Existing insurance protocols run on Ethereum, gate claims behind a 14-day DAO vote, and don't cover Solana routes at all. OILSHIP is a Solana-native protocol that prices bridge risk in real time, pulls a 10 bps toll per transit, and pays principal back from the on-chain Wreck Fund in the same block when a covered bridge gets quarantined.

## Features

| Feature | Status |
|---|---|
| Anchor program (`programs/oilship`) | stable |
| Risk scoring engine (`watch`) | stable |
| TypeScript SDK (`sdk`) | stable |
| Operator CLI (`cli`) | beta |
| Wreck Fund accounting (PDA) | stable |
| Multi-bridge router | beta |
| Same-block payout flow | stable |
| Dark fleet (split + time-spaced) | alpha |

---

## Architecture

```mermaid
flowchart LR
  subgraph user[User wallet]
    U[swap intent]
  end

  subgraph sdk[OILSHIP SDK]
    Q[escort.quote]
    P[escort.prepareOpen]
  end

  subgraph chain[Solana program]
    OP[open_policy]
    SP[settle_policy]
    CL[claim_payout]
    WF[(Wreck Fund PDA)]
    BR[(Bridge accounts)]
  end

  subgraph watch[Rust watch engine]
    RPC[multi-chain RPC]
    SIG[signal extractors]
    SCO[risk scorer]
    UR[update_risk]
  end

  U --> Q --> P --> OP --> WF
  RPC --> SIG --> SCO --> UR --> BR
  BR -. risk .-> OP
  OP -- earmarks --> WF
  CL -- payout --> U
  SP -- release --> WF
```

Four components, all in this repo:

| Path | Component | Stack |
|---|---|---|
| `programs/oilship/` | On-chain program | Rust, Anchor 0.30, solana-program 1.18 |
| `watch/`            | Risk scoring engine | Rust, Tokio, solana-client |
| `sdk/`              | TypeScript SDK (zero runtime deps) | TypeScript, @solana/web3.js, @coral-xyz/anchor |
| `cli/`              | Operator CLI | Python 3.12, Typer, anchorpy |

---

## Mechanism

```mermaid
sequenceDiagram
  participant U as User
  participant SDK as SDK
  participant P as Anchor program
  participant W as Wreck Fund
  participant E as Watch Engine

  U->>SDK: cargo + lifetime
  SDK->>P: open_policy(cargo, lifetime)
  P->>W: earmark coverage
  P-->>U: policy account
  loop every poll
    E->>P: update_risk(score)
  end
  alt healthy
    U->>P: settle_policy()
    P->>W: release earmark
  else quarantined
    U->>P: claim_payout()
    P->>W: pay principal back
    W-->>U: lamports same block
  end
```

The toll a user pays is the **base toll** (10 bps of cargo) multiplied by a **risk multiplier** read off the bridge's current score:

| Score   | Multiplier | Tier   | State            |
|---------|------------|--------|------------------|
| 0-20    | 0.95x      | Tier 1 | primary route    |
| 21-40   | 1.00x      | Tier 1 | normal           |
| 41-60   | 1.15x      | Tier 2 | fallback         |
| 61-80   | 1.35x      | Tier 3 | rate-limited     |
| 81-100  | quarantine | block  | new policies revert |

Above score 80, the bridge is quarantined and the program refuses to open new policies on it.

## Performance

| Metric                       | Value           |
|------------------------------|-----------------|
| Risk score recompute window  | 1s              |
| Watch poll latency (p50)     | ~180ms          |
| `open_policy` instruction CU | ~28k            |
| `claim_payout` instruction CU| ~22k            |
| Same-block payout latency    | 1 slot (~400ms) |
| Wreck Fund accounting        | atomic per ix   |
| Concurrent policies / bridge | unbounded       |
| Coverage oversell guard      | hard revert     |

---

## On-chain accounts

```mermaid
classDiagram
  class GlobalConfig {
    Pubkey admin
    Pubkey wreckFund
    u16 tollBps
    u16 fundSplitBps
    u16 buybackSplitBps
    u16 opsSplitBps
    bool paused
  }
  class Bridge {
    String symbol
    Pubkey operator
    u8 riskScore
    u8 tier
    bool routable
    bool quarantined
    u32 openPolicies
    u64 openCoverage
  }
  class Policy {
    Pubkey beneficiary
    Pubkey bridge
    u64 cargo
    u64 tollPaid
    u8 riskAtOpen
    u8 class
    u8 state
  }
  class WreckFund {
    Pubkey authority
    u64 balance
    u64 openCoverage
    u64 lifetimePayouts
  }
  class Treasury {
    Pubkey authority
    u64 balance
  }
  GlobalConfig --> WreckFund
  GlobalConfig --> Treasury
  GlobalConfig --> Bridge : registers
  Bridge --> Policy : hosts
  Policy --> WreckFund : earmarks
```

---

## Token economics

`$OIL` is the company share. The protocol does one thing and the share captures cashflow from that one thing.

```
toll = bps_of(cargo, 10) * risk_multiplier(score)
   |
   +-- 60% --> wreck_fund    (grows the coverage cap)
   +-- 30% --> $OIL buyback  (direct to holders)
   +-- 10% --> operations    (RPCs, signers, infra)
```

| Quantity | Formula                                            |
|----------|----------------------------------------------------|
| NAV      | `wreck_fund + accrued_tolls - open_risk`           |
| APR      | `(tolls - payouts) / wreck_fund`                   |
| Floor    | `wreck_fund / circulating_supply`                  |
| TAM      | Solana monthly bridge inflow, measurable on chain  |

---

## Project structure

```
OILSHIP/
├── Anchor.toml                anchor workspace
├── Cargo.toml                 rust workspace
├── package.json               sdk workspace
├── rust-toolchain.toml
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── SECURITY.md
├── CHANGELOG.md
├── .github/workflows/
│   └── ci.yml                 rust + sdk + cli + docs jobs
├── assets/
│   ├── banner.png
│   └── logo.png
├── programs/oilship/src/
│   ├── lib.rs                 program entrypoint
│   ├── state.rs               GlobalConfig, Bridge, Policy, WreckFund, Treasury
│   ├── errors.rs              custom error codes
│   └── instructions/          initialize, register_bridge, update_risk,
│                              open_policy, settle_policy, claim_payout
├── watch/src/
│   ├── main.rs                tokio runtime entrypoint
│   ├── rpc.rs                 multi-RPC streaming
│   ├── filters.rs             anomaly extractors (12 signals)
│   ├── notifier.rs            risk score writer (update_risk caller)
│   ├── http_api.rs            local query api
│   ├── backfill.rs            historical replay
│   └── admin_audit.rs         admin key + signer rotation tracker
├── sdk/src/
│   ├── client.ts              OilshipClient
│   ├── escort.ts              quote, prepareOpen, openPolicy
│   ├── pda.ts                 PDA derivation helpers
│   ├── events.ts              event decoder
│   ├── simulator.ts           dry-run risk + toll
│   ├── receipts.ts            policy + payout receipts
│   └── version.ts
├── cli/oilship/
│   ├── state/dump.py          oilship state dump <pubkey>
│   ├── threat/simulate.py     oilship threat simulate scenario.json
│   └── policy/inspect.py      oilship policy inspect <pubkey>
└── docs/
    └── architecture.md
```

---

## Build

```bash
git clone https://github.com/OilShip/OILSHIP.git
cd OILSHIP

# on-chain program
anchor build

# risk scoring engine
cargo build -p oilship-watch --release

# typescript sdk
npm install
npm run build --workspace sdk

# operator cli
cd cli && pip install -e .
```

Anchor 0.30+, Rust 1.78+, Node 20+, Python 3.12+.

---

## Quick start

### Quote a transit (TypeScript SDK)

```ts
import { Connection, PublicKey } from "@solana/web3.js";
import { OilshipClient, Escort, solToLamports, pubkey } from "@oilship/sdk";

const client = new OilshipClient({
  connection: new Connection("https://api.mainnet-beta.solana.com"),
  programId: pubkey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"),
});

const escort = new Escort(client, 10); // base toll, bps

const quote = await escort.quote({
  cargo: solToLamports(1.5),
  preferredBridge: "mayan",
});
// quote = {
//   cargo: 1500000000,
//   bridge: "mayan",
//   riskScore: 18,
//   tier: 1,
//   tollLamports: 1425000,
//   multiplier: 0.95,
//   route: ["mayan"],
//   coverageEarmark: 1500000000,
// }

const ix = await escort.prepareOpen(quote, walletPubkey);
// returns a TransactionInstruction ready to sign + send
```

### Read protocol state

```ts
const state = await client.getProtocolState();
// state = {
//   admin: "...",
//   wreckFund: "...",
//   tollBps: 10,
//   fundSplitBps: 6000,
//   buybackSplitBps: 3000,
//   opsSplitBps: 1000,
//   paused: false,
// }

const fund = await client.getWreckFund();
// fund = {
//   balance: 482730000000,
//   openCoverage: 311500000000,
//   solvency: 171230000000,   // balance - openCoverage
//   lifetimePayouts: 0,
// }
```

### Watch a bridge (Rust engine)

```bash
oilship-watch sample mayan
# polling mayan via 3 rpcs ...
# tvl baseline 41200000 sol, holders 12873
# admin key inspector ok
# signer set hash 0x9fa1...c4b2 (stable 7d)
# emitted score: 18 (tier 1, multiplier 0.95x)
```

### Simulate a risk scenario (CLI)

```bash
oilship threat simulate ./scenario.json
# bridge       : mayan
# baseline     : 18
# applied      : TvlDrop (high) + AdminKeyRotation (critical)
# new score    : 84
# verdict      : QUARANTINED (>= 81)
# action       : new policies on mayan will revert
```

`scenario.json`:

```json
{
  "bridge": "mayan",
  "anomalies": [
    { "kind": "TvlDrop", "severity": "high", "message": "tvl down 27% in 24h" },
    { "kind": "AdminKeyRotation", "severity": "critical", "message": "admin key moved twice" }
  ]
}
```

---

## Status

OILSHIP is **pre-launch**. The Wreck Fund is seeded at launch from the token raise, and the very first transit will be the team's own.

---

## Links

- **Website:** [oilship.xyz](https://oilship.xyz/)
- **Docs:** [oilship.xyz/docs](https://oilship.xyz/docs/)
- **X:** [@Oilship2026](https://x.com/Oilship2026)
- **GitHub:** [OilShip/OILSHIP](https://github.com/OilShip/OILSHIP)
- **Chain:** Solana
- **Ticker:** $OIL
