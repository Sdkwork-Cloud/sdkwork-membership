import type { AdminMembershipPackageGroupItem } from './admin-membership-package-group-item';

export interface AdminMembershipPackageGroupResponse {
  code: 0;
  data: unknown & { item: AdminMembershipPackageGroupItem; };
  /** Server-owned request correlation id. */
  traceId: string;
}
