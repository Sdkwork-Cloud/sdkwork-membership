import {
  createSdkworkMembershipListQuery,
  getSdkworkMembershipService,
  getSdkworkOrderAppService,
  hasSdkworkMembershipSession,
  requireSdkworkMembershipSession,
  toNullableSdkworkMembershipNumber,
  toSdkworkMembershipMutationStatus,
  toSdkworkMembershipNumber,
  toSdkworkMembershipOptionalString,
  unwrapSdkworkMembershipPageItems,
  unwrapSdkworkMembershipResponse,
  readSdkworkMediaResource,
  type SdkworkMembershipAppService,
  type SdkworkMediaResource,
} from "@sdkwork/membership-service";
import {
  createSdkworkMembershipMessages,
  type SdkworkMembershipMessages,
  type SdkworkMembershipMessagesOverrides,
} from "./membership-copy";

export interface SdkworkMembershipBenefit {
  benefitKey?: string;
  claimed: boolean;
  description?: string;
  id: string;
  name: string;
  type?: string;
  usageLimit: number | null;
  usedCount: number | null;
}

export interface SdkworkMembershipLevel {
  badge?: string;
  description?: string;
  icon?: SdkworkMediaResource;
  id: string;
  isCurrent: boolean;
  levelValue: number;
  name: string;
  requiredPoints: number | null;
}

export interface SdkworkMembershipPlan {
  description?: string;
  durationDays: number | null;
  id: string;
  includedPoints: number;
  levelName?: string;
  name: string;
  originalPriceCny: number | null;
  packageId: number;
  priceCny: number;
  recommended: boolean;
  tags: string[];
}

export interface SdkworkMembershipSummary {
  currentLevelName: string;
  currentLevelValue: number | null;
  expireTime?: string;
  growthValue: number | null;
  isAuthenticated: boolean;
  isMember: boolean;
  pointBalance: number | null;
  points: number | null;
  remainingDays: number | null;
  status: "active" | "free" | "guest";
  totalSpent: number | null;
  upgradeGrowthValue: number | null;
}

export interface SdkworkMembershipDashboardData {
  benefits: SdkworkMembershipBenefit[];
  levels: SdkworkMembershipLevel[];
  plans: SdkworkMembershipPlan[];
  summary: SdkworkMembershipSummary;
}

export interface SdkworkMembershipMutationInput {
  couponId?: string;
  packageId: number;
  paymentMethod?: string;
}

export interface SdkworkMembershipPurchaseResult {
  amountCny: number | null;
  cashierUrl?: string;
  durationDays: number | null;
  orderId?: string;
  packageId: number | null;
  packageName?: string;
  qrCode?: string;
  status: "completed" | "failed" | "pending";
  targetLevelName?: string;
}

export interface CreateSdkworkMembershipServiceOptions {
  membershipAppService?: SdkworkMembershipAppService;
  locale?: string | null;
  messages?: SdkworkMembershipMessagesOverrides;
}

export interface SdkworkMembershipService {
  getDashboard(): Promise<SdkworkMembershipDashboardData>;
  getEmptyDashboard(): SdkworkMembershipDashboardData;
  purchaseMembership(input: SdkworkMembershipMutationInput): Promise<SdkworkMembershipPurchaseResult>;
  renewMembership(input: SdkworkMembershipMutationInput): Promise<SdkworkMembershipPurchaseResult>;
  upgradeMembership(input: SdkworkMembershipMutationInput): Promise<SdkworkMembershipPurchaseResult>;
}

interface RemoteMembershipBenefit {
  benefitKey?: string;
  claimed?: boolean;
  description?: string;
  id?: number | string;
  name?: string;
  type?: string;
  usageLimit?: number | string;
  usedCount?: number | string;
}

interface RemoteMembershipLevel {
  badge?: string;
  description?: string;
  icon?: unknown;
  id?: number | string;
  levelValue?: number | string;
  name?: string;
  requiredPoints?: number | string;
}

interface RemoteMembershipInfo {
  expireTime?: string;
  growthValue?: number | string;
  membershipStatus?: string;
  planName?: string;
  planRank?: number | string;
  points?: number | string;
  remainingDays?: number | string;
  totalSpent?: number | string;
  upgradeGrowthValue?: number | string;
}

interface RemoteMembershipStatus {
  active?: boolean;
  expireTime?: string;
  pointBalance?: number | string;
  planRank?: number | string;
}

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

