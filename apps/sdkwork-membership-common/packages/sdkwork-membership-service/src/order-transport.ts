import type { AuthTokenManager } from "@sdkwork/sdk-common";
import {
  createClient,
  type SdkworkAppClient as OrderAppTransportClient,
  type SdkworkAppConfig,
} from "@sdkwork/order-app-sdk";
import { resolveMembershipAppApiOrigin } from "./transport.ts";

export type { OrderAppTransportClient };

// ─── Global provider registry ──────────────────────────────────────────────
//
// Same dual-package hazard fix as index.ts — see comment there.
const ORDER_REGISTRY_INIT_KEY = Symbol.for("sdkwork.membership.orderRegistryInitialized");
const ORDER_PROVIDER_KEY = Symbol.for("sdkwork.membership.orderAppServiceProvider");

interface GlobalOrderRegistry {
  [ORDER_REGISTRY_INIT_KEY]: boolean;
  [ORDER_PROVIDER_KEY]: (() => OrderAppTransportClient) | null;
}

function getGlobalOrderRegistry(): GlobalOrderRegistry {
  const global = globalThis as typeof globalThis & Partial<GlobalOrderRegistry>;
  if (!global[ORDER_REGISTRY_INIT_KEY]) {
    Object.assign(global, {
      [ORDER_REGISTRY_INIT_KEY]: true,
      [ORDER_PROVIDER_KEY]: null,
    } satisfies GlobalOrderRegistry);
  }
  return global as unknown as GlobalOrderRegistry;
}

export interface SdkworkOrderWriteCommandHeaders {
  idempotencyKey: string;
  sdkworkRequestHash: string;
  xIdempotencyFingerprint: string;
}

export interface BootstrapSdkworkOrderAppServiceInput {
  baseUrl: string;
  authToken?: string;
  accessToken?: string;
  tenantId?: string;
  organizationId?: string;
  platform?: string;
  tokenManager?: AuthTokenManager;
}

export function configureSdkworkOrderAppServiceProvider(
  provider: (() => OrderAppTransportClient) | null,
): void {
  getGlobalOrderRegistry()[ORDER_PROVIDER_KEY] = provider;
}

export function getSdkworkOrderAppService(): OrderAppTransportClient {
  const provider = getGlobalOrderRegistry()[ORDER_PROVIDER_KEY];
  if (!provider) {
    throw new Error(
      "SDKWork order service provider is not configured. Call bootstrapSdkworkOrderAppService() or configureSdkworkOrderAppServiceProvider() from membership PC bootstrap.",
    );
  }
  return provider();
}

export function createOrderAppTransportClient(
  input: BootstrapSdkworkOrderAppServiceInput,
): OrderAppTransportClient {
  const config: SdkworkAppConfig = {
    baseUrl: resolveMembershipAppApiOrigin(input.baseUrl),
    authToken: input.authToken,
    accessToken: input.accessToken,
    tenantId: input.tenantId,
    organizationId: input.organizationId,
    platform: input.platform,
    tokenManager: input.tokenManager,
  };
  return createClient(config);
}

export function bootstrapSdkworkOrderAppService(
  input: BootstrapSdkworkOrderAppServiceInput,
): OrderAppTransportClient {
  const client = createOrderAppTransportClient(input);
  configureSdkworkOrderAppServiceProvider(() => client);
  return client;
}

export function createSdkworkOrderWriteCommandHeaders(
  scope: string,
  payload: unknown,
  idempotencyKey: string,
): SdkworkOrderWriteCommandHeaders {
  const requestHash = stableJsonRequestHash(scope, payload);
  return {
    idempotencyKey,
    sdkworkRequestHash: requestHash,
    xIdempotencyFingerprint: requestHash,
  };
}

function stableJsonRequestHash(scope: string, payload: unknown): string {
  return [scope, canonicalJsonString(payload)].map(normalizeRequestHashPart).join("-");
}

function normalizeRequestHashPart(part: string): string {
  return part
    .split("")
    .map((character) =>
      /^[a-zA-Z0-9]$/.test(character) || character === "-" || character === "_" || character === "."
        ? character
        : "-",
    )
    .join("");
}

function canonicalJsonString(value: unknown): string {
  if (value === null || value === undefined) {
    return "null";
  }
  if (typeof value === "boolean" || typeof value === "number") {
    return String(value);
  }
  if (typeof value === "string") {
    return JSON.stringify(value);
  }
  if (Array.isArray(value)) {
    return `[${value.map((entry) => canonicalJsonString(entry)).join(",")}]`;
  }
  if (typeof value === "object") {
    const record = value as Record<string, unknown>;
    const items = Object.keys(record)
      .sort()
      .filter((key) => record[key] !== null && record[key] !== undefined)
      .map((key) => `${JSON.stringify(key)}:${canonicalJsonString(record[key])}`);
    return `{${items.join(",")}}`;
  }
  return JSON.stringify(value);
}
