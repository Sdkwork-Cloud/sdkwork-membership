import type { AdminMembershipMemberItem } from './admin-membership-member-item';
import type { PageInfo } from './page-info';

export interface AdminMembershipMemberListResponse {
  code: 0;
  data: unknown & { items: AdminMembershipMemberItem[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
