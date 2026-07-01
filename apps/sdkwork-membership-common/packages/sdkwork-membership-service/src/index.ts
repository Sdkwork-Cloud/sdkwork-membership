import {
  APP_MEMBERSHIP_METHOD_TREE,
  type ClientFromMethodTree,
  type MembershipAppSdkClient,
  type MembershipSdkMethod,
} from "@sdkwork/membership-sdk-ports";
import type { SdkworkMembershipMutationStatus } from "@sdkwork/membership-contracts";
import { formatCurrency as formatSdkworkCurrency } from "@sdkwork/utils";

type ServiceTemplate = { readonly [key: string]: true | ServiceTemplate };

export type SdkworkMembershipMembershipsService = ClientFromMethodTree<
  (typeof APP_MEMBERSHIP_METHOD_TREE)["memberships"]
>;

export type SdkworkMembershipAppService = {
  memberships: SdkworkMembershipMembershipsService;
};

export type SdkworkMembershipAppServiceProvider = () => SdkworkMembershipAppService;

let sdkworkMembershipAppServiceProvider: SdkworkMembershipAppServiceProvider | null = null;

export interface SdkworkMembershipSessionTokens {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
}

export type SdkworkMembershipSessionTokenProvider = () => SdkworkMembershipSessionTokens;

let sdkworkMembershipSessionTokenProvider: SdkworkMembershipSessionTokenProvider = () => ({});

export interface CreateSdkworkMembershipAppServiceInput {
  appClient: MembershipAppSdkClient;
}

/**
 * Canonical SDKWork success envelope. See `API_SPEC.md` §15.
 *
 * Success bodies always carry `code: 0`, a typed `data` payload, and the
 * server-owned `traceId`. The legacy `message` / `msg` / `requestId` fields
 * are forbidden on success responses.
 */
export interface SdkworkMembershipResponseEnvelope<T> {
  code: 0;
  data: T;
  traceId: string;
}

/**
 * Canonical SDKWork error envelope (RFC 9457 `application/problem+json`).
 * See `API_SPEC.md` §16. The numeric `code` is the platform error code
 * (40001–79999) and is always non-zero on errors.
 */
export interface SdkworkMembershipProblemDetail {
  type?: string;
  title?: string;
  status?: number;
  detail?: string;
  instance?: string;
  operationId?: string;
  code: number;
  traceId: string;
  errors?: Array<{ field: string; code: string; message?: string }>;
}

export type SdkworkMediaKind =
  | "archive"
  | "audio"
  | "document"
  | "image"
  | "model"
  | "other"
  | "video";

export type SdkworkMediaSource =
  | "data_url"
  | "external_url"
  | "generated"
  | "object_storage"
  | "provider_asset";

export interface SdkworkMediaResource {
  kind: SdkworkMediaKind;
  publicUrl?: string;
  source: SdkworkMediaSource;
  url?: string;
  [key: string]: unknown;
}

export function configureSdkworkMembershipAppServiceProvider(
  provider: SdkworkMembershipAppServiceProvider | null,
): void {
  sdkworkMembershipAppServiceProvider = provider;
}

export function configureSdkworkMembershipSessionTokenProvider(
  provider: SdkworkMembershipSessionTokenProvider | null,
): void {
  sdkworkMembershipSessionTokenProvider = provider ?? (() => ({}));
}

export function getSdkworkMembershipService(): SdkworkMembershipAppService {
  if (!sdkworkMembershipAppServiceProvider) {
    throw new Error(
      "SDKWork membership service provider is not configured. Call configureSdkworkMembershipAppServiceProvider() from membership PC bootstrap.",
    );
  }
  return sdkworkMembershipAppServiceProvider();
}

export function getSdkworkMembershipSessionTokens(): SdkworkMembershipSessionTokens {
  const tokens = sdkworkMembershipSessionTokenProvider();
  return {
    accessToken: normalizeSessionToken(tokens.accessToken),
    authToken: normalizeSessionToken(tokens.authToken),
    refreshToken: normalizeSessionToken(tokens.refreshToken),
  };
}

export function hasSdkworkMembershipSession(): boolean {
  const tokens = getSdkworkMembershipSessionTokens();
  return Boolean(normalizeSessionToken(tokens.authToken) || normalizeSessionToken(tokens.accessToken));
}

export function requireSdkworkMembershipSession(message = "Authentication required"): void {
  if (!hasSdkworkMembershipSession()) {
    throw new Error(message);
  }
}

