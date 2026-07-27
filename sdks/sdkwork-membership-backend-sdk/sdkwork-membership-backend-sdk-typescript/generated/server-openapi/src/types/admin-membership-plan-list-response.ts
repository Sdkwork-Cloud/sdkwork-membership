import type { AdminMembershipPlanItem } from './admin-membership-plan-item';
import type { PageInfo } from './page-info';

export interface AdminMembershipPlanListResponse {
  code: 0;
  data: unknown & { items: AdminMembershipPlanItem[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
