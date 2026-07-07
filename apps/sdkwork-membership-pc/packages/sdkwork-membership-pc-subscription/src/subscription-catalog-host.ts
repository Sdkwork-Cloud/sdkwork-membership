import type { ComponentType } from "react";

export { SDKWORK_SUBSCRIPTION_CATALOG_UNAVAILABLE_TIER_KEY } from "./subscription-catalog-content";

export interface SdkworkSubscriptionCatalogModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export interface SdkworkSubscriptionCatalogCheckoutPlan {
  id: string;
  membershipTierKey: string;
  name: string;
  originalPrice?: string;
  packageNumericId: number;
  packagePeriodLabel: string;
  priceLabel: string;
}

export interface SdkworkSubscriptionCatalogCheckoutModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  plan: SdkworkSubscriptionCatalogCheckoutPlan | null;
}

export interface SdkworkSubscriptionMemberSummary {
  membershipTierKey: string;
}

export interface SdkworkSubscriptionCatalogHostComponents {
  checkoutModal: ComponentType<SdkworkSubscriptionCatalogCheckoutModalProps>;
  pointsDetailsModal: ComponentType<SdkworkSubscriptionCatalogModalProps>;
  pointsPurchaseModal: ComponentType<SdkworkSubscriptionCatalogModalProps>;
  redeemModal: ComponentType<SdkworkSubscriptionCatalogModalProps>;
}

export interface SdkworkSubscriptionCatalogPageProps {
  components: SdkworkSubscriptionCatalogHostComponents;
  memberSummary?: SdkworkSubscriptionMemberSummary | null;
  notifyOutlet: ComponentType;
  onMembershipTierUpdated: (membershipTierKey: string, durationDays: number) => void;
  onNotify: (message: string, tone: "error" | "info" | "success") => void;
}

export type SdkworkSubscriptionCatalogTranslate = (
  key: string,
  defaultValue?: string,
) => string;
