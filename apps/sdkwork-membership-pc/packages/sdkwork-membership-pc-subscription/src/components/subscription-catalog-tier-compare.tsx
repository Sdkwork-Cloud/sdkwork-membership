import { Check } from "lucide-react";
import type {
  SdkworkSubscriptionCatalogBillingCycleOption,
  SdkworkSubscriptionCatalogComparisonCategory,
} from "../subscription-catalog-content";
import type { SdkworkSubscriptionCatalogTierColumnModel } from "../subscription-catalog-mapper";
import { SubscriptionCatalogBillingTabs } from "./subscription-catalog-billing-tabs";

interface SubscriptionCatalogTierCompareProps {
  basicPlanLabel: string;
  billingCycleIndex: number;
  billingCycles: SdkworkSubscriptionCatalogBillingCycleOption[];
  comingSoonLabel: string;
  comparisonCategories: SdkworkSubscriptionCatalogComparisonCategory[];
  currentPlanLabel: string;
  firstYear58Label: string;
  firstYear60Label: string;
  onSelectBillingCycle: (index: number) => void;
  onSelectPackage: (
    packageId: string,
    membershipTierKey: string,
    packageName: string,
    priceLabel: string,
    originalPriceLabel: string | undefined,
    packagePeriodLabel: string,
  ) => void;
  perMonthShortLabel: string;
  perYearShortLabel: string;
  premiumPlanLabel: string;
  sectionTitle: string;
  standardPlanLabel: string;
  superPlanLabel: string;
  tierColumns?: SdkworkSubscriptionCatalogTierColumnModel[];
}

export function SubscriptionCatalogTierCompare({
  billingCycleIndex,
  billingCycles,
  comingSoonLabel,
  comparisonCategories,
  currentPlanLabel,
  onSelectBillingCycle,
  onSelectPackage,
  perMonthShortLabel,
  perYearShortLabel,
  superPlanLabel,
  sectionTitle,
  tierColumns = [],
}: SubscriptionCatalogTierCompareProps) {
  const [basicColumn, standardColumn, premiumColumn] = tierColumns;

  return (
    <div className="w-full w-full mx-auto px-6 mt-32 overflow-x-auto pb-4">
      <h2 className="text-center text-2xl font-black text-zinc-900 dark:text-white mb-8">
        {sectionTitle}
      </h2>

      <div className="flex items-center justify-center mb-12">
        <SubscriptionCatalogBillingTabs
          billingCycleIndex={billingCycleIndex}
          billingCycles={billingCycles}
          keyPrefix="compare"
          onSelectBillingCycle={onSelectBillingCycle}
        />
      </div>

      <div className="min-w-[1000px] grid grid-cols-6 gap-4 mb-4">
        <div className="px-2" />
        <div className="flex flex-col items-start px-2">
          <h4 className="text-zinc-900 dark:text-white text-[16px] font-bold mb-2">Free</h4>
          <div className="flex items-baseline mb-4">
            <span className="text-zinc-900 dark:text-white text-[14px] font-medium mr-1">¥</span>
            <span className="text-zinc-900 dark:text-white text-[24px] font-black leading-none">0</span>
            <span className="text-zinc-500 dark:text-zinc-400 text-[12px] ml-1 font-medium">{perMonthShortLabel}</span>
          </div>
          <button
            className="w-full py-2.5 rounded-full text-[14px] font-bold bg-zinc-100 dark:bg-zinc-800 text-zinc-500 dark:text-zinc-400 border border-zinc-200 dark:border-zinc-700"
            type="button"
          >
            {currentPlanLabel}
          </button>
        </div>

        {[basicColumn, standardColumn, premiumColumn].map((column) => (
          column ? (
            <div className="flex flex-col items-start px-2" key={column.packageId}>
              <h4 className="text-zinc-900 dark:text-white text-[16px] font-bold mb-2">{column.name}</h4>
              <div className="flex items-baseline mb-4">
                <span className="text-zinc-900 dark:text-white text-[14px] font-medium mr-1">¥</span>
                <span className="text-zinc-900 dark:text-white text-[24px] font-black leading-none">{column.priceLabel}</span>
                <span className="text-zinc-500 dark:text-zinc-400 text-[12px] ml-1 font-medium">{column.packagePeriodLabel}</span>
              </div>
              <button
                className={`w-full py-2.5 rounded-full text-[14px] font-bold shadow-sm transition-colors ${
                  column.buttonDisabled
                    ? "bg-zinc-100 dark:bg-zinc-800 text-zinc-500 dark:text-zinc-400 border border-zinc-200 dark:border-zinc-700"
                    : "bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-100"
                }`}
                disabled={column.buttonDisabled}
                onClick={() => onSelectPackage(
                  column.packageId,
                  column.membershipTierKey,
                  column.name,
                  column.priceLabel,
                  column.originalPriceLabel,
                  column.packagePeriodLabel,
                )}
                type="button"
              >
                {column.buttonText}
              </button>
            </div>
          ) : null
        ))}

        <div className="flex flex-col items-start px-2">
          <h4 className="text-zinc-900 dark:text-white text-[16px] font-bold mb-2">{superPlanLabel}</h4>
          <div className="flex items-baseline mb-4">
            <span className="text-zinc-900 dark:text-white text-[14px] font-medium mr-1">¥</span>
            <span className="text-zinc-900 dark:text-white text-[24px] font-black leading-none">4???9</span>
            <span className="text-zinc-500 dark:text-zinc-400 text-[12px] ml-1 font-medium">{perYearShortLabel}</span>
          </div>
          <button
            className="w-full py-2.5 rounded-full text-[14px] font-bold bg-zinc-100 dark:bg-zinc-800 text-zinc-400 dark:text-zinc-500 border border-zinc-200 dark:border-zinc-700 cursor-not-allowed"
            type="button"
          >
            {comingSoonLabel}
          </button>
        </div>
      </div>

      <div className="min-w-[1000px] mt-8">
        {comparisonCategories.map((category) => (
          <div className="mb-8" key={category.categoryLabel}>
            <div className="bg-zinc-100 dark:bg-zinc-800/50 rounded-xl px-4 py-3 mb-4">
              <h5 className="text-[14px] font-bold text-zinc-900 dark:text-white">{category.categoryLabel}</h5>
            </div>
            <div className="flex flex-col gap-2">
              {category.rows.map((row) => (
                <div
                  className="grid grid-cols-6 gap-4 px-4 py-4 border-b border-zinc-100 dark:border-zinc-800/50 hover:bg-zinc-50 dark:hover:bg-zinc-800/30 transition-colors rounded-lg"
                  key={row.benefitLabel}
                >
                  <div className="flex items-center text-[13px] font-medium text-zinc-700 dark:text-zinc-300">
                    {row.benefitLabel}
                  </div>
                  {row.values.map((value, valueIndex) => (
                    <div
                      className="flex items-center text-[13px] font-medium text-zinc-600 dark:text-zinc-400"
                      key={`${row.benefitLabel}-${valueIndex}`}
                    >
                      {value === true ? (
                        <Check className="text-zinc-900 dark:text-white" size={16} />
                      ) : (
                        value
                      )}
                    </div>
                  ))}
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
