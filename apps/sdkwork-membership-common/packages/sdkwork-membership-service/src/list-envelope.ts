import {
  type PageInfo,
  type SdkWorkPageData,
} from "@sdkwork/utils";

export const SDKWORK_MEMBERSHIP_DEFAULT_LIST_PAGE_SIZE = 20;

export interface SdkworkMembershipResponseEnvelope<T> {
  code: 0;
  data: T;
  traceId: string;
}

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

/**
 * Creates a standard list query object for SDK membership list endpoints.
 *
 * The generated TypeScript SDK methods (e.g. MembershipsBenefitsApi.list,
 * MembershipsPackagesApi.list) accept camelCase parameter names
 * (`pageSize`, `planId`) that the SDK internally serialises to snake_case
 * query string keys (`page_size`, `plan_id`).  Returning camelCase keys
 * here ensures the SDK correctly forwards pagination and filter
 * parameters to the backend.
 */
export function createSdkworkMembershipListQuery(
  page = 1,
  pageSize = SDKWORK_MEMBERSHIP_DEFAULT_LIST_PAGE_SIZE,
): { page: number; pageSize: number } {
  return {
    page,
    pageSize,
  };
}

export function unwrapSdkworkMembershipResponse<T>(value: unknown, fallbackMessage = "Request failed."): T {
  if (!value || typeof value !== "object") {
    return value as T;
  }
  if (!("code" in value)) {
    return value as T;
  }
  const candidate = value as Partial<SdkworkMembershipResponseEnvelope<T>> &
    Partial<SdkworkMembershipProblemDetail>;
  if (typeof candidate.code !== "number") {
    throw new Error("Invalid SDKWork membership response envelope.");
  }
  if (candidate.code === 0) {
    if (!("data" in value)) {
      throw new Error("Invalid SDKWork membership response envelope.");
    }
    return candidate.data as T;
  }
  const detail = typeof candidate.detail === "string" && candidate.detail.trim()
    ? candidate.detail
    : typeof candidate.title === "string" && candidate.title.trim()
      ? candidate.title
      : fallbackMessage;
  throw new Error(detail);
}

export function unwrapSdkworkMembershipPageItems<T>(
  value: unknown,
  fallbackMessage = "Request failed.",
): T[] {
  const data = unwrapSdkworkMembershipResponse<SdkWorkPageData<T> | T[]>(value, fallbackMessage);
  if (Array.isArray(data)) {
    return data;
  }
  if (data && typeof data === "object" && Array.isArray((data as SdkWorkPageData<T>).items)) {
    return (data as SdkWorkPageData<T>).items;
  }
  return [];
}

export type { PageInfo, SdkWorkPageData };
