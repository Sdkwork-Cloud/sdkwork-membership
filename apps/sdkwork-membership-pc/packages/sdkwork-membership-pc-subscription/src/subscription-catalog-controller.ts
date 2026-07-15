import {
  useCallback,
  useMemo,
  useRef,
  useSyncExternalStore,
} from "react";
import {
  createSdkworkSubscriptionCatalogService,
  type SdkworkSubscriptionCatalogData,
  type SdkworkSubscriptionCatalogService,
  type SdkworkSubscriptionCatalogViewModel,
} from "./subscription-catalog-service";
import type { SdkworkSubscriptionPurchaseResult } from "./subscription-service";
import type { SdkworkSubscriptionCatalogCheckoutPlan } from "./subscription-catalog-host";

export interface SdkworkSubscriptionCatalogControllerState extends SdkworkSubscriptionCatalogViewModel {
  catalog: SdkworkSubscriptionCatalogData | null;
  isBootstrapped: boolean;
  isLoading: boolean;
  isMutating: boolean;
  lastError?: string;
  selectedCheckoutPlan: SdkworkSubscriptionCatalogCheckoutPlan | null;
}

export interface SdkworkSubscriptionCatalogController {
  bootstrap(): Promise<SdkworkSubscriptionCatalogControllerState>;
  clearCheckoutPlan(): void;
  getState(): SdkworkSubscriptionCatalogControllerState;
  getPurchaseStatus(orderId: string): Promise<SdkworkSubscriptionPurchaseResult>;
  purchaseSelectedPlan(): Promise<SdkworkSubscriptionPurchaseResult>;
  refresh(): Promise<SdkworkSubscriptionCatalogControllerState>;
  retry(): Promise<SdkworkSubscriptionCatalogControllerState>;
  selectBillingCycle(index: number): SdkworkSubscriptionCatalogControllerState;
  selectCheckoutPlan(plan: SdkworkSubscriptionCatalogCheckoutPlan | null): void;
  service: SdkworkSubscriptionCatalogService;
  subscribe(listener: () => void): () => void;
}

export interface CreateSdkworkSubscriptionCatalogControllerOptions {
  initialState?: Partial<SdkworkSubscriptionCatalogControllerState>;
  locale?: string | null;
  service?: Partial<SdkworkSubscriptionCatalogService>;
  translate?: (key: string, defaultValue?: string) => string;
}

/**
 * Default billing cycle index for the token plan catalog.
 *
 * The package groups are sorted by sort_weight: annual(1), monthly(2),
 * quarterly(3), single(4). Index 3 selects "单月购买" (single month
 * purchase) as the default active billing cycle tab, matching the
 * token plan default activation requirement.
 */
const DEFAULT_BILLING_CYCLE_INDEX = 3;

function createInitialState(
  service: SdkworkSubscriptionCatalogService,
): SdkworkSubscriptionCatalogControllerState {
  const fallback = service.getFallbackViewModel(DEFAULT_BILLING_CYCLE_INDEX);
  return {
    ...fallback,
    catalog: null,
    isBootstrapped: false,
    isLoading: false,
    isMutating: false,
    selectedCheckoutPlan: null,
  };
}

