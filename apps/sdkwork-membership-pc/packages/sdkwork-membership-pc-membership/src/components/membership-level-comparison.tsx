import { Check, Crown, Lock, Minus, ShieldCheck } from "lucide-react";
import { Button } from "@sdkwork/ui-pc-react/components/ui/button";
import {
  createSdkworkMembershipPanelStyle,
  createSdkworkMembershipToneStyle,
} from "../membership-appearance";
import { useSdkworkMembershipIntl } from "../membership-intl";
import type { SdkworkMembershipLevel } from "../membership-service";

export interface SdkworkMembershipLevelComparisonProps {
  levels: SdkworkMembershipLevel[];
}

export function SdkworkMembershipLevelComparison({
  levels,
}: SdkworkMembershipLevelComparisonProps) {
  const { copy, formatIncludedPoints } = useSdkworkMembershipIntl();

  const sortedLevels = [...levels].sort((left, right) => left.levelValue - right.levelValue);
  const currentIndex = sortedLevels.findIndex((level) => level.isCurrent);

  return (
    <section className="space-y-5">
      {sortedLevels.length > 0 ? (
        <div className="rounded-[1.5rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-6 py-6 shadow-[var(--sdk-shadow-sm)]">
          <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">{copy.levels.ladderEyebrow}</div>
          <h2 className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{copy.levels.ladderTitle}</h2>

          <div className="mt-6 flex items-start gap-2 overflow-x-auto pb-2">
            {sortedLevels.map((level, index) => {
              const isPassed = currentIndex >= 0 && index < currentIndex;
              const isCurrent = level.isCurrent;
              const isLocked = currentIndex >= 0 && index > currentIndex;
              const tone = isCurrent ? "accent" : isPassed ? "success" : "neutral";

              return (
                <div className="flex flex-1 min-w-[7rem] flex-col items-center text-center" key={level.id}>
                  <div className="flex w-full items-center">
                    {index === 0 ? <div className="h-0.5 flex-1 bg-transparent" /> : (
                      <div
                        className="h-0.5 flex-1"
                        style={createSdkworkMembershipToneStyle(isPassed ? "success" : "neutral", {
                          backgroundWeight: isPassed ? 40 : 12,
                          borderWeight: 0,
                        })}
                      />
                    )}
                    <div
                      className="flex h-11 w-11 shrink-0 items-center justify-center rounded-full border-2"
                      style={createSdkworkMembershipToneStyle(tone, {
                        backgroundWeight: isCurrent ? 24 : isPassed ? 18 : 8,
                        borderWeight: isCurrent ? 48 : 32,
                      })}
                    >
                      {isCurrent ? (
                        <Crown className="h-5 w-5" />
                      ) : isPassed ? (
                        <Check className="h-5 w-5" />
                      ) : (
                        <Lock className="h-4 w-4" />
                      )}
                    </div>
                    {index === sortedLevels.length - 1 ? <div className="h-0.5 flex-1 bg-transparent" /> : (
                      <div
                        className="h-0.5 flex-1"
                        style={createSdkworkMembershipToneStyle(isPassed ? "success" : "neutral", {
                          backgroundWeight: 12,
                          borderWeight: 0,
                        })}
                      />
                    )}
                  </div>
                  <div className={`mt-3 text-sm font-semibold ${isCurrent ? "text-[var(--sdk-color-text-primary)]" : isLocked ? "text-[var(--sdk-color-text-muted)]" : "text-[var(--sdk-color-text-secondary)]"}`}>
                    {level.name}
                  </div>
                  <div className="mt-1 text-xs text-[var(--sdk-color-text-muted)]">
                    {level.requiredPoints !== null ? formatIncludedPoints(level.requiredPoints) : copy.common.noValue}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      ) : null}

      <div className="rounded-[1.5rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]">
        <div className="border-b border-[var(--sdk-color-border-subtle)] px-6 py-5">
          <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">{copy.levels.eyebrow}</div>
          <h2 className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{copy.levels.title}</h2>
        </div>

        <div className="grid gap-4 px-6 py-6 md:grid-cols-3">
          {sortedLevels.length === 0 ? (
            <div className="col-span-full rounded-[1.25rem] border border-dashed border-[var(--sdk-color-border-default)] px-5 py-12 text-center">
              <div className="mx-auto flex h-12 w-12 items-center justify-center rounded-[1rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)]">
                <Minus className="h-5 w-5 text-[var(--sdk-color-text-muted)]" />
              </div>
              <div className="mt-4 text-base font-semibold text-[var(--sdk-color-text-primary)]">{copy.levels.emptyTitle}</div>
              <div className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">{copy.levels.emptyDescription}</div>
            </div>
          ) : sortedLevels.map((level) => (
            <article
              className="flex flex-col rounded-[1.5rem] border bg-[var(--sdk-color-surface-panel-muted)] p-5"
              key={level.id}
              style={createSdkworkMembershipPanelStyle(level.isCurrent ? "brand" : "neutral", {
                backgroundWeight: level.isCurrent ? 10 : 6,
                borderWeight: level.isCurrent ? 24 : 16,
                surfaceColor: "var(--sdk-color-surface-panel-muted)",
              })}
            >
              <div className="flex items-center justify-between gap-3">
                <div className="text-lg font-semibold text-[var(--sdk-color-text-primary)]">{level.name}</div>
                {level.isCurrent ? (
                  <div
                    className="inline-flex items-center gap-1 rounded-full border px-3 py-1 text-[0.7rem] font-semibold uppercase tracking-[0.16em]"
                    style={createSdkworkMembershipToneStyle("success", {
                      backgroundWeight: 10,
                      borderWeight: 18,
                    })}
                  >
                    <ShieldCheck className="h-3.5 w-3.5" />
                    {copy.levels.currentLabel}
                  </div>
                ) : null}
              </div>
              <div className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">
                {level.description || copy.levels.descriptionFallback}
              </div>
              <div className="mt-5 rounded-[1rem] bg-[var(--sdk-color-surface-panel)] px-4 py-3 text-sm text-[var(--sdk-color-text-secondary)]">
                <div className="flex items-center justify-between gap-4">
                  <span className="text-[var(--sdk-color-text-muted)]">{copy.levels.requiredPoints}</span>
                  <span className="font-semibold tabular-nums text-[var(--sdk-color-text-primary)]">
                    {level.requiredPoints !== null ? formatIncludedPoints(level.requiredPoints) : copy.common.noValue}
                  </span>
                </div>
              </div>
              <Button className="mt-5 w-full" disabled={!level.isCurrent} type="button" variant={level.isCurrent ? "secondary" : "ghost"}>
                {level.isCurrent ? copy.levels.currentLevelAction : copy.levels.compareLevel}
              </Button>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}
