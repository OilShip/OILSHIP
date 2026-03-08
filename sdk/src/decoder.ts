// Binary decoder for OILSHIP on-chain accounts.

import {
  Bridge,
  Policy,
  Pubkey,
  Tier,
  WreckFundView,
  TreasuryView,
  GlobalConfigView,
  PolicyState,
  VesselClass,
  pubkey,
} from "./types.js";
import { base58Encode } from "./utils.js";

class Reader {
  private offset = 0;
  constructor(private readonly buf: Uint8Array) {}

  remaining(): number { return this.buf.length - this.offset; }

  skip(n: number): void {
    this.checkRoom(n);
    this.offset += n;
  }

  u8(): number {
    this.checkRoom(1);
    return this.buf[this.offset++];
  }

  u16(): number {
    this.checkRoom(2);
    const v = this.buf[this.offset] | (this.buf[this.offset + 1] << 8);
    this.offset += 2;
    return v >>> 0;
  }

  u32(): number {
    this.checkRoom(4);
    const v =
      this.buf[this.offset] |
      (this.buf[this.offset + 1] << 8) |
      (this.buf[this.offset + 2] << 16) |
      (this.buf[this.offset + 3] << 24);
    this.offset += 4;
    return v >>> 0;
  }

  u64(): bigint {
    this.checkRoom(8);
    const lo = BigInt(this.u32());
    const hi = BigInt(this.u32());
    return (hi << 32n) | lo;
  }

  bool(): boolean {
    return this.u8() !== 0;
  }

  bytes(n: number): Uint8Array {
    this.checkRoom(n);
    const slice = this.buf.slice(this.offset, this.offset + n);
    this.offset += n;
    return slice;
  }

  pubkey(): Pubkey {
    return pubkey(base58Encode(this.bytes(32)));
  }

  fixedString(n: number): string {
    const bytes = this.bytes(n);
    let end = bytes.indexOf(0);
    if (end === -1) end = n;
    return new TextDecoder("utf-8").decode(bytes.subarray(0, end));
  }

  private checkRoom(n: number): void {
    if (this.offset + n > this.buf.length) {
      throw new Error(`reader overrun at offset ${this.offset} need ${n}`);
    }
  }
}

const MAX_NAME_LEN = 48;
const MAX_SYMBOL_LEN = 12;
const DISCRIMINATOR = 8;

function tierFromU8(v: number): Tier {
  switch (v) {
    case 1: return "tier_1";
    case 2: return "tier_2";
    case 3: return "tier_3";
    case 4: return "quarantined";
    default: return "tier_2";
  }
}

function policyStateFromU8(v: number): PolicyState {
  switch (v) {
    case 0: return "pending";
    case 1: return "active";
    case 2: return "settled";
    case 3: return "claimed";
    case 4: return "expired";
    default: return "pending";
  }
}

function vesselClassFromU8(v: number): VesselClass {
  switch (v) {
    case 0: return "coaster";
    case 1: return "tanker";
    case 2: return "capesize";
    case 3: return "dark_fleet";
    default: return "tanker";
  }
}

export function decodeBridge(buf: Uint8Array): Bridge {
  const r = new Reader(buf);
  r.skip(DISCRIMINATOR);
  const symbol = r.fixedString(MAX_SYMBOL_LEN);
  const name = r.fixedString(MAX_NAME_LEN);
  const operator = r.pubkey();
  const riskScore = r.u8();
  const tierByte = r.u8();
  const routable = r.bool();
  const quarantined = r.bool();
  const lastUpdateSlot = r.u64();
  const openPolicies = r.u32();
  const openCoverage = r.u64();
  const _throughputSlot = r.u64();
  const _throughputCount = r.u32();
  const lifetimeTolls = r.u64();
  const lifetimePayouts = r.u64();
  const quarantineCount = r.u16();
  return {
    symbol,
    name,
    operator,
    riskScore,
    tier: tierFromU8(tierByte),
    routable,
    quarantined,
    lastUpdateSlot,
    openPolicies,
    openCoverage,
    lifetimeTolls,
    lifetimePayouts,
    quarantineCount,
  };
}

export function decodePolicy(addr: Pubkey, buf: Uint8Array): Policy {
  const r = new Reader(buf);
  r.skip(DISCRIMINATOR);
  const beneficiary = r.pubkey();
  const bridge = r.pubkey();
  const _convoy = r.pubkey();
  const cargo = r.u64();
  const tollPaid = r.u64();
  const riskAtOpen = r.u8();
  const classByte = r.u8();
  const openedSlot = r.u64();
  const matureSlot = r.u64();
  const expiresSlot = r.u64();
  const stateByte = r.u8();
  return {
    pubkey: addr,
    beneficiary,
    bridge,
    cargo,
    tollPaid,
    riskAtOpen,
    vesselClass: vesselClassFromU8(classByte),
    openedSlot,
    matureSlot,
    expiresSlot,
    state: policyStateFromU8(stateByte),
  };
}

export function decodeWreckFund(buf: Uint8Array): WreckFundView {
  const r = new Reader(buf);
  r.skip(DISCRIMINATOR);
  const authority = r.pubkey();
  const balance = r.u64();
  const openCoverage = r.u64();
  const lifetimeDeposits = r.u64();
  const lifetimePayouts = r.u64();
  const payoutCount = r.u64();
  return {
    authority,
    balance,
    openCoverage,
    lifetimeDeposits,
    lifetimePayouts,
    payoutCount,
  };
}

export function decodeTreasury(buf: Uint8Array): TreasuryView {
  const r = new Reader(buf);
  r.skip(DISCRIMINATOR);
  const authority = r.pubkey();
  const balance = r.u64();
  const lifetimeIn = r.u64();
  const lifetimeOut = r.u64();
  return { authority, balance, lifetimeIn, lifetimeOut };
}

export function decodeGlobalConfig(buf: Uint8Array): GlobalConfigView {
  const r = new Reader(buf);
  r.skip(DISCRIMINATOR);
  const admin = r.pubkey();
  const oilMint = r.pubkey();
  const treasury = r.pubkey();
  const wreckFund = r.pubkey();
  const tollBps = r.u16();
  const fundSplitBps = r.u16();
  const buybackSplitBps = r.u16();
  const opsSplitBps = r.u16();
  const bridgesRegistered = r.u16();
  const policiesOpened = r.u64();
  const policiesSettled = r.u64();
  const wreckClaimsPaid = r.u64();
  const lifetimeTolls = r.u64();
  const lifetimePayouts = r.u64();
  const paused = r.bool();
  return {
    admin,
    oilMint,
    treasury,
    wreckFund,
    tollBps,
    fundSplitBps,
    buybackSplitBps,
    opsSplitBps,
    bridgesRegistered,
    policiesOpened,
    policiesSettled,
    wreckClaimsPaid,
    lifetimeTolls,
    lifetimePayouts,
    paused,
  };
}

export function isAccount(buf: Uint8Array, expectedDiscriminator: Uint8Array): boolean {
  if (buf.length < 8) return false;
  for (let i = 0; i < 8; i++) {
    if (buf[i] !== expectedDiscriminator[i]) return false;
  }
  return true;
}
