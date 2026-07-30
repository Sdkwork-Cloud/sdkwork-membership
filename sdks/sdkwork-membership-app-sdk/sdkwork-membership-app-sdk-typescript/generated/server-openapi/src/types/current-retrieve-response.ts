import type { SdkWorkResourceDataMembershipInfo } from './sdk-work-resource-data-membership-info';

export interface CurrentRetrieveResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataMembershipInfo;
  /** Server-owned request correlation id. */
  traceId: string;
}
