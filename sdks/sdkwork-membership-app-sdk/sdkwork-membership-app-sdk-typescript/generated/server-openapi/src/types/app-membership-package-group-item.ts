import type { AppMembershipPackageItem } from './app-membership-package-item';
import type { MembershipCategory } from './membership-category';

/** Membership package group item. */
export interface AppMembershipPackageGroupItem {
  id: string;
  name: string;
  description?: string;
  sortWeight: string;
  packages: AppMembershipPackageItem[];
  category: MembershipCategory;
}
