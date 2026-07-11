import {
  Crown,
  Sparkles,
  TrendingUp,
} from "lucide-react";
import { Button } from "@sdkwork/ui-pc-react/components/ui/actions";
import {
  formatSdkworkMembershipCurrencyCny as formatSdkworkCurrencyCny,
} from "@sdkwork/membership-service";
import {
  createSdkworkMembershipGlassStyle,
  createSdkworkMembershipHeroStyle,
  createSdkworkMembershipHeroTextStyle,
  createSdkworkMembershipToneStyle,
} from "../membership-appearance";
import { useSdkworkMembershipIntl } from "../membership-intl";
import type {
  SdkworkMembershipLevel,
  SdkworkMembershipPlan,
  SdkworkMembershipSummary,
} from "../membership-service";

export interface SdkworkMembershipMembershipHeroProps {
  isMutating: boolean;
  levels?: SdkworkMembershipLevel[];
  onPurchase: () => void;
  onRenew: () => void;
  onUpgrade: () => void;
  selectedPlan?: SdkworkMembershipPlan | null;
  summary: SdkworkMembershipSummary;
}

interface SdkworkMembershipHeroProgress {
  isTopLevel: boolean;
  nextLevelName: string | null;
  percent: number;
  pointsToNext: number | null;
}

function resolveHeroProgress(
  summary: SdkworkMembershipSummary,
  levels: SdkworkMembershipLevel[] | undefined,
): SdkworkMembershipHeroProgress {
  const currentLevelValue = summary.currentLevelValue;
  const growthValue = summary.growthValue ?? 0;

  if (!levels || levels.length === 0 || currentLevelValue === null) {
    return {
      isTopLevel: false,
      nextLevelName: null,
      percent: 0,
      pointsToNext: null,
    };
  }

  const sortedLevels = [...levels].sort((left, right) => left.levelValue - right.levelValue);
  const currentIndex = sortedLevels.findIndex((level) => level.levelValue === currentLevelValue);
  const nextLevel = currentIndex >= 0 && currentIndex < sortedLevels.length - 1
    ? sortedLevels[currentIndex + 1]
    : null;

  if (!nextLevel) {
    return {
      isTopLevel: true,
      nextLevelName: null,
      percent: 100,
      pointsToNext: null,
    };
  }

  const currentLevel = sortedLevels[currentIndex];
  const floor = currentLevel?.requiredPoints ?? 0;
  const ceiling = nextLevel.requiredPoints ?? growthValue;
  const span = Math.max(ceiling - floor, 1);
  const clampedGrowth = Math.min(Math.max(growthValue - floor, 0), span);
  const percent = Math.round((clampedGrowth / span) * 100);
  const pointsToNext = Math.max(ceiling - growthValue, 0);

  return {
    isTopLevel: false,
    nextLevelName: nextLevel.name,
    percent,
    pointsToNext,
  };
}

