import type { MembershipBenefitItem } from './membership-benefit-item';
import type { MembershipCategory } from './membership-category';

export interface AdminMembershipPlanMutation {
  code: string;
  name: string;
  rank: string;
  benefits?: MembershipBenefitItem[] | null;
  status: 'active' | 'inactive' | 'disabled';
  category: MembershipCategory;
}
