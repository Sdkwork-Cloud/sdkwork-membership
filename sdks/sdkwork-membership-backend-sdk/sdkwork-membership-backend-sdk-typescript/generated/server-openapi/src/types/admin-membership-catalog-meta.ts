import type { MembershipCategory } from './membership-category';

export interface AdminMembershipCatalogMeta {
  categories: MembershipCategory[];
  planStatuses: string[];
  packageStatuses: string[];
  packageGroupStatuses: string[];
  billingCycles: string[];
  benefitTypes: string[];
  subscriptionStatuses: string[];
}
