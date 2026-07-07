import type { AdminMembershipPackageItem } from './admin-membership-package-item';
import type { PageInfo } from './page-info';

export interface AdminMembershipPackageListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
