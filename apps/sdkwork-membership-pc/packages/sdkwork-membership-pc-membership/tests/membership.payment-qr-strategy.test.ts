import { describe, expect, it } from "vitest";
import {
  SDKWORK_MEMBERSHIP_QR_PAYMENT_STRATEGIES,
  resolveSdkworkMembershipQrPaymentStrategy,
} from "../src/payment-qr-strategy";

describe("membership QR payment strategies", () => {
  const paymentPayloads = {
    cashierUrl: "https://cashier.sdkwork.com/mobile/orders/order-1",
    providerQrCode: "weixin://wxpay/bizpayurl?pr=order-1",
  };

  it("defaults to a mobile H5 cashier link QR payload", () => {
    const strategy = resolveSdkworkMembershipQrPaymentStrategy();

    expect(strategy.id).toBe("mobile_cashier_h5");
    expect(strategy.productType).toBe("h5");
    expect(strategy.resolvePayload(paymentPayloads)).toBe(paymentPayloads.cashierUrl);
  });

  it("supports WeChat and Alipay native provider QR products", () => {
    const wechat = resolveSdkworkMembershipQrPaymentStrategy("wechat_native");
    const alipay = resolveSdkworkMembershipQrPaymentStrategy("alipay_native");

    expect(wechat).toMatchObject({ paymentMethod: "wechat_pay", productType: "native" });
    expect(alipay).toMatchObject({ paymentMethod: "alipay", productType: "native" });
    expect(wechat.resolvePayload(paymentPayloads)).toBe(paymentPayloads.providerQrCode);
    expect(alipay.resolvePayload(paymentPayloads)).toBe(paymentPayloads.providerQrCode);
  });

  it("keeps all built-in strategies usable when one response field is absent", () => {
    for (const strategy of Object.values(SDKWORK_MEMBERSHIP_QR_PAYMENT_STRATEGIES)) {
      expect(strategy.resolvePayload({ cashierUrl: paymentPayloads.cashierUrl })).toBe(
        paymentPayloads.cashierUrl,
      );
      expect(strategy.resolvePayload({ providerQrCode: paymentPayloads.providerQrCode })).toBe(
        paymentPayloads.providerQrCode,
      );
    }
  });
});
