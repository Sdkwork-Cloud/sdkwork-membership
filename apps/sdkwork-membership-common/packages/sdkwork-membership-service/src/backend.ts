import type {
  AdminMembershipEntitlementItem,
  AdminMembershipMemberItem,
  AdminMembershipMemberStatusUpdate,
  AdminMembershipPackageGroupItem,
  AdminMembershipPackageGroupMutation,
  AdminMembershipPackageItem,
  AdminMembershipPackageMutation,
  AdminMembershipPlanItem,
  AdminMembershipPlanMutation,
  SdkworkBackendClient as SdkworkMembershipBackendClient,
} from "@sdkwork/membership-backend-sdk";

export type {
  AdminMembershipEntitlementItem,
  AdminMembershipMemberItem,
  AdminMembershipMemberStatusUpdate,
  AdminMembershipPackageGroupItem,
  AdminMembershipPackageGroupMutation,
  AdminMembershipPackageItem,
  AdminMembershipPackageMutation,
  AdminMembershipPlanItem,
  AdminMembershipPlanMutation,
} from "@sdkwork/membership-backend-sdk";

export const SDKWORK_MEMBERSHIP_BACKEND_DEFAULT_PAGE_SIZE = 20;
export const SDKWORK_MEMBERSHIP_BACKEND_MAX_PAGE_SIZE = 200;

export interface MembershipBackendPageInfo {
  mode: "cursor" | "offset";
  page?: number;
  pageSize: number;
  totalItems?: number;
  totalPages?: number;
  nextCursor?: string | null;
  hasMore?: boolean;
}

export interface MembershipBackendPage<T> {
  items: T[];
  pageInfo: MembershipBackendPageInfo;
}

export interface MembershipBackendListQuery {
  page?: number;
  pageSize?: number;
  status?: string;
}

export interface MembershipMemberListQuery extends MembershipBackendListQuery {
  userId?: string;
  planId?: string;
}

export interface MembershipPackageListQuery extends MembershipBackendListQuery {
  packageGroupId?: string;
  planId?: string;
}

export interface MembershipEntitlementListQuery extends MembershipBackendListQuery {
  membershipId?: string;
  planId?: string;
}

export interface SdkworkMembershipBackendService {
  listPlans(query?: MembershipBackendListQuery): Promise<MembershipBackendPage<AdminMembershipPlanItem>>;
  createPlan(input: AdminMembershipPlanMutation): Promise<AdminMembershipPlanItem>;
  updatePlan(id: string, input: AdminMembershipPlanMutation): Promise<AdminMembershipPlanItem>;
  deletePlan(id: string): Promise<void>;
  listPackageGroups(query?: MembershipBackendListQuery): Promise<MembershipBackendPage<AdminMembershipPackageGroupItem>>;
  createPackageGroup(input: AdminMembershipPackageGroupMutation): Promise<AdminMembershipPackageGroupItem>;
  updatePackageGroup(id: string, input: AdminMembershipPackageGroupMutation): Promise<AdminMembershipPackageGroupItem>;
  deletePackageGroup(id: string): Promise<void>;
  listPackages(query?: MembershipPackageListQuery): Promise<MembershipBackendPage<AdminMembershipPackageItem>>;
  createPackage(input: AdminMembershipPackageMutation): Promise<AdminMembershipPackageItem>;
  updatePackage(id: string, input: AdminMembershipPackageMutation): Promise<AdminMembershipPackageItem>;
  deletePackage(id: string): Promise<void>;
  listMembers(query?: MembershipMemberListQuery): Promise<MembershipBackendPage<AdminMembershipMemberItem>>;
  getMember(id: string): Promise<AdminMembershipMemberItem>;
  updateMemberStatus(id: string, input: AdminMembershipMemberStatusUpdate): Promise<AdminMembershipMemberItem>;
  listEntitlements(query?: MembershipEntitlementListQuery): Promise<MembershipBackendPage<AdminMembershipEntitlementItem>>;
}

export function createSdkworkMembershipBackendService(
  client: SdkworkMembershipBackendClient,
): SdkworkMembershipBackendService {
  return {
    listPlans: (query) => loadPage<AdminMembershipPlanItem>(
      client.memberships.plans.list(toListParams(query)),
      query,
    ),
    createPlan: (input) => client.memberships.plans.create(input),
    updatePlan: (id, input) => client.memberships.plans.update(id, input),
    deletePlan: (id) => client.memberships.plans.delete(id),
    listPackageGroups: (query) => loadPage<AdminMembershipPackageGroupItem>(
      client.memberships.packageGroups.list(toListParams(query)),
      query,
    ),
    createPackageGroup: (input) => client.memberships.packageGroups.create(input),
    updatePackageGroup: (id, input) => client.memberships.packageGroups.update(id, input),
    deletePackageGroup: (id) => client.memberships.packageGroups.delete(id),
    listPackages: (query) => loadPage<AdminMembershipPackageItem>(
      client.memberships.packages.list(toPackageListParams(query)),
      query,
    ),
    createPackage: (input) => client.memberships.packages.create(input),
    updatePackage: (id, input) => client.memberships.packages.update(id, input),
    deletePackage: (id) => client.memberships.packages.delete(id),
    listMembers: (query) => loadPage<AdminMembershipMemberItem>(
      client.memberships.members.list(toMemberListParams(query)),
      query,
    ),
    getMember: (id) => client.memberships.members.retrieve(id),
    updateMemberStatus: (id, input) => client.memberships.members.status.update(id, input),
    listEntitlements: (query) => loadPage<AdminMembershipEntitlementItem>(
      client.memberships.entitlements.list(toEntitlementListParams(query)),
      query,
    ),
  };
}

