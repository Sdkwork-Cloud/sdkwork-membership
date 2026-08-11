export interface AdminMembershipPackageItem {
  id: string;
  code: string;
  packageGroupId: string;
  planId: string;
  name: string;
  priceAmount: string;
  currencyCode: string;
  durationDays: string;
  /** Discount rate percentage: 100 means no discount, 90 means pay 90 percent of the price. */
  discount: number;
  status: string;
}
