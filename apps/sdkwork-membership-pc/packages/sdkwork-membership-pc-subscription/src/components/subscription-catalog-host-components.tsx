import { Button } from "@sdkwork/ui-pc-react";
import type {
  SdkworkSubscriptionCatalogCheckoutModalProps,
  SdkworkSubscriptionCatalogModalProps,
} from "../subscription-catalog-host";

export function SubscriptionCatalogCheckoutModal({
  isOpen,
  onClose,
  onSuccess,
  plan,
}: SdkworkSubscriptionCatalogCheckoutModalProps) {
  if (!isOpen || !plan) {
    return null;
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 px-4">
      <div
        className="w-full max-w-md rounded-3xl border border-zinc-200 bg-white p-6 shadow-xl dark:border-zinc-800 dark:bg-zinc-900"
        role="dialog"
        aria-modal="true"
        aria-labelledby="subscription-catalog-checkout-title"
      >
        <h3
          className="text-lg font-bold text-zinc-900 dark:text-white"
          id="subscription-catalog-checkout-title"
        >
          {plan.name}
        </h3>
        <p className="mt-2 text-sm text-zinc-500 dark:text-zinc-400">
          ¥{plan.priceLabel} / {plan.packagePeriodLabel}
          {plan.originalPrice ? ` · ${plan.originalPrice}` : ""}
        </p>
        <div className="mt-6 flex justify-end gap-3">
          <Button onClick={onClose} type="button" variant="secondary">
            取消
          </Button>
          <Button onClick={onSuccess} type="button">
            确认订阅
          </Button>
        </div>
      </div>
    </div>
  );
}

export function SubscriptionCatalogPlaceholderModal({
  isOpen,
  onClose,
  title,
}: SdkworkSubscriptionCatalogModalProps & { title: string }) {
  if (!isOpen) {
    return null;
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 px-4">
      <div className="w-full max-w-sm rounded-3xl border border-zinc-200 bg-white p-6 shadow-xl dark:border-zinc-800 dark:bg-zinc-900">
        <h3 className="text-lg font-bold text-zinc-900 dark:text-white">{title}</h3>
        <p className="mt-2 text-sm text-zinc-500 dark:text-zinc-400">功能即将上线。</p>
        <div className="mt-6 flex justify-end">
          <Button onClick={onClose} type="button" variant="secondary">
            关闭
          </Button>
        </div>
      </div>
    </div>
  );
}

export const sdkworkSubscriptionCatalogHostComponents = {
  checkoutModal: SubscriptionCatalogCheckoutModal,
  pointsDetailsModal: (props: SdkworkSubscriptionCatalogModalProps) => (
    <SubscriptionCatalogPlaceholderModal {...props} title="积分明细" />
  ),
  pointsPurchaseModal: (props: SdkworkSubscriptionCatalogModalProps) => (
    <SubscriptionCatalogPlaceholderModal {...props} title="购买积分" />
  ),
  redeemModal: (props: SdkworkSubscriptionCatalogModalProps) => (
    <SubscriptionCatalogPlaceholderModal {...props} title="会员兑换" />
  ),
};