interface RemoteMembershipPurchaseResult {
  amount?: number | string;
  cashierUrl?: string;
  cashier_url?: string;
  durationDays?: number | string;
  orderId?: string;
  packageId?: number | string;
  packageName?: string;
  qrCode?: string;
  qrCodePayload?: string;
  qr_code_payload?: string;
  requestNo?: string;
  status?: string;
  success?: boolean;
  targetLevelName?: string;
  targetPlanName?: string;
}

function mapPlan(membershipPackage: RemoteMembershipPackage): SdkworkMembershipPlan {
  const packageId = toSdkworkMembershipNumber(membershipPackage.id);

  return {
    description: toSdkworkMembershipOptionalString(membershipPackage.description),
    durationDays: toNullableSdkworkMembershipNumber(membershipPackage.durationDays),
    id: `membership-package-${packageId}`,
    includedPoints: toSdkworkMembershipNumber(membershipPackage.pointAmount),
    levelName: toSdkworkMembershipOptionalString(membershipPackage.levelName ?? membershipPackage.planName),
    name: toSdkworkMembershipOptionalString(membershipPackage.name) || "Membership package",
    originalPriceCny: toNullableSdkworkMembershipNumber(membershipPackage.originalPrice),
    packageId,
    priceCny: toSdkworkMembershipNumber(membershipPackage.price),
    recommended: Boolean(membershipPackage.recommended),
    tags: Array.isArray(membershipPackage.tags)
      ? membershipPackage.tags.map((tag) => tag.trim()).filter(Boolean)
      : [],
  };
}

function sortPlans(plans: SdkworkMembershipPlan[]): SdkworkMembershipPlan[] {
  return [...plans].sort(
    (left, right) =>
      Number(right.recommended) - Number(left.recommended)
      || right.includedPoints - left.includedPoints
      || left.priceCny - right.priceCny
      || left.id.localeCompare(right.id),
  );
}

function sortBenefits(benefits: SdkworkMembershipBenefit[]): SdkworkMembershipBenefit[] {
  return [...benefits].sort(
    (left, right) =>
      Number(right.claimed) - Number(left.claimed)
      || left.name.localeCompare(right.name),
  );
}

function sortLevels(levels: SdkworkMembershipLevel[]): SdkworkMembershipLevel[] {
  return [...levels].sort(
    (left, right) => left.levelValue - right.levelValue || left.name.localeCompare(right.name),
  );
}

function mapSummary(
  membershipInfo: RemoteMembershipInfo | null | undefined,
  membershipStatus: RemoteMembershipStatus | null | undefined,
): SdkworkMembershipSummary {
  const isMember = Boolean(membershipStatus?.active || (membershipInfo?.membershipStatus || "").toUpperCase() === "ACTIVE");
  const currentLevelValue = toNullableSdkworkMembershipNumber(membershipStatus?.planRank ?? membershipInfo?.planRank);

  return {
    currentLevelName: toSdkworkMembershipOptionalString(membershipInfo?.planName) || (isMember ? "Member" : "Free"),
    currentLevelValue,
    expireTime: toSdkworkMembershipOptionalString(membershipStatus?.expireTime) || toSdkworkMembershipOptionalString(membershipInfo?.expireTime),
    growthValue: toNullableSdkworkMembershipNumber(membershipInfo?.growthValue),
    isAuthenticated: true,
    isMember,
    pointBalance: toNullableSdkworkMembershipNumber(membershipStatus?.pointBalance),
    points: toNullableSdkworkMembershipNumber(membershipInfo?.points),
    remainingDays: toNullableSdkworkMembershipNumber(membershipInfo?.remainingDays),
    status: isMember ? "active" : "free",
    totalSpent: toNullableSdkworkMembershipNumber(membershipInfo?.totalSpent),
    upgradeGrowthValue: toNullableSdkworkMembershipNumber(membershipInfo?.upgradeGrowthValue),
  };
}

function mapLevels(
  levels: RemoteMembershipLevel[],
  currentLevelValue: number | null,
): SdkworkMembershipLevel[] {
  return sortLevels(levels.map((level) => ({
    badge: toSdkworkMembershipOptionalString(level.badge),
    description: toSdkworkMembershipOptionalString(level.description),
    icon: readSdkworkMediaResource(level.icon),
    id: `membership-level-${toSdkworkMembershipNumber(level.id)}`,
    isCurrent: currentLevelValue !== null && toSdkworkMembershipNumber(level.levelValue) === currentLevelValue,
    levelValue: toSdkworkMembershipNumber(level.levelValue),
    name: toSdkworkMembershipOptionalString(level.name) || "Membership level",
    requiredPoints: toNullableSdkworkMembershipNumber(level.requiredPoints),
  })));
}

