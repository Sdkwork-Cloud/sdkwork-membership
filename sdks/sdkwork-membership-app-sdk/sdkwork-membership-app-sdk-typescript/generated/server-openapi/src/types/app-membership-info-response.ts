import type { AppMembershipBenefitItem } from './app-membership-benefit-item';

/** Membership info response. */
export interface AppMembershipInfoResponse {
  planRank: string;
  planName: string;
  membershipStatus: string;
  startedAt?: string;
  expiresAt?: string;
  remainingDays?: string;
  totalDays?: string;
  totalSpent?: string;
  points?: string;
  growthValue?: string;
  upgradeGrowthValue?: string;
  benefits: AppMembershipBenefitItem[];
}
