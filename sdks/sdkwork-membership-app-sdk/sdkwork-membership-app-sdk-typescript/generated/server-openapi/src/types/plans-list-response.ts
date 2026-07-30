import type { SdkWorkPageDataPlans } from './sdk-work-page-data-plans';

export interface PlansListResponse {
  code: 0;
  data: unknown & SdkWorkPageDataPlans;
  /** Server-owned request correlation id. */
  traceId: string;
}