export function createSdkworkMembershipAppService(
  input: CreateSdkworkMembershipAppServiceInput,
): SdkworkMembershipAppService {
  return {
    memberships: buildServiceTree<SdkworkMembershipMembershipsService>(
      APP_MEMBERSHIP_METHOD_TREE.memberships,
      input.appClient.commerce.memberships,
      ["commerce", "memberships"],
    ),
  };
}

export function unwrapSdkworkMembershipResponse<T>(value: unknown, fallbackMessage = "Request failed."): T {
  if (!value || typeof value !== "object") {
    return value as T;
  }
  if (!("data" in value) && !("code" in value)) {
    return value as T;
  }
  const candidate = value as Partial<SdkworkMembershipResponseEnvelope<T>> &
    Partial<SdkworkMembershipProblemDetail>;
  if (candidate.code === 0) {
    return candidate.data as T;
  }
  if (typeof candidate.code === "number" && candidate.code !== 0) {
    const detail = typeof candidate.detail === "string" && candidate.detail.trim()
      ? candidate.detail
      : typeof candidate.title === "string" && candidate.title.trim()
        ? candidate.title
        : fallbackMessage;
    throw new Error(detail);
  }
  // No canonical code field — treat as raw payload for backward compatibility.
  return (candidate.data ?? value) as T;
}

export function toSdkworkMembershipOptionalString(value: unknown): string | undefined {
  const normalized = typeof value === "string" ? value.trim() : String(value ?? "").trim();
  return normalized || undefined;
}

export function toNullableSdkworkMembershipNumber(value: unknown): number | null {
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === "string" && value.trim()) {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : null;
  }
  return null;
}

export function toSdkworkMembershipNumber(value: unknown, fallback = 0): number {
  return toNullableSdkworkMembershipNumber(value) ?? fallback;
}

export function toSdkworkMembershipMutationStatus(status: unknown): SdkworkMembershipMutationStatus {
  const normalized = String(status ?? "").trim().toUpperCase();
  if (normalized === "SUCCESS" || normalized === "COMPLETED" || normalized === "PAID") {
    return "completed";
  }
  if (normalized === "FAILED" || normalized === "REJECTED") {
    return "failed";
  }
  return "pending";
}

export function formatSdkworkMembershipCurrencyCny(value: number | null | undefined, language = "en-US"): string {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "--";
  }
  return formatSdkworkCurrency(value, "CNY", language) ?? "--";
}

export function formatSdkworkMembershipPoints(value: number, language = "en-US"): string {
  return new Intl.NumberFormat(language).format(value);
}

export function readSdkworkMediaResource(value: unknown): SdkworkMediaResource | undefined {
  if (!value || typeof value !== "object") {
    return undefined;
  }
  const resource = value as SdkworkMediaResource;
  if (!resource.kind || !resource.source) {
    return undefined;
  }
  return resource;
}

function buildServiceTree<TService>(
  template: ServiceTemplate,
  client: unknown,
  missingPathPrefix: readonly string[],
  servicePath: readonly string[] = [],
): TService {
  const service: Record<string, unknown> = {};
  for (const [key, marker] of Object.entries(template)) {
    const nextServicePath = [...servicePath, key];
    if (marker === true) {
      const missingPath = [...missingPathPrefix, ...nextServicePath].join(".");
      service[key] = (...args: Parameters<MembershipSdkMethod>) =>
        callMembership(readMethod(client, nextServicePath), missingPath, ...args);
    } else {
      service[key] = buildServiceTree<Record<string, unknown>>(
        marker,
        client,
        missingPathPrefix,
        nextServicePath,
      );
    }
  }
  return service as TService;
}

function readMethod(root: unknown, path: readonly string[]): MembershipSdkMethod | undefined {
  let node: unknown = root;
  for (const segment of path) {
    if (!node || typeof node !== "object") {
      return undefined;
    }
    const parent = node;
    node = (parent as Record<string, unknown>)[segment];
    if (typeof node === "function") {
      return node.bind(parent) as MembershipSdkMethod;
    }
  }
  return typeof node === "function" ? (node as MembershipSdkMethod) : undefined;
}

async function callMembership(
  method: MembershipSdkMethod | undefined,
  name: string,
  ...args: Parameters<MembershipSdkMethod>
): Promise<unknown> {
  if (!method) {
    throw new Error(`Missing SDKWork membership SDK resource: ${name}`);
  }
  return method(...args);
}

function normalizeSessionToken(value: unknown): string | undefined {
  const normalized = typeof value === "string" ? value.trim() : "";
  return normalized || undefined;
}
