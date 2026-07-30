/** Membership status response. */
export interface AppMembershipStatusResponse {
  active: boolean;
  planRank: string;
  expiresAt?: string;
  pointBalance?: string;
}
