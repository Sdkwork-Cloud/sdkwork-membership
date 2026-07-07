import { sdkworkMembershipPcRuntimeIdentity } from "../index.js";

/**
 * Describes the host application that owns the membership PC composition.
 * Consumed by composition bootstrap to register runtime identity, route
 * prefix, and surface metadata with embedding mall/commerce hosts.
 */
export interface SdkworkMembershipPcHostDescriptor {
  readonly appKey: string;
  readonly architecture: string;
  readonly capability: string;
  readonly domain: string;
  readonly routePrefix: string;
  readonly runtimeFamily: string;
  readonly surface: SdkworkMembershipPcHostSurface;
}

export type SdkworkMembershipPcHostSurface = "app" | "backend-admin";

/**
 * Builds the host registry descriptor for the membership PC application.
 * The descriptor mirrors {@link sdkworkMembershipPcRuntimeIdentity} and
 * augments it with the route prefix and default surface used by the shell.
 */
export function createSdkworkCoreHostRegistry(): SdkworkMembershipPcHostDescriptor {
  return {
    appKey: sdkworkMembershipPcRuntimeIdentity.appKey,
    architecture: sdkworkMembershipPcRuntimeIdentity.architecture,
    capability: sdkworkMembershipPcRuntimeIdentity.capability,
    domain: sdkworkMembershipPcRuntimeIdentity.domain,
    routePrefix: "/membership",
    runtimeFamily: sdkworkMembershipPcRuntimeIdentity.runtimeFamily,
    surface: "app",
  };
}
