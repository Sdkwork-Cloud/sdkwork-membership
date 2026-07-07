import {
  createSdkworkMembershipListQuery,
  getSdkworkMembershipService,
  hasSdkworkMembershipSession,
  requireSdkworkMembershipSession,
  toNullableSdkworkMembershipNumber,
  toSdkworkMembershipNumber,
  unwrapSdkworkMembershipPageItems,
  unwrapSdkworkMembershipResponse,
  type SdkworkMembershipAppService,
} from "@sdkwork/membership-service";
import {
  createSdkworkSubscriptionService,
  type SdkworkSubscriptionMutationInput,
  type SdkworkSubscriptionPurchaseResult,
  type SdkworkSubscriptionService,
} from "./subscription-service";
import {
  buildComparisonCategories,
  mapPackageGroupsToBillingCycles,
  mapPackagesToPlanCards,
  mapPackagesToTierColumns,
  resolveMemberSummaryFromPlanRank,
  resolvePlanRankFromPackage,
  type RemoteMembershipCatalogBenefit,
  type RemoteMembershipCatalogPackage,
  type RemoteMembershipCatalogPackageGroup,
  type RemoteMembershipCatalogPlan,
  type SdkworkSubscriptionCatalogMemberSummaryModel,
  type SdkworkSubscriptionCatalogTierColumnModel,
} from "./subscription-catalog-mapper";
import type {
  SdkworkSubscriptionCatalogBillingCycleOption,
  SdkworkSubscriptionCatalogComparisonCategory,
  SdkworkSubscriptionCatalogPlanCardModel,
} from "./subscription-catalog-content";
import {
  createSdkworkSubscriptionCatalogBillingCycles,
  createSdkworkSubscriptionCatalogComparisonCategories,
  createSdkworkSubscriptionCatalogPlanCards,
} from "./subscription-catalog-content";

export interface SdkworkSubscriptionCatalogData {
  benefitsByRank: Readonly<Record<number, readonly RemoteMembershipCatalogBenefit[]>>;
  billingCycles: SdkworkSubscriptionCatalogBillingCycleOption[];
  comparisonCategories: SdkworkSubscriptionCatalogComparisonCategory[];
  memberSummary: SdkworkSubscriptionCatalogMemberSummaryModel | null;
  packageGroupIds: readonly number[];
  packageGroups: readonly RemoteMembershipCatalogPackageGroup[];
  plans: readonly RemoteMembershipCatalogPlan[];
}

export interface SdkworkSubscriptionCatalogViewModel {
  billingCycleIndex: number;
  billingCycles: SdkworkSubscriptionCatalogBillingCycleOption[];
  comparisonCategories: SdkworkSubscriptionCatalogComparisonCategory[];
  memberSummary: SdkworkSubscriptionCatalogMemberSummaryModel | null;
  planCards: SdkworkSubscriptionCatalogPlanCardModel[];
  selectedPackageGroupId: number | null;
  tierColumns: SdkworkSubscriptionCatalogTierColumnModel[];
}

export interface CreateSdkworkSubscriptionCatalogServiceOptions {
  locale?: string | null;
  membershipAppService?: SdkworkMembershipAppService;
  subscriptionService?: SdkworkSubscriptionService;
  translate?: (key: string, defaultValue?: string) => string;
}

export interface SdkworkSubscriptionCatalogService {
  getCatalog(): Promise<SdkworkSubscriptionCatalogData>;
  getViewModel(
    catalog: SdkworkSubscriptionCatalogData,
    billingCycleIndex: number,
  ): SdkworkSubscriptionCatalogViewModel;
  getFallbackViewModel(billingCycleIndex?: number): SdkworkSubscriptionCatalogViewModel;
  purchasePackage(input: SdkworkSubscriptionMutationInput): Promise<SdkworkSubscriptionPurchaseResult>;
}

const CATALOG_LIST_QUERY = createSdkworkMembershipListQuery(1, 200);

async function fetchBenefitsByRank(
  membershipAppService: SdkworkMembershipAppService,
): Promise<Readonly<Record<number, readonly RemoteMembershipCatalogBenefit[]>>> {
  const entries = await Promise.all(
    [0, 1, 2, 3].map(async (rank) => {
      const payload = await membershipAppService.memberships.benefits.list({
        ...CATALOG_LIST_QUERY,
        ...(rank > 0 ? { plan_id: rank } : {}),
      });
      const benefits = unwrapSdkworkMembershipPageItems<RemoteMembershipCatalogBenefit>(payload);
      return [rank, benefits] as const;
    }),
  );

  return Object.fromEntries(entries);
}

async function fetchMemberSummary(
  membershipAppService: SdkworkMembershipAppService,
): Promise<SdkworkSubscriptionCatalogMemberSummaryModel | null> {
  if (!hasSdkworkMembershipSession()) {
    return null;
  }

  try {
    const statusPayload = await membershipAppService.memberships.current.status.retrieve();
    const status = unwrapSdkworkMembershipResponse<{ planRank?: number | string } | null>(statusPayload);
    const planRank = toNullableSdkworkMembershipNumber(status?.planRank);
    return resolveMemberSummaryFromPlanRank(planRank);
  } catch {
    return null;
  }
}

function resolveTranslate(
  translate: ((key: string, defaultValue?: string) => string) | undefined,
): (key: string, defaultValue?: string) => string {
  return translate ?? ((_, defaultValue) => defaultValue ?? "");
}

