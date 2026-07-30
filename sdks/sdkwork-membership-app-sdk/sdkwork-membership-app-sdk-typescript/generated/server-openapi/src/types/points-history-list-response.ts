import type { SdkWorkPageDataPointsHistory } from './sdk-work-page-data-points-history';

export interface PointsHistoryListResponse {
  code: 0;
  data: unknown & SdkWorkPageDataPointsHistory;
  /** Server-owned request correlation id. */
  traceId: string;
}
