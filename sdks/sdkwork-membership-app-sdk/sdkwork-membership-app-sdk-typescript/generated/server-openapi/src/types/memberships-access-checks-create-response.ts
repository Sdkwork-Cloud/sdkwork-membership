import type { MembershipFeatureAccessCheckResult } from './membership-feature-access-check-result';

export interface MembershipsAccessChecksCreateResponse {
  code: 0;
  data: unknown & { item: MembershipFeatureAccessCheckResult; };
  /** Server-owned request correlation id. */
  traceId: string;
}
