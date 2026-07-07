import { Button } from "@sdkwork/ui-pc-react";
import type { SdkworkMembershipPlan, SdkworkMembershipSummary } from "@sdkwork/membership-pc-membership";
import type { SdkworkSubscriptionPackageGroup, SdkworkSubscriptionPlanEstimateInput } from "../subscription";
import { useSdkworkSubscriptionIntl } from "../subscription-intl";

export interface SdkworkSubscriptionPlanGridProps {
  hideHeader?: boolean;
  onSelectPackageGroup: (packageGroupId: number) => void;
  onSelectPlan: (packageId: number) => void;
  packageGroups: SdkworkSubscriptionPackageGroup[];
  plans?: SdkworkMembershipPlan[];
  selectedPackageGroupId: number | null;
  selectedPackageId: number | null;
  summary: SdkworkMembershipSummary;
}

function formatPlanPriceAmount(
  price: number,
  formatCurrencyCny: (value: number | null | undefined) => string,
): string {
  return formatCurrencyCny(price).replace(/^¥\s*/, "");
}

function resolvePeriodLabel(days: number | null | undefined, copy: ReturnType<typeof useSdkworkSubscriptionIntl>["copy"]): string {
  if (!days || days <= 0) return "";
  if (days >= 365) return `/ ${copy.common.days === "days" ? "year" : "年"}`;
  if (days >= 90) return `/ ${copy.common.days === "days" ? "quarter" : "季"}`;
  if (days >= 30) return `/ ${copy.common.days === "days" ? "month" : "月"}`;
  return `/ ${days} ${copy.common.days}`;
}

