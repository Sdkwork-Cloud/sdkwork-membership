import type { SdkWorkResourceDataPackage } from './sdk-work-resource-data-package';

export interface PackagesRetrieveResponse {
  code: 0;
  data: unknown & SdkWorkResourceDataPackage;
  /** Server-owned request correlation id. */
  traceId: string;
}
