import type { MembershipBenefitItem } from './membership-benefit-item';
import type { MembershipCategory } from './membership-category';

export interface AdminMembershipPlanItem {
  id: string;
  code: string;
  name: string;
  rank: string;
  benefits: MembershipBenefitItem[];
  status: 'active' | 'inactive' | 'disabled';
  createdAt?: string;
  updatedAt?: string;
  category: MembershipCategory;
}
