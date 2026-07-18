import {
  APP_MEMBERSHIP_METHOD_TREE,
  type ClientFromMethodTree,
  type MembershipAppSdkClient,
  type MembershipSdkMethod,
} from "@sdkwork/membership-sdk-ports";
import type { SdkworkMembershipMutationStatus } from "@sdkwork/membership-contracts";
import {
  formatCurrency as formatSdkworkCurrency,
} from "@sdkwork/utils";
import {
  isSdkworkIamSessionAuthenticated,
} from "@sdkwork/iam-runtime";
import {
  createMembershipAppSdkClientFromTransport,
  createMembershipAppTransportClient,
  type BootstrapSdkworkMembershipAppServiceInput,
} from "./transport.ts";
import {
  SDKWORK_MEMBERSHIP_DEFAULT_LIST_PAGE_SIZE,
  createSdkworkMembershipListQuery,
  unwrapSdkworkMembershipPageItems,
  unwrapSdkworkMembershipResponse,
  type PageInfo,
  type SdkWorkPageData,
  type SdkworkMembershipProblemDetail,
  type SdkworkMembershipResponseEnvelope,
} from "./list-envelope.ts";

export {
  createMembershipAppSdkClientFromTransport,
  createMembershipAppTransportClient,
  type BootstrapSdkworkMembershipAppServiceInput,
} from "./transport.ts";

export type { PageInfo, SdkWorkPageData };

export * from "./backend.ts";

export {
  SDKWORK_MEMBERSHIP_DEFAULT_LIST_PAGE_SIZE,
  createSdkworkMembershipListQuery,
  unwrapSdkworkMembershipPageItems,
  unwrapSdkworkMembershipResponse,
  type SdkworkMembershipProblemDetail,
  type SdkworkMembershipResponseEnvelope,
} from "./list-envelope.ts";

type ServiceTemplate = { readonly [key: string]: true | ServiceTemplate };

export type SdkworkMembershipMembershipsService = ClientFromMethodTree<
  (typeof APP_MEMBERSHIP_METHOD_TREE)["memberships"]
>;

export type SdkworkMembershipAppService = {
  memberships: SdkworkMembershipMembershipsService;
};

export type SdkworkMembershipAppServiceProvider = () => SdkworkMembershipAppService;

export interface SdkworkMembershipSessionTokens {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
}

export type SdkworkMembershipSessionTokenProvider = () => SdkworkMembershipSessionTokens;

export interface CreateSdkworkMembershipAppServiceInput {
  appClient: MembershipAppSdkClient;
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

// ─── Global provider registry ──────────────────────────────────────────────
//
// Providers are stored on `globalThis` instead of module-level variables to
// survive the "dual-package hazard" where Vite, pnpm, or bundler
// deduplication may load this module twice.  When that happens, a
// module-level `let` would be reset to `null` in the duplicate copy, causing
// the clawrouter's `configureSdkworkMembershipAppServiceProvider()` call to
// write to one instance while `getSdkworkMembershipService()` reads from
// another — resulting in "provider is not configured" errors even though
// the host already called configure.
//
// Storing on `globalThis` guarantees a single shared slot regardless of how
// many times the module is instantiated.

const MEMBERSHIP_REGISTRY_INIT_KEY = Symbol.for("sdkwork.membership.registryInitialized");
const MEMBERSHIP_PROVIDER_KEY = Symbol.for("sdkwork.membership.appServiceProvider");
const MEMBERSHIP_TOKEN_PROVIDER_KEY = Symbol.for("sdkwork.membership.sessionTokenProvider");

interface GlobalMembershipRegistry {
  [MEMBERSHIP_REGISTRY_INIT_KEY]: boolean;
  [MEMBERSHIP_PROVIDER_KEY]: SdkworkMembershipAppServiceProvider | null;
  [MEMBERSHIP_TOKEN_PROVIDER_KEY]: SdkworkMembershipSessionTokenProvider;
}

function getGlobalRegistry(): GlobalMembershipRegistry {
  const global = globalThis as typeof globalThis & Partial<GlobalMembershipRegistry>;
  if (!global[MEMBERSHIP_REGISTRY_INIT_KEY]) {
    const registry: GlobalMembershipRegistry = {
      [MEMBERSHIP_REGISTRY_INIT_KEY]: true,
      [MEMBERSHIP_PROVIDER_KEY]: null,
      [MEMBERSHIP_TOKEN_PROVIDER_KEY]: () => ({}),
    };
    Object.assign(global, registry);
  }
  return global as unknown as GlobalMembershipRegistry;
}

export function configureSdkworkMembershipAppServiceProvider(
  provider: SdkworkMembershipAppServiceProvider | null,
): void {
  getGlobalRegistry()[MEMBERSHIP_PROVIDER_KEY] = provider;
}

export function configureSdkworkMembershipSessionTokenProvider(
  provider: SdkworkMembershipSessionTokenProvider | null,
): void {
  getGlobalRegistry()[MEMBERSHIP_TOKEN_PROVIDER_KEY] = provider ?? (() => ({}));
}

export function getSdkworkMembershipService(): SdkworkMembershipAppService {
  const provider = getGlobalRegistry()[MEMBERSHIP_PROVIDER_KEY];
  if (!provider) {
    throw new Error(
      "SDKWork membership service provider is not configured. Call configureSdkworkMembershipAppServiceProvider() from membership PC bootstrap.",
    );
  }
  return provider();
}

export function getSdkworkMembershipSessionTokens(): SdkworkMembershipSessionTokens {
  const tokens = getGlobalRegistry()[MEMBERSHIP_TOKEN_PROVIDER_KEY]();
  return {
    accessToken: normalizeSessionToken(tokens.accessToken),
    authToken: normalizeSessionToken(tokens.authToken),
    refreshToken: normalizeSessionToken(tokens.refreshToken),
  };
}

export function hasSdkworkMembershipSession(): boolean {
  return isSdkworkIamSessionAuthenticated(getSdkworkMembershipSessionTokens());
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

export function bootstrapSdkworkMembershipAppService(
  input: BootstrapSdkworkMembershipAppServiceInput,
): SdkworkMembershipAppService {
  const transport = createMembershipAppTransportClient(input);
  const service = createSdkworkMembershipAppService({
    appClient: createMembershipAppSdkClientFromTransport(transport),
  });
  configureSdkworkMembershipAppServiceProvider(() => service);
  configureSdkworkMembershipSessionTokenProvider(() => {
    if (input.tokenManager) {
      return input.tokenManager.getTokens();
    }
    return {
      accessToken: input.accessToken,
      authToken: input.authToken,
    };
  });
  return service;
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
  if (
    normalized === "FAILED"
    || normalized === "REJECTED"
    || normalized === "CANCELLED"
    || normalized === "CANCELED"
    || normalized === "CLOSED"
    || normalized === "EXPIRED"
  ) {
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
