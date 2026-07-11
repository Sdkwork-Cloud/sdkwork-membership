import { useEffect } from "react";
import { Button } from "@sdkwork/ui-pc-react/components/ui/actions";
import {
  LoadingBlock,
  StatusNotice,
} from "@sdkwork/ui-pc-react/components/ui/feedback";
import { formatSdkworkMembershipCurrencyCny as formatSdkworkCurrencyCny } from "@sdkwork/membership-service";
import type { SdkworkMembershipMessagesOverrides } from "../membership-copy";
import type { SdkworkMembershipController } from "../membership-controller";
import {
  useSdkworkMembershipController,
  useSdkworkMembershipControllerState,
} from "../membership-controller";
import {
  createMembershipCheckoutRouteIntent,
  type SdkworkMembershipPurchaseMode,
} from "../membership.ts";
import {
  createSdkworkMembershipBackdropStyle,
  createSdkworkMembershipPanelStyle,
  createSdkworkMembershipToneStyle,
} from "../membership-appearance";
import {
  SdkworkMembershipIntlProvider,
  useSdkworkMembershipIntl,
} from "../membership-intl";
import { SdkworkMembershipBenefitsGrid } from "../components/membership-benefits-grid";
import { SdkworkMembershipLevelComparison } from "../components/membership-level-comparison";
import { SdkworkMembershipMembershipHero } from "../components/membership-hero";

export interface SdkworkMembershipPageProps {
  checkoutBasePath?: string;
  controller?: SdkworkMembershipController;
  locale?: string | null;
  messages?: SdkworkMembershipMessagesOverrides;
  onNavigate?: (route: string) => void;
  purchaseFlow?: "checkout" | "direct";
}

interface SdkworkMembershipPageContentProps {
  checkoutBasePath?: string;
  controller?: SdkworkMembershipController;
  onNavigate?: (route: string) => void;
  purchaseFlow?: "checkout" | "direct";
}

function resolveMembershipPurchaseFlow(
  purchaseFlow: SdkworkMembershipPageProps["purchaseFlow"],
  onNavigate?: (route: string) => void,
): "checkout" | "direct" {
  if (purchaseFlow === "direct") {
    return "direct";
  }

  if (purchaseFlow === "checkout") {
    return "checkout";
  }

  return onNavigate ? "checkout" : "direct";
}

const MEMBERSHIP_SECTION_IDS = {
  benefits: "membership-section-benefits",
  levels: "membership-section-levels",
  plans: "membership-section-plans",
} as const;

type SdkworkMembershipSectionView = "benefits" | "levels" | "plans";

function scrollToMembershipSection(view: SdkworkMembershipSectionView): void {
  const element = document.getElementById(MEMBERSHIP_SECTION_IDS[view]);
  if (element) {
    element.scrollIntoView({ behavior: "smooth", block: "start" });
  }
}

function resolveSavingsPercent(price: number, original: number | null): number | null {
  if (original === null || original <= price || original <= 0) {
    return null;
  }
  return Math.round((1 - price / original) * 100);
}

