import type { SdkWorkResourceDataPackageGroup } from './sdk-work-resource-data-package-group';

export interface PackageGroupsRetrieveResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataPackageGroup;
  /** Server-owned request correlation id. */
  traceId: string;
}
