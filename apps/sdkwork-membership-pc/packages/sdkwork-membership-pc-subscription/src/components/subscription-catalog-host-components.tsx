import { useTranslation } from "react-i18next";
import { Button } from "@sdkwork/ui-pc-react";

import type {
  SdkworkSubscriptionCatalogCheckoutModalProps,
  SdkworkSubscriptionCatalogModalProps,
} from "../subscription-catalog-host";
import { SDKWORK_SUBSCRIPTION_I18N_KEYS } from "../i18n";

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

export function SubscriptionCatalogCheckoutModal({
  isOpen,
  onClose,
}: SdkworkSubscriptionCatalogCheckoutModalProps) {
  return (
    <SubscriptionCatalogPlaceholderModal
      isOpen={isOpen}
      onClose={onClose}
      titleKey={SDKWORK_SUBSCRIPTION_I18N_KEYS.checkout.paymentUnavailableTitle}
    />
  );
}

export function SubscriptionCatalogPointsPurchaseModal({
  isOpen,
  onClose,
}: SdkworkSubscriptionCatalogModalProps) {
  return (
    <SubscriptionCatalogPlaceholderModal
      isOpen={isOpen}
      onClose={onClose}
      titleKey={SDKWORK_SUBSCRIPTION_I18N_KEYS.dialogs.tokenPurchaseTitle}
    />
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
  pointsPurchaseModal: SubscriptionCatalogPointsPurchaseModal,
  redeemModal: (props: SdkworkSubscriptionCatalogModalProps) => (
    <SubscriptionCatalogPlaceholderModal
      {...props}
      titleKey={SDKWORK_SUBSCRIPTION_I18N_KEYS.dialogs.redemptionTitle}
    />
  ),
};
