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
