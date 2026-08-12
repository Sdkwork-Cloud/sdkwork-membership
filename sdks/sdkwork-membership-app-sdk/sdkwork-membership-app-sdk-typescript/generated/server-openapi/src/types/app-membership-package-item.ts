import type { MembershipCategory } from './membership-category';

/** Membership package item. */
export interface AppMembershipPackageItem {
  id: string;
  name: string;
  description?: string;
  price: string;
  originalPrice?: string;
  pointAmount: string;
  durationDays: string;
  planName?: string;
  sortWeight: string;
  recommended: boolean;
  tags: string[];
  category: MembershipCategory;
}
