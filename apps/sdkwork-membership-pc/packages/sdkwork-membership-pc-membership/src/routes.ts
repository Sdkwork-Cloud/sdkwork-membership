import type { SdkworkMembershipPcRouteContribution } from "@sdkwork/membership-pc-core";

export const sdkworkMembershipPcMembershipRoutes = [
  {
    auth: "required",
    capability: "membership",
    domain: "membership",
    id: "app.commerce.membership.dashboard",
    packageName: "@sdkwork/membership-pc-membership",
    path: "/app/membership",
    screen: "dashboard",
    surface: "app",
    title: "Membership",
    titleKey: "membership.routes.dashboard.title",
  },
] as const satisfies readonly SdkworkMembershipPcRouteContribution[];
