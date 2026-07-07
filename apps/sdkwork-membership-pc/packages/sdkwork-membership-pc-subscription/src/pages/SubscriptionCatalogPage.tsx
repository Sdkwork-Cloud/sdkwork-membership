import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { SubscriptionCatalogHero } from "../components/subscription-catalog-hero";
import { SubscriptionCatalogPlanGrid } from "../components/subscription-catalog-plan-grid";
import { SubscriptionCatalogTierCompare } from "../components/subscription-catalog-tier-compare";
import type { SdkworkSubscriptionCatalogPlanCardModel } from "../subscription-catalog-content";
import {
  useSdkworkSubscriptionCatalogController,
  useSdkworkSubscriptionCatalogControllerState,
  type SdkworkSubscriptionCatalogController,
} from "../subscription-catalog-controller";
import {
  SDKWORK_SUBSCRIPTION_CATALOG_UNAVAILABLE_TIER_KEY,
  type SdkworkSubscriptionCatalogCheckoutPlan,
  type SdkworkSubscriptionCatalogPageProps as SdkworkSubscriptionCatalogHostPageProps,
} from "../subscription-catalog-host";

export interface SdkworkSubscriptionCatalogPageProps extends SdkworkSubscriptionCatalogHostPageProps {
  catalogController?: SdkworkSubscriptionCatalogController;
}

export function SdkworkSubscriptionCatalogPage({
  catalogController: catalogControllerProp,
  components,
  memberSummary: memberSummaryProp,
  notifyOutlet: NotifyOutlet,
  onMembershipTierUpdated,
  onNotify,
}: SdkworkSubscriptionCatalogPageProps) {
  const { t } = useTranslation();
  const {
    checkoutModal: CheckoutModal,
    pointsDetailsModal: PointsDetailsModal,
    pointsPurchaseModal: PointsPurchaseModal,
    redeemModal: RedeemModal,
  } = components;

  const controller = useSdkworkSubscriptionCatalogController(catalogControllerProp, {
    translate: (key, defaultValue) => String(t(key, defaultValue ?? key)),
  });
  const state = useSdkworkSubscriptionCatalogControllerState(controller);

  const [isPointsModalOpen, setIsPointsModalOpen] = useState(false);
  const [isPointsDetailsModalOpen, setIsPointsDetailsModalOpen] = useState(false);
  const [isRedeemModalOpen, setIsRedeemModalOpen] = useState(false);
  const [isCheckoutModalOpen, setIsCheckoutModalOpen] = useState(false);

  const memberSummary = memberSummaryProp ?? state.memberSummary;

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

  async function handleCheckoutSuccess() {
    const selectedCheckoutPlan = state.selectedCheckoutPlan;
    if (!selectedCheckoutPlan) {
      return;
    }

    try {
      const result = await controller.purchaseSelectedPlan();
      if (result.status === "failed") {
        onNotify(t("subscription_failed", "订阅失败，请稍后重试。"), "error");
        return;
      }

      onMembershipTierUpdated(
        selectedCheckoutPlan.membershipTierKey,
        result.durationDays ?? 365,
      );
      onNotify(t("subscription_success", "订阅成功！"), "success");
      setIsCheckoutModalOpen(false);
      await controller.refresh();
    } catch (error) {
      onNotify(
        error instanceof Error ? error.message : t("subscription_failed", "订阅失败，请稍后重试。"),
        "error",
      );
    }
  }

  return (
    <div className="w-full pb-20 font-sans">
      <NotifyOutlet />

      <CheckoutModal
        isOpen={isCheckoutModalOpen}
        onClose={() => {
          setIsCheckoutModalOpen(false);
          controller.clearCheckoutPlan();
        }}
        onSuccess={() => {
          void handleCheckoutSuccess();
        }}
        plan={state.selectedCheckoutPlan}
      />

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
        subtitlePointsActionLabel={t("buy_points", "购买积分")}
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
    </div>
  );
}
