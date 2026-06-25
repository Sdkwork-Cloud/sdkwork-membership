export type SdkworkMembershipPcRouteSurface = "app" | "backend-admin";

export interface SdkworkMembershipPcRouteContribution {
  readonly auth: "public" | "required";
  readonly capability: string;
  readonly domain: "membership";
  readonly id: string;
  readonly packageName: string;
  readonly path: string;
  readonly permissionHint?: string;
  readonly screen: string;
  readonly surface: SdkworkMembershipPcRouteSurface;
  readonly title: string;
  readonly titleKey: string;
}

export const sdkworkMembershipPcRuntimeIdentity = {
  appKey: "sdkwork-membership-pc",
  architecture: "pc-react",
  domain: "membership",
  capability: "membership",
  runtimeFamily: "web",
} as const;

export function createSdkworkMembershipPcRouteRegistry(
  ...routeGroups: readonly (readonly SdkworkMembershipPcRouteContribution[])[]
): readonly SdkworkMembershipPcRouteContribution[] {
  return routeGroups.flat();
}
