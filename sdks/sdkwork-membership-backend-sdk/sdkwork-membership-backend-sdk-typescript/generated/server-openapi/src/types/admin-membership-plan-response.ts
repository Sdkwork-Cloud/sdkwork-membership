import type { AdminMembershipPlanItem } from './admin-membership-plan-item';

export interface AdminMembershipPlanResponse {
  code: 0;
  data: unknown & { item: AdminMembershipPlanItem; };
  /** Server-owned request correlation id. */
  traceId: string;
}
