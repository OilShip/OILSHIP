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
