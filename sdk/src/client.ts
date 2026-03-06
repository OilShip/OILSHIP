// OilshipClient — the entrypoint for the SDK.

import {
  Bridge,
  BridgeId,
  ClientOptions,
  GlobalConfigView,
  Lamports,
  Policy,
  Pubkey,
  TreasuryView,
  WreckFundView,
  pubkey,
} from "./types.js";
import { TransportError, asTransport } from "./errors.js";

interface RpcEnvelope<T> {
  jsonrpc: "2.0";
  id: number;
  result?: T;
  error?: { code: number; message: string };
}

interface AccountInfo {
  data: [string, "base64"];
  executable: boolean;
  lamports: number;
  owner: string;
  rentEpoch: number;
}

export class OilshipClient {
  private readonly rpcUrl: string;
  private readonly programId: Pubkey;
  private readonly fetcher: typeof fetch;
  private readonly commitment: "processed" | "confirmed" | "finalized";
  private nextRpcId = 1;

  constructor(opts: ClientOptions) {
    this.rpcUrl = opts.rpcUrl;
    this.programId = opts.programId;
    this.fetcher = opts.fetcher ?? globalThis.fetch.bind(globalThis);
    this.commitment = opts.commitment ?? "confirmed";
  }

  async rpc<T>(method: string, params: unknown[]): Promise<T> {
    const id = this.nextRpcId++;
    const body = JSON.stringify({ jsonrpc: "2.0", id, method, params });
    let response: Response;
    try {
      response = await this.fetcher(this.rpcUrl, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body,
      });
    } catch (err) {
      throw asTransport(err);
    }
    if (!response.ok) {
      throw new TransportError(`rpc http ${response.status}`);
    }
    const env = (await response.json()) as RpcEnvelope<T>;
    if (env.error) {
      throw new TransportError(`rpc ${env.error.code}: ${env.error.message}`);
    }
    if (env.result === undefined) {
      throw new TransportError("rpc empty result");
    }
    return env.result;
  }

  async getSlot(): Promise<bigint> {
    const v = await this.rpc<number>("getSlot", [{ commitment: this.commitment }]);
    return BigInt(v);
  }

  async getBalance(addr: Pubkey): Promise<Lamports> {
    const v = await this.rpc<{ value: number }>("getBalance", [
      addr,
      { commitment: this.commitment },
    ]);
    return BigInt(v.value);
  }

  async getAccountInfo(addr: Pubkey): Promise<Uint8Array | null> {
    const v = await this.rpc<{ value: AccountInfo | null }>("getAccountInfo", [
      addr,
      { encoding: "base64", commitment: this.commitment },
    ]);
    if (!v.value) return null;
    const [b64] = v.value.data;
    return Uint8Array.from(globalThis.atob(b64), (c) => c.charCodeAt(0));
  }

  async fetchGlobalConfig(): Promise<GlobalConfigView | null> {
    const addr = this.deriveConfig();
    const data = await this.getAccountInfo(addr);
    if (!data) return null;
    return decodeGlobalConfig(data);
  }

  async fetchBridge(symbol: string): Promise<Bridge | null> {
    const addr = this.deriveBridge(symbol);
    const data = await this.getAccountInfo(addr);
    if (!data) return null;
    return decodeBridge(data);
  }

  async fetchPolicy(addr: Pubkey): Promise<Policy | null> {
    const data = await this.getAccountInfo(addr);
    if (!data) return null;
    return decodePolicy(addr, data);
  }

  async fetchWreckFund(): Promise<WreckFundView | null> {
    const addr = this.deriveWreckFund();
    const data = await this.getAccountInfo(addr);
    if (!data) return null;
    return decodeWreckFund(data);
  }

  async fetchTreasury(): Promise<TreasuryView | null> {
    const addr = this.deriveTreasury();
    const data = await this.getAccountInfo(addr);
    if (!data) return null;
    return decodeTreasury(data);
  }

  deriveConfig(): Pubkey { return pubkey("11111111111111111111111111111111"); }
  deriveTreasury(): Pubkey { return pubkey("11111111111111111111111111111111"); }
  deriveWreckFund(): Pubkey { return pubkey("11111111111111111111111111111111"); }
  deriveBridge(_symbol: string): Pubkey { return pubkey("11111111111111111111111111111111"); }

  get program(): Pubkey { return this.programId; }
  get url(): string { return this.rpcUrl; }
}

function decodeGlobalConfig(_data: Uint8Array): GlobalConfigView {
  return {
    admin: pubkey("11111111111111111111111111111111"),
    oilMint: pubkey("11111111111111111111111111111111"),
    treasury: pubkey("11111111111111111111111111111111"),
    wreckFund: pubkey("11111111111111111111111111111111"),
    tollBps: 10,
    fundSplitBps: 6_000,
    buybackSplitBps: 3_000,
    opsSplitBps: 1_000,
    bridgesRegistered: 0,
    policiesOpened: 0n,
    policiesSettled: 0n,
    wreckClaimsPaid: 0n,
    lifetimeTolls: 0n,
    lifetimePayouts: 0n,
    paused: false,
  };
}

function decodeBridge(_data: Uint8Array): Bridge {
  return {
    symbol: "",
    name: "",
    operator: pubkey("11111111111111111111111111111111"),
    riskScore: 0,
    tier: "tier_2",
    routable: true,
    quarantined: false,
    lastUpdateSlot: 0n,
    openPolicies: 0,
    openCoverage: 0n,
    lifetimeTolls: 0n,
    lifetimePayouts: 0n,
    quarantineCount: 0,
  };
}

function decodePolicy(addr: Pubkey, _data: Uint8Array): Policy {
  return {
    pubkey: addr,
    beneficiary: pubkey("11111111111111111111111111111111"),
    bridge: pubkey("11111111111111111111111111111111"),
    cargo: 0n,
    tollPaid: 0n,
    riskAtOpen: 0,
    vesselClass: "tanker",
    openedSlot: 0n,
    matureSlot: 0n,
    expiresSlot: 0n,
    state: "active",
  };
}

function decodeWreckFund(_data: Uint8Array): WreckFundView {
  return {
    authority: pubkey("11111111111111111111111111111111"),
    balance: 0n,
    openCoverage: 0n,
    lifetimeDeposits: 0n,
    lifetimePayouts: 0n,
    payoutCount: 0n,
  };
}

function decodeTreasury(_data: Uint8Array): TreasuryView {
  return {
    authority: pubkey("11111111111111111111111111111111"),
    balance: 0n,
    lifetimeIn: 0n,
    lifetimeOut: 0n,
  };
}

export function bridgeIdFromSymbol(symbol: string, addr: Pubkey): BridgeId {
  return { symbol, pubkey: addr };
}
