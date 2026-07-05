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

export function createSdkworkMembershipListQuery(
  page = 1,
  pageSize = SDKWORK_MEMBERSHIP_DEFAULT_LIST_PAGE_SIZE,
): { page: number; page_size: number } {
  return {
    page,
    page_size: pageSize,
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
  return (candidate.data ?? value) as T;
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
