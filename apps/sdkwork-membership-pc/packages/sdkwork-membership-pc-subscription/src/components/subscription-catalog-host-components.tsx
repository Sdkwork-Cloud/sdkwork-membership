import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@sdkwork/ui-pc-react";
import {
  SdkworkOrderCheckoutDialog,
  type SdkworkOrderCheckoutDialogCopy,
  type SdkworkOrderCheckoutPayment,
} from "@sdkwork/order-pc-checkout";
import { SdkworkPointsRechargeDialog } from "@sdkwork/order-pc-recharge";
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

export function SubscriptionCatalogPointsPurchaseModal({
  currentPoints,
  isOpen,
  onClose,
}: SdkworkSubscriptionCatalogModalProps) {
  const { t } = useTranslation();
  return (
    <SdkworkPointsRechargeDialog
      copy={{
        account: t("points_recharge.account", "积分账户"),
        agreement: t("points_recharge.agreement", "我已阅读并同意《积分充值服务协议》"),
        agreementAccepted: t("points_recharge.agreement_accepted", "您已同意《积分充值服务协议》"),
        agreementRequired: t("points_recharge.agreement_required", "请先同意积分充值服务协议"),
        close: t(SDKWORK_SUBSCRIPTION_I18N_KEYS.dialogs.close, "关闭"),
        completed: t("points_recharge.completed", "支付完成，积分已到账"),
        confirmPayment: t("points_recharge.confirm_payment", "同意并支付"),
        creatingPayment: t("points_recharge.creating_payment", "正在生成支付二维码..."),
        emptyPackages: t("points_recharge.empty_packages", "暂无可用充值套餐"),
        loadFailed: t("points_recharge.load_failed", "充值套餐加载失败"),
        loadingPackages: t("points_recharge.loading_packages", "正在加载充值套餐..."),
        myPoints: t("points_recharge.my_points", "我的积分"),
        notice: t("points_recharge.notice", "积分不可转赠、不可提现，充值后有效期以平台规则为准。"),
        paymentUnavailable: t("points_recharge.payment_unavailable", "支付暂不可用"),
        paymentUnavailableDescription: t("points_recharge.payment_unavailable_description", "暂时无法生成支付二维码，请稍后重试。"),
        pointsUnit: t("points_recharge.points_unit", "积分"),
        retry: t("points_recharge.retry", "重新加载"),
        scanPrompt: t("points_recharge.scan_prompt", "请扫码完成支付"),
        title: t("points_recharge.title", "积分购买"),
      }}
      currentPoints={currentPoints}
      isOpen={isOpen}
      onClose={onClose}
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
