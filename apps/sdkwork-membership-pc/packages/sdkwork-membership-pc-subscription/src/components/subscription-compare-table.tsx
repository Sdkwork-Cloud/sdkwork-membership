import { Fragment, useMemo } from "react";
import type {
  SdkworkMembershipBenefit,
  SdkworkMembershipLevel,
} from "@sdkwork/membership-pc-membership";
import { useSdkworkSubscriptionIntl } from "../subscription-intl";

export interface SdkworkSubscriptionCompareTableProps {
  benefits: SdkworkMembershipBenefit[];
  levels?: SdkworkMembershipLevel[];
}

type TierKey = "free" | "basic" | "standard" | "premium";

interface FeatureRow {
  category: string;
  features: Array<{
    name: string;
    values: Record<TierKey, string | boolean>;
  }>;
}

function CheckMark() {
  return (
    <span className="inline-flex h-6 w-6 items-center justify-center rounded-full bg-[color-mix(in_srgb,var(--sdk-color-state-success)_14%,transparent)] text-[var(--sdk-color-state-success)]">
      <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
      </svg>
    </span>
  );
}

function CrossMark() {
  return (
    <span className="inline-flex h-6 w-6 items-center justify-center rounded-full bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-muted)]">
      <svg className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </span>
  );
}

function FeatureCell({ value }: { value: string | boolean }) {
  if (typeof value === "boolean") {
    return value ? <CheckMark /> : <CrossMark />;
  }
  return <span className="text-sm font-medium text-[var(--sdk-color-text-primary)]">{value}</span>;
}

function formatTemplate(template: string, values: Record<string, number | string>): string {
  return Object.entries(values).reduce(
    (output, [key, value]) => output.replaceAll(`{${key}}`, String(value)),
    template,
  );
}

function resolveTierLevel(tier: TierKey): number {
  switch (tier) {
    case "free":
      return 0;
    case "basic":
      return 1;
    case "standard":
      return 2;
    case "premium":
      return 3;
  }
}

function resolveBenefitMinTier(benefit: SdkworkMembershipBenefit, index: number): number {
  const benefitKey = benefit.benefitKey?.trim().toLowerCase();
  if (benefitKey === "daily_points") {
    return 0;
  }
  if (
    benefitKey === "standard_queue"
    || benefitKey === "no_watermark"
    || benefitKey === "priority_queue"
  ) {
    return 1;
  }
  if (benefitKey === "fast_queue" || benefitKey === "priority_speed_up") {
    return 2;
  }
  if (
    benefitKey === "vip_queue"
    || benefitKey === "vip_support"
    || benefitKey === "ai_quota"
    || benefitKey === "exclusive_model"
  ) {
    return 3;
  }
  return Math.min(index, 3);
}

function resolveCategoryLabel(
  benefit: SdkworkMembershipBenefit,
  copy: ReturnType<typeof useSdkworkSubscriptionIntl>["copy"],
): string {
  switch ((benefit.type ?? "").trim().toLowerCase()) {
    case "points":
      return copy.compareTable.categoryGeneration;
    case "queue":
      return copy.compareTable.categorySpeed;
    case "service":
      return copy.compareTable.categorySupport;
    case "feature":
      return copy.compareTable.categoryGeneration;
    default:
      return copy.planGrid.benefitsEyebrow;
  }
}

