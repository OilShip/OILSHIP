# oilship cli

Operator CLI for the OILSHIP protocol. Inspect on-chain state, simulate threat scenarios, and dump policy receipts.

## Install

```bash
git clone https://github.com/OilShip/OILSHIP.git
cd OILSHIP/cli
pip install -e .
```

Python 3.12+ required.

## Commands

| Command | What it does |
|---|---|
| `oilship state dump <pubkey>` | Decode and print any OILSHIP account by pubkey |
| `oilship threat simulate <scenario.json>` | Run a risk score scenario through the local simulator |
| `oilship policy inspect <pubkey>` | Show open/settled/claimed state for a policy |
| `oilship-watch sample <bridge>` | Sample a bridge once via the watch engine |

## Example

```bash
oilship threat simulate ./scenario.json
# bridge       : mayan
# baseline     : 18
# applied      : TvlDrop (high) + AdminKeyRotation (critical)
# new score    : 84
# verdict      : QUARANTINED (>= 81)
```

## Configuration

Configure via environment variables. See [`.env.example`](../.env.example).

## License

MIT. See [LICENSE](../LICENSE).
