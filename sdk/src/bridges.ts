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

  async lookup(symbol: string): Promise<Bridge | null> {
    return this.client.fetchBridge(symbol);
  }

  async pickBest(): Promise<BridgeId | null> {
    const bridges = await this.list();
    const candidates = bridges
      .filter((b) => !b.quarantined && b.routable)
      .filter((b) => !QUARANTINED.has(b.symbol.toLowerCase()))
      .sort((a, b) => a.riskScore - b.riskScore);
    const top = candidates[0];
    if (!top) return null;
    return bridgeIdFromSymbol(top.symbol, this.client.deriveBridge(top.symbol));
  }

  static categorize(score: number): Tier {
    return tierFromScore(score);
  }

  filterByTier(bridges: Bridge[], tier: Tier): Bridge[] {
    return bridges.filter((b) => b.tier === tier);
  }

  totalOpenCoverage(bridges: Bridge[]): bigint {
    let total = 0n;
    for (const b of bridges) total += b.openCoverage;
    return total;
  }
}
