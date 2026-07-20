import {
  createClient as createGeneratedBackendClient,
  SdkworkBackendClient,
} from '../generated/server-openapi/src/index.ts';
import type { SdkworkBackendConfig } from '../generated/server-openapi/src/types/common.ts';

export { SdkworkBackendClient, SdkworkBackendClient as SdkworkMembershipBackendClient, createGeneratedBackendClient };
export type { SdkworkBackendConfig };
export * from '../generated/server-openapi/src/types/index.ts';
export * from '../generated/server-openapi/src/api/index.ts';
export * from '../generated/server-openapi/src/http/index.ts';
export * from '../generated/server-openapi/src/auth/index.ts';

export function createClient(config: SdkworkBackendConfig): SdkworkBackendClient {
  return createGeneratedBackendClient(config);
}
