import type { AdminMembershipMemberItem } from './admin-membership-member-item';
import type { PageInfo } from './page-info';

export interface AdminMembershipMemberListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
