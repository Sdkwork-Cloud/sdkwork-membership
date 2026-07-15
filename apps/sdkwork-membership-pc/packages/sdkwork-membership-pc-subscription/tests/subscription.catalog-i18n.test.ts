import { describe, expect, it } from "vitest";
import {
  SDKWORK_SUBSCRIPTION_I18N_KEYS,
  sdkworkSubscriptionCheckoutMessages,
} from "../src/catalog";

function flattenKeyValues(value: unknown): string[] {
  if (typeof value === "string") {
    return [value];
  }
  if (!value || typeof value !== "object") {
    return [];
  }
  return Object.values(value).flatMap(flattenKeyValues);
}

describe("subscription catalog checkout i18n", () => {
  it("keeps English and Chinese checkout resources aligned", () => {
    const englishKeys = Object.keys(sdkworkSubscriptionCheckoutMessages["en-US"]).sort();
    const chineseKeys = Object.keys(sdkworkSubscriptionCheckoutMessages["zh-CN"]).sort();

    expect(chineseKeys).toEqual(englishKeys);
    expect(englishKeys).toEqual(flattenKeyValues(SDKWORK_SUBSCRIPTION_I18N_KEYS).sort());
  });

  it("resolves every checkout key from package-owned English and Chinese resources", () => {
    for (const key of flattenKeyValues(SDKWORK_SUBSCRIPTION_I18N_KEYS)) {
      const english = sdkworkSubscriptionCheckoutMessages["en-US"][key];
      const chinese = sdkworkSubscriptionCheckoutMessages["zh-CN"][key];

      expect(english).toBeTruthy();
      expect(chinese).toBeTruthy();
      expect(english).not.toBe(key);
      expect(chinese).not.toBe(key);
    }
  });

  it("keeps shared checkout copy application-neutral", () => {
    const copy = JSON.stringify(sdkworkSubscriptionCheckoutMessages);
    expect(copy).not.toMatch(/ClawRouter|Claw Router|BirdCoder|SDKWork IM/i);
  });
});
