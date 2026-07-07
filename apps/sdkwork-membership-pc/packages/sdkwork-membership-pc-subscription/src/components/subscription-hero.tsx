import { useMemo } from "react";
import type { SdkworkMembershipSummary } from "@sdkwork/membership-pc-membership";
import { createSdkworkHeroStyle } from "@sdkwork/ui-pc-react/theme";
import {
  resolveAvailableSubscriptionActions,
  type SdkworkSubscriptionAction,
} from "../subscription";
import { useSdkworkSubscriptionIntl } from "../subscription-intl";

export interface SdkworkSubscriptionHeroProps {
  activeAction: SdkworkSubscriptionAction;
  onActionChange: (action: SdkworkSubscriptionAction) => void;
  summary: SdkworkMembershipSummary;
}

function interpolate(template: string, values: Record<string, number | string>): string {
  return Object.entries(values).reduce(
    (output, [key, value]) => output.replaceAll(`{${key}}`, String(value)),
    template,
  );
}

export function SdkworkSubscriptionHero({
  activeAction,
  onActionChange,
  summary,
}: SdkworkSubscriptionHeroProps) {
  const {
    copy,
    formatCurrencyCny,
    formatPoints,
    resolveActionLabel,
  } = useSdkworkSubscriptionIntl();

  const isVip = summary.isMember;
  const availableActions = useMemo(
    () => resolveAvailableSubscriptionActions(summary),
    [summary.isMember],
  );
  const displayName = summary.currentLevelName || copy.hero.freeTierLabel;
  const balanceValue = summary.pointBalance ?? summary.points ?? 0;

  return (
    <section
      className="relative overflow-hidden rounded-3xl px-8 py-10 text-white shadow-2xl sm:px-12 sm:py-14"
      style={createSdkworkHeroStyle()}
    >
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_rgba(255,255,255,0.15),_transparent_60%)]" />
      <div className="absolute -right-20 -top-20 h-80 w-80 rounded-full bg-white/5 blur-3xl" />
      <div className="absolute -bottom-20 -left-20 h-60 w-60 rounded-full bg-[color-mix(in_srgb,var(--sdk-color-brand-accent)_12%,transparent)] blur-3xl" />

      <div className="relative flex flex-col gap-8 lg:flex-row lg:items-end lg:justify-between">
        <div className="space-y-5">
          <div className="inline-flex items-center gap-2 rounded-full bg-white/15 px-4 py-1.5 backdrop-blur-sm">
            <span className="h-2 w-2 rounded-full bg-[var(--sdk-color-state-warning)] animate-pulse" />
            <span className="text-xs font-semibold uppercase tracking-wider text-white/90">
              {isVip ? copy.hero.memberBadgeLabel : copy.hero.upgradeBadgeLabel}
            </span>
          </div>

          <div>
            <h1 className="text-3xl font-bold tracking-tight sm:text-5xl">
              {isVip ? (
                <>
                  {copy.hero.welcomeBackLabel}
                  <span className="bg-gradient-to-r from-[var(--sdk-color-brand-accent)] to-[var(--sdk-color-brand-primary)] bg-clip-text text-transparent">
                    {displayName}
                  </span>
                </>
              ) : (
                <>
                  {copy.hero.freeUserTitle}
                  <span className="bg-gradient-to-r from-[var(--sdk-color-brand-accent)] to-[var(--sdk-color-brand-primary)] bg-clip-text text-transparent">
                    {copy.hero.freeUserTitleHighlight}
                  </span>
                  {copy.hero.freeUserTitleSuffix}
                </>
              )}
            </h1>
            <p className="mt-3 max-w-xl text-base text-white/75 leading-relaxed sm:text-lg">
              {isVip ? copy.hero.memberDescription : copy.hero.guestDescription}
            </p>
          </div>

          <div className="flex flex-wrap gap-2">
            {availableActions.map((action) => (
              <button
                key={action}
                onClick={() => onActionChange(action)}
                className={`rounded-full px-5 py-2 text-sm font-medium transition-all duration-200 ${
                  activeAction === action
                    ? "bg-[var(--sdk-color-surface-panel)] text-[var(--sdk-color-brand-primary)] shadow-lg"
                    : "bg-white/10 text-white/80 hover:bg-white/20 hover:text-white backdrop-blur-sm"
                }`}
                type="button"
              >
                {resolveActionLabel(action)}
              </button>
            ))}
          </div>
        </div>

        <div className="flex gap-4 sm:gap-6">
          <div className="rounded-2xl bg-white/10 px-6 py-5 backdrop-blur-sm">
            <div className="text-xs font-medium uppercase tracking-wider text-white/60">
              {copy.hero.currentStatusLabel}
            </div>
            <div className="mt-2 text-xl font-bold">{displayName}</div>
            {summary.remainingDays !== null && summary.remainingDays !== undefined ? (
              <div className="mt-1 text-sm text-white/70">
                {interpolate(copy.hero.remainingDaysLabel, { count: summary.remainingDays })}
              </div>
            ) : null}
          </div>

          <div className="rounded-2xl bg-white/10 px-6 py-5 backdrop-blur-sm">
            <div className="text-xs font-medium uppercase tracking-wider text-white/60">
              {isVip ? copy.hero.totalSpentLabel : copy.hero.currentPointsLabel}
            </div>
            <div className="mt-2 text-xl font-bold">
              {summary.totalSpent !== null && summary.totalSpent !== undefined && summary.totalSpent > 0
                ? formatCurrencyCny(summary.totalSpent)
                : `${balanceValue}${copy.hero.pointsUnit}`}
            </div>
            <div className="mt-1 text-sm text-white/70">
              {isVip ? copy.hero.thanksForSupportLabel : copy.hero.dailyPointsLabel}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