function SdkworkMembershipPageContent({
  checkoutBasePath,
  controller: controllerProp,
  onNavigate,
  purchaseFlow,
}: SdkworkMembershipPageContentProps) {
  const controller = useSdkworkMembershipController(controllerProp);
  const state = useSdkworkMembershipControllerState(controller);
  const {
    copy,
    formatDuration,
    formatIncludedPoints,
    formatPriceWas,
    formatSave,
    locale,
  } = useSdkworkMembershipIntl();
  const selectedPlan = state.dashboard.plans.find((plan) => plan.packageId === state.selectedPlanId) ?? null;
  const resolvedPurchaseFlow = resolveMembershipPurchaseFlow(purchaseFlow, onNavigate);

  useEffect(() => {
    if (!state.isBootstrapped && !state.isLoading && !state.lastError) {
      void controller.bootstrap().catch(() => undefined);
    }
  }, [controller, state.isBootstrapped, state.isLoading, state.lastError]);

  function navigateToMembershipCheckout(mode: SdkworkMembershipPurchaseMode) {
    if (!selectedPlan || !onNavigate || resolvedPurchaseFlow !== "checkout") {
      return false;
    }

    onNavigate(
      createMembershipCheckoutRouteIntent({
        basePath: checkoutBasePath,
        mode,
        plan: selectedPlan,
      }).route,
    );
    return true;
  }

  const sectionTabs: ReadonlyArray<{ view: SdkworkMembershipSectionView; label: string }> = [
    { view: "plans", label: copy.actions.plans },
    { view: "benefits", label: copy.actions.benefits },
    { view: "levels", label: copy.actions.levels },
  ];

  return (
    <div className="relative h-full overflow-y-auto">
      <div
        className="pointer-events-none absolute inset-x-0 top-0 h-72"
        style={createSdkworkMembershipBackdropStyle()}
      />

      <div className="relative px-4 py-5 sm:px-6 sm:py-6">
        <div className="mx-auto max-w-[80rem] space-y-5">
          <SdkworkMembershipMembershipHero
            isMutating={state.isMutating}
            levels={state.dashboard.levels}
            onPurchase={() => {
              if (navigateToMembershipCheckout("purchase")) {
                return;
              }

              void controller.purchaseSelectedPlan();
            }}
            onRenew={() => {
              if (navigateToMembershipCheckout("renew")) {
                return;
              }

              void controller.renewSelectedPlan();
            }}
            onUpgrade={() => {
              if (navigateToMembershipCheckout("upgrade")) {
                return;
              }

              void controller.upgradeSelectedPlan();
            }}
            selectedPlan={selectedPlan}
            summary={state.dashboard.summary}
          />

          {state.isLoading && !state.isBootstrapped ? <LoadingBlock label={copy.page.loading} /> : null}

          {state.lastError ? (
            <StatusNotice title={copy.page.errorTitle} tone="danger">
              {state.lastError}
            </StatusNotice>
          ) : null}

          <nav className="sticky top-0 z-10 flex flex-wrap items-center gap-1.5 rounded-[1.25rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3 py-2 shadow-[var(--sdk-shadow-sm)] backdrop-blur-md">
            {sectionTabs.map((tab) => (
              <button
                key={tab.view}
                onClick={() => {
                  controller.setView(tab.view);
                  scrollToMembershipSection(tab.view);
                }}
                type="button"
                className={`rounded-[0.75rem] px-4 py-1.5 text-sm font-medium transition-colors ${
                  state.activeView === tab.view
                    ? "bg-[var(--sdk-color-brand-primary)] text-white"
                    : "text-[var(--sdk-color-text-secondary)] hover:bg-[var(--sdk-color-surface-panel-muted)] hover:text-[var(--sdk-color-text-primary)]"
                }`}
              >
                {tab.label}
              </button>
            ))}
            <div className="ml-auto">
              <Button onClick={() => void controller.refresh()} type="button" variant="ghost" size="sm">
                {copy.actions.refresh}
              </Button>
            </div>
          </nav>

          <section
            id={MEMBERSHIP_SECTION_IDS.plans}
            className="scroll-mt-24 rounded-[1.5rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]"
          >
            <div className="border-b border-[var(--sdk-color-border-subtle)] px-6 py-5">
              <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">{copy.plans.eyebrow}</div>
              <h2 className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{copy.plans.title}</h2>
              <p className="mt-1.5 text-sm text-[var(--sdk-color-text-secondary)]">{copy.plans.subtitle}</p>
            </div>

            <div className="grid gap-4 px-6 py-6 lg:grid-cols-3">
              {state.dashboard.plans.length === 0 ? (
                <div className="col-span-full rounded-[1.25rem] border border-dashed border-[var(--sdk-color-border-default)] px-5 py-12 text-center">
                  <div className="text-base font-semibold text-[var(--sdk-color-text-primary)]">{copy.plans.emptyTitle}</div>
                  <div className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">{copy.plans.emptyDescription}</div>
                </div>
              ) : state.dashboard.plans.map((plan) => {
                const isSelected = plan.packageId === state.selectedPlanId;
                const tone = isSelected ? "accent" : plan.recommended ? "brand" : "neutral";
                const originalPriceLabel = plan.originalPriceCny !== null && plan.originalPriceCny > plan.priceCny
                  ? formatPriceWas(formatSdkworkCurrencyCny(plan.originalPriceCny, locale))
                  : null;
                const isAnnual = plan.durationDays !== null && plan.durationDays >= 360;
                const savingsPercent = resolveSavingsPercent(plan.priceCny, plan.originalPriceCny);

                return (
                  <article
                    className={`relative flex flex-col rounded-[1.5rem] border bg-[var(--sdk-color-surface-panel-muted)] p-6 ${isSelected ? "ring-2 ring-[var(--sdk-color-brand-accent)]" : ""}`}
                    key={plan.id}
                    style={createSdkworkMembershipPanelStyle(tone, {
                      backgroundWeight: isSelected ? 12 : 8,
                      borderWeight: isSelected ? 24 : 18,
                      surfaceColor: "var(--sdk-color-surface-panel-muted)",
                    })}
                  >
                    {plan.recommended ? (
                      <div
                        className="absolute -top-3 left-1/2 -translate-x-1/2 rounded-full border px-4 py-1 text-[0.7rem] font-semibold uppercase tracking-[0.16em] whitespace-nowrap"
                        style={createSdkworkMembershipToneStyle("accent", {
                          backgroundWeight: 24,
                          borderWeight: 36,
                        })}
                      >
                        {copy.plans.popular}
                      </div>
                    ) : null}

                    <div className="text-lg font-semibold text-[var(--sdk-color-text-primary)]">{plan.name}</div>
                    <div className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">
                      {plan.description || copy.plans.descriptionFallback}
                    </div>

                    <div className="mt-5 flex items-baseline gap-1.5">
                      <span className="text-4xl font-semibold tabular-nums text-[var(--sdk-color-text-primary)]">
                        {formatSdkworkCurrencyCny(plan.priceCny, locale)}
                      </span>
                      {isAnnual ? (
                        <span className="text-sm text-[var(--sdk-color-text-muted)]">{copy.common.perYear}</span>
                      ) : null}
                    </div>
                    <div className="mt-1.5 flex items-center gap-2 text-xs">
                      {originalPriceLabel ? (
                        <span className="text-[var(--sdk-color-text-muted)] line-through">
                          {originalPriceLabel}
                        </span>
                      ) : null}
                      {savingsPercent !== null ? (
                        <span
                          className="rounded-full border px-2 py-0.5 font-semibold"
                          style={createSdkworkMembershipToneStyle("success", {
                            backgroundWeight: 10,
                            borderWeight: 18,
                          })}
                        >
                          {formatSave(savingsPercent)}
                        </span>
                      ) : null}
                      {isAnnual ? (
                        <span className="text-[var(--sdk-color-text-muted)]">{copy.common.billedYearly}</span>
                      ) : null}
                    </div>

                    <div className="mt-5 space-y-2.5 rounded-[1rem] border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)] p-4 text-sm">
                      <div className="flex items-center justify-between gap-4">
                        <span className="text-[var(--sdk-color-text-muted)]">{copy.plans.duration}</span>
                        <span className="font-medium tabular-nums text-[var(--sdk-color-text-primary)]">{formatDuration(plan.durationDays)}</span>
                      </div>
                      <div className="flex items-center justify-between gap-4">
                        <span className="text-[var(--sdk-color-text-muted)]">{copy.plans.pointsIncluded}</span>
                        <span className="font-medium tabular-nums text-[var(--sdk-color-text-primary)]">{formatIncludedPoints(plan.includedPoints)}</span>
                      </div>
                    </div>

                    {plan.tags.length > 0 ? (
                      <div className="mt-4 flex flex-wrap gap-2">
                        {plan.tags.map((tag) => (
                          <span
                            className="rounded-full border px-3 py-1 text-xs font-medium text-[var(--sdk-color-text-secondary)]"
                            key={tag}
                            style={createSdkworkMembershipToneStyle(tone, {
                              backgroundWeight: 8,
                              borderWeight: 14,
                            })}
                          >
                            {tag}
                          </span>
                        ))}
                      </div>
                    ) : null}

                    <Button
                      className="mt-6 w-full"
                      onClick={() => controller.selectPlan(plan.packageId)}
                      type="button"
                      variant={isSelected ? "secondary" : plan.recommended ? "primary" : "outline"}
                    >
                      {isSelected ? copy.actions.selected : copy.actions.selectPlan}
                    </Button>
                  </article>
                );
              })}
            </div>
          </section>

          <div id={MEMBERSHIP_SECTION_IDS.benefits} className="scroll-mt-24">
            <SdkworkMembershipBenefitsGrid benefits={state.dashboard.benefits} />
          </div>

          <div id={MEMBERSHIP_SECTION_IDS.levels} className="scroll-mt-24">
            <SdkworkMembershipLevelComparison levels={state.dashboard.levels} />
          </div>
        </div>
      </div>
    </div>
  );
}

export function SdkworkMembershipPage({
  locale,
  messages,
  onNavigate,
  purchaseFlow,
  ...props
}: SdkworkMembershipPageProps) {
  const resolvedPurchaseFlow = purchaseFlow ?? (onNavigate ? "checkout" : "direct");
  const content = (
    <SdkworkMembershipPageContent
      {...props}
      onNavigate={onNavigate}
      purchaseFlow={resolvedPurchaseFlow}
    />
  );

  if (locale || messages) {
    return (
      <SdkworkMembershipIntlProvider locale={locale} messages={messages}>
        {content}
      </SdkworkMembershipIntlProvider>
    );
  }

  return content;
}
