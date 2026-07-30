import type { SdkWorkResourceDataPurchaseOutcome } from './sdk-work-resource-data-purchase-outcome';

export interface PurchasesRenewResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataPurchaseOutcome;
  /** Server-owned request correlation id. */
  traceId: string;
}
