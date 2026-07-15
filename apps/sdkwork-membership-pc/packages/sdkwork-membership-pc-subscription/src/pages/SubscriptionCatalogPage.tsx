import { useCallback, useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { LoadingBlock, StatusNotice } from "@sdkwork/ui-pc-react";
import { hasSdkworkMembershipSession } from "@sdkwork/membership-service";
import { SubscriptionCatalogHero } from "../components/subscription-catalog-hero";
import { SubscriptionCatalogPlanGrid } from "../components/subscription-catalog-plan-grid";
import { SubscriptionCatalogTierCompare } from "../components/subscription-catalog-tier-compare";
import { sdkworkSubscriptionCatalogHostComponents } from "../components/subscription-catalog-host-components";
import type { SdkworkSubscriptionCatalogPlanCardModel } from "../subscription-catalog-content";
import {
  useSdkworkSubscriptionCatalogController,
  useSdkworkSubscriptionCatalogControllerState,
  type SdkworkSubscriptionCatalogController,
} from "../subscription-catalog-controller";
import {
  SDKWORK_SUBSCRIPTION_CATALOG_UNAVAILABLE_TIER_KEY,
  type SdkworkSubscriptionCatalogCheckoutPayment,
  type SdkworkSubscriptionCatalogCheckoutPlan,
  type SdkworkSubscriptionCatalogPageProps as SdkworkSubscriptionCatalogHostPageProps,
} from "../subscription-catalog-host";
import type { SdkworkSubscriptionPurchaseResult } from "../subscription-service";

export interface SdkworkSubscriptionCatalogPageProps extends SdkworkSubscriptionCatalogHostPageProps {
  catalogController?: SdkworkSubscriptionCatalogController;
}

/** A no-op component used when no `notifyOutlet` prop is supplied. */
function EmptyNotifyOutlet() {
  return null;
}

function redirectToDefaultLogin(): void {
  if (typeof window === "undefined") {
    return;
  }
  const returnPath = `${window.location.pathname}${window.location.search}${window.location.hash}`;
  window.location.assign(`/auth/login?redirect=${encodeURIComponent(returnPath || "/")}`);
}

export function SdkworkSubscriptionCatalogPage({
  catalogController: catalogControllerProp,
  components,
  memberSummary: memberSummaryProp,
  notifyOutlet: NotifyOutletProp,
  onLoginRequired,
  onMembershipTierUpdated,
  onNotify,
}: SdkworkSubscriptionCatalogPageProps) {
  const { t } = useTranslation();

  // Use provided host components or fall back to built-in defaults so the
  // page works with zero configuration: <SdkworkSubscriptionCatalogPage />
  const hostComponents = components ?? sdkworkSubscriptionCatalogHostComponents;
  const NotifyOutlet = NotifyOutletProp ?? EmptyNotifyOutlet;

  // Stable no-op-safe callback wrappers for when the caller doesn't supply them.
  const handleMembershipTierUpdated = useCallback(
    (membershipTierKey: string, durationDays: number) => {
      onMembershipTierUpdated?.(membershipTierKey, durationDays);
    },
    [onMembershipTierUpdated],
  );
  const handleNotify = useCallback(
    (message: string, tone: "error" | "info" | "success") => {
      if (onNotify) {
        onNotify(message, tone);
      } else if (tone === "error") {
        // eslint-disable-next-line no-console
        console.error(`[subscription-catalog] ${message}`);
      }
    },
    [onNotify],
  );

  const {
    checkoutModal: CheckoutModal,
    pointsDetailsModal: PointsDetailsModal,
    pointsPurchaseModal: PointsPurchaseModal,
    redeemModal: RedeemModal,
  } = hostComponents;

  const controller = useSdkworkSubscriptionCatalogController(catalogControllerProp, {
    translate: (key, defaultValue) => String(t(key, defaultValue ?? key)),
  });
  const state = useSdkworkSubscriptionCatalogControllerState(controller);

  const [isPointsModalOpen, setIsPointsModalOpen] = useState(false);
  const [isPointsDetailsModalOpen, setIsPointsDetailsModalOpen] = useState(false);
  const [isRedeemModalOpen, setIsRedeemModalOpen] = useState(false);
  const [isCheckoutModalOpen, setIsCheckoutModalOpen] = useState(false);
  const pendingPurchaseRef = useRef<SdkworkSubscriptionPurchaseResult | null>(null);
  const completedPaymentKeyRef = useRef<string | null>(null);

  const memberSummary = memberSummaryProp ?? state.memberSummary;
  const handleLoginRequired = onLoginRequired ?? redirectToDefaultLogin;

  useEffect(() => {
    if (!state.isBootstrapped && !state.isLoading) {
      void controller.bootstrap().catch(() => undefined);
    }
  }, [controller, state.isBootstrapped, state.isLoading]);

  function openCheckoutForPlan(
    packageId: string,
    membershipTierKey: string,
    packageName: string,
    priceLabel: string,
    packageNumericId: number,
    originalPriceLabel?: string,
    packagePeriodLabel = "年",
  ) {
    if (!hasSdkworkMembershipSession()) {
      handleLoginRequired();
      return;
    }

    if (
      membershipTierKey === SDKWORK_SUBSCRIPTION_CATALOG_UNAVAILABLE_TIER_KEY
      || memberSummary?.membershipTierKey === membershipTierKey
    ) {
      return;
    }

    const checkoutPlan: SdkworkSubscriptionCatalogCheckoutPlan = {
      id: packageId,
      membershipTierKey,
      name: packageName,
      originalPrice: originalPriceLabel,
      packageNumericId,
      packagePeriodLabel,
      priceLabel,
    };
    controller.selectCheckoutPlan(checkoutPlan);
    pendingPurchaseRef.current = null;
    completedPaymentKeyRef.current = null;
    setIsCheckoutModalOpen(true);
  }

  function handlePlanCardSelect(plan: SdkworkSubscriptionCatalogPlanCardModel) {
    const packageNumericId = Number(plan.id);
    if (!Number.isFinite(packageNumericId) || packageNumericId <= 0) {
      return;
    }

    openCheckoutForPlan(
      plan.id,
      plan.membershipTierKey,
      plan.name,
      plan.priceLabel,
      packageNumericId,
      plan.originalPriceLabel || undefined,
      plan.packagePeriodLabel,
    );
  }

  async function handleCheckoutPurchase() {
    const selectedCheckoutPlan = state.selectedCheckoutPlan;
    if (!selectedCheckoutPlan) {
      throw new Error("No subscription plan has been selected.");
    }

    try {
      const result = await controller.purchaseSelectedPlan();
      pendingPurchaseRef.current = result;
      return result;
    } catch (error) {
      handleNotify(
        error instanceof Error ? error.message : t("subscription_failed", "订阅失败，请稍后重试。"),
        "error",
      );
      throw error;
    }
  }

  async function handleCheckoutPaymentStatus(orderId: string) {
    return controller.getPurchaseStatus(orderId);
  }

  async function handleCheckoutPaymentCompleted(
    payment: SdkworkSubscriptionCatalogCheckoutPayment,
  ) {
    if (payment.status !== "completed") {
      return;
    }

    const selectedCheckoutPlan = state.selectedCheckoutPlan;
    const paymentKey = payment.orderId ?? selectedCheckoutPlan?.id;
    if (!selectedCheckoutPlan || !paymentKey || completedPaymentKeyRef.current === paymentKey) {
      return;
    }

    completedPaymentKeyRef.current = paymentKey;
    handleMembershipTierUpdated(
      selectedCheckoutPlan.membershipTierKey,
      pendingPurchaseRef.current?.durationDays ?? 365,
    );
    handleNotify(t("subscription_success", "订阅成功！"), "success");
    await controller.refresh();
  }

  function closeCheckoutModal() {
    pendingPurchaseRef.current = null;
    completedPaymentKeyRef.current = null;
    setIsCheckoutModalOpen(false);
    controller.clearCheckoutPlan();
  }

  return (
    <div className="w-full pb-20 font-sans">
      <NotifyOutlet />

      <CheckoutModal
        isOpen={isCheckoutModalOpen}
        onClose={closeCheckoutModal}
        onPaymentCompleted={handleCheckoutPaymentCompleted}
        onPaymentStatus={handleCheckoutPaymentStatus}
        onPurchase={handleCheckoutPurchase}
        plan={state.selectedCheckoutPlan}
      />

      {state.isLoading && !state.isBootstrapped ? (
        <div className="px-6 py-12">
          <LoadingBlock label={t("loading_catalog", "正在加载套餐信息...")} />
        </div>
      ) : null}

      {state.lastError && state.catalog === null && !state.isLoading ? (
        <div className="px-6 py-12">
          <StatusNotice title={t("load_failed", "加载失败")} tone="danger">
            {state.lastError}
          </StatusNotice>
          <div className="mt-4 text-center">
            <button
              className="rounded-lg bg-zinc-900 px-6 py-2 text-sm font-medium text-white hover:bg-zinc-800 dark:bg-white dark:text-zinc-900 dark:hover:bg-zinc-100"
              onClick={() => {
                void controller.retry().catch(() => undefined);
              }}
              type="button"
            >
              {t("retry", "重试")}
            </button>
          </div>
        </div>
      ) : null}

      {state.isBootstrapped && state.catalog !== null ? (
        <>
          <SubscriptionCatalogHero
            billingCycleIndex={state.billingCycleIndex}
            billingCycles={state.billingCycles}
            onOpenPointsDetails={() => setIsPointsDetailsModalOpen(true)}
            onOpenPointsPurchase={() => setIsPointsModalOpen(true)}
            onOpenRedeem={() => setIsRedeemModalOpen(true)}
            onSelectBillingCycle={(index) => {
              controller.selectBillingCycle(index);
            }}
            subtitleLead={t("choose_suitable_plan", "选择合适你的套餐，或直接")}
            subtitlePointsActionLabel={t("buy_points", "购买算力元")}
            subtitleRedeemActionLabel={t("redeem_vip", "会员兑换")}
            title={t("unlock_infinite", "订阅特权，解锁无尽竞技能力")}
          />

          <SubscriptionCatalogPlanGrid onSelectPlan={handlePlanCardSelect} plans={state.planCards} />

          <SubscriptionCatalogTierCompare
            billingCycleIndex={state.billingCycleIndex}
            billingCycles={state.billingCycles}
            comingSoonLabel={t("coming_soon", "敬请期待")}
            comparisonCategories={state.comparisonCategories}
            currentPlanLabel={t("current_plan", "当前计划")}
            firstYear58Label={t("first_year_58", "首年5.8折")}
            firstYear60Label={t("first_year_60", "首年6折")}
            onSelectBillingCycle={(index) => {
              controller.selectBillingCycle(index);
            }}
            onSelectPackage={(packageId, membershipTierKey, packageName, priceLabel, originalPriceLabel, packagePeriodLabel) => {
              const tierColumn = state.tierColumns.find((column) => column.packageId === packageId);
              if (!tierColumn) {
                return;
              }
              openCheckoutForPlan(
                packageId,
                membershipTierKey,
                packageName,
                priceLabel,
                tierColumn.packageNumericId,
                originalPriceLabel,
                packagePeriodLabel,
              );
            }}
            perMonthShortLabel={t("per_month_short", "每月")}
            perYearShortLabel={t("per_year_short", "每年")}
            premiumPlanLabel={t("premium_plan", "高级会员")}
            sectionTitle={t("which_plan_suits_you", "哪个计划更适合你")}
            standardPlanLabel={t("standard_plan", "标准会员")}
            superPlanLabel={t("super_plan", "超级会员")}
            tierColumns={state.tierColumns}
            basicPlanLabel={t("basic_plan", "基础会员")}
          />

          <PointsPurchaseModal
            isOpen={isPointsModalOpen}
            onClose={() => setIsPointsModalOpen(false)}
          />

          <PointsDetailsModal
            isOpen={isPointsDetailsModalOpen}
            onClose={() => setIsPointsDetailsModalOpen(false)}
          />

          <RedeemModal
            isOpen={isRedeemModalOpen}
            onClose={() => setIsRedeemModalOpen(false)}
          />
        </>
      ) : null}
    </div>
  );
}
