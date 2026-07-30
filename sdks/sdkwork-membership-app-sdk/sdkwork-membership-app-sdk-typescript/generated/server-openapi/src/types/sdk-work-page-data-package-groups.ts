import type { AppMembershipPackageGroupItem } from './app-membership-package-group-item';
import type { PageInfo } from './page-info';

/** Typed page data wrapper for package groups list. */
export interface SdkWorkPageDataPackageGroups {
  items: AppMembershipPackageGroupItem[];
  pageInfo: PageInfo;
}
