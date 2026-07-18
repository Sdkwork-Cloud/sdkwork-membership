export interface MembershipBenefitItem {
  id: string;
  name: string;
  benefitKey?: string | null;
  type?: string | null;
  description?: string | null;
  icon?: string | null;
  claimed: boolean;
  usageLimit?: string | null;
  usedCount?: string | null;
  /** Raw text value for non-numeric benefit display (e.g. 2K, 4K/8K, 8折算力元). Present when grant_quantity is not a pure number. */
  displayValue?: string | null;
}
