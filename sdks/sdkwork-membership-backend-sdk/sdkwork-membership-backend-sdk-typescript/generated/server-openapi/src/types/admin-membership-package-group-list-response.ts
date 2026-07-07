import type { AdminMembershipPackageGroupItem } from './admin-membership-package-group-item';
import type { PageInfo } from './page-info';

export interface AdminMembershipPackageGroupListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
