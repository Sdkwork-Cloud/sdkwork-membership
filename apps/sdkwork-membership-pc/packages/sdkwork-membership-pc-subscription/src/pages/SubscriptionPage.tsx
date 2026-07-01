import { useEffect } from "react";
import {
  LoadingBlock,
  StatusNotice,
} from "@sdkwork/ui-pc-react";
import type { SdkworkSubscriptionMessagesOverrides } from "../subscription-copy";
import type { SdkworkSubscriptionController } from "../subscription-controller";
import {
  useSdkworkSubscriptionController,
  useSdkworkSubscriptionControllerState,
} from "../subscription-controller";
import { createSdkworkSubscriptionBackdropStyle } from "../subscription-appearance";
import {
  SdkworkSubscriptionIntlProvider,
  useSdkworkSubscriptionIntl,
} from "../subscription-intl";
import type {
  SdkworkSubscriptionPurchaseResult,
  SdkworkSubscriptionService,
} from "../subscription-service";
import { SdkworkSubscriptionCheckoutPanel } from "../components/subscription-checkout-panel";
import { SdkworkSubscriptionHero } from "../components/subscription-hero";
import { SdkworkSubscriptionPlanGrid } from "../components/subscription-plan-grid";
import { SdkworkSubscriptionCompareTable } from "../components/subscription-compare-table";
import { SdkworkSubscriptionStageShell } from "../components/subscription-stage-shell";

export interface SdkworkSubscriptionPageProps {
  controller?: SdkworkSubscriptionController;
  locale?: string | null;
  messages?: SdkworkSubscriptionMessagesOverrides;
  onCheckoutComplete?: (result: SdkworkSubscriptionPurchaseResult) => void;
  onCheckoutError?: (error: unknown) => void;
  service?: Partial<SdkworkSubscriptionService>;
}

interface SdkworkSubscriptionPageContentProps {
  controller?: SdkworkSubscriptionController;
  locale?: string | null;
  messages?: SdkworkSubscriptionMessagesOverrides;
  onCheckoutComplete?: (result: SdkworkSubscriptionPurchaseResult) => void;
  onCheckoutError?: (error: unknown) => void;
  service?: Partial<SdkworkSubscriptionService>;
}

function SdkworkSubscriptionPageContent({
  controller: controllerProp,
  locale,
  messages,
  onCheckoutComplete,
  onCheckoutError,
  service,
}: SdkworkSubscriptionPageContentProps) {
  const controller = useSdkworkSubscriptionController(controllerProp, {
    locale,
    messages,
    service,
  });
  const state = useSdkworkSubscriptionControllerState(controller);
  const { copy } = useSdkworkSubscriptionIntl();
  const selectedPlan = state.dashboard.plans.find((plan) => plan.packageId === state.selectedPackageId) ?? null;

  useEffect(() => {
    if (!state.isBootstrapped && !state.isLoading && !state.lastError) {
      void controller.bootstrap().catch(() => undefined);
    }
  }, [controller, state.isBootstrapped, state.isLoading, state.lastError]);

  function handleSubmit(): void {
    void controller.submitCheckout()
      .then((result) => {
        onCheckoutComplete?.(result);
      })
      .catch((error) => {
        onCheckoutError?.(error);
      });
  }

  const planCount = state.dashboard.plans.length;
  const couponCount = state.dashboard.coupons.length;

  const planContent = (
    <SdkworkSubscriptionPlanGrid
      hideHeader
      onSelectPackageGroup={(packageGroupId) => controller.selectPackageGroup(packageGroupId)}
      onSelectPlan={(packageId) => controller.selectPackage(packageId)}
      packageGroups={state.dashboard.packageGroups}
      plans={state.dashboard.plans}
      selectedPackageGroupId={state.selectedPackageGroupId}
      selectedPackageId={state.selectedPackageId}
      summary={state.dashboard.summary}
    />
  );

  const paymentContent = (
    <SdkworkSubscriptionCheckoutPanel
      activeAction={state.activeAction}
      checkout={state.checkout}
      coupons={state.dashboard.coupons}
      isAuthenticated={state.dashboard.summary.isAuthenticated}
      isMutating={state.isMutating}
      lastError={state.isMutating ? undefined : state.lastError}
      onBackToPlans={() => controller.setStage("plans")}
      onClearCoupon={() => controller.clearCoupon()}
      onSelectCoupon={(couponId) => controller.selectCoupon(couponId)}
      onSelectPaymentMethod={(paymentMethodId) => controller.selectPaymentMethod(paymentMethodId)}
      onSubmit={handleSubmit}
      paymentMethods={state.dashboard.paymentMethods}
      selectedCouponId={state.selectedCouponId}
      selectedPlan={selectedPlan}
    />
  );

  return (
    <div className="relative min-h-full bg-gradient-to-b from-[var(--sdk-color-surface-base)] to-[var(--sdk-color-surface-panel)]">
      <div
        className="pointer-events-none absolute inset-x-0 top-0 h-[500px] opacity-50"
        style={createSdkworkSubscriptionBackdropStyle()}
      />

      <div className="relative px-4 py-6 sm:px-6 sm:py-8 lg:px-8">
        <div className="mx-auto max-w-[1280px] space-y-8">
          <SdkworkSubscriptionHero
            activeAction={state.activeAction}
            onActionChange={(action) => controller.setAction(action)}
            summary={state.dashboard.summary}
          />

          <h2 className="sr-only">{copy.page.title}</h2>

          {state.isLoading && !state.isBootstrapped ? <LoadingBlock label={copy.page.loading} /> : null}

          {state.lastError && !state.isMutating ? (
            <StatusNotice title={copy.page.errorTitle} tone="danger">
              {state.lastError}
            </StatusNotice>
          ) : null}

          {state.isBootstrapped ? (
            <SdkworkSubscriptionStageShell
              activeAction={state.activeAction}
              activeStage={state.activeStage}
              checkout={state.checkout}
              couponCount={couponCount}
              isAuthenticated={state.dashboard.summary.isAuthenticated}
              onBackToPlans={() => controller.setStage("plans")}
              onContinueToCheckout={() => controller.setStage("checkout")}
              paymentContent={paymentContent}
              planContent={planContent}
              planCount={planCount}
              selectedPlan={selectedPlan}
              summary={state.dashboard.summary}
            />
          ) : null}

          <SdkworkSubscriptionCompareTable benefits={state.dashboard.benefits} />

          <div className="pb-12 text-center">
            <p className="text-sm text-[var(--sdk-color-text-muted)]">
              {copy.page.agreementText}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export function SdkworkSubscriptionPage({
  locale,
  messages,
  ...props
}: SdkworkSubscriptionPageProps) {
  const content = (
    <SdkworkSubscriptionPageContent
      {...props}
      locale={locale}
      messages={messages}
    />
  );

  if (locale || messages) {
    return (
      <SdkworkSubscriptionIntlProvider locale={locale} messages={messages}>
        {content}
      </SdkworkSubscriptionIntlProvider>
    );
  }

  return content;
}
