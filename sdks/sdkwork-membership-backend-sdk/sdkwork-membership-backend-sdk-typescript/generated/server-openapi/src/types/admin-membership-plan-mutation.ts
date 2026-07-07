import type { MembershipBenefitItem } from './membership-benefit-item';

export interface AdminMembershipPlanMutation {
  code: string;
  name: string;
  rank: string;
  benefits?: MembershipBenefitItem[] | null;
  status: string;
}