function mapBenefits(benefits: RemoteMembershipBenefit[]): SdkworkMembershipBenefit[] {
  return sortBenefits(benefits.map((benefit) => ({
    benefitKey: toSdkworkMembershipOptionalString(benefit.benefitKey),
    claimed: Boolean(benefit.claimed),
    description: toSdkworkMembershipOptionalString(benefit.description),
    id: `membership-benefit-${toSdkworkMembershipNumber(benefit.id)}`,
    name: toSdkworkMembershipOptionalString(benefit.name) || "Membership benefit",
    type: toSdkworkMembershipOptionalString(benefit.type),
    usageLimit: toNullableSdkworkMembershipNumber(benefit.usageLimit),
    usedCount: toNullableSdkworkMembershipNumber(benefit.usedCount),
  })));
}

function mapPurchaseResult(result: RemoteMembershipPurchaseResult | null | undefined): SdkworkMembershipPurchaseResult {
  const statusStr = toSdkworkMembershipOptionalString(result?.status);
  return {
    amountCny: toNullableSdkworkMembershipNumber(result?.amount),
    cashierUrl: toSdkworkMembershipOptionalString(result?.cashierUrl ?? result?.cashier_url),
    durationDays: toNullableSdkworkMembershipNumber(result?.durationDays),
    orderId: toSdkworkMembershipOptionalString(result?.orderId ?? result?.requestNo),
    packageId: toNullableSdkworkMembershipNumber(result?.packageId),
    packageName: toSdkworkMembershipOptionalString(result?.packageName),
    qrCode: toSdkworkMembershipOptionalString(result?.qrCode ?? result?.qrCodePayload ?? result?.qr_code_payload),
    status: result?.success === false ? "failed" : toSdkworkMembershipMutationStatus(statusStr),
    targetLevelName: toSdkworkMembershipOptionalString(result?.targetLevelName ?? result?.targetPlanName),
  };
}

function toSdkworkMembershipWirePaymentMethod(method: string | null | undefined): string {
  const normalized = String(method ?? "").trim().toLowerCase().replace(/-/g, "_");
  switch (normalized) {
    case "wechat":
    case "wechat_pay":
      return "wechat_pay";
    case "alipay":
      return "alipay";
    case "paypal":
      return "paypal";
    case "card":
      return "card";
    case "apple_pay":
      return "apple_pay";
    case "google_pay":
      return "google_pay";
    case "wallet_balance":
      return "wallet_balance";
    default:
      return normalized || "wechat_pay";
  }
}

async function runPurchaseMutation(
  getCommerceService: () => SdkworkMembershipAppService,
  copy: SdkworkMembershipMessages["service"],
  action: "purchase" | "renew" | "upgrade",
  input: SdkworkMembershipMutationInput,
): Promise<SdkworkMembershipPurchaseResult> {
  requireSdkworkMembershipSession(copy.signInRequired);
  const membershipAppService = getCommerceService();
  const orderAppService = getSdkworkOrderAppService();
  const paymentMethod = toSdkworkMembershipWirePaymentMethod(input.paymentMethod);
  const packageId = String(input.packageId);

  const idempotencyKey = `membership-checkout:${packageId}:${action}`;
  const orderPayload = unwrapSdkworkMembershipResponse<Record<string, unknown>>(
    await orderAppService.memberships.orders.create(
      {
        packageId,
        paymentMethod,
      },
      {
        idempotencyKey,
        sdkworkRequestHash: idempotencyKey,
      },
    ),
    copy.purchaseFailed,
  );
  const orderId = toSdkworkMembershipOptionalString(orderPayload.orderId);
  const requestNo = toSdkworkMembershipOptionalString(orderPayload.orderNo ?? orderPayload.requestNo);
  if (!orderId || !requestNo) {
    throw new Error(copy.purchaseFailed);
  }

  const reserveBody = {
    couponId: toSdkworkMembershipOptionalString(input.couponId),
    orderId,
    packageId: input.packageId,
    paymentMethod,
    requestNo,
  };
  const reserve = unwrapSdkworkMembershipResponse<RemoteMembershipPurchaseResult>(
    await (
      action === "purchase"
        ? membershipAppService.memberships.purchases.create(reserveBody)
        : action === "renew"
          ? membershipAppService.memberships.purchases.renew(reserveBody)
          : membershipAppService.memberships.purchases.upgrade(reserveBody)
    ),
    action === "purchase"
      ? copy.purchaseFailed
      : action === "renew"
        ? copy.renewFailed
        : copy.upgradeFailed,
  );

  const payPayload = unwrapSdkworkMembershipResponse<Record<string, unknown>>(
    await orderAppService.orders.pay(
      orderId,
      { paymentMethod },
      {
        idempotencyKey: `membership-pay:${orderId}`,
        sdkworkRequestHash: `membership-pay:${orderId}`,
      },
    ),
    copy.purchaseFailed,
  );
  const paymentParams = (payPayload.paymentParams ?? {}) as Record<string, unknown>;
  const cashierUrl = toSdkworkMembershipOptionalString(paymentParams.cashierUrl);

  return mapPurchaseResult({
    ...reserve,
    cashierUrl: cashierUrl ?? reserve.cashierUrl,
    orderId,
    qr_code_payload: cashierUrl ?? reserve.qrCodePayload ?? reserve.qr_code_payload,
    requestNo,
    status: reserve.status ?? "pending",
    success: reserve.success !== false,
  });
}

