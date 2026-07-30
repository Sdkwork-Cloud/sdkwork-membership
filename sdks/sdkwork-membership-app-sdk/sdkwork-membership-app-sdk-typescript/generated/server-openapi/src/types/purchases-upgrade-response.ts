import type { SdkWorkResourceDataPurchaseOutcome } from './sdk-work-resource-data-purchase-outcome';

export interface PurchasesUpgradeResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataPurchaseOutcome;
  /** Server-owned request correlation id. */
  traceId: string;
}
