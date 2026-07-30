import type { SdkWorkResourceDataPointsBalance } from './sdk-work-resource-data-points-balance';

export interface PointsBalanceRetrieveResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataPointsBalance;
  /** Server-owned request correlation id. */
  traceId: string;
}
