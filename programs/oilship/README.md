# oilship (anchor program)

The on-chain program for the OILSHIP convoy protocol. Built with Anchor 0.30 on Solana 1.18.

## Build

```bash
git clone https://github.com/OilShip/OILSHIP.git
cd OILSHIP
anchor build
```

## Instructions

| Instruction | What it does |
|---|---|
| `initialize` | Bootstrap GlobalConfig, WreckFund, Treasury PDAs |
| `register_bridge` | Admin only. Register a bridge for routing |
| `update_risk` | Watch engine writes the latest risk score (0-100) |
| `open_policy` | User pays toll, fund earmarks coverage, mints Policy account |
| `settle_policy` | Healthy close. Fund releases its earmark |
| `claim_payout` | Quarantined close. Fund pays principal back same block |

## Accounts

| Account | Kind | Holds |
|---|---|---|
| `GlobalConfig` | singleton | admin, toll bps, split bps, paused flag |
| `Bridge` | per bridge | symbol, risk score, tier, routable, open coverage |
| `Policy` | per transit | beneficiary, cargo, toll, risk at open, state |
| `WreckFund` | PDA | balance, open coverage, lifetime payouts |
| `Treasury` | PDA | ops + buyback bucket |

## Risk score multiplier

| Score | Multiplier | Tier | State |
|---|---|---|---|
| 0-20 | 0.95x | Tier 1 | primary route |
| 21-40 | 1.00x | Tier 1 | normal |
| 41-60 | 1.15x | Tier 2 | fallback |
| 61-80 | 1.35x | Tier 3 | rate-limited |
| 81-100 | quarantine | block | new policies revert |

## Wreck Fund accounting

```
solvency = balance - open_coverage
```

If a new policy would push `solvency` negative, the instruction reverts.
The fund cannot oversell coverage by design.

## Program ID

`Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS` (placeholder, not deployed)

## License

MIT. See [LICENSE](../../LICENSE).
