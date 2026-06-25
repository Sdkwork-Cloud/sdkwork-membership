import type { SdkworkMembershipPcRouteContribution } from "@sdkwork/membership-pc-core";

export const sdkworkMembershipPcSubscriptionRoutes = [
  {
    auth: "required",
    capability: "subscription",
    domain: "membership",
    id: "app.membership.subscription.dashboard",
    packageName: "@sdkwork/membership-pc-subscription",
    path: "/app/subscription",
    screen: "dashboard",
    surface: "app",
    title: "Subscription",
    titleKey: "subscription.routes.dashboard.title",
  },
] as const satisfies readonly SdkworkMembershipPcRouteContribution[];
