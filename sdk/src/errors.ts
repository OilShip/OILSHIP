// Typed errors thrown by the OILSHIP SDK.

export class OilshipError extends Error {
  public readonly code: string;
  public readonly meta?: Record<string, unknown>;

  constructor(code: string, message: string, meta?: Record<string, unknown>) {
    super(message);
    this.name = "OilshipError";
    this.code = code;
    this.meta = meta;
  }
}

export class ValidationError extends OilshipError {
  constructor(message: string, meta?: Record<string, unknown>) {
    super("VALIDATION", message, meta);
    this.name = "ValidationError";
  }
}

export class QuarantinedError extends OilshipError {
  constructor(bridge: string) {
    super("QUARANTINED", `bridge ${bridge} is quarantined`, { bridge });
    this.name = "QuarantinedError";
  }
}

export class CapacityError extends OilshipError {
  constructor(requested: bigint, available: bigint) {
    super(
      "CAPACITY",
      `wreck fund cannot cover ${requested} (available ${available})`,
      { requested: requested.toString(), available: available.toString() }
    );
    this.name = "CapacityError";
  }
}

export class TransportError extends OilshipError {
  constructor(message: string, cause?: unknown) {
    super("TRANSPORT", message, { cause: String(cause) });
    this.name = "TransportError";
  }
}

export class ProgramError extends OilshipError {
  public readonly programCode: number;
  public readonly programLog: string[];
  constructor(programCode: number, programLog: string[]) {
    super("PROGRAM", `on-chain program error ${programCode}`, { programCode });
    this.programCode = programCode;
    this.programLog = programLog;
    this.name = "ProgramError";
  }
}

export function asTransport(err: unknown): TransportError {
  if (err instanceof TransportError) return err;
  if (err instanceof Error) return new TransportError(err.message, err);
  return new TransportError(String(err));
}

export function programCodeName(code: number): string {
  return PROGRAM_ERROR_NAMES[code] ?? `Unknown(${code})`;
}

const PROGRAM_ERROR_NAMES: Record<number, string> = {
  6000: "AlreadyInitialized",
  6001: "NotAdmin",
  6002: "NotBridgeOperator",
  6003: "InvalidBridgeName",
  6004: "InvalidBridgeSymbol",
  6005: "BridgeRegistryFull",
  6006: "BridgeNotFound",
  6007: "BridgeQuarantined",
  6008: "InvalidRiskScore",
  6009: "TollTooHigh",
  6010: "InvalidSplit",
  6011: "CargoTooSmall",
  6012: "CargoTooLarge",
  6013: "PolicyTooShort",
  6014: "PolicyTooLong",
  6015: "PolicyAlreadySettled",
  6016: "PolicyNotMature",
  6017: "PolicyExpired",
  6018: "BeneficiaryMismatch",
  6019: "InsufficientReserve",
  6020: "ReserveRatioBreach",
  6021: "ConvoyFull",
  6022: "ConvoyWindowClosed",
  6023: "MathOverflow",
  6024: "MathUnderflow",
  6025: "DivisionByZero",
  6026: "Paused",
  6027: "AccountMismatch",
  6028: "PdaMismatch",
  6029: "OperatorAlreadyRegistered",
  6030: "CoverageCapExceeded",
  6031: "BridgeStillHealthy",
  6032: "ClaimWindowClosed",
  6033: "AlreadyClaimed",
  6034: "ThroughputExceeded",
};
