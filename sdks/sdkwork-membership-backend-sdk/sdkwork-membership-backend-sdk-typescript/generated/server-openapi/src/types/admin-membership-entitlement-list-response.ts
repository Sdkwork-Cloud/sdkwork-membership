import type { AdminMembershipEntitlementItem } from './admin-membership-entitlement-item';
import type { PageInfo } from './page-info';

export interface AdminMembershipEntitlementListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
