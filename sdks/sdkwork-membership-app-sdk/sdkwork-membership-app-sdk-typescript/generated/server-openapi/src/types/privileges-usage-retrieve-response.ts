import type { SdkWorkResourceDataPrivilegeUsage } from './sdk-work-resource-data-privilege-usage';

export interface PrivilegesUsageRetrieveResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataPrivilegeUsage;
  /** Server-owned request correlation id. */
  traceId: string;
}
