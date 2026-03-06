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
