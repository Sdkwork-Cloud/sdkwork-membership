import type { SdkWorkResourceDataMembershipStatus } from './sdk-work-resource-data-membership-status';

export interface CurrentStatusRetrieveResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataMembershipStatus;
  /** Server-owned request correlation id. */
  traceId: string;
}
