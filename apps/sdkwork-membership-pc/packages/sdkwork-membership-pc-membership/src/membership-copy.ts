import { formatSdkworkMembershipPoints as formatSdkworkPoints } from "@sdkwork/membership-service";

export type SdkworkMembershipLocale = "en-US" | "zh-CN";

export type SdkworkMembershipMessagesOverrides = DeepPartial<SdkworkMembershipMessages>;

export interface SdkworkMembershipMessages {
  actions: {
    benefits: string;
    levels: string;
    plans: string;
    refresh: string;
    renew: string;
    selectPlan: string;
    selected: string;
    upgrade: string;
    claim: string;
    compare: string;
    viewBenefits: string;
  };
  benefits: {
    claimed: string;
    descriptionFallback: string;
    emptyDescription: string;
    emptyTitle: string;
    eyebrow: string;
    pending: string;
    title: string;
    typeFallback: string;
    usageValue: string;
    locked: string;
    unlockAt: string;
    unlimited: string;
  };
  common: {
    flexibleDuration: string;
    noValue: string;
    perYear: string;
    billedYearly: string;
    save: string;
  };
  format: {
    daysValue: string;
    priceWasValue: string;
    pointsToNext: string;
    expiresOn: string;
    memberSince: string;
  };
  headerEntry: {
    ariaLabel: string;
    fallbackLevel: string;
    title: string;
  };
  hero: {
    currentLevel: string;
    description: string;
    eyebrow: string;
    includedPoints: string;
    noPackageDescription: string;
    noPackageSelected: string;
    price: string;
    remaining: string;
    selectedOffer: string;
    status: string;
    title: string;
    points: string;
    progressToNext: string;
    nextLevel: string;
    topLevel: string;
    pointsLabel: string;
    growthLabel: string;
  };
  levels: {
    compareLevel: string;
    currentLevelAction: string;
    currentLabel: string;
    descriptionFallback: string;
    emptyDescription: string;
    emptyTitle: string;
    eyebrow: string;
    requiredPoints: string;
    title: string;
    ladderEyebrow: string;
    ladderTitle: string;
    locked: string;
    perks: string;
  };
  menu: {
    continueCheckout: string;
    emptyDescription: string;
    emptyTitle: string;
    openCenter: string;
    signInRequiredDescription: string;
    signInRequiredTitle: string;
    title: string;
  };
  page: {
    errorTitle: string;
    loading: string;
    subtitle: string;
  };
  plans: {
    descriptionFallback: string;
    emptyDescription: string;
    emptyTitle: string;
    eyebrow: string;
    popular: string;
    title: string;
    subtitle: string;
    features: string;
    allFeatures: string;
    duration: string;
    pointsIncluded: string;
  };
  service: {
    purchaseFailed: string;
    renewFailed: string;
    signInRequired: string;
    upgradeFailed: string;
  };
  status: {
    active: string;
    free: string;
    guest: string;
  };
}

type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends (...args: never[]) => unknown
    ? T[K]
    : T[K] extends object
      ? DeepPartial<T[K]>
      : T[K];
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function mergeDeep<T>(base: T, overrides?: DeepPartial<T>): T {
  if (!overrides) {
    return base;
  }

  const output: Record<string, unknown> = {
    ...(base as Record<string, unknown>),
  };

  for (const [key, value] of Object.entries(overrides)) {
    if (value === undefined) {
      continue;
    }

    const baseValue = output[key];
    output[key] = isRecord(baseValue) && isRecord(value)
      ? mergeDeep(baseValue, value)
      : value;
  }

  return output as T;
}

function interpolateTemplate(
  template: string,
  replacements: Record<string, string>,
): string {
  return Object.entries(replacements).reduce(
    (current, [key, value]) => current.replaceAll(`{${key}}`, value),
    template,
  );
}

