import type { AdminMembershipCatalogMeta } from './admin-membership-catalog-meta';

export interface AdminMembershipCatalogMetaResponse {
  code: 0;
  data: unknown & { item: AdminMembershipCatalogMeta; };
  /** Server-owned request correlation id. */
  traceId: string;
}