export function SdkworkSubscriptionPlanGrid({
  hideHeader = false,
  onSelectPackageGroup,
  onSelectPlan,
  packageGroups,
  plans: plansProp,
  selectedPackageGroupId,
  selectedPackageId,
  summary,
}: SdkworkSubscriptionPlanGridProps) {
  const { copy, formatCurrencyCny, formatPoints } = useSdkworkSubscriptionIntl();

  const groups = packageGroups ?? [];
  const directPlans = plansProp ?? [];

  const hasGroups = groups.length > 0;
  const selectedGroup = hasGroups
    ? (groups.find((g) => g.packageGroupId === selectedPackageGroupId) ?? groups[0] ?? null)
    : null;

  const paidPlans: SdkworkSubscriptionPlanEstimateInput[] = hasGroups && selectedGroup
    ? selectedGroup.packages
    : directPlans.map((p) => ({
        description: p.description ?? null,
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

  return (
    <section className="space-y-8">
      {!hideHeader ? (
        <div className="flex flex-col items-center space-y-4 text-center">
          <div className="text-[0.7rem] font-semibold uppercase tracking-[0.22em] text-[var(--sdk-color-text-muted)]">
            {copy.planGrid.titleEyebrow}
          </div>
          <h2 className="text-3xl font-bold tracking-tight text-[var(--sdk-color-text-primary)] sm:text-4xl">
            {copy.planGrid.title}
          </h2>
          <p className="max-w-2xl text-base text-[var(--sdk-color-text-secondary)]">
            {copy.planGrid.freeMembershipDescription}
          </p>
        </div>
      ) : null}

      {hasGroups && groups.length > 1 ? (
        <div className="flex justify-center">
          <div className="inline-flex rounded-full bg-[var(--sdk-color-surface-panel-muted)] p-1">
            {groups.map((group) => {
              const isSelected = group.packageGroupId === selectedPackageGroupId
                || (selectedPackageGroupId === null && group === groups[0]);
              return (
                <button
                  key={group.id}
                  type="button"
                  onClick={() => onSelectPackageGroup(group.packageGroupId)}
                  className={`relative rounded-full px-6 py-2.5 text-sm font-medium transition-all duration-200 ${
                    isSelected
                      ? "bg-[var(--sdk-color-brand-primary)] text-white shadow-lg"
                      : "text-[var(--sdk-color-text-secondary)] hover:text-[var(--sdk-color-text-primary)]"
                  }`}
                >
                  {group.name}
                  {group.description ? (
                    <span className="ml-1.5 text-xs opacity-80">{group.description}</span>
                  ) : null}
                </button>
              );
            })}
          </div>
        </div>
      ) : null}

      {paidPlans.length > 0 ? (
      <div className="grid gap-5 lg:grid-cols-4">
        {paidPlans.map((plan) => {
          const isSelected = plan.packageId === selectedPackageId;
          const isRecommended = plan.recommended;
          const isFree = plan.priceCny === 0;
          const isCurrentFree = isFree && !summary.isMember;

          return (
            <article
              key={plan.id}
              className={`relative flex flex-col rounded-3xl border-2 p-6 transition-all duration-300 ${
                isRecommended
                  ? "border-[var(--sdk-color-brand-primary)] bg-gradient-to-b from-[color-mix(in_srgb,var(--sdk-color-brand-primary)_8%,var(--sdk-color-surface-panel))] to-[var(--sdk-color-surface-panel)] shadow-xl scale-[1.02]"
                  : isSelected
                    ? "border-[var(--sdk-color-brand-primary)] bg-[var(--sdk-color-surface-panel)] shadow-lg"
                    : "border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)] hover:border-[var(--sdk-color-border-default)] hover:shadow-md"
              }`}
            >
              {isRecommended ? (
                <div className="absolute -top-3.5 left-1/2 -translate-x-1/2">
                  <span className="rounded-full bg-gradient-to-r from-amber-400 to-orange-500 px-4 py-1 text-xs font-bold text-white shadow-lg">
                    {copy.paymentMethods.recommended}
                  </span>
                </div>
              ) : null}

              {isFree ? (
                <div className="absolute -top-3.5 left-1/2 -translate-x-1/2">
                  <span className="rounded-full bg-[var(--sdk-color-surface-panel-muted)] border border-[var(--sdk-color-border-subtle)] px-4 py-1 text-xs font-medium text-[var(--sdk-color-text-secondary)]">
                    {copy.planGrid.entryTier}
                  </span>
                </div>
              ) : null}

              <div className="mb-5">
                <h3 className={`text-xl font-bold ${
                  isRecommended ? "text-[var(--sdk-color-brand-primary)]" : "text-[var(--sdk-color-text-primary)]"
                }`}>
                  {plan.name}
                </h3>
                <p className="mt-1.5 text-sm text-[var(--sdk-color-text-secondary)] leading-relaxed">
                  {plan.description}
                </p>
              </div>

              <div className="mb-5">
                <div className="flex items-baseline">
                  {plan.priceCny > 0 ? (
                    <>
                      <span className="text-sm text-[var(--sdk-color-text-secondary)]">¥</span>
                      <span className={`text-5xl font-bold tracking-tight ${
                        isRecommended ? "text-[var(--sdk-color-brand-primary)]" : "text-[var(--sdk-color-text-primary)]"
                      }`}>
                        {formatPlanPriceAmount(plan.priceCny, formatCurrencyCny)}
                      </span>
                    </>
                  ) : (
                    <span className={`text-5xl font-bold tracking-tight ${
                      isRecommended ? "text-[var(--sdk-color-brand-primary)]" : "text-[var(--sdk-color-text-primary)]"
                    }`}>
                      ¥0
                    </span>
                  )}
                  {!isFree && plan.durationDays ? (
                    <span className="ml-2 text-sm text-[var(--sdk-color-text-muted)]">
                      {resolvePeriodLabel(plan.durationDays, copy)}
                    </span>
                  ) : null}
                </div>
                {plan.originalPriceCny && plan.originalPriceCny > plan.priceCny ? (
                  <div className="mt-1 flex items-center gap-2">
                    <span className="text-sm text-[var(--sdk-color-text-muted)] line-through">
                      ¥{formatPlanPriceAmount(plan.originalPriceCny, formatCurrencyCny)}
                    </span>
                    <span className="rounded bg-red-50 px-1.5 py-0.5 text-xs font-medium text-red-500">
                      {copy.planGrid.savingsLabel}¥{formatPlanPriceAmount(plan.originalPriceCny - plan.priceCny, formatCurrencyCny)}
                    </span>
                  </div>
                ) : null}
              </div>

              <div className="mb-5 rounded-2xl bg-[var(--sdk-color-surface-panel-muted)] p-4">
                <div className="flex items-center gap-2">
                  <svg className="h-5 w-5 text-amber-500" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                  </svg>
                  <span className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">
                    {formatPoints(plan.includedPoints)} {copy.common.points}
                  </span>
                </div>
                {plan.tags.length > 0 ? (
                  <div className="mt-2 flex flex-wrap gap-1.5">
                    {plan.tags.map((tag) => (
                      <span
                        key={tag}
                        className="rounded-full bg-[var(--sdk-color-surface-panel)] px-2 py-0.5 text-xs text-[var(--sdk-color-text-secondary)]"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                ) : null}
              </div>

              {isFree ? (
                <Button
                  className="w-full rounded-2xl py-3.5 text-sm font-semibold"
                  disabled={isCurrentFree}
                  type="button"
                  variant={isCurrentFree ? "secondary" : "outline"}
                >
                  {isCurrentFree ? copy.planGrid.currentPlanButton : copy.planGrid.freeBaselineButton}
                </Button>
              ) : (
                <Button
                  className={`w-full rounded-2xl py-3.5 text-sm font-semibold transition-all duration-200 ${
                    isRecommended ? "shadow-lg hover:shadow-xl" : ""
                  }`}
                  onClick={() => onSelectPlan(plan.packageId)}
                  type="button"
                  variant={isSelected ? "primary" : isRecommended ? "primary" : "outline"}
                >
                  {isSelected ? copy.actions.selectedPlan : isRecommended ? copy.actions.activateMembership : copy.actions.selectPlan}
                </Button>
              )}
            </article>
          );
        })}
      </div>
      ) : (
        <div className="rounded-3xl border border-dashed border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] px-8 py-12 text-center text-sm text-[var(--sdk-color-text-secondary)]">
          {copy.planGrid.noPlans}
        </div>
      )}
    </section>
  );
}
