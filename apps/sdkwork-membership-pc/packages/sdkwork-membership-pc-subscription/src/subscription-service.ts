import {
  createSdkworkMembershipListQuery,
  getSdkworkMembershipService,
  requireSdkworkMembershipSession,
  toNullableSdkworkMembershipNumber,
  toSdkworkMembershipNumber,
  toSdkworkMembershipOptionalString,
  unwrapSdkworkMembershipPageItems,
  type SdkworkMembershipAppService,
} from "@sdkwork/membership-service";
import {
  createSdkworkCouponService,
  normalizeSdkworkRemoteUserCoupon,
  sortSdkworkUserCoupons,
  type SdkworkCouponService,
  type SdkworkRemoteUserCouponLike,
  type SdkworkUserCoupon,
} from "@sdkwork/promotion-pc-coupon";
import type { SdkworkPromotionAppService } from "@sdkwork/promotion-service";
import {
  createSdkworkMembershipService,
  type SdkworkMembershipBenefit,
  type SdkworkMembershipDashboardData,
  type SdkworkMembershipLevel,
  type SdkworkMembershipPlan,
  type SdkworkMembershipPurchaseResult,
  type SdkworkMembershipQrPaymentStrategy,
  type SdkworkMembershipQrPaymentStrategyId,
  type SdkworkMembershipService,
  type SdkworkMembershipSummary,
} from "@sdkwork/membership-pc-membership";
import {
  createDefaultSdkworkSubscriptionPaymentMethodOptions,
  estimateSdkworkSubscriptionCheckout,
  resolveSdkworkSubscriptionPaymentMethod,
  resolveSdkworkSubscriptionPaymentMethodOption,
  type SdkworkSubscriptionAction,
  type SdkworkSubscriptionCheckoutEstimate,
  type SdkworkSubscriptionPlanEstimateInput,
  type SdkworkSubscriptionPaymentMethod,
  type SdkworkSubscriptionPaymentMethodKind,
  type SdkworkSubscriptionPaymentMethodOption,
  type SdkworkSubscriptionPaymentProductType,
  type SdkworkSubscriptionPaymentProductTypeOption,
} from "./subscription";
import {
  createSdkworkSubscriptionMessages,
  type SdkworkSubscriptionMessagesOverrides,
} from "./subscription-copy";
import type { SdkworkSubscriptionPackageGroup } from "./subscription";

interface RemoteMembershipPackage {
  description?: string;
  durationDays?: number | string;
  id?: number | string;
  levelName?: string;
  name?: string;
  originalPrice?: number | string;
  planName?: string;
  pointAmount?: number | string;
  price?: number | string;
  recommended?: boolean;
  sortWeight?: number | string;
  tags?: string[];
}

interface RemoteMembershipPackageGroup {
  description?: string;
  id?: number | string;
  name?: string;
  packages?: RemoteMembershipPackage[];
  sortWeight?: number | string;
}

function mapPackageToPlan(pkg: RemoteMembershipPackage): SdkworkSubscriptionPlanEstimateInput {
  const packageId = toSdkworkMembershipNumber(pkg.id);
  return {
    description: toSdkworkMembershipOptionalString(pkg.description),
    durationDays: toNullableSdkworkMembershipNumber(pkg.durationDays),
    id: `membership-package-${packageId}`,
    includedPoints: toSdkworkMembershipNumber(pkg.pointAmount),
    levelName: toSdkworkMembershipOptionalString(pkg.levelName ?? pkg.planName),
    name: toSdkworkMembershipOptionalString(pkg.name) || "Membership package",
    originalPriceCny: toNullableSdkworkMembershipNumber(pkg.originalPrice),
    packageId,
    priceCny: toSdkworkMembershipNumber(pkg.price),
    recommended: Boolean(pkg.recommended),
    tags: Array.isArray(pkg.tags)
      ? pkg.tags.map((tag) => tag.trim()).filter(Boolean)
      : [],
  };
}

export interface SdkworkSubscriptionCoupon extends SdkworkUserCoupon {
  discountAmountCny: number | null;
}

