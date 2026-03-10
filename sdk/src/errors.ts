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
