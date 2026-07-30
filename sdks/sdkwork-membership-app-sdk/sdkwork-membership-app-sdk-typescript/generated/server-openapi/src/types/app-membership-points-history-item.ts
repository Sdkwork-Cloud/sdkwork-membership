/** Membership points history item. */
export interface AppMembershipPointsHistoryItem {
  id: string;
  changeType: string;
  changeAmount: string;
  beforeBalance?: string;
  afterBalance: string;
  sourceType: string;
  remark?: string;
  createdAt?: string;
}