export interface SdkworkSubscriptionDashboardData {
  benefits: SdkworkMembershipBenefit[];
  checkout: SdkworkSubscriptionCheckoutEstimate;
  coupons: SdkworkSubscriptionCoupon[];
  levels: SdkworkMembershipLevel[];
  packageGroups: SdkworkSubscriptionPackageGroup[];
  paymentMethods: SdkworkSubscriptionPaymentMethodOption[];
  plans: SdkworkMembershipPlan[];
  summary: SdkworkMembershipSummary;
}

export interface SdkworkSubscriptionMutationInput {
  couponId?: string;
  packageId: number;
  paymentMethod?: SdkworkSubscriptionPaymentMethod;
}

export type SdkworkSubscriptionPurchaseResult = SdkworkMembershipPurchaseResult;

export interface SdkworkSubscriptionPaymentMethodSource {
  available?: boolean;
  code: string;
  description?: string;
  id: string;
  kind?: SdkworkSubscriptionPaymentMethodKind;
  label: string;
  paymentMethod?: SdkworkSubscriptionPaymentMethod;
  productTypes: SdkworkSubscriptionPaymentProductTypeOption[];
  recommended?: boolean;
  recommendedProductType: SdkworkSubscriptionPaymentProductType;
  sort?: number;
}

export interface SdkworkSubscriptionPaymentMethodDashboard {
  methods: readonly SdkworkSubscriptionPaymentMethodSource[];
}

export interface SdkworkSubscriptionPaymentMethodService {
  getDashboard(): Promise<SdkworkSubscriptionPaymentMethodDashboard>;
  getEmptyDashboard(): SdkworkSubscriptionPaymentMethodDashboard;
}

export interface CreateSdkworkSubscriptionServiceOptions {
  promotionAppService?: SdkworkPromotionAppService;
  membershipAppService?: SdkworkMembershipAppService;
  locale?: string | null;
  messages?: SdkworkSubscriptionMessagesOverrides;
  couponService?: Pick<SdkworkCouponService, "getDashboard">;
  paymentMethodService?: Partial<SdkworkSubscriptionPaymentMethodService>;
  membershipService?: Partial<SdkworkMembershipService>;
  qrPaymentStrategy?: SdkworkMembershipQrPaymentStrategyId | SdkworkMembershipQrPaymentStrategy;
}

export interface SdkworkSubscriptionService {
  getDashboard(): Promise<SdkworkSubscriptionDashboardData>;
  getEmptyDashboard(): SdkworkSubscriptionDashboardData;
  getPurchaseStatus(orderId: string): Promise<SdkworkSubscriptionPurchaseResult>;
  purchaseSubscription(input: SdkworkSubscriptionMutationInput): Promise<SdkworkSubscriptionPurchaseResult>;
  renewSubscription(input: SdkworkSubscriptionMutationInput): Promise<SdkworkSubscriptionPurchaseResult>;
  upgradeSubscription(input: SdkworkSubscriptionMutationInput): Promise<SdkworkSubscriptionPurchaseResult>;
}

function normalizeSdkworkSubscriptionCoupon(
  coupon: SdkworkRemoteUserCouponLike,
  index: number,
): SdkworkSubscriptionCoupon {
  const normalized = normalizeSdkworkRemoteUserCoupon(coupon, index);

  return {
    ...normalized,
    discountAmountCny: normalized.amountCny,
  };
}

function mapAvailableCoupon(coupon: SdkworkUserCoupon): SdkworkSubscriptionCoupon {
  return {
    ...coupon,
    discountAmountCny: coupon.amountCny,
  };
}

function sortPlansForSubscription<T extends { recommended: boolean; includedPoints: number; priceCny: number; id: string }>(plans: T[]): T[] {
  return [...plans].sort(
    (left, right) =>
      Number(right.recommended) - Number(left.recommended)
      || right.includedPoints - left.includedPoints
      || left.priceCny - right.priceCny
      || left.id.localeCompare(right.id),
  );
}

