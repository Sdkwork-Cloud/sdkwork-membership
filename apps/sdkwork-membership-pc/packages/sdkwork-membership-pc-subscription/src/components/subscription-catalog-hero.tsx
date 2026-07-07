import type { SdkworkSubscriptionCatalogBillingCycleOption } from "../subscription-catalog-content";
import { SubscriptionCatalogBillingTabs } from "./subscription-catalog-billing-tabs";

interface SubscriptionCatalogHeroProps {
  billingCycleIndex: number;
  billingCycles: SdkworkSubscriptionCatalogBillingCycleOption[];
  onOpenPointsDetails: () => void;
  onOpenPointsPurchase: () => void;
  onOpenRedeem: () => void;
  onSelectBillingCycle: (index: number) => void;
  subtitleLead: string;
  subtitlePointsActionLabel: string;
  subtitleRedeemActionLabel: string;
  title: string;
}

export function SubscriptionCatalogHero({
  billingCycleIndex,
  billingCycles,
  onOpenPointsDetails,
  onOpenPointsPurchase,
  onOpenRedeem,
  onSelectBillingCycle,
  subtitleLead,
  subtitlePointsActionLabel,
  subtitleRedeemActionLabel,
  title,
}: SubscriptionCatalogHeroProps) {
  return (
    <div className="text-center pt-12 pb-12 w-full w-full mx-auto px-6">
      <h1 className="text-3xl md:text-4xl font-black text-zinc-900 dark:text-white mb-4 tracking-tight">
        {title}
      </h1>
      <div className="text-zinc-500 dark:text-zinc-400 text-[15px] flex items-center justify-center gap-1 font-medium">
        <span>{subtitleLead}</span>
        <button
          className="text-zinc-700 dark:text-zinc-300 hover:text-zinc-900 dark:hover:text-white transition-colors"
          onClick={onOpenPointsPurchase}
          type="button"
        >
          {subtitlePointsActionLabel}
        </button>
        <span className="mx-1">|</span>
        <button
          className="text-zinc-700 dark:text-zinc-300 hover:text-zinc-900 dark:hover:text-white transition-colors"
          onClick={onOpenRedeem}
          type="button"
        >
          {subtitleRedeemActionLabel}
        </button>
      </div>

      <div className="flex items-center justify-center mt-10 relative">
        <SubscriptionCatalogBillingTabs
          billingCycleIndex={billingCycleIndex}
          billingCycles={billingCycles}
          onSelectBillingCycle={onSelectBillingCycle}
        />
        <button
          className="absolute right-0 text-zinc-500 dark:text-zinc-400 text-[14px] font-medium hover:text-zinc-900 dark:hover:text-white transition-colors hidden md:block"
          onClick={onOpenPointsDetails}
          type="button"
        >
          积分详情
        </button>
      </div>
    </div>
  );
}
