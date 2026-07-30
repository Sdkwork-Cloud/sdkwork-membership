import type { AppMembershipPointsHistoryItem } from './app-membership-points-history-item';
import type { PageInfo } from './page-info';

/** Typed page data wrapper for points history list. */
export interface SdkWorkPageDataPointsHistory {
  items: AppMembershipPointsHistoryItem[];
  pageInfo: PageInfo;
}
