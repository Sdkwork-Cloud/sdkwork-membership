import {
  formatSdkworkMembershipCurrencyCny,
  formatSdkworkMembershipPoints,
  toNullableSdkworkMembershipNumber,
  toSdkworkMembershipNumber,
  toSdkworkMembershipOptionalString,
} from "@sdkwork/membership-service";
import { SDKWORK_SUBSCRIPTION_CATALOG_UNAVAILABLE_TIER_KEY } from "./subscription-catalog-content";
import type {
  SdkworkSubscriptionCatalogBillingCycleOption,
  SdkworkSubscriptionCatalogComparisonCategory,
  SdkworkSubscriptionCatalogComparisonCell,
  SdkworkSubscriptionCatalogPlanCardModel,
  SdkworkSubscriptionCatalogPlanFeature,
} from "./subscription-catalog-content";

export interface RemoteMembershipCatalogPackage {
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

export interface RemoteMembershipCatalogPackageGroup {
  description?: string;
  id?: number | string;
  name?: string;
  packages?: RemoteMembershipCatalogPackage[];
  sortWeight?: number | string;
}

export interface RemoteMembershipCatalogPlan {
  id?: number | string;
  name?: string;
  rank?: number | string;
}

export interface RemoteMembershipCatalogBenefit {
  benefitKey?: string;
  description?: string;
  id?: number | string;
  name?: string;
  type?: string;
  usageLimit?: number | string;
}

export interface SdkworkSubscriptionCatalogTierColumnModel {
  buttonDisabled?: boolean;
  buttonText: string;
  isCurrentPlan?: boolean;
  membershipTierKey: string;
  name: string;
  packageId: string;
  packageNumericId: number;
  packagePeriodLabel: string;
  priceLabel: string;
  originalPriceLabel?: string;
}

export interface SdkworkSubscriptionCatalogMemberSummaryModel {
  membershipTierKey: string;
  planRank: number | null;
}

interface PlanRankDisplayConfig {
  membershipTierKey: string;
  nameKey: string;
}

const PLAN_RANK_DISPLAY: Record<number, PlanRankDisplayConfig> = {
  1: { membershipTierKey: "pro", nameKey: "basic_plan" },
  2: { membershipTierKey: "peak", nameKey: "pro_plan" },
  3: { membershipTierKey: "peak", nameKey: "peak_plan" },
};

const COMPARE_BENEFIT_ROWS: ReadonlyArray<{
  benefitKeys: readonly string[];
  categoryLabel: string;
  label: string;
  valueForRank: (rank: number, benefits: readonly RemoteMembershipCatalogBenefit[]) => SdkworkSubscriptionCatalogComparisonCell;
}> = [
  {
    benefitKeys: ["daily_points"],
    categoryLabel: "积分",
    label: "订阅会员积分",
    valueForRank: (rank, benefits) => {
      if (rank <= 0) {
        return "-";
      }
      const benefit = findBenefit(benefits, "daily_points");
      const quantity = toNullableSdkworkMembershipNumber(benefit?.usageLimit);
      return quantity ? `每月${formatSdkworkMembershipPoints(quantity, "zh-CN")}积分` : "-";
    },
  },
  {
    benefitKeys: ["standard_queue", "fast_queue", "vip_queue"],
    categoryLabel: "视频生成",
    label: "视频生成专享加速",
    valueForRank: (rank) => {
      if (rank <= 0) {
        return "-";
      }
      if (rank === 1) {
        return "标准生成通道";
      }
      if (rank === 2) {
        return "标准生成通道";
      }
      return "快速生成通道";
    },
  },
  {
    benefitKeys: ["no_watermark"],
    categoryLabel: "全局能力",
    label: "去除品牌水印",
    valueForRank: (rank, benefits) => rank > 0 && hasBenefit(benefits, "no_watermark"),
  },
  {
    benefitKeys: ["vip_support"],
    categoryLabel: "全局能力",
    label: "无忧退款",
    valueForRank: (rank) => rank > 0,
  },
];

function findBenefit(
  benefits: readonly RemoteMembershipCatalogBenefit[],
  benefitKey: string,
): RemoteMembershipCatalogBenefit | undefined {
  return benefits.find((benefit) => toSdkworkMembershipOptionalString(benefit.benefitKey) === benefitKey);
}

function hasBenefit(
  benefits: readonly RemoteMembershipCatalogBenefit[],
  benefitKey: string,
): boolean {
  return Boolean(findBenefit(benefits, benefitKey));
}

export function resolvePlanRankFromPackage(
  pkg: RemoteMembershipCatalogPackage,
  plans: readonly RemoteMembershipCatalogPlan[],
): number {
  const planName = toSdkworkMembershipOptionalString(pkg.planName ?? pkg.levelName);
  if (planName) {
    const matchedPlan = plans.find((plan) => toSdkworkMembershipOptionalString(plan.name) === planName);
    if (matchedPlan) {
      return toSdkworkMembershipNumber(matchedPlan.rank ?? matchedPlan.id);
    }
    if (planName.includes("基础")) {
      return 1;
    }
    if (planName.includes("标准")) {
      return 2;
    }
    if (planName.includes("高级")) {
      return 3;
    }
  }

  const packageName = toSdkworkMembershipOptionalString(pkg.name) ?? "";
  if (packageName.includes("基础")) {
    return 1;
  }
  if (packageName.includes("标准")) {
    return 2;
  }
  if (packageName.includes("高级")) {
    return 3;
  }

  return 0;
}

export function mapPackageGroupsToBillingCycles(
  groups: readonly RemoteMembershipCatalogPackageGroup[],
): SdkworkSubscriptionCatalogBillingCycleOption[] {
  return [...groups]
    .sort(
      (left, right) =>
        toSdkworkMembershipNumber(left.sortWeight) - toSdkworkMembershipNumber(right.sortWeight)
        || toSdkworkMembershipNumber(left.id) - toSdkworkMembershipNumber(right.id),
    )
    .map((group) => ({
      discountLabel: toSdkworkMembershipOptionalString(group.description),
      label: toSdkworkMembershipOptionalString(group.name) || "Subscription",
    }));
}

function resolvePeriodLabel(durationDays: number, translate: (key: string, defaultValue?: string) => string): string {
  if (durationDays >= 365) {
    return translate("per_year_short", "每年");
  }
  if (durationDays >= 90) {
    return translate("per_quarter_short", "每季");
  }
  return translate("per_month_short", "每月");
}

function resolveMonthlyPointAllowance(pointAmount: number, durationDays: number): number {
  if (durationDays >= 365) {
    return Math.max(1, Math.round(pointAmount / 12));
  }
  if (durationDays >= 90) {
    return Math.max(1, Math.round(pointAmount / 3));
  }
  return pointAmount;
}

function resolveFeatureTemplates(
  rank: number,
  pointAmount: number,
  durationDays: number,
  translate: (key: string, defaultValue?: string) => string,
): SdkworkSubscriptionCatalogPlanFeature[] {
  if (rank === 1) {
    return [
      { text: "智能体上限", tag: "3个" },
      { text: "每日匹配次数", tag: "无限制" },
      { text: "标准算力通道", tag: translate("tag_free", "Free") },
      { text: "参与常规赛事" },
      { text: translate("worry_free_refund", "Worry-free Refund") },
    ];
  }

  if (rank === 2) {
    return [
      { text: "智能体上限", tag: "10个" },
      { text: "每日匹配次数", tag: "无限制" },
      { text: "进阶AI助手", tag: translate("tag_free", "Free") },
      { text: "高级算力通道", tag: translate("tag_free", "Free"), emptyCheck: true },
      { text: "创建专属赛事" },
      { text: translate("worry_free_refund", "Worry-free Refund") },
      { text: "商城专属折扣" },
    ];
  }

  return [
    { text: "智能体上限", tag: "无限" },
    { text: "每日匹配次数", tag: "无限制" },
    { text: translate("privilege_match", "Fast Match"), tag: translate("tag_priority", "Priority") },
    { text: "巅峰算力通道", tag: translate("tag_free", "Free") },
    { text: "主办官方赛事" },
    { text: translate("worry_free_refund", "Worry-free Refund") },
    { text: "巅峰专属特效" },
  ];
}

function formatPriceLabel(value: number | null): string {
  if (value === null) {
    return "--";
  }
  return formatSdkworkMembershipPoints(value, "zh-CN");
}

function formatOriginalPriceLabel(value: number | null): string {
  if (value === null) {
    return "";
  }
  return formatSdkworkMembershipCurrencyCny(value, "zh-CN");
}

function resolveButtonStyle(disabled: boolean): string {
  return disabled
    ? "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 dark:text-zinc-500 cursor-not-allowed"
    : "bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-100";
}

export function mapPackagesToPlanCards(
  packages: readonly RemoteMembershipCatalogPackage[],
  plans: readonly RemoteMembershipCatalogPlan[],
  memberSummary: SdkworkSubscriptionCatalogMemberSummaryModel | null,
  translate: (key: string, defaultValue?: string) => string,
): SdkworkSubscriptionCatalogPlanCardModel[] {
  const cards = [...packages]
    .sort(
      (left, right) =>
        toSdkworkMembershipNumber(left.sortWeight) - toSdkworkMembershipNumber(right.sortWeight)
        || toSdkworkMembershipNumber(left.id) - toSdkworkMembershipNumber(right.id),
    )
    .map((pkg) => {
      const packageNumericId = toSdkworkMembershipNumber(pkg.id);
      const rank = resolvePlanRankFromPackage(pkg, plans);
      const display = PLAN_RANK_DISPLAY[rank] ?? PLAN_RANK_DISPLAY[1];
      const priceCny = toNullableSdkworkMembershipNumber(pkg.price);
      const originalPriceCny = toNullableSdkworkMembershipNumber(pkg.originalPrice);
      const pointAmount = toSdkworkMembershipNumber(pkg.pointAmount);
      const durationDays = toSdkworkMembershipNumber(pkg.durationDays, 30);
      const disabled =
        memberSummary?.membershipTierKey === display.membershipTierKey
        || display.membershipTierKey === SDKWORK_SUBSCRIPTION_CATALOG_UNAVAILABLE_TIER_KEY;
      const priceLabel = formatPriceLabel(priceCny);
      const originalPriceLabel = originalPriceCny ? formatOriginalPriceLabel(originalPriceCny) : "";
      const discountTag = Array.isArray(pkg.tags) ? pkg.tags[0] : undefined;

      return {
        buttonStyle: resolveButtonStyle(disabled),
        buttonText: disabled
          ? translate("current_plan", "当前计划")
          : discountTag
            ? `¥${priceLabel} ${discountTag}`
            : `¥${priceLabel}`,
        disabled,
        features: resolveFeatureTemplates(rank, pointAmount, durationDays, translate),
        id: String(packageNumericId),
        membershipTierKey: display.membershipTierKey,
        name: translate(display.nameKey, pkg.name || "Membership"),
        originalPriceLabel,
        packagePeriodLabel: resolvePeriodLabel(durationDays, translate),
        pointsAllowanceLabel: `${formatSdkworkMembershipPoints(
          resolveMonthlyPointAllowance(pointAmount, durationDays),
          "zh-CN",
        )} 算力积分/月`,
        pointsConversionLabel: priceCny
          ? `换算¥10=${Math.max(1, Math.round(pointAmount / Math.max(priceCny, 1) * 10))}积分`
          : "",
        priceLabel,
        subtitle: pkg.description || "",
      };
    });

  cards.push(createUnavailableSuperPlanCard(translate));
  return cards;
}

export function createUnavailableSuperPlanCard(
  translate: (key: string, defaultValue?: string) => string,
): SdkworkSubscriptionCatalogPlanCardModel {
  return {
    buttonStyle: "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 dark:text-zinc-500 cursor-not-allowed",
    buttonText: translate("coming_soon", "敬请期待"),
    disabled: true,
    features: [
      { text: "智能体上限", tag: "无限" },
      { text: "每日匹配次数", tag: "无限制" },
      { text: translate("privilege_match", "Fast Match"), tag: translate("tag_priority", "Priority") },
      { text: translate("quantum_compute_channel", "Quantum Compute Channel"), tag: translate("tag_free", "Free") },
      { text: translate("host_global_tournaments", "Host Global Tournaments") },
      { text: translate("worry_free_refund", "Worry-free Refund") },
      { text: translate("super_exclusive_effects", "Super Exclusive Effects") },
    ],
    id: "super",
    membershipTierKey: SDKWORK_SUBSCRIPTION_CATALOG_UNAVAILABLE_TIER_KEY,
    name: translate("super_plan", "超级会员"),
    originalPriceLabel: "",
    packagePeriodLabel: translate("per_year_short", "每年"),
    pointsAllowanceLabel: "54600 算力积分/月",
    pointsConversionLabel: "",
    priceLabel: "4???9",
    subtitle: "",
  };
}

export function mapPackagesToTierColumns(
  packages: readonly RemoteMembershipCatalogPackage[],
  plans: readonly RemoteMembershipCatalogPlan[],
  memberSummary: SdkworkSubscriptionCatalogMemberSummaryModel | null,
  translate: (key: string, defaultValue?: string) => string,
): SdkworkSubscriptionCatalogTierColumnModel[] {
  return [...packages]
    .sort(
      (left, right) =>
        toSdkworkMembershipNumber(left.sortWeight) - toSdkworkMembershipNumber(right.sortWeight)
        || toSdkworkMembershipNumber(left.id) - toSdkworkMembershipNumber(right.id),
    )
    .map((pkg) => {
      const packageNumericId = toSdkworkMembershipNumber(pkg.id);
      const rank = resolvePlanRankFromPackage(pkg, plans);
      const display = PLAN_RANK_DISPLAY[rank] ?? PLAN_RANK_DISPLAY[1];
      const priceCny = toNullableSdkworkMembershipNumber(pkg.price);
      const originalPriceCny = toNullableSdkworkMembershipNumber(pkg.originalPrice);
      const durationDays = toSdkworkMembershipNumber(pkg.durationDays, 30);
      const isCurrentPlan = memberSummary?.membershipTierKey === display.membershipTierKey;
      const priceLabel = formatPriceLabel(priceCny);
      const discountTag = Array.isArray(pkg.tags) ? pkg.tags[0] : undefined;

      return {
        buttonDisabled: isCurrentPlan,
        buttonText: isCurrentPlan
          ? translate("current_plan", "当前计划")
          : discountTag
            ? `¥${priceLabel} ${discountTag}`
            : `¥${priceLabel} ${translate("first_year_60", "首年6折")}`,
        isCurrentPlan,
        membershipTierKey: display.membershipTierKey,
        name: translate(
          rank === 2 ? "standard_plan" : rank === 3 ? "premium_plan" : "basic_plan",
          pkg.name || "Membership",
        ),
        packageId: String(packageNumericId),
        packageNumericId,
        packagePeriodLabel: resolvePeriodLabel(durationDays, translate),
        priceLabel,
        originalPriceLabel: originalPriceCny ? formatOriginalPriceLabel(originalPriceCny) : undefined,
      };
    });
}

export function buildComparisonCategories(
  benefitsByRank: Readonly<Record<number, readonly RemoteMembershipCatalogBenefit[]>>,
): SdkworkSubscriptionCatalogComparisonCategory[] {
  const categories = new Map<string, SdkworkSubscriptionCatalogComparisonCategory>();

  for (const row of COMPARE_BENEFIT_ROWS) {
    const category = categories.get(row.categoryLabel) ?? {
      categoryLabel: row.categoryLabel,
      rows: [],
    };
    category.rows.push({
      benefitLabel: row.label,
      values: [0, 1, 2, 3, 4].map((rank) => row.valueForRank(rank, benefitsByRank[rank] ?? [])),
    });
    categories.set(row.categoryLabel, category);
  }

  return [...categories.values()];
}

export function resolveMemberSummaryFromPlanRank(planRank: number | null): SdkworkSubscriptionCatalogMemberSummaryModel {
  if (planRank === null || planRank <= 0) {
    return { membershipTierKey: "free", planRank: planRank ?? 0 };
  }

  const display = PLAN_RANK_DISPLAY[planRank];
  return {
    membershipTierKey: display?.membershipTierKey ?? "pro",
    planRank,
  };
}
