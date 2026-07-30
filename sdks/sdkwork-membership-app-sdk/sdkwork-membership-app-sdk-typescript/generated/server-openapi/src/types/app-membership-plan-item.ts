/** Membership plan item. */
export interface AppMembershipPlanItem {
  id: string;
  name: string;
  rank: string;
  requiredPoints?: string;
  description?: string;
  icon?: string;
  badge?: string;
}
