import type { SdkWorkResourceDataDailyReward } from './sdk-work-resource-data-daily-reward';

export interface PointsDailyRewardsCreateResponse201 {
  code: 0;
  data: unknown & SdkWorkResourceDataDailyReward;
  /** Server-owned request correlation id. */
  traceId: string;
}