export function SdkworkMembershipMembershipHero({
  isMutating,
  levels,
  onPurchase,
  onRenew,
  onUpgrade,
  selectedPlan,
  summary,
}: SdkworkMembershipMembershipHeroProps) {
  const {
    copy,
    formatDuration,
    formatIncludedPoints,
    formatPointsToNext,
    formatPriceWas,
    formatStatus,
    locale,
  } = useSdkworkMembershipIntl();
  const selectedPrice = selectedPlan
    ? formatSdkworkCurrencyCny(selectedPlan.priceCny, locale)
    : copy.common.noValue;
  const selectedOriginalPrice = selectedPlan?.originalPriceCny
    && selectedPlan.originalPriceCny > selectedPlan.priceCny
    ? formatPriceWas(formatSdkworkCurrencyCny(selectedPlan.originalPriceCny, locale))
    : null;
  const primaryHeroTextStyle = createSdkworkMembershipHeroTextStyle();
  const mutedHeroTextStyle = createSdkworkMembershipHeroTextStyle("muted");
  const subtleHeroTextStyle = createSdkworkMembershipHeroTextStyle("subtle");
  const progress = resolveHeroProgress(summary, levels);

  return (
    <section
      className="overflow-hidden rounded-[2rem] border border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] px-6 py-7 text-white shadow-[var(--sdk-shadow-lg)] sm:px-8 sm:py-9"
      style={createSdkworkMembershipHeroStyle()}
    >
      <div className="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between">
        <div className="max-w-3xl">
          <div className="flex flex-wrap items-center gap-3">
            <div
              className="inline-flex items-center gap-2 rounded-full border px-3 py-1 text-[0.7rem] font-semibold uppercase tracking-[0.18em]"
              style={{
                ...createSdkworkMembershipGlassStyle("accent", {
                  backgroundWeight: 12,
                  borderWeight: 22,
                  surfaceWeight: 82,
                }),
                ...mutedHeroTextStyle,
              }}
            >
              <Sparkles className="h-3.5 w-3.5" />
              {copy.hero.eyebrow}
            </div>
            <div
              className="inline-flex items-center gap-1.5 rounded-full border px-3 py-1 text-[0.7rem] font-semibold uppercase tracking-[0.16em]"
              style={createSdkworkMembershipToneStyle("warning", {
                backgroundWeight: 20,
                borderWeight: 32,
              })}
            >
              <Crown className="h-3.5 w-3.5" />
              {summary.currentLevelName}
            </div>
          </div>
          <h1 className="mt-4 text-4xl font-semibold tracking-tight" style={primaryHeroTextStyle}>{copy.hero.title}</h1>
          <p className="mt-3 text-sm leading-7" style={mutedHeroTextStyle}>
            {copy.hero.description}
          </p>
        </div>

        <div className="flex flex-wrap gap-3 lg:shrink-0">
          <Button disabled={!selectedPlan} loading={isMutating} onClick={summary.isMember ? onUpgrade : onPurchase} type="button" variant="secondary">
            {copy.actions.upgrade}
          </Button>
          <Button disabled={!selectedPlan || isMutating} onClick={onRenew} type="button" variant="outline">
            {copy.actions.renew}
          </Button>
        </div>
      </div>

      <div className="mt-8 grid gap-4 lg:grid-cols-[minmax(0,1.4fr)_minmax(20rem,1fr)]">
        <div
          className="rounded-[1.5rem] border p-6"
          style={createSdkworkMembershipGlassStyle("brand", {
            backgroundWeight: 12,
            borderWeight: 24,
            surfaceWeight: 84,
          })}
        >
          <div className="flex items-end justify-between gap-4">
            <div>
              <div className="text-xs font-semibold uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>{copy.hero.pointsLabel}</div>
              <div className="mt-2 text-5xl font-semibold tracking-tight tabular-nums" style={primaryHeroTextStyle}>
                {summary.points !== null ? formatIncludedPoints(summary.points) : copy.common.noValue}
              </div>
            </div>
            <div
              className="flex h-14 w-14 items-center justify-center rounded-[1.25rem] border"
              style={createSdkworkMembershipToneStyle("accent", {
                backgroundWeight: 18,
                borderWeight: 28,
              })}
            >
              <TrendingUp className="h-6 w-6" />
            </div>
          </div>

          <div className="mt-6">
            <div className="flex items-center justify-between gap-4 text-xs" style={subtleHeroTextStyle}>
              <span className="font-semibold uppercase tracking-[0.16em]">{copy.hero.currentLevel}</span>
              <span className="font-semibold tabular-nums" style={primaryHeroTextStyle}>{summary.currentLevelName}</span>
            </div>
            <div className="mt-4 flex items-center justify-between gap-4 text-xs" style={subtleHeroTextStyle}>
              <span className="font-semibold uppercase tracking-[0.16em]">{copy.hero.growthLabel}</span>
              <span className="tabular-nums">
                {progress.isTopLevel
                  ? copy.hero.topLevel
                  : progress.nextLevelName && progress.pointsToNext !== null
                    ? formatPointsToNext(progress.pointsToNext, progress.nextLevelName)
                    : copy.common.noValue}
              </span>
            </div>
            <div className="relative mt-2 h-2 overflow-hidden rounded-full" style={{ backgroundColor: "color-mix(in srgb, white 12%, transparent)" }}>
              <div
                className="absolute inset-y-0 left-0 rounded-full transition-[width] duration-500 ease-out"
                style={{
                  width: `${Math.min(Math.max(progress.percent, 0), 100)}%`,
                  backgroundImage: "linear-gradient(90deg, var(--sdk-color-brand-accent), var(--sdk-color-brand-primary))",
                }}
              />
            </div>
            <div className="mt-2 flex items-center justify-between gap-4 text-[0.7rem]" style={subtleHeroTextStyle}>
              <span>{summary.currentLevelName}</span>
              <span>{progress.nextLevelName ?? summary.currentLevelName}</span>
            </div>
          </div>

          <div className="mt-6 grid gap-3 sm:grid-cols-3">
            <div
              className="rounded-[1rem] border px-4 py-3"
              style={createSdkworkMembershipGlassStyle("neutral", {
                backgroundWeight: 10,
                borderWeight: 18,
                surfaceWeight: 78,
              })}
            >
              <div className="text-[0.7rem] uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>{copy.hero.status}</div>
              <div className="mt-2 text-sm font-semibold" style={primaryHeroTextStyle}>{formatStatus(summary.status)}</div>
            </div>
            <div
              className="rounded-[1rem] border px-4 py-3"
              style={createSdkworkMembershipGlassStyle("warning", {
                backgroundWeight: 10,
                borderWeight: 18,
                surfaceWeight: 78,
              })}
            >
              <div className="text-[0.7rem] uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>{copy.hero.remaining}</div>
              <div className="mt-2 text-sm font-semibold tabular-nums" style={primaryHeroTextStyle}>
                {formatDuration(summary.remainingDays)}
              </div>
            </div>
            <div
              className="rounded-[1rem] border px-4 py-3"
              style={createSdkworkMembershipGlassStyle("accent", {
                backgroundWeight: 10,
                borderWeight: 18,
                surfaceWeight: 78,
              })}
            >
              <div className="text-[0.7rem] uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>{copy.hero.points}</div>
              <div className="mt-2 text-sm font-semibold tabular-nums" style={primaryHeroTextStyle}>
                {summary.pointBalance !== null ? formatIncludedPoints(summary.pointBalance) : copy.common.noValue}
              </div>
            </div>
          </div>
        </div>

        <div
          className="rounded-[1.5rem] border p-6"
          style={createSdkworkMembershipGlassStyle("accent", {
            backgroundWeight: 12,
            borderWeight: 24,
            surfaceWeight: 84,
          })}
        >
          <div className="text-xs font-semibold uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>{copy.hero.selectedOffer}</div>
          <div className="mt-2 text-xl font-semibold" style={primaryHeroTextStyle}>
            {selectedPlan?.name ?? copy.hero.noPackageSelected}
          </div>

          <div
            className="mt-4 rounded-[1rem] border px-4 py-3 text-sm"
            style={{
              ...createSdkworkMembershipGlassStyle("neutral", {
                backgroundWeight: 10,
                borderWeight: 18,
                surfaceWeight: 76,
              }),
              ...mutedHeroTextStyle,
            }}
          >
            {selectedPlan?.description || copy.hero.noPackageDescription}
          </div>

          <div className="mt-4 grid gap-3 sm:grid-cols-2">
            <div
              className="rounded-[1rem] border px-4 py-3"
              style={createSdkworkMembershipGlassStyle("brand", {
                backgroundWeight: 10,
                borderWeight: 18,
                surfaceWeight: 78,
              })}
            >
              <div className="text-[0.7rem] uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>{copy.hero.price}</div>
              <div className="mt-2 text-lg font-semibold tabular-nums" style={primaryHeroTextStyle}>
                {selectedPrice}
              </div>
              {selectedOriginalPrice ? (
                <div className="mt-1 text-xs line-through" style={subtleHeroTextStyle}>
                  {selectedOriginalPrice}
                </div>
              ) : null}
            </div>
            <div
              className="rounded-[1rem] border px-4 py-3"
              style={createSdkworkMembershipGlassStyle("warning", {
                backgroundWeight: 10,
                borderWeight: 18,
                surfaceWeight: 78,
              })}
            >
              <div className="text-[0.7rem] uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>{copy.hero.includedPoints}</div>
              <div className="mt-2 text-lg font-semibold tabular-nums" style={primaryHeroTextStyle}>
                {selectedPlan ? formatIncludedPoints(selectedPlan.includedPoints) : copy.common.noValue}
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