const EN_US_MESSAGES: SdkworkMembershipMessages = {
  actions: {
    benefits: "Benefits",
    levels: "Levels",
    plans: "Plans",
    refresh: "Refresh",
    renew: "Renew",
    selectPlan: "Select plan",
    selected: "Selected",
    upgrade: "Upgrade now",
    claim: "Claim",
    compare: "Compare",
    viewBenefits: "View benefits",
  },
  benefits: {
    claimed: "Claimed",
    descriptionFallback: "Premium membership benefit",
    emptyDescription: "Benefit unlocks and usage allowances will appear here when they are available.",
    emptyTitle: "No membership benefits",
    eyebrow: "Benefits",
    pending: "Pending",
    title: "Membership benefits",
    typeFallback: "benefit",
    usageValue: "{used}/{limit} used",
    locked: "Locked",
    unlockAt: "Unlock at {level}",
    unlimited: "Unlimited",
  },
  common: {
    flexibleDuration: "Flexible duration",
    noValue: "--",
    perYear: "/year",
    billedYearly: "Billed yearly",
    save: "Save {percent}%",
  },
  format: {
    daysValue: "{value} days",
    priceWasValue: "Was {value}",
    pointsToNext: "{value} points to {level}",
    expiresOn: "Expires {date}",
    memberSince: "Member since {date}",
  },
  headerEntry: {
    ariaLabel: "Open Token Plan menu",
    fallbackLevel: "Token Plan",
    title: "Token Plan",
  },
  hero: {
    currentLevel: "Current level",
    description: "Manage your plan, track benefits, and grow your membership tier — all in one place.",
    eyebrow: "Membership",
    includedPoints: "Included points",
    noPackageDescription: "Pick a package to compare price, included points, and membership duration.",
    noPackageSelected: "No package selected",
    price: "Price",
    remaining: "Remaining",
    selectedOffer: "Selected offer",
    status: "Status",
    title: "Membership Center",
    points: "Membership points",
    progressToNext: "Progress to {level}",
    nextLevel: "Next level",
    topLevel: "Top level reached",
    pointsLabel: "Points",
    growthLabel: "Growth",
  },
  levels: {
    compareLevel: "Compare level",
    currentLevelAction: "Current level",
    currentLabel: "Current",
    descriptionFallback: "Premium membership level",
    emptyDescription: "Level comparison will appear when membership level data is available.",
    emptyTitle: "No membership levels",
    eyebrow: "Levels",
    requiredPoints: "Required points",
    title: "Level comparison",
    ladderEyebrow: "Tier ladder",
    ladderTitle: "Your membership journey",
    locked: "Locked",
    perks: "Perks",
  },
  menu: {
    continueCheckout: "Continue to checkout",
    emptyDescription: "Token plans will appear here when membership packages are available.",
    emptyTitle: "No token plans",
    openCenter: "Open membership center",
    signInRequiredDescription: "Sign in to view token plans and continue checkout.",
    signInRequiredTitle: "Sign in required",
    title: "Token plans",
  },
  page: {
    errorTitle: "Membership center error",
    loading: "Loading membership center...",
    subtitle: "Plan, benefits, and tier progression",
  },
  plans: {
    descriptionFallback: "Premium membership offer",
    emptyDescription: "No membership plans are currently available in this workspace.",
    emptyTitle: "No membership plans",
    eyebrow: "Plans",
    popular: "Most popular",
    title: "Membership plans",
    subtitle: "Pick the plan that fits your workflow",
    features: "What's included",
    allFeatures: "All features",
    duration: "Duration",
    pointsIncluded: "Points included",
  },
  service: {
    purchaseFailed: "Failed to purchase membership.",
    renewFailed: "Failed to renew membership.",
    signInRequired: "Please sign in to manage memberships.",
    upgradeFailed: "Failed to upgrade membership.",
  },
  status: {
    active: "Active",
    free: "Free",
    guest: "Guest",
  },
};

