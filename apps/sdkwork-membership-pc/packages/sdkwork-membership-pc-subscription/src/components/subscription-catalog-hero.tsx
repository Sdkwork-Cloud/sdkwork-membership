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
    <div className="mx-auto w-full px-4 pb-12 pt-12 text-center sm:px-6">
      <h1 className="text-3xl md:text-4xl font-black text-zinc-900 dark:text-white mb-4 tracking-tight">
        {title}
      </h1>
      <div className="flex flex-wrap items-center justify-center gap-1 text-[15px] font-medium text-zinc-500 dark:text-zinc-400">
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

      <div className="relative mt-7 flex items-center justify-center sm:mt-10">
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
          算力元详情
        </button>
      </div>
    </div>
  );
}
