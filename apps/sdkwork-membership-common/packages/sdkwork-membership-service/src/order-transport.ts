import type { AuthTokenManager } from "@sdkwork/sdk-common";
import {
  createClient,
  type SdkworkAppClient as OrderAppTransportClient,
  type SdkworkAppConfig,
} from "@sdkwork/order-app-sdk";
import { resolveMembershipAppApiOrigin } from "./transport.ts";

export type { OrderAppTransportClient };

let orderAppServiceProvider: (() => OrderAppTransportClient) | null = null;

export interface BootstrapSdkworkOrderAppServiceInput {
  baseUrl: string;
  authToken?: string;
  accessToken?: string;
  tenantId?: string;
  organizationId?: string;
  platform?: string;
  tokenManager?: AuthTokenManager;
}

export function configureSdkworkOrderAppServiceProvider(
  provider: (() => OrderAppTransportClient) | null,
): void {
  orderAppServiceProvider = provider;
}

export function getSdkworkOrderAppService(): OrderAppTransportClient {
  if (!orderAppServiceProvider) {
    throw new Error(
      "SDKWork order service provider is not configured. Call bootstrapSdkworkOrderAppService() from membership PC bootstrap.",
    );
  }
  return orderAppServiceProvider();
}

export function createOrderAppTransportClient(
  input: BootstrapSdkworkOrderAppServiceInput,
): OrderAppTransportClient {
  const config: SdkworkAppConfig = {
    baseUrl: resolveMembershipAppApiOrigin(input.baseUrl),
    authToken: input.authToken,
    accessToken: input.accessToken,
    tenantId: input.tenantId,
    organizationId: input.organizationId,
    platform: input.platform,
    tokenManager: input.tokenManager,
  };
  return createClient(config);
}

export function bootstrapSdkworkOrderAppService(
  input: BootstrapSdkworkOrderAppServiceInput,
): OrderAppTransportClient {
  const client = createOrderAppTransportClient(input);
  configureSdkworkOrderAppServiceProvider(() => client);
  return client;
}
