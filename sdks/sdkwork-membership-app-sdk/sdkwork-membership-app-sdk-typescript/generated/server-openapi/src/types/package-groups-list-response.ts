import type { SdkWorkPageDataPackageGroups } from './sdk-work-page-data-package-groups';

export interface PackageGroupsListResponse {
  code: 0;
  data: unknown & SdkWorkPageDataPackageGroups;
  /** Server-owned request correlation id. */
  traceId: string;
}
