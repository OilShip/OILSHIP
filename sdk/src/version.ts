// Version metadata for the OILSHIP SDK.

export const SDK_VERSION = "0.1.0";
export const PROTOCOL_VERSION = "0.1";
export const SUPPORTED_ANCHOR_VERSION = "0.30.1";
export const MIN_NODE_VERSION = "18.0.0";

export interface VersionInfo {
  sdk: string;
  protocol: string;
  anchor: string;
  node: string;
}
