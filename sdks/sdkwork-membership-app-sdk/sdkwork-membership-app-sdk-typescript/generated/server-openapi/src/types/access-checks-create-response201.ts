import type { MembershipFeatureAccessCheckResult } from './membership-feature-access-check-result';

export interface AccessChecksCreateResponse201 {
  code: 0;
  data: unknown & { item: MembershipFeatureAccessCheckResult; };
  /** Server-owned request correlation id. */
  traceId: string;
}
