import { sdkworkSubscriptionCheckoutEnUsResource } from "./en-US/commerce/subscription/checkout";
import { SDKWORK_SUBSCRIPTION_I18N_KEYS } from "../subscription-i18n-keys";
import { sdkworkSubscriptionCheckoutZhCnResource } from "./zh-CN/commerce/subscription/checkout";

type ResourceTree = { readonly [key: string]: string | ResourceTree };

function flattenResource(
  resource: ResourceTree,
  prefix = "",
  output: Record<string, string> = {},
): Record<string, string> {
  for (const [key, value] of Object.entries(resource)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "string") {
      output[path] = value;
    } else {
      flattenResource(value, path, output);
    }
  }
  return output;
}

export const sdkworkSubscriptionCheckoutResources = {
  "en-US": sdkworkSubscriptionCheckoutEnUsResource,
  "zh-CN": sdkworkSubscriptionCheckoutZhCnResource,
} as const;

export const sdkworkSubscriptionCheckoutMessages = {
  "en-US": flattenResource(sdkworkSubscriptionCheckoutEnUsResource),
  "zh-CN": flattenResource(sdkworkSubscriptionCheckoutZhCnResource),
} as const;

export const sdkworkSubscriptionCheckoutI18nBundle = {
  en: sdkworkSubscriptionCheckoutMessages["en-US"],
  zh: sdkworkSubscriptionCheckoutMessages["zh-CN"],
} as const;

export { SDKWORK_SUBSCRIPTION_I18N_KEYS };
