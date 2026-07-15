import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@sdkwork/ui-pc-react";
import {
  SdkworkOrderCheckoutDialog,
  type SdkworkOrderCheckoutDialogCopy,
  type SdkworkOrderCheckoutPayment,
} from "@sdkwork/order-pc-checkout";
import type {
  SdkworkSubscriptionCatalogCheckoutModalProps,
  SdkworkSubscriptionCatalogModalProps,
} from "../subscription-catalog-host";
import { SDKWORK_SUBSCRIPTION_I18N_KEYS } from "../i18n";

function createCheckoutCopy(
  translate: (key: string) => string,
): SdkworkOrderCheckoutDialogCopy {
  const keys = SDKWORK_SUBSCRIPTION_I18N_KEYS.checkout;
  return {
    activationDescription: translate(keys.activationDescription),
    activationTitle: translate(keys.activationTitle),
    close: translate(keys.close),
    completed: translate(keys.completed),
    creatingPayment: translate(keys.creatingPayment),
    paymentUnavailable: translate(keys.paymentUnavailableTitle),
    paymentUnavailableDescription: translate(keys.paymentUnavailableDescription),
    payByQr: translate(keys.payByQr),
    price: translate(keys.price),
    retry: translate(keys.retry),
    scanPrompt: translate(keys.scanPrompt),
    secureDescription: translate(keys.secureDescription),
    secureTitle: translate(keys.secureTitle),
    selectedItem: translate(keys.selectedPlan),
  };
}

export function SubscriptionCatalogCheckoutModal({
  isOpen,
  onClose,
  onPaymentCompleted,
  onPaymentStatus,
  onPurchase,
  plan,
}: SdkworkSubscriptionCatalogCheckoutModalProps) {
  const { t } = useTranslation();
  const copy = useMemo(
    () => createCheckoutCopy((key) => t(key)),
    [t],
  );
  const driver = useMemo(() => ({
    createPayment: onPurchase,
    getPaymentStatus: onPaymentStatus
      ? async (payment: SdkworkOrderCheckoutPayment) => {
          if (!payment.orderId) {
            return { ...payment, status: "failed" as const };
          }
          return onPaymentStatus(payment.orderId);
        }
      : undefined,
    onPaymentCompleted,
  }), [onPaymentCompleted, onPaymentStatus, onPurchase]);

  return (
    <SdkworkOrderCheckoutDialog
      copy={copy}
      driver={driver}
      isOpen={isOpen}
      onClose={onClose}
      summary={plan ? {
        id: plan.id,
        name: plan.name,
        originalPriceLabel: plan.originalPrice,
        periodLabel: plan.packagePeriodLabel,
        priceLabel: `CNY ${plan.priceLabel}`,
      } : null}
    />
  );
}

export function SubscriptionCatalogPlaceholderModal({
  isOpen,
  onClose,
  titleKey,
}: SdkworkSubscriptionCatalogModalProps & { titleKey: string }) {
  const { t } = useTranslation();
  if (!isOpen) {
    return null;
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 px-4">
      <div className="w-full max-w-sm rounded-lg border border-zinc-200 bg-white p-6 shadow-xl dark:border-zinc-800 dark:bg-zinc-900">
        <h3 className="text-lg font-bold text-zinc-900 dark:text-white">{t(titleKey)}</h3>
        <div className="mt-6 flex justify-end">
          <Button onClick={onClose} type="button" variant="secondary">
            {t(SDKWORK_SUBSCRIPTION_I18N_KEYS.dialogs.close)}
          </Button>
        </div>
      </div>
    </div>
  );
}

export const sdkworkSubscriptionCatalogHostComponents = {
  checkoutModal: SubscriptionCatalogCheckoutModal,
  pointsDetailsModal: (props: SdkworkSubscriptionCatalogModalProps) => (
    <SubscriptionCatalogPlaceholderModal
      {...props}
      titleKey={SDKWORK_SUBSCRIPTION_I18N_KEYS.dialogs.tokenDetailsTitle}
    />
  ),
  pointsPurchaseModal: (props: SdkworkSubscriptionCatalogModalProps) => (
    <SubscriptionCatalogPlaceholderModal
      {...props}
      titleKey={SDKWORK_SUBSCRIPTION_I18N_KEYS.dialogs.tokenPurchaseTitle}
    />
  ),
  redeemModal: (props: SdkworkSubscriptionCatalogModalProps) => (
    <SubscriptionCatalogPlaceholderModal
      {...props}
      titleKey={SDKWORK_SUBSCRIPTION_I18N_KEYS.dialogs.redemptionTitle}
    />
  ),
};