function mapPackageGroup(group: RemoteMembershipPackageGroup): SdkworkSubscriptionPackageGroup {
  const packageGroupId = toSdkworkMembershipNumber(group.id);
  return {
    description: group.description,
    id: `membership-package-group-${packageGroupId}`,
    name: group.name || "Subscription",
    packageGroupId,
    packages: Array.isArray(group.packages) ? group.packages.map(mapPackageToPlan) : [],
    sortWeight: toSdkworkMembershipNumber(group.sortWeight),
  };
}

function sortPackageGroups(groups: SdkworkSubscriptionPackageGroup[]): SdkworkSubscriptionPackageGroup[] {
  return [...groups].sort(
    (left, right) => left.sortWeight - right.sortWeight || left.name.localeCompare(right.name),
  );
}

function resolveDefaultAction(summary: SdkworkMembershipSummary): SdkworkSubscriptionAction {
  return summary.isMember ? "upgrade" : "purchase";
}

function resolveDefaultPlan(plans: readonly SdkworkMembershipPlan[]): SdkworkMembershipPlan | null {
  return plans.find((plan) => plan.recommended) ?? plans[0] ?? null;
}

function resolveBestCoupon(
  coupons: readonly SdkworkSubscriptionCoupon[],
  plan: SdkworkMembershipPlan | null,
  action: SdkworkSubscriptionAction,
): SdkworkSubscriptionCoupon | null {
  if (!plan) {
    return null;
  }

  return coupons
    .filter((coupon) => coupon.status === "available")
    .map((coupon) => ({
      coupon,
      discountAmountCny: estimateSdkworkSubscriptionCheckout({
        action,
        coupon,
        plan,
      }).discountAmountCny,
    }))
    .filter((item) => item.discountAmountCny > 0)
    .sort(
      (left, right) =>
        right.discountAmountCny - left.discountAmountCny
        || toSdkworkMembershipNumber(left.coupon.remainingDays, Number.MAX_SAFE_INTEGER) - toSdkworkMembershipNumber(right.coupon.remainingDays, Number.MAX_SAFE_INTEGER)
        || left.coupon.name.localeCompare(right.coupon.name),
    )[0]?.coupon ?? null;
}

function resolvePaymentMethodKind(
  method: Pick<SdkworkSubscriptionPaymentMethodSource, "code" | "productTypes" | "recommendedProductType">,
): SdkworkSubscriptionPaymentMethodKind {
  if (method.recommendedProductType === "native" || method.recommendedProductType === "jsapi" || method.recommendedProductType === "miniapp") {
    return "qr";
  }

  if (
    method.recommendedProductType === "online_bank"
    || method.code.includes("UNION")
    || method.code.includes("CARD")
  ) {
    return "card";
  }

  if (
    method.recommendedProductType === "app"
    || method.recommendedProductType === "h5"
    || method.code.includes("WALLET")
  ) {
    return "wallet";
  }

  if (method.productTypes.some((productType) => productType.code === "native")) {
    return "qr";
  }

  return "other";
}

function resolvePaymentMethodDescription(
  method: Pick<SdkworkSubscriptionPaymentMethodSource, "recommendedProductType">,
): string | undefined {
  if (method.recommendedProductType === "native" || method.recommendedProductType === "jsapi" || method.recommendedProductType === "miniapp") {
    return "Scan to pay";
  }

  if (method.recommendedProductType === "pc" || method.recommendedProductType === "online_bank") {
    return "Desktop payment";
  }

  if (method.recommendedProductType === "app" || method.recommendedProductType === "h5") {
    return "Open in payment app";
  }

  return undefined;
}

function mapPaymentMethod(
  method: SdkworkSubscriptionPaymentMethodSource,
  options: {
    recommendedSort: number;
  },
): SdkworkSubscriptionPaymentMethodOption | null {
  const paymentMethod = method.paymentMethod ?? resolveSdkworkSubscriptionPaymentMethod(method.code);

  if (!paymentMethod) {
    return null;
  }
  const sourceSort = typeof method.sort === "number" && Number.isFinite(method.sort)
    ? method.sort
    : 0;

  return {
    available: method.available !== false,
    code: method.code,
    description: method.description ?? resolvePaymentMethodDescription(method),
    id: method.id,
    kind: method.kind ?? resolvePaymentMethodKind(method),
    label: method.label,
    paymentMethod,
    productTypes: [...method.productTypes],
    recommended: method.recommended ?? sourceSort >= options.recommendedSort,
    recommendedProductType: method.recommendedProductType,
  };
}

