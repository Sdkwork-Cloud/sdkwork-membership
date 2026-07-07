import {
  useMemo,
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
  purchaseSelectedPlan(): Promise<SdkworkSubscriptionPurchaseResult>;
  refresh(): Promise<SdkworkSubscriptionCatalogControllerState>;
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

function createInitialState(
  service: SdkworkSubscriptionCatalogService,
): SdkworkSubscriptionCatalogControllerState {
  const fallback = service.getFallbackViewModel(0);
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

    async purchaseSelectedPlan() {
      const checkoutPlan = state.selectedCheckoutPlan;
      if (!checkoutPlan) {
        throw new Error("No checkout plan selected.");
      }

      setState({ isMutating: true, lastError: undefined });
      try {
        const result = await service.purchasePackage({
          packageId: checkoutPlan.packageNumericId,
        });
        setState({ isMutating: false, selectedCheckoutPlan: null });
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

export function useSdkworkSubscriptionCatalogController(
  controllerProp?: SdkworkSubscriptionCatalogController,
  options: CreateSdkworkSubscriptionCatalogControllerOptions = {},
): SdkworkSubscriptionCatalogController {
  return useMemo(
    () => controllerProp ?? createSdkworkSubscriptionCatalogController(options),
    [controllerProp, options.locale, options.service, options.translate],
  );
}
