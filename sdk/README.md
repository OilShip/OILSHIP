# @oilship/sdk

TypeScript client for the OILSHIP convoy protocol on Solana. Zero runtime dependencies beyond `@solana/web3.js` and `@coral-xyz/anchor`.

## Install

```bash
git clone https://github.com/OilShip/OILSHIP.git
cd OILSHIP/sdk
npm install
npm run build
```

## Quick start

```ts
import { Connection, PublicKey } from "@solana/web3.js";
import { OilshipClient, Escort, solToLamports, pubkey } from "@oilship/sdk";

const client = new OilshipClient({
  connection: new Connection("https://api.mainnet-beta.solana.com"),
  programId: pubkey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"),
});

const escort = new Escort(client, 10);

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
```

## Modules

| Module | What it does |
|---|---|
| `client.ts` | `OilshipClient` RPC adapter with high-level reads |
| `escort.ts` | `Escort` quote, prepareOpen, openPolicy |
| `pda.ts` | PDA derivation helpers |
| `events.ts` | Binary event decoder |
| `simulator.ts` | Local risk + toll simulator |
| `receipts.ts` | Policy and payout receipts |

## Scripts

```bash
npm run build         # tsc compile
npm run test          # node --test
npm run lint          # tsc --noEmit
npm run format        # prettier
npm run format:check  # prettier check
npm run clean         # rm -rf dist
```

## License

MIT. See [LICENSE](../LICENSE).
