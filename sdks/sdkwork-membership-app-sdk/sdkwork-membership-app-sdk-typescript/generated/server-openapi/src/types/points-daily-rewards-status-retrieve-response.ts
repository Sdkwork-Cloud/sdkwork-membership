import type { SdkWorkResourceDataDailyRewardStatus } from './sdk-work-resource-data-daily-reward-status';

export interface PointsDailyRewardsStatusRetrieveResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataDailyRewardStatus;
  /** Server-owned request correlation id. */
  traceId: string;
}
