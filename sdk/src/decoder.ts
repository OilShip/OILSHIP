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
