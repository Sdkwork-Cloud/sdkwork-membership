import type { MembershipBenefitItem } from './membership-benefit-item';

export interface AdminMembershipPlanItem {
  id: string;
  code: string;
  name: string;
  rank: string;
  benefits: MembershipBenefitItem[];
  status: string;
}
