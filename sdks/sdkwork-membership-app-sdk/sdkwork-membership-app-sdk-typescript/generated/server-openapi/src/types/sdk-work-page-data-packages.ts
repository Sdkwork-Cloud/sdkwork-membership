import type { AppMembershipPackageItem } from './app-membership-package-item';
import type { PageInfo } from './page-info';

/** Typed page data wrapper for packages list. */
export interface SdkWorkPageDataPackages {
  items: AppMembershipPackageItem[];
  pageInfo: PageInfo;
}