function resolvePaymentMethods(
  methods: readonly SdkworkSubscriptionPaymentMethodSource[],
): SdkworkSubscriptionPaymentMethodOption[] {
  const supportedMethods = methods.filter((method) => resolveSdkworkSubscriptionPaymentMethod(method.code));
  const recommendedSort = supportedMethods
    .filter((method) => method.available !== false)
    .reduce((highest, method) => Math.max(
      highest,
      typeof method.sort === "number" && Number.isFinite(method.sort) ? method.sort : 0,
    ), Number.NEGATIVE_INFINITY);
  const mappedMethods = supportedMethods
    .map((method) => mapPaymentMethod(method, {
      recommendedSort: Number.isFinite(recommendedSort)
        ? recommendedSort
        : typeof method.sort === "number" && Number.isFinite(method.sort)
          ? method.sort
          : 0,
    }))
    .filter((method): method is SdkworkSubscriptionPaymentMethodOption => Boolean(method))
    .sort(
      (left, right) =>
        Number(right.available) - Number(left.available)
        || Number(right.recommended) - Number(left.recommended)
        || left.label.localeCompare(right.label),
    );

  return mappedMethods.length > 0
    ? mappedMethods
    : createDefaultSdkworkSubscriptionPaymentMethodOptions();
}

export function createSdkworkSubscriptionPaymentMethodService(): SdkworkSubscriptionPaymentMethodService {
  function getEmptyDashboard(): SdkworkSubscriptionPaymentMethodDashboard {
    return {
      methods: createDefaultSdkworkSubscriptionPaymentMethodOptions(),
    };
  }

  return {
    async getDashboard() {
      return getEmptyDashboard();
    },
    getEmptyDashboard,
  };
}

function createDashboard(
  membershipDashboard: SdkworkMembershipDashboardData,
  coupons: readonly SdkworkSubscriptionCoupon[],
  paymentMethods: readonly SdkworkSubscriptionPaymentMethodOption[],
  packageGroups: readonly SdkworkSubscriptionPackageGroup[] = [],
): SdkworkSubscriptionDashboardData {
  const action = resolveDefaultAction(membershipDashboard.summary);
  const allPlansFromGroups = packageGroups.flatMap((g) => g.packages);
  const existingPlanIds = new Set(membershipDashboard.plans.map((p) => p.packageId));
  const additionalPlans = allPlansFromGroups
    .filter((p) => !existingPlanIds.has(p.packageId))
    .map((p) => ({
      description: p.description ?? undefined,
      durationDays: p.durationDays ?? null,
      id: p.id,
      includedPoints: p.includedPoints,
      levelName: p.levelName,
      name: p.name,
      originalPriceCny: p.originalPriceCny ?? null,
      packageId: p.packageId,
      priceCny: p.priceCny,
      recommended: p.recommended,
      tags: p.tags,
    }));
  const mergedPlans = sortPlansForSubscription([...membershipDashboard.plans, ...additionalPlans]);
  const plan = resolveDefaultPlan(mergedPlans);
  const coupon = resolveBestCoupon(coupons, plan, action);
  const selectedPaymentMethod = resolveSdkworkSubscriptionPaymentMethodOption(paymentMethods, null);

  return {
    benefits: membershipDashboard.benefits,
    checkout: estimateSdkworkSubscriptionCheckout({
      action,
      coupon,
      paymentMethodCode: selectedPaymentMethod?.code ?? null,
      paymentMethodId: selectedPaymentMethod?.id ?? null,
      plan,
    }),
    coupons: [...coupons],
    levels: membershipDashboard.levels,
    packageGroups: sortPackageGroups([...packageGroups]),
    paymentMethods: [...paymentMethods],
    plans: mergedPlans,
    summary: membershipDashboard.summary,
  };
}

function createEmptyDashboard(membershipService: Pick<SdkworkMembershipService, "getEmptyDashboard">): SdkworkSubscriptionDashboardData {
  return createDashboard(membershipService.getEmptyDashboard(), [], createDefaultSdkworkSubscriptionPaymentMethodOptions(), []);
}

