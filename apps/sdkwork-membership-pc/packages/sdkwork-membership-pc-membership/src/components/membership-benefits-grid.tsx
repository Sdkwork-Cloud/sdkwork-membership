import {
  CheckCircle2,
  Clock3,
  Gift,
  Lock,
  Shield,
  Sparkles,
  Star,
  Zap,
} from "lucide-react";
import {
  createSdkworkMembershipPanelStyle,
  createSdkworkMembershipToneStyle,
  type SdkworkMembershipVisualTone,
} from "../membership-appearance";
import { useSdkworkMembershipIntl } from "../membership-intl";
import type { SdkworkMembershipBenefit } from "../membership-service";

export interface SdkworkMembershipBenefitsGridProps {
  benefits: SdkworkMembershipBenefit[];
}

function resolveBenefitIcon(type: string | undefined) {
  const normalized = String(type || "").toLowerCase();
  if (normalized.includes("quota") || normalized.includes("credit") || normalized.includes("compute") || normalized.includes("render")) {
    return Zap;
  }
  if (normalized.includes("security") || normalized.includes("shield") || normalized.includes("protect")) {
    return Shield;
  }
  if (normalized.includes("gift") || normalized.includes("perk") || normalized.includes("reward")) {
    return Gift;
  }
  if (normalized.includes("star") || normalized.includes("premium") || normalized.includes("vip")) {
    return Star;
  }
  return Sparkles;
}

function resolveUsageTone(used: number, limit: number): SdkworkMembershipVisualTone {
  if (limit <= 0) {
    return "success";
  }
  const ratio = used / limit;
  if (ratio >= 1) {
    return "danger";
  }
  if (ratio >= 0.8) {
    return "warning";
  }
  return "success";
}

export function SdkworkMembershipBenefitsGrid({
  benefits,
}: SdkworkMembershipBenefitsGridProps) {
  const { copy, formatUsage } = useSdkworkMembershipIntl();

  return (
    <section className="rounded-[1.5rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]">
      <div className="border-b border-[var(--sdk-color-border-subtle)] px-6 py-5">
        <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">{copy.benefits.eyebrow}</div>
        <h2 className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{copy.benefits.title}</h2>
      </div>

      <div className="grid gap-4 px-6 py-6 md:grid-cols-2 xl:grid-cols-3">
        {benefits.length === 0 ? (
          <div className="col-span-full rounded-[1.25rem] border border-dashed border-[var(--sdk-color-border-default)] px-5 py-12 text-center">
            <div className="mx-auto flex h-12 w-12 items-center justify-center rounded-[1rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)]">
              <Gift className="h-5 w-5 text-[var(--sdk-color-text-muted)]" />
            </div>
            <div className="mt-4 text-base font-semibold text-[var(--sdk-color-text-primary)]">{copy.benefits.emptyTitle}</div>
            <div className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">{copy.benefits.emptyDescription}</div>
          </div>
        ) : benefits.map((benefit) => {
          const BenefitIcon = resolveBenefitIcon(benefit.type);
          const statusTone = benefit.claimed ? "success" : "warning";
          const usageRatio = benefit.usageLimit !== null && benefit.usageLimit > 0
            ? Math.min((benefit.usedCount ?? 0) / benefit.usageLimit, 1)
            : 0;
          const usageTone = benefit.usageLimit !== null
            ? resolveUsageTone(benefit.usedCount ?? 0, benefit.usageLimit)
            : "success";

          return (
            <article
              className="flex flex-col rounded-[1.5rem] border bg-[var(--sdk-color-surface-panel-muted)] p-5"
              key={benefit.id}
              style={createSdkworkMembershipPanelStyle(statusTone, {
                backgroundWeight: 6,
                borderWeight: 16,
                surfaceColor: "var(--sdk-color-surface-panel-muted)",
              })}
            >
              <div className="flex items-start justify-between gap-3">
                <div className="min-w-0">
                  <div className="truncate text-lg font-semibold text-[var(--sdk-color-text-primary)]">{benefit.name}</div>
                  <div className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">
                    {benefit.description || copy.benefits.descriptionFallback}
                  </div>
                </div>
                <div
                  className="flex h-11 w-11 shrink-0 items-center justify-center rounded-[1rem] border"
                  style={createSdkworkMembershipToneStyle(statusTone, {
                    backgroundWeight: 12,
                    borderWeight: 22,
                  })}
                >
                  <BenefitIcon className="h-5 w-5" />
                </div>
              </div>

              {benefit.displayValue ? (
                <div className="mt-5">
                  <div className="flex items-center justify-between gap-4 text-xs">
                    <span className="font-medium text-[var(--sdk-color-text-muted)]">
                      {copy.benefits.valueLabel || "Value"}
                    </span>
                    <span className="font-semibold tabular-nums text-[var(--sdk-color-text-primary)]">
                      {benefit.displayValue}
                    </span>
                  </div>
                </div>
              ) : benefit.usageLimit !== null ? (
                <div className="mt-5">
                  <div className="flex items-center justify-between gap-4 text-xs">
                    <span className="font-medium text-[var(--sdk-color-text-muted)]">
                      {formatUsage(benefit.usedCount, benefit.usageLimit)}
                    </span>
                    <span
                      className="font-semibold tabular-nums"
                      style={createSdkworkMembershipToneStyle(usageTone, {
                        backgroundWeight: 0,
                        borderWeight: 0,
                      })}
                    >
                      {Math.round(usageRatio * 100)}%
                    </span>
                  </div>
                  <div className="mt-2 h-1.5 overflow-hidden rounded-full bg-[var(--sdk-color-surface-panel)]">
                    <div
                      className="h-full rounded-full transition-[width] duration-500 ease-out"
                      style={{
                        width: `${Math.round(usageRatio * 100)}%`,
                        ...createSdkworkMembershipToneStyle(usageTone, {
                          backgroundWeight: 60,
                          borderWeight: 0,
                        }),
                      }}
                    />
                  </div>
                </div>
              ) : null}

              <div className="mt-auto flex flex-wrap gap-2 pt-5 text-xs">
                <span className="rounded-full bg-[var(--sdk-color-surface-panel)] px-3 py-1 font-medium text-[var(--sdk-color-text-secondary)]">
                  {benefit.type || copy.benefits.typeFallback}
                </span>
                <span
                  className="inline-flex items-center gap-1 rounded-full border px-3 py-1 font-medium"
                  style={createSdkworkMembershipToneStyle(statusTone, {
                    backgroundWeight: 10,
                    borderWeight: 18,
                  })}
                >
                  {benefit.claimed ? (
                    <>
                      <CheckCircle2 className="h-3 w-3" />
                      {copy.benefits.claimed}
                    </>
                  ) : (
                    <>
                      <Clock3 className="h-3 w-3" />
                      {copy.benefits.pending}
                    </>
                  )}
                </span>
              </div>
            </article>
          );
        })}
      </div>
    </section>
  );
}
