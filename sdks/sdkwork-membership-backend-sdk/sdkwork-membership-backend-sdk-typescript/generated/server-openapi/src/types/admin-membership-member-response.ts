import type { AdminMembershipMemberItem } from './admin-membership-member-item';

export interface AdminMembershipMemberResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
