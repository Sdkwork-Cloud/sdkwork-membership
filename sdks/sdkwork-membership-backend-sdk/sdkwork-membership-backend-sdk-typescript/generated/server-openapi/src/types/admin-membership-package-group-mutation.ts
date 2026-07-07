export interface AdminMembershipPackageGroupMutation {
  code: string;
  name: string;
  description?: string | null;
  billingCycle: string;
  durationDays: string;
  sortWeight: string;
  status: string;
}