const ZH_CN_MESSAGES: SdkworkMembershipMessages = {
  actions: {
    benefits: "权益",
    levels: "等级",
    plans: "方案",
    refresh: "刷新",
    renew: "续费",
    selectPlan: "选择方案",
    selected: "已选择",
    upgrade: "立即升级",
    claim: "领取",
    compare: "对比",
    viewBenefits: "查看权益",
  },
  benefits: {
    claimed: "已领取",
    descriptionFallback: "高级会员权益",
    emptyDescription: "当有可用的解锁权益和配额时，会在这里展示。",
    emptyTitle: "暂无会员权益",
    eyebrow: "权益",
    pending: "待使用",
    title: "会员权益",
    typeFallback: "权益",
    usageValue: "已用 {used}/{limit}",
    locked: "未解锁",
    unlockAt: "{level} 解锁",
    unlimited: "不限",
  },
  common: {
    flexibleDuration: "时长灵活",
    noValue: "--",
    perYear: "/年",
    billedYearly: "按年结算",
    save: "省 {percent}%",
  },
  format: {
    daysValue: "{value} 天",
    priceWasValue: "原价 {value}",
    pointsToNext: "距 {level} 还差 {value} 成长值",
    expiresOn: "{date} 到期",
    memberSince: "{date} 加入",
  },
  headerEntry: {
    ariaLabel: "打开 Token Plan 菜单",
    fallbackLevel: "Token Plan",
    title: "Token Plan",
  },
  hero: {
    currentLevel: "当前等级",
    description: "在一个中心管理你的方案、权益与等级成长。",
    eyebrow: "会员",
    includedPoints: "包含积分",
    noPackageDescription: "选择一个套餐查看价格、积分与会员时长。",
    noPackageSelected: "暂未选择套餐",
    price: "价格",
    remaining: "剩余时长",
    selectedOffer: "已选套餐",
    status: "状态",
    title: "会员中心",
    points: "会员积分",
    progressToNext: "升级到 {level}",
    nextLevel: "下一等级",
    topLevel: "已达到最高等级",
    pointsLabel: "积分",
    growthLabel: "成长值",
  },
  levels: {
    compareLevel: "对比等级",
    currentLevelAction: "当前等级",
    currentLabel: "当前",
    descriptionFallback: "高级会员等级",
    emptyDescription: "当会员等级数据可用时，这里会展示对比信息。",
    emptyTitle: "暂无会员等级",
    eyebrow: "等级",
    requiredPoints: "需要成长值",
    title: "等级对比",
    ladderEyebrow: "等级阶梯",
    ladderTitle: "你的会员成长之路",
    locked: "未解锁",
    perks: "专属权益",
  },
  menu: {
    continueCheckout: "前往结算",
    emptyDescription: "当有可用会员套餐时，Token Plan 会显示在这里。",
    emptyTitle: "暂无 Token Plan",
    openCenter: "打开会员中心",
    signInRequiredDescription: "登录后查看 Token Plan 并继续结算。",
    signInRequiredTitle: "请先登录",
    title: "Token Plan 方案",
  },
  page: {
    errorTitle: "会员中心异常",
    loading: "正在加载会员中心...",
    subtitle: "方案、权益与等级成长",
  },
  plans: {
    descriptionFallback: "高级会员套餐",
    emptyDescription: "当前工作区暂无可用的会员套餐。",
    emptyTitle: "暂无会员方案",
    eyebrow: "方案",
    popular: "最受欢迎",
    title: "会员方案",
    subtitle: "选择适合你的方案",
    features: "包含内容",
    allFeatures: "全部功能",
    duration: "时长",
    pointsIncluded: "包含积分",
  },
  service: {
    purchaseFailed: "购买会员失败。",
    renewFailed: "续费会员失败。",
    signInRequired: "请先登录后再管理会员。",
    upgradeFailed: "升级会员失败。",
  },
  status: {
    active: "生效中",
    free: "免费",
    guest: "游客",
  },
};

const SDKWORK_MEMBERSHIP_MESSAGES: Record<SdkworkMembershipLocale, SdkworkMembershipMessages> = {
  "en-US": EN_US_MESSAGES,
  "zh-CN": ZH_CN_MESSAGES,
};

export function normalizeSdkworkMembershipLocale(locale?: string | null): SdkworkMembershipLocale {
  const normalized = String(locale || "").trim().toLowerCase();
  if (normalized.startsWith("zh")) {
    return "zh-CN";
  }

  return "en-US";
}

export function createSdkworkMembershipMessages(
  locale?: string | null,
  overrides?: SdkworkMembershipMessagesOverrides,
): SdkworkMembershipMessages {
  return mergeDeep(
    SDKWORK_MEMBERSHIP_MESSAGES[normalizeSdkworkMembershipLocale(locale)],
    overrides,
  );
}

export function formatSdkworkMembershipTemplate(
  template: string,
  replacements: Record<string, string>,
): string {
  return interpolateTemplate(template, replacements);
}

export function formatSdkworkMembershipStatusLabel(
  value: "active" | "free" | "guest",
  locale?: string | null,
  overrides?: SdkworkMembershipMessagesOverrides,
): string {
  const copy = createSdkworkMembershipMessages(locale, overrides);
  return copy.status[value];
}

export function formatSdkworkMembershipDurationLabel(
  value: number | null,
  locale?: string | null,
  overrides?: SdkworkMembershipMessagesOverrides,
): string {
  const copy = createSdkworkMembershipMessages(locale, overrides);

  if (value === null) {
    return copy.common.flexibleDuration;
  }

  return interpolateTemplate(copy.format.daysValue, {
    value: String(value),
  });
}

export function formatSdkworkMembershipIncludedPointsLabel(
  value: number,
  locale?: string | null,
): string {
  return formatSdkworkPoints(value, normalizeSdkworkMembershipLocale(locale));
}

export function formatSdkworkMembershipUsageLabel(
  used: number | null,
  limit: number | null,
  locale?: string | null,
  overrides?: SdkworkMembershipMessagesOverrides,
): string {
  const copy = createSdkworkMembershipMessages(locale, overrides);

  if (limit === null) {
    return copy.common.noValue;
  }

  return interpolateTemplate(copy.benefits.usageValue, {
    limit: String(limit),
    used: String(used ?? 0),
  });
}

export function formatSdkworkMembershipPriceWasLabel(
  value: string,
  locale?: string | null,
  overrides?: SdkworkMembershipMessagesOverrides,
): string {
  const copy = createSdkworkMembershipMessages(locale, overrides);

  return interpolateTemplate(copy.format.priceWasValue, {
    value,
  });
}
