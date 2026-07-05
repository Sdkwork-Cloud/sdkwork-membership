import type { AuthTokenManager } from "@sdkwork/sdk-common";
import {
  createClient,
  type SdkworkAppClient as MembershipAppTransportClient,
  type SdkworkAppConfig,
} from "@sdkwork/membership-app-sdk";
import type { MembershipAppSdkClient } from "@sdkwork/membership-sdk-ports";

const APP_API_SUFFIX = "/app/v3/api";

export type { MembershipAppTransportClient };

export function resolveMembershipAppApiOrigin(baseUrl: string): string {
  const trimmed = baseUrl.trim().replace(/\/+$/u, "");
  if (trimmed.endsWith(APP_API_SUFFIX)) {
    return trimmed.slice(0, -APP_API_SUFFIX.length);
  }
  return trimmed;
}

export function createMembershipAppSdkClientFromTransport(
  transport: MembershipAppTransportClient,
): MembershipAppSdkClient {
  return {
    commerce: {
      memberships: transport.memberships,
    },
  } as MembershipAppSdkClient;
}

export interface BootstrapSdkworkMembershipAppServiceInput {
  baseUrl: string;
  authToken?: string;
  accessToken?: string;
  tenantId?: string;
  organizationId?: string;
  platform?: string;
  tokenManager?: AuthTokenManager;
}

export function createMembershipAppTransportClient(
  input: BootstrapSdkworkMembershipAppServiceInput,
): MembershipAppTransportClient {
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
