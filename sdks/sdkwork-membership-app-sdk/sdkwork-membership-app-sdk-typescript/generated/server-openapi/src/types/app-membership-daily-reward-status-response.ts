/** Membership daily reward status response. */
export interface AppMembershipDailyRewardStatusResponse {
  canClaim: boolean;
  claimedToday: boolean;
  consecutiveDays: string;
  totalDays: string;
}
