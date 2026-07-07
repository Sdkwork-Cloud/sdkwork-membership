import type { SdkworkMembershipPcRouteContribution } from "../index.js";

/**
 * Describes a PC module that contributes routes and screens to the
 * membership application shell.
 */
export interface SdkworkMembershipPcModuleDescriptor {
  readonly capability: string;
  readonly id: string;
  readonly packageName: string;
  readonly routes: readonly SdkworkMembershipPcRouteContribution[];
}

const MEMBERSHIP_MODULE_ROUTES: readonly SdkworkMembershipPcRouteContribution[] = [
  {
    auth: "public",
    capability: "membership",
    domain: "membership",
    id: "membership-home",
    packageName: "@sdkwork/membership-pc-membership",
    path: "/membership",
    screen: "membership",
    surface: "app",
    title: "Membership",
    titleKey: "membership.title",
  },
];

const SUBSCRIPTION_MODULE_ROUTES: readonly SdkworkMembershipPcRouteContribution[] = [
  {
    auth: "public",
    capability: "subscription",
    domain: "membership",
    id: "subscription-catalog",
    packageName: "@sdkwork/membership-pc-subscription",
    path: "/subscription/catalog",
    screen: "subscription-catalog",
    surface: "app",
    title: "Subscription Catalog",
    titleKey: "subscription.catalog.title",
  },
  {
    auth: "public",
    capability: "subscription",
    domain: "membership",
    id: "subscription-checkout",
    packageName: "@sdkwork/membership-pc-subscription",
    path: "/subscription/checkout",
    screen: "subscription-checkout",
    surface: "app",
    title: "Subscription",
    titleKey: "subscription.title",
  },
];

/**
 * Builds the module registry for the membership PC composition. Each entry
 * declares a feature module, its composed package name, and the routes it
 * contributes to the shell router.
 */
export function createSdkworkCoreModuleRegistry(): readonly SdkworkMembershipPcModuleDescriptor[] {
  return [
    {
      capability: "membership",
      id: "membership",
      packageName: "@sdkwork/membership-pc-membership",
      routes: MEMBERSHIP_MODULE_ROUTES,
    },
    {
      capability: "subscription",
      id: "subscription",
      packageName: "@sdkwork/membership-pc-subscription",
      routes: SUBSCRIPTION_MODULE_ROUTES,
    },
  ];
}
