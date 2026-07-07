import type { SdkworkSubscriptionCatalogBillingCycleOption } from "../subscription-catalog-content";

interface SubscriptionCatalogBillingTabsProps {
  billingCycleIndex: number;
  billingCycles: SdkworkSubscriptionCatalogBillingCycleOption[];
  keyPrefix?: string;
  onSelectBillingCycle: (index: number) => void;
}

export function SubscriptionCatalogBillingTabs({
  billingCycleIndex,
  billingCycles,
  keyPrefix = "billing",
  onSelectBillingCycle,
}: SubscriptionCatalogBillingTabsProps) {
  return (
    <div className="bg-zinc-100 dark:bg-zinc-900 rounded-full p-1 border border-zinc-200 dark:border-zinc-800 inline-flex">
      {billingCycles.map((cycle, index) => (
        <button
          className={`relative px-6 py-2 rounded-full text-[14px] font-medium transition-colors flex items-center gap-1.5 ${
            billingCycleIndex === index
              ? "bg-white dark:bg-zinc-800 text-zinc-900 dark:text-white shadow-sm"
              : "text-zinc-500 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white"
          }`}
          key={`${keyPrefix}-${cycle.label}`}
          onClick={() => onSelectBillingCycle(index)}
          type="button"
        >
          <span>{cycle.label}</span>
          {cycle.discountLabel ? (
            <span className="text-[12px] opacity-70 font-bold">{cycle.discountLabel}</span>
          ) : null}
        </button>
      ))}
    </div>
  );
}
