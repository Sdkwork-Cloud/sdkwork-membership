import { Fragment } from "react";
import type { SdkworkMembershipBenefit } from "@sdkwork/membership-pc-membership";
import { useSdkworkSubscriptionIntl } from "../subscription-intl";

export interface SdkworkSubscriptionCompareTableProps {
  benefits: SdkworkMembershipBenefit[];
}

interface FeatureRow {
  category: string;
  features: {
    name: string;
    free: string | boolean;
    basic: string | boolean;
    standard: string | boolean;
    premium: string | boolean;
  }[];
}

function CheckMark() {
  return (
    <span className="inline-flex h-6 w-6 items-center justify-center rounded-full bg-emerald-100 text-emerald-600">
      <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
      </svg>
    </span>
  );
}

function CrossMark() {
  return (
    <span className="inline-flex h-6 w-6 items-center justify-center rounded-full bg-gray-100 text-gray-400">
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

export function SdkworkSubscriptionCompareTable({ benefits }: SdkworkSubscriptionCompareTableProps) {
  const { copy } = useSdkworkSubscriptionIntl();

  const featureRows: FeatureRow[] = [
    {
      category: copy.compareTable.categoryGeneration,
      features: [
        { name: copy.compareTable.dailyPoints, free: copy.compareTable.pointsDaily, basic: formatTemplate(copy.compareTable.pointsPerMonth, { count: 725 }), standard: formatTemplate(copy.compareTable.pointsPerMonth, { count: 2210 }), premium: formatTemplate(copy.compareTable.pointsPerMonth, { count: 6160 }) },
        { name: copy.compareTable.seedanceVip, free: false, basic: true, standard: true, premium: true },
        { name: copy.compareTable.seedancePro, free: false, basic: copy.compareTable.discount20Off, standard: copy.compareTable.discount20Off, premium: copy.compareTable.discount20Off },
        { name: copy.compareTable.image2K, free: false, basic: true, standard: true, premium: true },
        { name: copy.compareTable.image4K, free: false, basic: false, standard: true, premium: true },
        { name: copy.compareTable.videoDuration, free: "5s", basic: "10s", standard: "15s", premium: "30s" },
      ],
    },
    {
      category: copy.compareTable.categorySpeed,
      features: [
        { name: copy.compareTable.standardSpeed, free: true, basic: true, standard: true, premium: false },
        { name: copy.compareTable.fastLane, free: false, basic: false, standard: true, premium: false },
        { name: copy.compareTable.vipLane, free: false, basic: false, standard: false, premium: true },
        { name: copy.compareTable.noWatermark, free: false, basic: true, standard: true, premium: true },
        { name: copy.compareTable.historyRetention, free: copy.compareTable.history7Days, basic: copy.compareTable.history30Days, standard: copy.compareTable.history90Days, premium: copy.compareTable.historyForever },
      ],
    },
    {
      category: copy.compareTable.categorySupport,
      features: [
        { name: copy.compareTable.refundGuarantee, free: false, basic: true, standard: true, premium: true },
        { name: copy.compareTable.supportDedicated, free: false, basic: false, standard: false, premium: true },
        { name: copy.compareTable.supportEarlyAccess, free: false, basic: false, standard: true, premium: true },
        { name: copy.compareTable.apiAccess, free: false, basic: false, standard: true, premium: true },
      ],
    },
  ];

  const planHeaders = [
    { key: "free", name: copy.compareTable.freePlan, color: "text-gray-600" },
    { key: "basic", name: copy.compareTable.planBasic, color: "text-blue-600" },
    { key: "standard", name: copy.compareTable.planStandard, color: "text-purple-600" },
    { key: "premium", name: copy.compareTable.planPremium, color: "text-amber-600" },
  ] as const;

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
                    <td className="px-4 py-4 text-center">
                      <div className="flex justify-center">
                        <FeatureCell value={feature.free} />
                      </div>
                    </td>
                    <td className="px-4 py-4 text-center">
                      <div className="flex justify-center">
                        <FeatureCell value={feature.basic} />
                      </div>
                    </td>
                    <td className="px-4 py-4 text-center">
                      <div className="flex justify-center">
                        <FeatureCell value={feature.standard} />
                      </div>
                    </td>
                    <td className="px-4 py-4 text-center">
                      <div className="flex justify-center">
                        <FeatureCell value={feature.premium} />
                      </div>
                    </td>
                  </tr>
                ))}
              </Fragment>
            ))}
          </tbody>
        </table>
      </div>

      {benefits.length > 0 ? (
        <div className="border-t border-[var(--sdk-color-border-subtle)] px-8 py-6">
          <div className="text-[0.7rem] font-semibold uppercase tracking-[0.22em] text-[var(--sdk-color-text-muted)]">
            {copy.planGrid.benefitsEyebrow}
          </div>
          <div className="mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            {benefits.map((benefit) => (
              <div
                key={benefit.id}
                className="rounded-2xl border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] p-4"
              >
                <div className="flex items-center justify-between">
                  <span className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">
                    {benefit.name}
                  </span>
                  {benefit.claimed ? (
                    <span className="inline-flex items-center gap-1 rounded-full bg-emerald-50 px-2 py-0.5 text-xs font-medium text-emerald-600">
                      <CheckMark />
                      {copy.common.current}
                    </span>
                  ) : null}
                </div>
                {benefit.usageLimit ? (
                  <div className="mt-2 text-xs text-[var(--sdk-color-text-secondary)]">
                    {benefit.usedCount ?? 0} / {benefit.usageLimit}
                  </div>
                ) : null}
              </div>
            ))}
          </div>
        </div>
      ) : null}
    </section>
  );
}