export function createSdkworkSubscriptionCatalogService(
  options: CreateSdkworkSubscriptionCatalogServiceOptions = {},
): SdkworkSubscriptionCatalogService {
  const resolveMembershipAppService = (): SdkworkMembershipAppService =>
    options.membershipAppService ?? getSdkworkMembershipService();
  const subscriptionService = options.subscriptionService
    ?? createSdkworkSubscriptionService({
      membershipAppService: options.membershipAppService,
      locale: options.locale,
    });
  const translate = resolveTranslate(options.translate);

  return {
    async getCatalog() {
      const membershipAppService = resolveMembershipAppService();
      const [packageGroupsPayload, plansPayload, memberSummary, benefitsByRank] = await Promise.all([
        membershipAppService.memberships.packageGroups.list(CATALOG_LIST_QUERY),
        membershipAppService.memberships.plans.list(CATALOG_LIST_QUERY),
        fetchMemberSummary(membershipAppService),
        fetchBenefitsByRank(membershipAppService),
      ]);

      const packageGroups = unwrapSdkworkMembershipPageItems<RemoteMembershipCatalogPackageGroup>(packageGroupsPayload);
      const plans = unwrapSdkworkMembershipPageItems<RemoteMembershipCatalogPlan>(plansPayload);
      const billingCycles = packageGroups.length > 0
        ? mapPackageGroupsToBillingCycles(packageGroups)
        : createSdkworkSubscriptionCatalogBillingCycles(translate);
      const comparisonCategories = buildComparisonCategories(benefitsByRank);

      return {
        benefitsByRank,
        billingCycles,
        comparisonCategories,
        memberSummary,
        packageGroupIds: packageGroups.map((group) => toSdkworkMembershipNumber(group.id)),
        packageGroups,
        plans,
      };
    },

    getViewModel(catalog, billingCycleIndex) {
      const safeIndex = Math.max(0, Math.min(billingCycleIndex, Math.max(catalog.packageGroups.length - 1, 0)));
      const selectedGroup = catalog.packageGroups[safeIndex];
      const packages = Array.isArray(selectedGroup?.packages) ? selectedGroup.packages : [];
      const memberSummary = catalog.memberSummary;

      return {
        billingCycleIndex: safeIndex,
        billingCycles: catalog.billingCycles,
        comparisonCategories: catalog.comparisonCategories,
        memberSummary,
        planCards: packages.length > 0
          ? mapPackagesToPlanCards(packages, catalog.plans, memberSummary, translate)
          : createSdkworkSubscriptionCatalogPlanCards(translate),
        selectedPackageGroupId: selectedGroup ? toSdkworkMembershipNumber(selectedGroup.id) : null,
        tierColumns: packages.length > 0
          ? mapPackagesToTierColumns(packages, catalog.plans, memberSummary, translate)
          : [],
      };
    },

    getFallbackViewModel(billingCycleIndex = 0) {
      const planCards = createSdkworkSubscriptionCatalogPlanCards(translate);
      const tierColumns: SdkworkSubscriptionCatalogTierColumnModel[] = planCards
        .filter((plan) => !plan.disabled && plan.id !== "super")
        .slice(0, 3)
        .map((plan) => ({
          buttonDisabled: false,
          buttonText: plan.buttonText,
          membershipTierKey: plan.membershipTierKey,
          name: plan.name,
          packageId: plan.id,
          packageNumericId: Number(plan.id) || 0,
          packagePeriodLabel: plan.packagePeriodLabel,
          priceLabel: plan.priceLabel,
          originalPriceLabel: plan.originalPriceLabel || undefined,
        }));

      return {
        billingCycleIndex,
        billingCycles: createSdkworkSubscriptionCatalogBillingCycles(translate),
        comparisonCategories: createSdkworkSubscriptionCatalogComparisonCategories(),
        memberSummary: null,
        planCards,
        selectedPackageGroupId: null,
        tierColumns,
      };
    },

    async purchasePackage(input) {
      requireSdkworkMembershipSession("Authentication required");
      if (input.packageId <= 0) {
        throw new Error("A valid membership package is required.");
      }

      const membershipAppService = resolveMembershipAppService();
      let action: "purchase" | "renew" | "upgrade" = "purchase";

      if (hasSdkworkMembershipSession()) {
        try {
          const [statusPayload, packagesPayload, plansPayload] = await Promise.all([
            membershipAppService.memberships.current.status.retrieve(),
            membershipAppService.memberships.packages.list(CATALOG_LIST_QUERY),
            membershipAppService.memberships.plans.list(CATALOG_LIST_QUERY),
          ]);
          const status = unwrapSdkworkMembershipResponse<{ active?: boolean; planRank?: number | string } | null>(
            statusPayload,
          );
          if (status?.active) {
            const packages = unwrapSdkworkMembershipPageItems<RemoteMembershipCatalogPackage>(packagesPayload);
            const plans = unwrapSdkworkMembershipPageItems<RemoteMembershipCatalogPlan>(plansPayload);
            const selected = packages.find((pkg) => toSdkworkMembershipNumber(pkg.id) === input.packageId);
            const currentRank = toNullableSdkworkMembershipNumber(status.planRank) ?? 0;
            const targetRank = selected ? resolvePlanRankFromPackage(selected, plans) : currentRank;
            action = targetRank > currentRank ? "upgrade" : "renew";
          }
        } catch {
          action = "purchase";
        }
      }

      if (action === "upgrade") {
        return subscriptionService.upgradeSubscription(input);
      }
      if (action === "renew") {
        return subscriptionService.renewSubscription(input);
      }
      return subscriptionService.purchaseSubscription(input);
    },
  };
}
