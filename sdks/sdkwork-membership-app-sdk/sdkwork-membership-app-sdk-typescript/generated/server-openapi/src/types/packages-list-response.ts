import type { SdkWorkPageDataPackages } from './sdk-work-page-data-packages';

export interface PackagesListResponse {
  code: 0;
  data: unknown & SdkWorkPageDataPackages;
  /** Server-owned request correlation id. */
  traceId: string;
}
