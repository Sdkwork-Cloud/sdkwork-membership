import { sdkworkMembershipPcRuntimeIdentity } from "../index.js";

/**
 * The SDK family that backs the membership PC application. The composed
 * TypeScript facade is exposed as `@sdkwork/membership-app-sdk`; UI packages
 * consume it through `@sdkwork/membership-service`.
 */
export const sdkworkMembershipPcSdkFamily = "sdkwork-membership-app-sdk" as const;

/**
 * Describes the SDK bootstrap contract for the membership PC application.
 * Consumed by the shell to wire the service provider before mounting.
 */
export interface SdkworkMembershipPcSdkBootstrap {
  readonly appKey: string;
  readonly composedFacade: string;
  readonly domain: string;
  readonly serviceFacade: string;
}

/**
 * Resolves the SDK bootstrap descriptor for the membership PC application.
 * The shell calls this to identify which composed facade and service facade
 * to wire into the application service provider.
 */
export function resolveSdkworkMembershipPcSdkBootstrap(): SdkworkMembershipPcSdkBootstrap {
  return {
    appKey: sdkworkMembershipPcRuntimeIdentity.appKey,
    composedFacade: "@sdkwork/membership-app-sdk",
    domain: sdkworkMembershipPcRuntimeIdentity.domain,
    serviceFacade: "@sdkwork/membership-service",
  };
}
