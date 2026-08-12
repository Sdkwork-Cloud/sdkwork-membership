import type { MembershipCategory } from './membership-category';

export interface AdminMembershipPackageGroupItem {
  id: string;
  code: string;
  name: string;
  description?: string | null;
  billingCycle: string;
  durationDays: string;
  sortWeight: string;
  status: 'active' | 'inactive' | 'disabled';
  category: MembershipCategory;
}
