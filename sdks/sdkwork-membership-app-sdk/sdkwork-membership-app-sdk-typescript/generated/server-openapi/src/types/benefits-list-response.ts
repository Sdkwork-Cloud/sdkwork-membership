import type { SdkWorkPageDataBenefits } from './sdk-work-page-data-benefits';

export interface BenefitsListResponse {
  code: 0;
  data: unknown & SdkWorkPageDataBenefits;
  /** Server-owned request correlation id. */
  traceId: string;
}
