import { Check, Sparkles } from "lucide-react";
import type { SdkworkSubscriptionCatalogPlanCardModel } from "../subscription-catalog-content";

interface SubscriptionCatalogPlanGridProps {
  onSelectPlan: (plan: SdkworkSubscriptionCatalogPlanCardModel) => void;
  plans: SdkworkSubscriptionCatalogPlanCardModel[];
}

export function SubscriptionCatalogPlanGrid({
  onSelectPlan,
  plans,
}: SubscriptionCatalogPlanGridProps) {
  return (
    <div className="w-full w-full mx-auto px-6">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {plans.map((plan) => (
          <div
            className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-3xl p-6 flex flex-col shadow-sm"
            key={plan.id}
          >
            <h3 className="text-zinc-900 dark:text-white text-[20px] font-bold mb-4">{plan.name}</h3>

            <div className="flex items-baseline mb-2">
              <span className="text-zinc-900 dark:text-white text-[16px] font-medium mr-1">¥</span>
              <span className="text-zinc-900 dark:text-white text-[40px] font-black leading-none">{plan.priceLabel}</span>
              <span className="text-zinc-500 dark:text-zinc-400 text-[14px] ml-2 font-medium">
                {plan.packagePeriodLabel}{" "}
                {plan.originalPriceLabel ? <span className="line-through">{plan.originalPriceLabel}</span> : null}
              </span>
            </div>

            <div className="h-10">
              {plan.subtitle ? (
                <p className="text-zinc-500 dark:text-zinc-400 text-[12px] leading-relaxed font-medium">
                  {plan.subtitle}
                </p>
              ) : null}
            </div>

            <div className="bg-zinc-50 dark:bg-zinc-950 border border-zinc-100 dark:border-zinc-800/50 rounded-2xl p-4 mt-4 mb-6">
              <div className="flex items-center gap-1.5 text-zinc-900 dark:text-white text-[14px] font-bold mb-1.5">
                <Sparkles className="text-orange-500" size={14} />
                <span>{plan.pointsAllowanceLabel}</span>
              </div>
              {plan.pointsConversionLabel ? (
                <div className="text-zinc-500 dark:text-zinc-400 text-[12px] font-medium">
                  {plan.pointsConversionLabel}
                </div>
              ) : null}
            </div>

            <button
              className={`w-full py-3.5 rounded-full text-[15px] font-bold transition-all mb-8 shadow-sm ${plan.buttonStyle}`}
              disabled={plan.disabled}
              onClick={() => onSelectPlan(plan)}
              type="button"
            >
              {plan.buttonText}
            </button>

            <div className="space-y-4 flex-1">
              {plan.features.map((feature) => (
                <div className="flex items-center justify-between" key={`${plan.id}-${feature.text}`}>
                  <div className="flex items-center gap-3">
                    {feature.emptyCheck ? (
                      <Check className="text-transparent" size={14} />
                    ) : (
                      <Check className="text-zinc-400 dark:text-zinc-500" size={14} />
                    )}
                    <span className="text-[13px] font-medium text-zinc-700 dark:text-zinc-300">{feature.text}</span>
                  </div>
                  {feature.tag ? (
                    <span className="bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 text-[11px] font-bold px-2 py-0.5 rounded text-xs border border-zinc-200 dark:border-zinc-700">
                      {feature.tag}
                    </span>
                  ) : null}
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
