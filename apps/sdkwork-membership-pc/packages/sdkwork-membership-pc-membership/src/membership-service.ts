import {
  createSdkworkMembershipListQuery,
  getSdkworkMembershipService,
  hasSdkworkMembershipSession,
  requireSdkworkMembershipSession,
  toNullableSdkworkMembershipNumber,
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
import {
  type SdkworkMembershipQrPaymentStrategyId,
} from "./payment-qr-strategy";

export interface SdkworkMembershipBenefit {
  benefitKey?: string;
  claimed: boolean;
  description?: string;
  displayValue?: string;
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
  qrPaymentStrategy?: SdkworkMembershipQrPaymentStrategyId;
  status: "completed" | "failed" | "pending";
  targetLevelName?: string;
}

export interface SdkworkMembershipCheckoutRequest extends SdkworkMembershipMutationInput {
  action: "purchase" | "renew" | "upgrade";
}

export interface SdkworkMembershipCheckoutPort {
  createCheckout(input: SdkworkMembershipCheckoutRequest): Promise<SdkworkMembershipPurchaseResult>;
  getCheckoutStatus(orderId: string): Promise<SdkworkMembershipPurchaseResult>;
}

export interface CreateSdkworkMembershipServiceOptions {
  checkoutPort?: SdkworkMembershipCheckoutPort;
  membershipAppService?: SdkworkMembershipAppService;
  locale?: string | null;
  messages?: SdkworkMembershipMessagesOverrides;
}

export interface SdkworkMembershipService {
  getDashboard(): Promise<SdkworkMembershipDashboardData>;
  getEmptyDashboard(): SdkworkMembershipDashboardData;
  getPurchaseStatus(orderId: string): Promise<SdkworkMembershipPurchaseResult>;
  purchaseMembership(input: SdkworkMembershipMutationInput): Promise<SdkworkMembershipPurchaseResult>;
  renewMembership(input: SdkworkMembershipMutationInput): Promise<SdkworkMembershipPurchaseResult>;
  upgradeMembership(input: SdkworkMembershipMutationInput): Promise<SdkworkMembershipPurchaseResult>;
}

interface RemoteMembershipBenefit {
  benefitKey?: string;
  claimed?: boolean;
  description?: string;
  displayValue?: string;
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
    displayValue: toSdkworkMembershipOptionalString(benefit.displayValue),
    id: `membership-benefit-${toSdkworkMembershipNumber(benefit.id)}`,
    name: toSdkworkMembershipOptionalString(benefit.name) || "Membership benefit",
    type: toSdkworkMembershipOptionalString(benefit.type),
    usageLimit: toNullableSdkworkMembershipNumber(benefit.usageLimit),
    usedCount: toNullableSdkworkMembershipNumber(benefit.usedCount),
  })));
}

async function runPurchaseMutation(
  copy: SdkworkMembershipMessages["service"],
  action: "purchase" | "renew" | "upgrade",
  input: SdkworkMembershipMutationInput,
  checkoutPort: SdkworkMembershipCheckoutPort | undefined,
): Promise<SdkworkMembershipPurchaseResult> {
  requireSdkworkMembershipSession(copy.signInRequired);
  if (!checkoutPort) {
    throw new Error("Membership checkout is not configured by the host application.");
  }
  return checkoutPort.createCheckout({ ...input, action });
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
        // Anonymous visitors can still browse membership plans/packages.
        // The packages endpoint serves public catalog data; if it fails,
        // return an empty dashboard so the page renders gracefully.
        try {
          const packagesPayload = await membershipAppService.memberships.packages.list(
            createSdkworkMembershipListQuery(1, 200),
          );
          const packages = unwrapSdkworkMembershipPageItems<RemoteMembershipPackage>(packagesPayload);

          return {
            ...createEmptyDashboard(),
            plans: sortPlans(packages.map(mapPlan)),
          };
        } catch {
          return createEmptyDashboard();
        }
      }

      const [membershipInfoPayload, membershipStatusPayload, levelsPayload, benefitsPayload, packagesPayload] = await Promise.all([
        membershipAppService.memberships.current.retrieve(),
        membershipAppService.memberships.current.status.retrieve(),
        membershipAppService.memberships.plans.list(createSdkworkMembershipListQuery(1, 200)),
        membershipAppService.memberships.benefits.list(createSdkworkMembershipListQuery(1, 200)),
        membershipAppService.memberships.packages.list(createSdkworkMembershipListQuery(1, 200)),
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

    async getPurchaseStatus(orderId) {
      requireSdkworkMembershipSession(copy.service.signInRequired);
      const normalizedOrderId = toSdkworkMembershipOptionalString(orderId);
      if (!normalizedOrderId || !options.checkoutPort) {
        throw new Error(copy.service.purchaseFailed);
      }
      return options.checkoutPort.getCheckoutStatus(normalizedOrderId);
    },

    async purchaseMembership(input) {
      return runPurchaseMutation(copy.service, "purchase", input, options.checkoutPort);
    },

    async renewMembership(input) {
      return runPurchaseMutation(copy.service, "renew", input, options.checkoutPort);
    },

    async upgradeMembership(input) {
      return runPurchaseMutation(copy.service, "upgrade", input, options.checkoutPort);
    },
  };
}

export const sdkworkMembershipService = createSdkworkMembershipService();
