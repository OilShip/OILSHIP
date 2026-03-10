// Escort — the highest-level public API of the SDK.

import {
  Bridge,
  EscortQuote,
  Lamports,
  Pubkey,
  classFromCargo,
} from "./types.js";
import { OilshipClient } from "./client.js";
import { Bridges } from "./bridges.js";
import { Router, RouteCandidate } from "./routing.js";
import { Policies, OpenPolicyRequest } from "./policy.js";
import { WreckFund } from "./wreckFund.js";
import { ValidationError, QuarantinedError } from "./errors.js";
import { bpsOf, fmtSol, fmtBps } from "./utils.js";

export interface EscortOpenInput {
  beneficiary: Pubkey;
  cargo: Lamports;
  lifetimeHours: number;
  preferredBridge?: string;
  excludeBridges?: string[];
  maxRiskScore?: number;
}

export interface EscortQuoteInput {
  cargo: Lamports;
  preferredBridge?: string;
  excludeBridges?: string[];
  maxRiskScore?: number;
}

export interface PreparedOpen {
  request: OpenPolicyRequest;
  quote: EscortQuote;
  route: RouteCandidate;
  derivedPolicyAccount: Pubkey;
}

export class Escort {
  private readonly bridges: Bridges;
  private readonly router: Router;
  private readonly policies: Policies;
  private readonly fund: WreckFund;

  constructor(
    private readonly client: OilshipClient,
    private readonly tollBps: number,
  ) {
    this.bridges = new Bridges(client);
    this.router = new Router(client, tollBps);
    this.policies = new Policies(client);
    this.fund = new WreckFund(client);
  }

  async quote(input: EscortQuoteInput): Promise<EscortQuote> {
    if (input.cargo <= 0n) throw new ValidationError("cargo must be > 0");
    const allBridges = await this.bridges.list();
    const route = await this.router.best(
      {
        cargo: input.cargo,
        preferredSymbols: input.preferredBridge ? [input.preferredBridge] : [],
        excludeSymbols: input.excludeBridges ?? [],
        maxRiskScore: input.maxRiskScore,
      },
      allBridges,
    );
    return {
      bridge: route.bridge,
      cargo: input.cargo,
      baseToll: bpsOf(input.cargo, this.tollBps),
      riskAdjustedToll: route.expectedToll,
      tier: route.tier,
      riskScore: route.riskScore,
      vesselClass: route.vesselClass,
      validUntilSlot: 0n,
    };
  }

  async prepareOpen(input: EscortOpenInput): Promise<PreparedOpen> {
    const allBridges = await this.bridges.list();
    const route = await this.router.best(
      {
        cargo: input.cargo,
        preferredSymbols: input.preferredBridge ? [input.preferredBridge] : [],
        excludeSymbols: input.excludeBridges ?? [],
        maxRiskScore: input.maxRiskScore,
      },
      allBridges,
    );
    if (route.bridgeAccount.quarantined) {
      throw new QuarantinedError(route.bridgeAccount.symbol);
    }
    await this.fund.assertCanCover(input.cargo);
    const request: OpenPolicyRequest = {
      beneficiary: input.beneficiary,
      bridgeSymbol: route.bridgeAccount.symbol,
      cargo: input.cargo,
      lifetimeHours: input.lifetimeHours,
      baseTollBps: this.tollBps,
    };
    this.policies.validate(request);
    const quote: EscortQuote = {
      bridge: route.bridge,
      cargo: input.cargo,
      baseToll: bpsOf(input.cargo, this.tollBps),
      riskAdjustedToll: route.expectedToll,
      tier: route.tier,
      riskScore: route.riskScore,
      vesselClass: route.vesselClass,
      validUntilSlot: await this.client.getSlot(),
    };
    return {
      request,
      quote,
      route,
      derivedPolicyAccount: this.client.deriveBridge(route.bridgeAccount.symbol),
    };
  }
