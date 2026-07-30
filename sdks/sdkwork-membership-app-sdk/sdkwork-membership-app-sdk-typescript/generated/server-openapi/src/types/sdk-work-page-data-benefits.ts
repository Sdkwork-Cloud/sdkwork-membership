import type { AppMembershipBenefitItem } from './app-membership-benefit-item';
import type { PageInfo } from './page-info';

/** Typed page data wrapper for benefits list. */
export interface SdkWorkPageDataBenefits {
  items: AppMembershipBenefitItem[];
  pageInfo: PageInfo;
}
