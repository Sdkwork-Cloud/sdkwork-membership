import { useEffect, useState } from "react";
import { Crown } from "lucide-react";
import { Button, StatusNotice } from "@sdkwork/ui-pc-react";
import { formatSdkworkMembershipCurrencyCny } from "@sdkwork/membership-service";
import {
  createMembershipCheckoutRouteIntent,
  resolveSdkworkMembershipPurchaseMode,
} from "../membership.ts";
import {
  createSdkworkMembershipPanelStyle,
  createSdkworkMembershipToneStyle,
} from "../membership-appearance.ts";
import type { SdkworkMembershipController } from "../membership-controller.ts";
import {
  useSdkworkMembershipController,
  useSdkworkMembershipControllerState,
} from "../membership-controller.ts";
import { useSdkworkMembershipIntl } from "../membership-intl.tsx";

export interface SdkworkMembershipHeaderMenuProps {
  checkoutBasePath?: string;
  controller?: SdkworkMembershipController;
  onNavigate?: (route: string) => void;
  onOpenCenter?: () => void;
}

export function SdkworkMembershipHeaderMenu({
  checkoutBasePath,
  controller: controllerProp,
  onNavigate,
  onOpenCenter,
}: SdkworkMembershipHeaderMenuProps) {
  const controller = useSdkworkMembershipController(controllerProp);
  const state = useSdkworkMembershipControllerState(controller);
  const [selectedPackageId, setSelectedPackageId] = useState<number | null>(
    controller.getState().selectedPlanId,
  );
  const {
    copy,
    formatDuration,
    formatIncludedPoints,
    locale,
  } = useSdkworkMembershipIntl();
  const selectedPlan = state.dashboard.plans.find((plan) => plan.packageId === selectedPackageId)
    ?? state.dashboard.plans.find((plan) => plan.packageId === state.selectedPlanId)
    ?? state.dashboard.plans[0]
    ?? null;
  const purchaseMode = resolveSdkworkMembershipPurchaseMode({
    plan: selectedPlan,
    summary: state.dashboard.summary,
  });
  const canContinue = Boolean(selectedPlan)
    && state.dashboard.summary.isAuthenticated
    && Boolean(onNavigate);

  useEffect(() => {
    if (!state.isBootstrapped && !state.isLoading && !state.lastError) {
      void controller.bootstrap().catch(() => undefined);
    }
  }, [controller, state.isBootstrapped, state.isLoading, state.lastError]);

  useEffect(() => {
    if (selectedPackageId && state.dashboard.plans.some((plan) => plan.packageId === selectedPackageId)) {
      return;
    }

    setSelectedPackageId(
      state.dashboard.plans.find((plan) => plan.recommended)?.packageId
      ?? state.dashboard.plans[0]?.packageId
      ?? null,
    );
  }, [selectedPackageId, state.dashboard.plans]);

  function continueToCheckout(): void {
    if (!selectedPlan || !onNavigate) {
      return;
    }

    controller.selectPlan(selectedPlan.packageId);
    onNavigate(
      createMembershipCheckoutRouteIntent({
        basePath: checkoutBasePath,
        mode: purchaseMode,
        plan: selectedPlan,
      }).route,
    );
  }

  return (
    <div
      className="w-[min(92vw,30rem)] rounded-[1.5rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4 shadow-[var(--sdk-shadow-lg)]"
      style={createSdkworkMembershipPanelStyle("brand", {
        backgroundWeight: 4,
        borderWeight: 18,
      })}
    >
      <div className="flex items-start justify-between gap-4">
        <div>
          <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
            {copy.headerEntry.title}
          </div>
          <div className="mt-1 text-lg font-semibold text-[var(--sdk-color-text-primary)]">
            {copy.menu.title}
          </div>
        </div>
        <div
          className="flex h-10 w-10 items-center justify-center rounded-[1rem] border"
          style={createSdkworkMembershipToneStyle("accent", {
            backgroundWeight: 12,
            borderWeight: 24,
          })}
        >
          <Crown className="h-5 w-5" />
        </div>
      </div>

      {!state.dashboard.summary.isAuthenticated ? (
        <StatusNotice className="mt-4" title={copy.menu.signInRequiredTitle} tone="warning">
          {copy.menu.signInRequiredDescription}
        </StatusNotice>
      ) : null}

      <div className="mt-4 grid gap-3">
        {state.dashboard.plans.length === 0 ? (
          <div className="rounded-[1.25rem] border border-dashed border-[var(--sdk-color-border-default)] px-4 py-6 text-sm text-[var(--sdk-color-text-secondary)]">
            <div className="font-semibold text-[var(--sdk-color-text-primary)]">{copy.menu.emptyTitle}</div>
            <div className="mt-2">{copy.menu.emptyDescription}</div>
          </div>
        ) : state.dashboard.plans.map((plan) => {
          const isSelected = selectedPlan?.packageId === plan.packageId;

          return (
            <button
              aria-pressed={isSelected}
              className="rounded-[1.25rem] border px-4 py-4 text-left"
              key={plan.id}
              onClick={() => {
                setSelectedPackageId(plan.packageId);
                controller.selectPlan(plan.packageId);
              }}
              style={createSdkworkMembershipPanelStyle(isSelected ? "accent" : "neutral", {
                backgroundWeight: isSelected ? 10 : 4,
                borderWeight: isSelected ? 28 : 14,
                surfaceColor: "var(--sdk-color-surface-panel-muted)",
              })}
              type="button"
            >
              <div className="flex items-start justify-between gap-3">
                <div>
                  <div className="font-semibold text-[var(--sdk-color-text-primary)]">{plan.name}</div>
                  <div className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">
                    {formatDuration(plan.durationDays)}
                    {" · "}
                    {formatIncludedPoints(plan.includedPoints)}
                  </div>
                </div>
                <div className="text-right">
                  <div className="font-semibold text-[var(--sdk-color-text-primary)]">
                    {formatSdkworkMembershipCurrencyCny(plan.priceCny, locale)}
                  </div>
                  <div className="mt-1 text-xs text-[var(--sdk-color-text-muted)]">
                    {isSelected ? copy.actions.selected : copy.actions.selectPlan}
                  </div>
                </div>
              </div>
            </button>
          );
        })}
      </div>

      <div className="mt-4 flex flex-wrap justify-end gap-2">
        {onOpenCenter ? (
          <Button onClick={onOpenCenter} type="button" variant="ghost">
            {copy.menu.openCenter}
          </Button>
        ) : null}
        <Button disabled={!canContinue} onClick={continueToCheckout} type="button">
          {purchaseMode === "purchase"
            ? copy.menu.continueCheckout
            : purchaseMode === "renew"
              ? copy.actions.renew
              : copy.actions.upgrade}
        </Button>
      </div>
    </div>
  );
}
