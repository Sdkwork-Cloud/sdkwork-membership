/** Membership benefit item. */
export interface AppMembershipBenefitItem {
  id: string;
  name: string;
  benefitKey?: string;
  type?: string;
  description?: string;
  icon?: string;
  claimed: boolean;
  usageLimit?: string;
  /** Raw text value for non-numeric benefit display (e.g. 2K, 4K/8K, 8折算力元). Present when grant_quantity is not a pure number. */
  displayValue?: string;
  usedCount?: string;
}