export function createSdkworkSubscriptionCatalogController(
  options: CreateSdkworkSubscriptionCatalogControllerOptions = {},
): SdkworkSubscriptionCatalogController {
  const service: SdkworkSubscriptionCatalogService = options.service
    ? {
        ...createSdkworkSubscriptionCatalogService({
          locale: options.locale,
          translate: options.translate,
        }),
        ...options.service,
      }
    : createSdkworkSubscriptionCatalogService({
        locale: options.locale,
        translate: options.translate,
      });
  const listeners = new Set<() => void>();
  let state: SdkworkSubscriptionCatalogControllerState = {
    ...createInitialState(service),
    ...options.initialState,
  };

  function emit(): void {
    listeners.forEach((listener) => listener());
  }

  function setState(
    next:
      | Partial<SdkworkSubscriptionCatalogControllerState>
      | ((current: SdkworkSubscriptionCatalogControllerState) => Partial<SdkworkSubscriptionCatalogControllerState>),
  ): void {
    const partial = typeof next === "function" ? next(state) : next;
    state = {
      ...state,
      ...partial,
    };
    emit();
  }

  function applyViewModel(
    catalog: SdkworkSubscriptionCatalogData,
    billingCycleIndex: number,
  ): void {
    const viewModel = service.getViewModel(catalog, billingCycleIndex);
    setState({
      ...viewModel,
      catalog,
      isBootstrapped: true,
      isLoading: false,
      isMutating: false,
    });
  }

  return {
    async bootstrap() {
      if (state.isLoading) {
        return state;
      }
      setState({
        isLoading: true,
        lastError: undefined,
      });

      try {
        const catalog = await service.getCatalog();
        applyViewModel(catalog, state.billingCycleIndex);
      } catch (error) {
        const fallback = service.getFallbackViewModel(state.billingCycleIndex);
        setState({
          ...fallback,
          catalog: null,
          isBootstrapped: true,
          isLoading: false,
          lastError: error instanceof Error ? error.message : "Failed to load subscription catalog.",
        });
      }

      return state;
    },

    clearCheckoutPlan() {
      setState({ selectedCheckoutPlan: null });
    },

    getState() {
      return state;
    },

    async getPurchaseStatus(orderId) {
      return service.getPurchaseStatus(orderId);
    },

    async purchaseSelectedPlan() {
      const checkoutPlan = state.selectedCheckoutPlan;
      if (!checkoutPlan) {
        throw new Error("No checkout plan selected.");
      }

      setState({ isMutating: true, lastError: undefined });
      try {
        const result = await service.purchasePackage({
          packageId: checkoutPlan.packageNumericId,
          paymentMethod: "WECHAT",
        });
        setState({ isMutating: false });
        return result;
      } catch (error) {
        setState({
          isMutating: false,
          lastError: error instanceof Error ? error.message : "Purchase failed.",
        });
        throw error;
      }
    },

    async refresh() {
      return this.bootstrap();
    },

    async retry() {
      setState({ isBootstrapped: false, lastError: undefined });
      return this.bootstrap();
    },

    selectBillingCycle(index) {
      if (!state.catalog) {
        setState({ billingCycleIndex: index });
        return state;
      }

      applyViewModel(state.catalog, index);
      return state;
    },

    selectCheckoutPlan(plan) {
      setState({ selectedCheckoutPlan: plan });
    },

    service,

    subscribe(listener) {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };
}

export function useSdkworkSubscriptionCatalogControllerState(
  controller: SdkworkSubscriptionCatalogController,
): SdkworkSubscriptionCatalogControllerState {
  return useSyncExternalStore(
    controller.subscribe,
    controller.getState,
    controller.getState,
  );
}

/**
 * React hook that returns a stable `SdkworkSubscriptionCatalogController`.
 *
 * The `translate` option is captured in a ref so it does NOT participate in
 * the `useMemo` dependency array.  This is critical because callers typically
 * pass an inline arrow function for `translate` (e.g.
 * `(k, d) => t(k, d ?? k)`), which would otherwise create a new function
 * reference on every render, causing `useMemo` to rebuild the controller on
 * every render, which in turn triggers `useSyncExternalStore` to detect a
 * "new" store and re-render – producing an infinite update loop.
 *
 * The ref pattern lets the latest `translate` be used inside the controller
 * without invalidating the memoised controller identity.
 */
export function useSdkworkSubscriptionCatalogController(
  controllerProp?: SdkworkSubscriptionCatalogController,
  options: CreateSdkworkSubscriptionCatalogControllerOptions = {},
): SdkworkSubscriptionCatalogController {
  const translateRef = useRef(options.translate);
  translateRef.current = options.translate;

  const stableTranslate = useCallback(
    (key: string, defaultValue?: string) => {
      const fn = translateRef.current;
      if (fn) return fn(key, defaultValue);
      return defaultValue ?? key;
    },
    [],
  );

  return useMemo(
    () => controllerProp ?? createSdkworkSubscriptionCatalogController({
      ...options,
      translate: stableTranslate,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps -- translate is via ref, locale and service are stable references when properly memoised by the caller
    [controllerProp, options.locale, options.service, stableTranslate],
  );
}