function buildFeatureRows(
  benefits: SdkworkMembershipBenefit[],
  levels: SdkworkMembershipLevel[],
  copy: ReturnType<typeof useSdkworkSubscriptionIntl>["copy"],
): FeatureRow[] {
  const sortedLevels = [...levels].sort((left, right) => left.levelValue - right.levelValue);
  const tierKeys: TierKey[] = ["free", "basic", "standard", "premium"];
  const grouped = new Map<string, FeatureRow["features"]>();

  benefits.forEach((benefit, index) => {
    const category = resolveCategoryLabel(benefit, copy);
    const minTier = resolveBenefitMinTier(benefit, index);
    const values = tierKeys.reduce<Record<TierKey, string | boolean>>((accumulator, tier) => {
      const tierLevel = resolveTierLevel(tier);
      if (benefit.benefitKey === "daily_points") {
        if (tierLevel === 0) {
          accumulator[tier] = copy.compareTable.pointsDaily;
          return accumulator;
        }
        const level = sortedLevels.find((entry) => entry.levelValue === tierLevel);
        if (level?.requiredPoints != null) {
          accumulator[tier] = formatTemplate(copy.compareTable.pointsPerMonth, {
            count: level.requiredPoints,
          });
          return accumulator;
        }
      }

      if (tierLevel < minTier) {
        accumulator[tier] = false;
        return accumulator;
      }

      if (benefit.usageLimit != null) {
        accumulator[tier] = String(benefit.usageLimit);
        return accumulator;
      }

      if (benefit.description?.trim()) {
        accumulator[tier] = benefit.description;
        return accumulator;
      }

      accumulator[tier] = true;
      return accumulator;
    }, {
      free: false,
      basic: false,
      standard: false,
      premium: false,
    });

    const existing = grouped.get(category) ?? [];
    existing.push({
      name: benefit.name,
      values,
    });
    grouped.set(category, existing);
  });

  return [...grouped.entries()].map(([category, features]) => ({
    category,
    features,
  }));
}

export function SdkworkSubscriptionCompareTable({
  benefits,
  levels = [],
}: SdkworkSubscriptionCompareTableProps) {
  const { copy } = useSdkworkSubscriptionIntl();
  const featureRows = useMemo(
    () => buildFeatureRows(benefits, levels, copy),
    [benefits, levels, copy],
  );

  const planHeaders = [
    { key: "free" as const, name: copy.compareTable.freePlan, color: "text-[var(--sdk-color-text-muted)]" },
    { key: "basic" as const, name: copy.compareTable.planBasic, color: "text-[var(--sdk-color-brand-accent)]" },
    { key: "standard" as const, name: copy.compareTable.planStandard, color: "text-[var(--sdk-color-brand-primary)]" },
    { key: "premium" as const, name: copy.compareTable.planPremium, color: "text-[var(--sdk-color-state-warning)]" },
  ];

  return (
    <section className="overflow-hidden rounded-3xl border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)] shadow-lg">
      <div className="border-b border-[var(--sdk-color-border-subtle)] px-8 py-6">
        <h2 className="text-2xl font-bold text-[var(--sdk-color-text-primary)]">
          {copy.compareTable.title}
        </h2>
        <p className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">
          {copy.compareTable.description}
        </p>
      </div>

      {featureRows.length > 0 ? (
        <div className="overflow-x-auto">
          <table className="w-full min-w-[800px]">
            <thead>
              <tr className="border-b border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)]">
                <th className="w-[22%] px-6 py-4 text-left text-sm font-semibold text-[var(--sdk-color-text-secondary)]">
                  {copy.compareTable.featureHeader}
                </th>
                {planHeaders.map((plan) => (
                  <th key={plan.key} className="w-[19.5%] px-4 py-4 text-center">
                    <span className={`text-base font-bold ${plan.color}`}>{plan.name}</span>
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {featureRows.map((row, rowIndex) => (
                <Fragment key={`row-group-${rowIndex}`}>
                  <tr className="bg-[color-mix(in_srgb,var(--sdk-color-brand-primary)_4%,transparent)]">
                    <td colSpan={5} className="px-6 py-3">
                      <span className="text-sm font-bold text-[var(--sdk-color-brand-primary)]">
                        {row.category}
                      </span>
                    </td>
                  </tr>
                  {row.features.map((feature, featureIndex) => (
                    <tr
                      key={`feature-${rowIndex}-${featureIndex}`}
                      className="border-b border-[var(--sdk-color-border-subtle)] transition-colors hover:bg-[var(--sdk-color-surface-panel-muted)]"
                    >
                      <td className="px-6 py-4 text-sm text-[var(--sdk-color-text-primary)]">
                        {feature.name}
                      </td>
                      {planHeaders.map((plan) => (
                        <td key={`${feature.name}-${plan.key}`} className="px-4 py-4 text-center">
                          <div className="flex justify-center">
                            <FeatureCell value={feature.values[plan.key]} />
                          </div>
                        </td>
                      ))}
                    </tr>
                  ))}
                </Fragment>
              ))}
            </tbody>
          </table>
        </div>
      ) : (
        <div className="px-8 py-10 text-center text-sm text-[var(--sdk-color-text-secondary)]">
          {copy.compareTable.emptyDescription}
        </div>
      )}
    </section>
  );
}
