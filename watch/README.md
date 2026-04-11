# oilship-watch

Risk scoring engine for the OILSHIP protocol. Tails every supported Solana bridge across every chain it touches, extracts 12 anomaly signals, and writes a 0-100 risk score on-chain via `update_risk`.

## Build

```bash
git clone https://github.com/OilShip/OILSHIP.git
cd OILSHIP
cargo build -p oilship-watch --release
```

Rust 1.78+ required.

## Run

```bash
./target/release/oilship-watch --rpc https://api.devnet.solana.com
```

Or sample a single bridge once:

```bash
oilship-watch sample mayan
# polling mayan via 3 rpcs ...
# tvl baseline 41200000 sol, holders 12873
# admin key inspector ok
# signer set hash 0x9fa1...c4b2 (stable 7d)
# emitted score: 18 (tier 1, multiplier 0.95x)
```

## Modules

| Path | What it does |
|---|---|
| `src/main.rs` | Tokio runtime entrypoint |
| `src/rpc.rs` | Multi-RPC streaming and quorum |
| `src/filters.rs` | 12 signal extractors |
| `src/notifier.rs` | Risk score writer (calls `update_risk`) |
| `src/http_api.rs` | Local query API |
| `src/backfill.rs` | Historical replay |
| `src/admin_audit.rs` | Admin key and signer rotation tracker |

## Signals

- TVL deltas at 1s resolution
- Signer set rotations
- Admin key tx history
- Oracle drift vs reference feeds
- Withdrawal rate vs 30d baseline
- Guardian rotations
- Bridge contract upgrades
- Anomaly score windows

## License

MIT. See [LICENSE](../LICENSE).
