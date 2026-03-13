// Routing decisions for an escort.

import { Bridge, BridgeId, Lamports, Tier, VesselClass, classFromCargo } from "./types.js";
import { OilshipClient, bridgeIdFromSymbol } from "./client.js";
import { tollMultiplierBps, isSailable, tierLabel } from "./risk.js";
import { ValidationError, CapacityError } from "./errors.js";
import { bpsOf } from "./utils.js";

export interface RouteCandidate {
  bridge: BridgeId;
  bridgeAccount: Bridge;
  expectedToll: Lamports;
  tier: Tier;
  riskScore: number;
  vesselClass: VesselClass;
}

export interface RouteRequest {
  cargo: Lamports;
  preferredSymbols?: string[];
  excludeSymbols?: string[];
  maxRiskScore?: number;
}

export class Router {
  constructor(private readonly client: OilshipClient, private readonly baseTollBps: number) {}

  async pickRoutes(req: RouteRequest, bridges: Bridge[]): Promise<RouteCandidate[]> {
    if (req.cargo <= 0n) throw new ValidationError("cargo must be > 0");

    const exclude = new Set((req.excludeSymbols ?? []).map((s) => s.toLowerCase()));
    const prefer = new Set((req.preferredSymbols ?? []).map((s) => s.toLowerCase()));
    const maxRisk = req.maxRiskScore ?? 80;

    const candidates: RouteCandidate[] = [];
    for (const b of bridges) {
      const sym = b.symbol.toLowerCase();
      if (exclude.has(sym)) continue;
      if (b.quarantined) continue;
      if (!b.routable) continue;
      if (b.riskScore > maxRisk) continue;
      if (!isSailable(b.riskScore)) continue;

      const baseToll = bpsOf(req.cargo, this.baseTollBps);
      const adjustedToll = (baseToll * BigInt(tollMultiplierBps(b.riskScore))) / 10_000n;

      candidates.push({
        bridge: bridgeIdFromSymbol(b.symbol, this.client.deriveBridge(b.symbol)),
        bridgeAccount: b,
        expectedToll: adjustedToll,
        tier: b.tier,
        riskScore: b.riskScore,
        vesselClass: classFromCargo(req.cargo),
      });
    }

    candidates.sort((a, c) => {
      const aPref = prefer.has(a.bridgeAccount.symbol.toLowerCase()) ? -1 : 0;
      const cPref = prefer.has(c.bridgeAccount.symbol.toLowerCase()) ? -1 : 0;
      if (aPref !== cPref) return aPref - cPref;
      if (a.tier !== c.tier) return tierOrder(a.tier) - tierOrder(c.tier);
      const tollCmp = a.expectedToll - c.expectedToll;
      if (tollCmp !== 0n) return tollCmp < 0n ? -1 : 1;
      return a.riskScore - c.riskScore;
    });
