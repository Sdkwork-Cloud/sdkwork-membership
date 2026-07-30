import type { AppMembershipPlanItem } from './app-membership-plan-item';
import type { PageInfo } from './page-info';

/** Typed page data wrapper for plans list. */
export interface SdkWorkPageDataPlans {
  items: AppMembershipPlanItem[];
  pageInfo: PageInfo;
}
