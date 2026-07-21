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
    <div className="inline-flex rounded-full border border-zinc-200 bg-zinc-100 p-1 dark:border-zinc-800 dark:bg-zinc-900 max-sm:grid max-sm:w-full max-sm:grid-cols-2 max-sm:rounded-lg">
      {billingCycles.map((cycle, index) => (
        <button
          className={`relative flex items-center gap-1.5 rounded-full px-6 py-2 text-[14px] font-medium transition-colors max-sm:justify-center max-sm:rounded-md max-sm:px-2 ${
            billingCycleIndex === index
              ? "bg-white dark:bg-zinc-800 text-zinc-900 dark:text-white shadow-sm"
              : "text-zinc-500 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white"
          }`}
          key={`${keyPrefix}-${cycle.label}`}
          onClick={() => onSelectBillingCycle(index)}
          type="button"
        >
          <span className="whitespace-nowrap">{cycle.label}</span>
          {cycle.discountLabel ? (
            <span className="whitespace-nowrap text-[12px] font-bold opacity-70">{cycle.discountLabel}</span>
          ) : null}
        </button>
      ))}
    </div>
  );
}