async function runMembershipMutation(
  membershipService: SdkworkMembershipService,
  name: "memberships.purchase" | "memberships.renew" | "memberships.upgrade",
  payload: SdkworkSubscriptionMutationInput,
): Promise<SdkworkSubscriptionPurchaseResult> {
  return name === "memberships.purchase"
    ? membershipService.purchaseMembership(payload)
    : name === "memberships.renew"
      ? membershipService.renewMembership(payload)
      : membershipService.upgradeMembership(payload);
}

export function createSdkworkSubscriptionService(
  options: CreateSdkworkSubscriptionServiceOptions = {},
): SdkworkSubscriptionService {
  const copy = createSdkworkSubscriptionMessages(options.locale, options.messages);
  const resolveMembershipAppService = (): SdkworkMembershipAppService => {
    if (options.membershipAppService) return options.membershipAppService;
    return getSdkworkMembershipService();
  };
  const membershipService: SdkworkMembershipService = options.membershipService
    ? {
        ...createSdkworkMembershipService({
          membershipAppService: options.membershipAppService,
          locale: options.locale,
          qrPaymentStrategy: options.qrPaymentStrategy,
        }),
        ...options.membershipService,
      }
    : createSdkworkMembershipService({
        membershipAppService: options.membershipAppService,
        locale: options.locale,
        qrPaymentStrategy: options.qrPaymentStrategy,
      });
  const couponService = options.couponService ?? createSdkworkCouponService({
    promotionAppService: options.promotionAppService,
    locale: options.locale,
  });
  const paymentMethodService: SdkworkSubscriptionPaymentMethodService = options.paymentMethodService
    ? {
        ...createSdkworkSubscriptionPaymentMethodService(),
        ...options.paymentMethodService,
      }
    : createSdkworkSubscriptionPaymentMethodService();

  async function fetchPackageGroups(): Promise<SdkworkSubscriptionPackageGroup[]> {
    try {
      const membershipAppService = resolveMembershipAppService();
      const payload = await membershipAppService.memberships.packageGroups.list(
        createSdkworkMembershipListQuery(1, 200),
      );
      const groups = unwrapSdkworkMembershipPageItems<RemoteMembershipPackageGroup>(payload);
      return sortPackageGroups(groups.map(mapPackageGroup));
    } catch {
      return [];
    }
  }

  return {
    async getDashboard() {
      const [membershipDashboard, packageGroups] = await Promise.all([
        membershipService.getDashboard(),
        fetchPackageGroups(),
      ]);
      if (!membershipDashboard.summary.isAuthenticated) {
        return createDashboard(membershipDashboard, [], createDefaultSdkworkSubscriptionPaymentMethodOptions(), packageGroups);
      }

      const [couponDashboard, paymentDashboard] = await Promise.all([
        couponService.getDashboard(),
        paymentMethodService.getDashboard(),
      ]);
      const coupons = sortSdkworkUserCoupons(
        couponDashboard.availableCoupons.map(mapAvailableCoupon),
      ) as SdkworkSubscriptionCoupon[];
      const paymentMethods = resolvePaymentMethods(paymentDashboard.methods);

      return createDashboard(membershipDashboard, coupons, paymentMethods, packageGroups);
    },

    getEmptyDashboard() {
      return createEmptyDashboard(membershipService);
    },

    async getPurchaseStatus(orderId) {
      requireSdkworkMembershipSession(copy.service.signInRequired);
      return membershipService.getPurchaseStatus(orderId);
    },

    async purchaseSubscription(input) {
      requireSdkworkMembershipSession(copy.service.signInRequired);
      return runMembershipMutation(membershipService, "memberships.purchase", input);
    },

    async renewSubscription(input) {
      requireSdkworkMembershipSession(copy.service.signInRequired);
      return runMembershipMutation(membershipService, "memberships.renew", input);
    },

    async upgradeSubscription(input) {
      requireSdkworkMembershipSession(copy.service.signInRequired);
      return runMembershipMutation(membershipService, "memberships.upgrade", input);
    },
  };
}