function toListParams(query: MembershipBackendListQuery = {}) {
  return {
    page: normalizePage(query.page),
    pageSize: normalizePageSize(query.pageSize),
    status: normalizeOptionalText(query.status),
  };
}

function toMemberListParams(query: MembershipMemberListQuery = {}) {
  return {
    ...toListParams(query),
    planId: normalizeOptionalText(query.planId),
    userId: normalizeOptionalText(query.userId),
  };
}

function toPackageListParams(query: MembershipPackageListQuery = {}) {
  return {
    ...toListParams(query),
    packageGroupId: normalizeOptionalText(query.packageGroupId),
    planId: normalizeOptionalText(query.planId),
  };
}

function toEntitlementListParams(query: MembershipEntitlementListQuery = {}) {
  return {
    ...toListParams(query),
    membershipId: normalizeOptionalText(query.membershipId),
    planId: normalizeOptionalText(query.planId),
  };
}

async function loadPage<T>(
  request: Promise<unknown>,
  query: MembershipBackendListQuery = {},
): Promise<MembershipBackendPage<T>> {
  return unwrapMembershipBackendPage<T>(
    await request,
    normalizePage(query.page),
    normalizePageSize(query.pageSize),
  );
}

function unwrapMembershipBackendPage<T>(
  value: unknown,
  fallbackPage: number,
  fallbackPageSize: number,
): MembershipBackendPage<T> {
  const root = requireRecord(value, "membership backend list response");
  if ("code" in root) {
    if (root.code !== 0) {
      throw new Error(readProblemDetail(root));
    }
    return unwrapPageData<T>(root.data, fallbackPage, fallbackPageSize);
  }
  return unwrapPageData<T>(root, fallbackPage, fallbackPageSize);
}

function unwrapPageData<T>(
  value: unknown,
  fallbackPage: number,
  fallbackPageSize: number,
): MembershipBackendPage<T> {
  const data = requireRecord(value, "membership backend page data");
  if (!Array.isArray(data.items)) {
    throw new Error("Invalid membership backend page: items must be an array.");
  }
  const rawPageInfo = requireRecord(data.pageInfo, "membership backend pageInfo");
  const mode = rawPageInfo.mode;
  if (mode !== "offset" && mode !== "cursor") {
    throw new Error("Invalid membership backend page: pageInfo.mode is required.");
  }

  return {
    items: data.items as T[],
    pageInfo: {
      mode,
      page: readOptionalPositiveInteger(rawPageInfo.page) ?? (mode === "offset" ? fallbackPage : undefined),
      pageSize: readOptionalPositiveInteger(rawPageInfo.pageSize) ?? fallbackPageSize,
      totalItems: readOptionalNonNegativeNumber(rawPageInfo.totalItems),
      totalPages: readOptionalNonNegativeInteger(rawPageInfo.totalPages),
      nextCursor: readOptionalNullableText(rawPageInfo.nextCursor),
      hasMore: typeof rawPageInfo.hasMore === "boolean" ? rawPageInfo.hasMore : undefined,
    },
  };
}

function normalizePage(value: number | undefined): number {
  return Number.isInteger(value) && Number(value) > 0 ? Number(value) : 1;
}

function normalizePageSize(value: number | undefined): number {
  if (!Number.isInteger(value) || Number(value) <= 0) {
    return SDKWORK_MEMBERSHIP_BACKEND_DEFAULT_PAGE_SIZE;
  }
  return Math.min(Number(value), SDKWORK_MEMBERSHIP_BACKEND_MAX_PAGE_SIZE);
}

function normalizeOptionalText(value: string | undefined): string | undefined {
  const normalized = value?.trim();
  return normalized || undefined;
}

function requireRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid ${label}.`);
  }
  return value as Record<string, unknown>;
}

function readProblemDetail(value: Record<string, unknown>): string {
  for (const key of ["detail", "title"]) {
    if (typeof value[key] === "string" && value[key].trim()) {
      return value[key].trim();
    }
  }
  return "Membership backend request failed.";
}

function readOptionalPositiveInteger(value: unknown): number | undefined {
  const parsed = Number(value);
  return Number.isInteger(parsed) && parsed > 0 ? parsed : undefined;
}

function readOptionalNonNegativeInteger(value: unknown): number | undefined {
  const parsed = Number(value);
  return Number.isInteger(parsed) && parsed >= 0 ? parsed : undefined;
}

function readOptionalNonNegativeNumber(value: unknown): number | undefined {
  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed >= 0 ? parsed : undefined;
}

function readOptionalNullableText(value: unknown): string | null | undefined {
  if (value === null) {
    return null;
  }
  return normalizeOptionalText(typeof value === "string" ? value : undefined);
}
