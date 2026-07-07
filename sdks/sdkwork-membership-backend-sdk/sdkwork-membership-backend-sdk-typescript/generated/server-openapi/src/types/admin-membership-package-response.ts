import type { AdminMembershipPackageItem } from './admin-membership-package-item';

export interface AdminMembershipPackageResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
