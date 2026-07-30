/** Membership reservation outcome. Order and payment modules own cashier and payment execution fields. */
export interface AppMembershipPurchaseOutcome {
  /** Human-readable order number for cashier display. */
  requestNo: string;
  /** Commerce order UUID (canonical resource identifier). */
  orderId: string;
  packageId: string;
  packageName: string;
  amount: string;
  durationDays: string;
  targetPlanRank: string;
  targetPlanName: string;
  status: string;
}
