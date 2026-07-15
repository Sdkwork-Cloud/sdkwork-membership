export type SdkworkMembershipQrPaymentStrategyId =
  | "mobile_cashier_h5"
  | "wechat_native"
  | "alipay_native";

export type SdkworkMembershipQrPaymentProductType = "h5" | "native";

export interface SdkworkMembershipQrPaymentStrategy {
  id: SdkworkMembershipQrPaymentStrategyId;
  paymentMethod?: "wechat_pay" | "alipay";
  productType: SdkworkMembershipQrPaymentProductType;
  resolvePayload(input: {
    cashierUrl?: string;
    providerQrCode?: string;
  }): string | undefined;
}

const MOBILE_CASHIER_H5: SdkworkMembershipQrPaymentStrategy = {
  id: "mobile_cashier_h5",
  productType: "h5",
  resolvePayload: ({ cashierUrl, providerQrCode }) => cashierUrl ?? providerQrCode,
};

const WECHAT_NATIVE: SdkworkMembershipQrPaymentStrategy = {
  id: "wechat_native",
  paymentMethod: "wechat_pay",
  productType: "native",
  resolvePayload: ({ providerQrCode, cashierUrl }) => providerQrCode ?? cashierUrl,
};

const ALIPAY_NATIVE: SdkworkMembershipQrPaymentStrategy = {
  id: "alipay_native",
  paymentMethod: "alipay",
  productType: "native",
  resolvePayload: ({ providerQrCode, cashierUrl }) => providerQrCode ?? cashierUrl,
};

export const SDKWORK_MEMBERSHIP_QR_PAYMENT_STRATEGIES: Readonly<Record<
  SdkworkMembershipQrPaymentStrategyId,
  SdkworkMembershipQrPaymentStrategy
>> = {
  mobile_cashier_h5: MOBILE_CASHIER_H5,
  wechat_native: WECHAT_NATIVE,
  alipay_native: ALIPAY_NATIVE,
};

export function resolveSdkworkMembershipQrPaymentStrategy(
  strategy?: SdkworkMembershipQrPaymentStrategyId | SdkworkMembershipQrPaymentStrategy,
): SdkworkMembershipQrPaymentStrategy {
  if (typeof strategy === "object") {
    return strategy;
  }

  return SDKWORK_MEMBERSHIP_QR_PAYMENT_STRATEGIES[strategy ?? "mobile_cashier_h5"];
}
