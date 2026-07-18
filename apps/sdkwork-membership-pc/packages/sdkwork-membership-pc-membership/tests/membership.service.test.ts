import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  configureMembershipServiceMockSession,
  createMembershipAppServiceMock,
  resetMembershipServiceMockSession,
} from "../../../tests/test-utils/membership-service-mock";
import {
  createSdkworkMembershipService,
  type SdkworkMembershipCheckoutPort,
} from "../src";

function wrapListResponse<T>(items: T[]) {
  return {
    code: 0,
    data: {
      items,
      pageInfo: { hasMore: false, mode: "offset" as const, page: 1, pageSize: 20 },
    },
    traceId: "membership-test-trace",
  };
}

function wrapResourceResponse<T>(data: T) {
  return { code: 0, data, traceId: "membership-test-trace" };
}

describe("sdkwork-membership-pc-membership service", () => {
  beforeEach(() => {
    configureMembershipServiceMockSession({
      accessToken: "membership-access-token",
      authToken: "membership-auth-token",
    });
  });

  afterEach(() => {
    resetMembershipServiceMockSession();
  });

  it("maps membership-owned catalog and state into the dashboard", async () => {
    const service = createSdkworkMembershipService({
      membershipAppService: createMembershipAppServiceMock({
        memberships: {
          current: {
            retrieve: vi.fn().mockResolvedValue(wrapResourceResponse({
              membershipStatus: "ACTIVE",
              planName: "Pro",
              planRank: 3,
              points: 3200,
            })),
            status: {
              retrieve: vi.fn().mockResolvedValue(wrapResourceResponse({
                isMember: true,
                pointBalance: 2400,
                planRank: 3,
              })),
            },
          },
          benefits: {
            list: vi.fn().mockResolvedValue(wrapListResponse([{
              benefitKey: "priority-rendering",
              claimed: true,
              id: 2,
              name: "Priority rendering",
              type: "quota",
              usageLimit: 10,
              usedCount: 2,
            }])),
          },
          plans: {
            list: vi.fn().mockResolvedValue(wrapListResponse([
              { id: 3, levelValue: 3, name: "Pro", requiredPoints: 500 },
            ])),
          },
          packages: {
            list: vi.fn().mockResolvedValue(wrapListResponse([{
              durationDays: 30,
              id: 2,
              levelName: "Pro",
              name: "Pro Monthly",
              pointAmount: 5000,
              price: 199,
              recommended: true,
            }])),
          },
        },
      }),
    });

    const dashboard = await service.getDashboard();

    expect(dashboard.summary).toMatchObject({
      currentLevelName: "Pro",
      isAuthenticated: true,
      isMember: true,
      points: 3200,
      status: "active",
    });
    expect(dashboard.benefits[0]).toMatchObject({ claimed: true, name: "Priority rendering" });
    expect(dashboard.plans[0]).toMatchObject({
      name: "Pro Monthly",
      packageId: 2,
      recommended: true,
    });
  });

  it("delegates checkout actions and status reads to the host-provided port", async () => {
    const createCheckout = vi.fn().mockImplementation(async (input) => ({
      amountCny: 58,
      durationDays: 30,
      orderId: `membership-order-${input.action}`,
      packageId: input.packageId,
      status: "pending" as const,
    }));
    const getCheckoutStatus = vi.fn().mockResolvedValue({
      amountCny: null,
      durationDays: null,
      orderId: "membership-order-paid",
      packageId: null,
      status: "completed" as const,
    });
    const checkoutPort: SdkworkMembershipCheckoutPort = { createCheckout, getCheckoutStatus };
    const service = createSdkworkMembershipService({
      checkoutPort,
      membershipAppService: createMembershipAppServiceMock(),
    });

    await service.purchaseMembership({ packageId: 2, paymentMethod: "WECHAT" });
    await service.renewMembership({ packageId: 3, paymentMethod: "ALIPAY" });
    await service.upgradeMembership({ packageId: 4 });
    await expect(service.getPurchaseStatus("membership-order-paid")).resolves.toMatchObject({
      orderId: "membership-order-paid",
      status: "completed",
    });

    expect(createCheckout).toHaveBeenNthCalledWith(1, {
      action: "purchase",
      packageId: 2,
      paymentMethod: "WECHAT",
    });
    expect(createCheckout).toHaveBeenNthCalledWith(2, {
      action: "renew",
      packageId: 3,
      paymentMethod: "ALIPAY",
    });
    expect(createCheckout).toHaveBeenNthCalledWith(3, { action: "upgrade", packageId: 4 });
    expect(getCheckoutStatus).toHaveBeenCalledWith("membership-order-paid");
  });

  it("requires authentication and explicit host checkout composition", async () => {
    resetMembershipServiceMockSession();
    const anonymousService = createSdkworkMembershipService({
      checkoutPort: { createCheckout: vi.fn(), getCheckoutStatus: vi.fn() },
      messages: { service: { signInRequired: "Please sign in before managing memberships." } },
    });

    await expect(anonymousService.purchaseMembership({ packageId: 2 })).rejects.toThrow(
      "Please sign in before managing memberships.",
    );

    configureMembershipServiceMockSession({
      accessToken: "membership-access-token",
      authToken: "membership-auth-token",
    });
    await expect(createSdkworkMembershipService().purchaseMembership({ packageId: 2 })).rejects.toThrow(
      "Membership checkout is not configured by the host application.",
    );
  });
});
