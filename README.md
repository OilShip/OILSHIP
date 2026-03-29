![OILSHIP — Strait Convoy](./assets/banner.png)

[![status](https://img.shields.io/badge/status-pre--launch-ff5b1f?style=for-the-badge)](#)
[![chain](https://img.shields.io/badge/chain-solana-1c1c1f?style=for-the-badge)](#)
[![program](https://img.shields.io/badge/program-anchor-ff5b1f?style=for-the-badge)](#)
[![sdk](https://img.shields.io/badge/sdk-typescript-1c1c1f?style=for-the-badge)](#)
[![cli](https://img.shields.io/badge/cli-python-ff5b1f?style=for-the-badge)](#)
[![watch](https://img.shields.io/badge/watch-rust-1c1c1f?style=for-the-badge)](#)
[![license](https://img.shields.io/badge/license-MIT-ff5b1f?style=for-the-badge)](#)

# OILSHIP — Strait Convoy

> Solana has one chokepoint to the rest of crypto: **the bridges**.
> Pirates wait there. OILSHIP is a convoy company that escorts your
> transit, monitors the strait, and pays you out of its **Wreck Fund**
> if anything sinks.

Since 2021, cross-chain bridges have lost more than **$2.8 billion** to
pirates. Wormhole. Ronin. Nomad. Multichain. Orbit. The pirates know
exactly where to wait. The existing answer is "buy insurance from a DAO
that votes on your claim for two weeks". That isn't insurance — that's
a wake.

OILSHIP is the answer that actually fits a Solana trader's life:

- **One toll, one decision, one transit.** You pay 10 bps and a tanker
  carries your cargo through the strait.
- **Same-block payouts.** If the watch engine flags the bridge while
  your policy is open, you walk away with your principal in the same
  block. No DAO vote. No claim form. The code pays.
- **Owned by shareholders.** Holding `$OIL` is a share in the fleet,
  the tolls and the Wreck Fund.

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
