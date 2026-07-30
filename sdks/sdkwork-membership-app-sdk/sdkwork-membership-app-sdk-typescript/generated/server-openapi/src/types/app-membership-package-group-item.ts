import type { AppMembershipPackageItem } from './app-membership-package-item';

/** Membership package group item. */
export interface AppMembershipPackageGroupItem {
  id: string;
  name: string;
  description?: string;
  sortWeight: string;
  packages: AppMembershipPackageItem[];
}
