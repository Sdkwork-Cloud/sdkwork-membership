import type { AdminMembershipPackageItem } from './admin-membership-package-item';

export interface AdminMembershipPackageResponse {
  code: 0;
  data: unknown & { item: AdminMembershipPackageItem; };
  /** Server-owned request correlation id. */
  traceId: string;
}
