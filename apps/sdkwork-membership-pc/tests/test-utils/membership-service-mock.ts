import {
  configureSdkworkMembershipSessionTokenProvider,
  configureSdkworkOrderAppServiceProvider,
  type SdkworkMembershipAppService,
  type SdkworkMembershipSessionTokens,
} from "@sdkwork/membership-service";
import type { OrderAppTransportClient } from "@sdkwork/membership-service";

type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends (...args: infer TArgs) => infer TReturn
    ? (...args: TArgs) => TReturn
    : DeepPartial<T[K]>;
};

export function createMembershipAppServiceMock(
  overrides: DeepPartial<SdkworkMembershipAppService> = {},
): SdkworkMembershipAppService {
  const base: SdkworkMembershipAppService = {
    memberships: createMissingMembershipsTree(),
  };
  return mergeMembershipAppService(base, overrides);
}

export function configureMembershipServiceMockSession(
  tokens: SdkworkMembershipSessionTokens = { authToken: "membership-auth-token" },
): void {
  configureSdkworkMembershipSessionTokenProvider(() => tokens);
}

export function resetMembershipServiceMockSession(): void {
  configureSdkworkMembershipSessionTokenProvider(null);
  configureSdkworkOrderAppServiceProvider(null);
}

export function createOrderAppServiceMock(
  overrides: DeepPartial<OrderAppTransportClient> = {},
): OrderAppTransportClient {
  const base = {
    memberships: {
      orders: {
        create: async () => {
          throw new Error("Missing order service test method: memberships.orders.create");
        },
      },
    },
    orders: {
      payments: {
        create: async () => {
          throw new Error("Missing order service test method: orders.payments.create");
        },
      },
    },
  } as unknown as OrderAppTransportClient;
  return mergeMembershipAppService(base, overrides as DeepPartial<OrderAppTransportClient>);
}

export function configureOrderServiceMock(client: OrderAppTransportClient): void {
  configureSdkworkOrderAppServiceProvider(() => client);
}

function createMissingMembershipsTree(): SdkworkMembershipAppService["memberships"] {
  const tree: Record<string, unknown> = {};
  for (const method of [
    "current.retrieve",
    "current.status.retrieve",
    "plans.list",
    "benefits.list",
    "packages.list",
    "packageGroups.list",
    "purchases.create",
    "purchases.renew",
    "purchases.upgrade",
  ]) {
    addMissingMethod(tree, method);
  }
  return tree as SdkworkMembershipAppService["memberships"];
}

function addMissingMethod(root: Record<string, unknown>, method: string): void {
  let node = root;
  const segments = method.split(".");
  for (const segment of segments.slice(0, -1)) {
    if (!node[segment] || typeof node[segment] === "function") {
      node[segment] = {};
    }
    node = node[segment] as Record<string, unknown>;
  }
  node[segments.at(-1)!] = async () => {
    throw new Error(`Missing membership service test method: ${method}`);
  };
}

function mergeMembershipAppService<T>(base: T, overrides: DeepPartial<T>): T {
  for (const [key, value] of Object.entries(overrides as Record<string, unknown>)) {
    if (
      value &&
      typeof value === "object" &&
      !Array.isArray(value) &&
      typeof (base as Record<string, unknown>)[key] === "object"
    ) {
      mergeMembershipAppService((base as Record<string, unknown>)[key], value as DeepPartial<unknown>);
    } else {
      (base as Record<string, unknown>)[key] = value;
    }
  }
  return base;
}
