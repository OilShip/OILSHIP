# Contributing to OILSHIP

Thanks for taking an interest in the convoy.

## Quick start

```bash
git clone https://github.com/OilShip/OILSHIP.git
cd OILSHIP

# build the on-chain program
anchor build

# build the watcher
cargo build -p oilship-watch --release

# build the sdk
npm install
npm run build --workspace sdk
```

## Structure

| Path | What it is |
|---|---|
| `programs/oilship/` | Anchor program (Rust) |
| `watch/` | Risk scoring engine (Rust + Tokio) |
| `sdk/` | TypeScript client |
| `cli/` | Operator CLI (Python) |
| `docs/` | Architecture and protocol notes |

## Pull requests

1. Open an issue first for anything beyond a typo or one-line fix.
2. Keep PRs small and focused. One concern per PR.
3. Make sure CI is green before requesting review.
4. Conventional commit messages: `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `perf:`, `test:`.

## Code style

- Rust: `cargo fmt` + `cargo clippy --all-targets`
- TypeScript: `npm run lint` inside `sdk/`
- Python (cli): `ruff check` + `ruff format`

## Reporting issues

Use the issue tracker. Include:
- What you expected
- What actually happened
- Steps to reproduce
- Anchor / Rust / Node versions

## Security

If you find a vulnerability, please do not open a public issue. Reach the team on X at [@Oilship2026](https://x.com/Oilship2026) and we will set up a private channel.
