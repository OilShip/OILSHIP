// Bridge registry helpers.

import { Bridge, BridgeId, Tier } from "./types.js";
import { OilshipClient, bridgeIdFromSymbol } from "./client.js";
import { tierFromScore } from "./utils.js";

export const KNOWN_BRIDGES: ReadonlyArray<{ symbol: string; name: string; chains: string[] }> = [
  { symbol: "mayan", name: "Mayan Finance", chains: ["solana", "ethereum", "base"] },
  { symbol: "debridge", name: "deBridge", chains: ["solana", "arbitrum", "optimism"] },
  { symbol: "wormhole", name: "Wormhole Portal", chains: ["solana", "ethereum", "polygon", "bsc"] },
  { symbol: "allbridge", name: "Allbridge Core", chains: ["solana", "ethereum"] },
];

export const QUARANTINED: ReadonlySet<string> = new Set([
  "orbit", "multichain", "nomad", "ronin", "qubit",
]);

export class Bridges {
  constructor(private readonly client: OilshipClient) {}

  async list(): Promise<Bridge[]> {
    const out: Bridge[] = [];
    for (const meta of KNOWN_BRIDGES) {
      const b = await this.client.fetchBridge(meta.symbol);
      if (b) out.push(b);
    }
    return out;
  }
