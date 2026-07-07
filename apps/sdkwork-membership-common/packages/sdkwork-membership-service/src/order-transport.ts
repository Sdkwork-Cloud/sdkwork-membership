import type { AuthTokenManager } from "@sdkwork/sdk-common";
import {
  createClient,
  type SdkworkAppClient as OrderAppTransportClient,
  type SdkworkAppConfig,
} from "@sdkwork/order-app-sdk";
import { resolveMembershipAppApiOrigin } from "./transport.ts";

export type { OrderAppTransportClient };

let orderAppServiceProvider: (() => OrderAppTransportClient) | null = null;

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
  orderAppServiceProvider = provider;
}

export function getSdkworkOrderAppService(): OrderAppTransportClient {
  if (!orderAppServiceProvider) {
    throw new Error(
      "SDKWork order service provider is not configured. Call bootstrapSdkworkOrderAppService() from membership PC bootstrap.",
    );
  }
  return orderAppServiceProvider();
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
