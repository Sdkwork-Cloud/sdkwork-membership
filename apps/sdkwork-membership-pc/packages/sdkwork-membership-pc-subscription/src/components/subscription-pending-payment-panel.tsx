import { ExternalLink, RefreshCw } from "lucide-react";
import { Button, StatusNotice } from "@sdkwork/ui-pc-react";
import { formatSdkworkMembershipCurrencyCny as formatSdkworkCurrencyCny } from "@sdkwork/membership-service";
import type { SdkworkSubscriptionPendingPayment } from "../subscription-controller";
import { useSdkworkSubscriptionIntl } from "../subscription-intl";

export interface SdkworkSubscriptionPendingPaymentPanelProps {
  onDismiss?: () => void;
  onRefresh: () => void;
  pendingPayment: SdkworkSubscriptionPendingPayment;
}

export function SdkworkSubscriptionPendingPaymentPanel({
  onDismiss,
  onRefresh,
  pendingPayment,
}: SdkworkSubscriptionPendingPaymentPanelProps) {
  const { copy, locale } = useSdkworkSubscriptionIntl();
  const amountLabel = pendingPayment.amountCny !== null
    ? formatSdkworkCurrencyCny(pendingPayment.amountCny, locale)
    : copy.common.noValue;

  return (
    <StatusNotice className="mt-5" title={copy.pendingPayment.title} tone="warning">
      <div className="space-y-4">
        <p className="text-sm leading-6 text-[var(--sdk-color-text-secondary)]">
          {copy.pendingPayment.description}
        </p>

        <div className="rounded-2xl border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] p-4 text-sm">
          <div className="font-semibold text-[var(--sdk-color-text-primary)]">
            {pendingPayment.packageName ?? copy.pendingPayment.packageFallback}
          </div>
          {pendingPayment.orderId ? (
            <div className="mt-2 text-[var(--sdk-color-text-secondary)]">
              {copy.pendingPayment.orderLabel}: {pendingPayment.orderId}
            </div>
          ) : null}
          <div className="mt-2 tabular-nums text-[var(--sdk-color-text-primary)]">
            {copy.pendingPayment.amountLabel}: {amountLabel}
          </div>
        </div>

        {pendingPayment.qrCode ? (
          <div className="rounded-2xl border border-dashed border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4">
            <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
              {copy.pendingPayment.qrLabel}
            </div>
            <pre className="mt-3 overflow-x-auto whitespace-pre-wrap break-all rounded-xl bg-[var(--sdk-color-surface-panel-muted)] p-3 text-xs text-[var(--sdk-color-text-primary)]">
              {pendingPayment.qrCode}
            </pre>
          </div>
        ) : null}

        <div className="flex flex-wrap gap-3">
          {pendingPayment.cashierUrl ? (
            <Button asChild type="button" variant="secondary">
              <a href={pendingPayment.cashierUrl} rel="noreferrer" target="_blank">
                <ExternalLink className="mr-2 h-4 w-4" />
                {copy.pendingPayment.openCashier}
              </a>
            </Button>
          ) : null}
          <Button onClick={onRefresh} type="button" variant="outline">
            <RefreshCw className="mr-2 h-4 w-4" />
            {copy.pendingPayment.refresh}
          </Button>
          {onDismiss ? (
            <Button onClick={onDismiss} type="button" variant="ghost">
              {copy.pendingPayment.dismiss}
            </Button>
          ) : null}
        </div>
      </div>
    </StatusNotice>
  );
}