function createEmptyDashboard(): SdkworkMembershipDashboardData {
  return {
    benefits: [],
    levels: [],
    plans: [],
    summary: {
      currentLevelName: "Guest",
      currentLevelValue: null,
      growthValue: null,
      isAuthenticated: false,
      isMember: false,
      pointBalance: null,
      points: null,
      remainingDays: null,
      status: "guest",
      totalSpent: null,
      upgradeGrowthValue: null,
    },
  };
}

export function createSdkworkMembershipService(
  options: CreateSdkworkMembershipServiceOptions = {},
): SdkworkMembershipService {
  const copy = createSdkworkMembershipMessages(options.locale, options.messages);
  const getCommerceService = () => options.membershipAppService ?? getSdkworkMembershipService();

  return {
    async getDashboard() {
      const membershipAppService = getCommerceService();

      if (!hasSdkworkMembershipSession()) {
        const packagesPayload = await membershipAppService.memberships.packages.list(
          createSdkworkMembershipListQuery(),
        );
        const packages = unwrapSdkworkMembershipPageItems<RemoteMembershipPackage>(packagesPayload);

        return {
          ...createEmptyDashboard(),
          plans: sortPlans(packages.map(mapPlan)),
        };
      }

      const [membershipInfoPayload, membershipStatusPayload, levelsPayload, benefitsPayload, packagesPayload] = await Promise.all([
        membershipAppService.memberships.current.retrieve(),
        membershipAppService.memberships.current.status.retrieve(),
        membershipAppService.memberships.plans.list(createSdkworkMembershipListQuery()),
        membershipAppService.memberships.benefits.list(createSdkworkMembershipListQuery()),
        membershipAppService.memberships.packages.list(createSdkworkMembershipListQuery()),
      ]);
      const membershipInfo = unwrapSdkworkMembershipResponse<RemoteMembershipInfo | null>(membershipInfoPayload);
      const membershipStatus = unwrapSdkworkMembershipResponse<RemoteMembershipStatus | null>(membershipStatusPayload);
      const levels = unwrapSdkworkMembershipPageItems<RemoteMembershipLevel>(levelsPayload);
      const benefits = unwrapSdkworkMembershipPageItems<RemoteMembershipBenefit>(benefitsPayload);
      const packages = unwrapSdkworkMembershipPageItems<RemoteMembershipPackage>(packagesPayload);
      const summary = mapSummary(membershipInfo, membershipStatus);

      return {
        benefits: mapBenefits(benefits),
        levels: mapLevels(levels, summary.currentLevelValue),
        plans: sortPlans(packages.map(mapPlan)),
        summary,
      };
    },

    getEmptyDashboard() {
      return createEmptyDashboard();
    },

    async purchaseMembership(input) {
      return runPurchaseMutation(getCommerceService, copy.service, "purchase", input);
    },

    async renewMembership(input) {
      return runPurchaseMutation(getCommerceService, copy.service, "renew", input);
    },

    async upgradeMembership(input) {
      return runPurchaseMutation(getCommerceService, copy.service, "upgrade", input);
    },
  };
}

export const sdkworkMembershipService = createSdkworkMembershipService();
