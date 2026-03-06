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
