# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace `Cargo.toml` and root `Anchor.toml`
- Root `package.json` for the SDK workspace
- `LICENSE`, `CONTRIBUTING.md`, `SECURITY.md`
- CI workflow under `.github/workflows/ci.yml`

## [0.1.0] - 2026-04-09

### Added
- Anchor program: `programs/oilship/`
  - `initialize`, `register_bridge`, `update_risk`, `open_policy`, `settle_policy`, `claim_payout`
  - `GlobalConfig`, `Bridge`, `Policy`, `WreckFund`, `Treasury` accounts
- Watch engine: `watch/`
  - Multi-RPC streaming, 12 risk signals, 0–100 scorer
  - HTTP API, audit trail, anomaly filters
- TypeScript SDK: `sdk/`
  - Zero runtime deps
  - `OilshipClient`, `Escort`, PDA helpers, event decoder, simulator, receipts
- Operator CLI: `cli/`
  - `oilship state dump`, `oilship threat simulate`, `oilship-watch sample`
- Documentation: `docs/architecture.md`

[Unreleased]: https://github.com/OilShip/OILSHIP/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/OilShip/OILSHIP/releases/tag/v0.1.0
