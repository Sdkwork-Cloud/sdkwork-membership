import { useState } from "react";
import { Banknote, Zap } from "lucide-react";
import { createSdkworkMembershipToneStyle } from "../membership-appearance";
import { useSdkworkMembershipIntl } from "../membership-intl";

export interface SdkworkMembershipQuotaRechargeInput {
  grantQuantity: number;
  amountCny: string;
}

export interface SdkworkMembershipQuotaRechargePanelProps {
  disabled?: boolean;
  isMember?: boolean;
  isSubmitting?: boolean;
  onRecharge: (input: SdkworkMembershipQuotaRechargeInput) => void;
}

/**
 * 订阅期额度充值面板：输入充值数量与金额，向当前有效订阅追加权益额度。
 * 充值走会员订单（action=recharge）→ 支付 → 结算入账。
 */
export function SdkworkMembershipQuotaRechargePanel({
  disabled = false,
  isMember = false,
  isSubmitting = false,
  onRecharge,
}: SdkworkMembershipQuotaRechargePanelProps) {
  const { copy } = useSdkworkMembershipIntl();
  const [quantity, setQuantity] = useState("");
  const [amount, setAmount] = useState("");
  const [error, setError] = useState<string | null>(null);

  function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    const parsedQuantity = Number(quantity);
    const parsedAmount = Number(amount);
    if (!Number.isInteger(parsedQuantity) || parsedQuantity <= 0 || !(parsedAmount > 0)) {
      setError(copy.quota.error);
      return;
    }
    setError(null);
    onRecharge({
      grantQuantity: parsedQuantity,
      amountCny: amount.trim(),
    });
  }

  return (
    <section
      className="rounded-[1.5rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]"
      data-sdkwork-membership-quota-recharge
    >
      <div className="px-5 py-4 sm:px-6">
        <div className="flex items-center gap-2">
          <span className="flex h-8 w-8 items-center justify-center rounded-full bg-[var(--sdk-color-brand-soft)] text-[var(--sdk-color-brand)]">
            <Zap className="h-4 w-4" aria-hidden="true" />
          </span>
          <div>
            <h3 className="text-sm font-bold text-[var(--sdk-color-text-strong)]">
              {copy.quota.title}
            </h3>
            <p className="mt-0.5 text-xs leading-5 text-[var(--sdk-color-text-muted)]">
              {copy.quota.description}
            </p>
          </div>
        </div>

        {!isMember ? (
          <p className="mt-4 rounded-xl bg-[var(--sdk-color-surface-raised)] px-3 py-2 text-xs text-[var(--sdk-color-text-muted)]">
            {copy.quota.onlyForMembers}
          </p>
        ) : (
          <form className="mt-4 grid gap-3 sm:grid-cols-[1fr_1fr_auto]" onSubmit={handleSubmit}>
            <label className="flex flex-col gap-1 text-xs font-medium text-[var(--sdk-color-text-muted)]">
              {copy.quota.quantityLabel}
              <input
                className="h-10 rounded-xl border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-base)] px-3 text-sm text-[var(--sdk-color-text-strong)] outline-none focus:border-[var(--sdk-color-brand)]"
                disabled={disabled || isSubmitting}
                inputMode="numeric"
                onChange={(event) => setQuantity(event.target.value)}
                placeholder={copy.quota.quantityPlaceholder}
                value={quantity}
              />
            </label>
            <label className="flex flex-col gap-1 text-xs font-medium text-[var(--sdk-color-text-muted)]">
              {copy.quota.amountLabel}
              <input
                className="h-10 rounded-xl border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-base)] px-3 text-sm text-[var(--sdk-color-text-strong)] outline-none focus:border-[var(--sdk-color-brand)]"
                disabled={disabled || isSubmitting}
                inputMode="decimal"
                onChange={(event) => setAmount(event.target.value)}
                placeholder={copy.quota.amountPlaceholder}
                value={amount}
              />
            </label>
            <button
              className="inline-flex h-10 items-center justify-center gap-2 self-end rounded-xl bg-[var(--sdk-color-brand)] px-4 text-sm font-semibold text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
              disabled={disabled || isSubmitting}
              type="submit"
            >
              <Banknote className="h-4 w-4" aria-hidden="true" />
              {isSubmitting ? copy.quota.submitting : copy.quota.submit}
            </button>
          </form>
        )}

        {error ? (
          <p
            className="mt-3 rounded-xl px-3 py-2 text-xs"
            style={createSdkworkMembershipToneStyle("danger")}
          >
            {error}
          </p>
        ) : null}
      </div>
    </section>
  );
}
