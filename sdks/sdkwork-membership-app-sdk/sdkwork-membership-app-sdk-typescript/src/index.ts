import {
  createClient as createGeneratedAppClient,
  SdkworkAppClient,
} from "../generated/server-openapi/src/index.ts";
import type { SdkworkAppConfig } from "../generated/server-openapi/src/types/common.ts";

export { SdkworkAppClient, SdkworkAppClient as SdkworkMembershipAppClient, createGeneratedAppClient };
export type { SdkworkAppConfig };
export * from "../generated/server-openapi/src/types/index.ts";
export * from "../generated/server-openapi/src/api/index.ts";
export * from "../generated/server-openapi/src/http/index.ts";
export * from "../generated/server-openapi/src/auth/index.ts";

export function createClient(config: SdkworkAppConfig): SdkworkAppClient {
  return createGeneratedAppClient(config);
}
