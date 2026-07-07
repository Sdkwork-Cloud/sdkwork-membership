import type { AdminMembershipPackageGroupItem } from './admin-membership-package-group-item';

export interface AdminMembershipPackageGroupResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
