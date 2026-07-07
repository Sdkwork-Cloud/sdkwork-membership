import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  configureMembershipServiceMockSession,
  configureOrderServiceMock,
  createMembershipAppServiceMock,
  createOrderAppServiceMock,
  resetMembershipServiceMockSession,
} from "../../../tests/test-utils/membership-service-mock";
import { createSdkworkMembershipService } from "../src";

const proLevelIcon = {
  kind: "image",
  publicUrl: "https://cdn.sdkwork.ai/memberships/pro.png",
  source: "external_url",
  url: "https://cdn.sdkwork.ai/memberships/pro.png",
} as const;

function wrapSdkworkMembershipListResponse<T>(items: T[]) {
  return {
    code: 0,
    data: {
      items,
      pageInfo: {
        hasMore: false,
        mode: "offset" as const,
        page: 1,
        pageSize: 20,
      },
    },
    traceId: "membership-test-trace",
  };
}

function wrapSdkworkMembershipResourceResponse<T>(data: T) {
  return {
    code: 0,
    data,
    traceId: "membership-test-trace",
  };
}

describe("sdkwork-membership-pc-membership service", () => {
  beforeEach(() => {
    configureMembershipServiceMockSession({ authToken: "membership-auth-token" });
  });

  afterEach(() => {
    resetMembershipServiceMockSession();
  });

  it("maps membership plans, benefits, and packages into a reusable membership dashboard", async () => {
    const membershipAppService = createMembershipAppServiceMock({
      memberships: {
        current: {
          retrieve: vi.fn().mockResolvedValue(
            wrapSdkworkMembershipResourceResponse({
              expireTime: "2026-06-30T00:00:00.000Z",
              growthValue: 180,
              remainingDays: 88,
              totalSpent: 399,
              upgradeGrowthValue: 500,
              planRank: 3,
              planName: "Pro",
              points: 3200,
              membershipStatus: "ACTIVE",
            }),
          ),
          status: {
            retrieve: vi.fn().mockResolvedValue(
              wrapSdkworkMembershipResourceResponse({
                isMember: true,
                pointBalance: 2400,
                planRank: 3,
              }),
            ),
          },
        },
        benefits: {
          list: vi.fn().mockResolvedValue(
            wrapSdkworkMembershipListResponse([
              {
                benefitKey: "priority-rendering",
                claimed: true,
                description: "Jump the queue for premium workloads.",
                id: 2,
                name: "Priority rendering",
                type: "quota",
                usageLimit: 10,
                usedCount: 2,
              },
              {
                benefitKey: "membership-support",
                claimed: false,
                description: "Priority support responses.",
                id: 1,
                name: "Priority support",
                type: "service",
                usageLimit: 1,
                usedCount: 0,
              },
            ]),
          ),
        },
        plans: {
          list: vi.fn().mockResolvedValue(
            wrapSdkworkMembershipListResponse([
              {
                description: "Entry tier",
                id: 1,
                levelValue: 1,
                name: "Free",
                requiredPoints: 0,
              },
              {
                description: "Professional tier",
                icon: proLevelIcon,
                id: 3,
                levelValue: 3,
                name: "Pro",
                requiredPoints: 500,
              },
              {
                description: "Growing teams",
                id: 2,
                levelValue: 2,
                name: "Plus",
                requiredPoints: 200,
              },
            ]),
          ),
        },
        packages: {
          list: vi.fn().mockResolvedValue(
            wrapSdkworkMembershipListResponse([
              {
                description: "Best for teams",
                id: 2,
                levelName: "Pro",
                name: "Pro Monthly",
                originalPrice: 249,
                pointAmount: 5000,
                price: 199,
                recommended: true,
                sortWeight: 20,
                tags: ["Popular"],
                durationDays: 30,
              },
              {
                description: "Long-running annual plan",
                id: 3,
                levelName: "Pro",
                name: "Pro Annual",
                originalPrice: 899,
                pointAmount: 60000,
                price: 699,
                recommended: false,
                sortWeight: 15,
                tags: ["Annual"],
                durationDays: 365,
              },
            ]),
          ),
        },
      },
    });

    const service = createSdkworkMembershipService({
      membershipAppService,
    });

    const dashboard = await service.getDashboard();

    expect(dashboard.summary).toMatchObject({
      currentLevelName: "Pro",
      isAuthenticated: true,
      isMember: true,
      points: 3200,
      remainingDays: 88,
      status: "active",
    });
    expect(
      dashboard.levels.map((level) => ({
        isCurrent: level.isCurrent,
        name: level.name,
      })),
    ).toEqual([
      { isCurrent: false, name: "Free" },
      { isCurrent: false, name: "Plus" },
      { isCurrent: true, name: "Pro" },
    ]);
    expect(dashboard.levels[2].icon).toEqual(proLevelIcon);
    expect(dashboard.benefits[0]).toMatchObject({
      claimed: true,
      id: "membership-benefit-2",
      name: "Priority rendering",
      usedCount: 2,
    });
    expect(dashboard.plans[0]).toMatchObject({
      name: "Pro Monthly",
      packageId: 2,
      recommended: true,
    });
  });

  it("returns a guest-safe membership dashboard with public package plans when the wallet overview is anonymous", async () => {
    resetMembershipServiceMockSession();
    const packagesList = vi.fn().mockResolvedValue(
      wrapSdkworkMembershipListResponse([
        {
          description: "Starter public plan",
          id: 1,
          levelName: "Plus",
          name: "Plus Monthly",
          originalPrice: 99,
          pointAmount: 1200,
          price: 39,
          recommended: true,
          tags: ["Starter"],
          durationDays: 30,
        },
      ]),
    );
    const service = createSdkworkMembershipService({
      membershipAppService: createMembershipAppServiceMock({
        memberships: {
          packages: {
            list: packagesList,
          },
        },
      }),
    });

    const dashboard = await service.getDashboard();

    expect(dashboard.summary.isAuthenticated).toBe(false);
    expect(dashboard.summary.status).toBe("guest");
    expect(dashboard.levels).toEqual([]);
    expect(dashboard.benefits).toEqual([]);
    expect(dashboard.plans).toEqual([
      expect.objectContaining({
        name: "Plus Monthly",
        packageId: 1,
        recommended: true,
      }),
    ]);
    expect(packagesList).toHaveBeenCalledTimes(1);
  });

  it("purchases, renews, and upgrades membership through the generated SDK boundary", async () => {
    const purchase = vi.fn().mockResolvedValue(
      wrapSdkworkMembershipResourceResponse({
        amount: 199,
        durationDays: 30,
        orderId: "order-uuid-purchase",
        packageId: 2,
        packageName: "Pro Monthly",
        status: "pending",
        targetPlanName: "Pro",
      }),
    );
    const renew = vi.fn().mockResolvedValue(
      wrapSdkworkMembershipResourceResponse({
        amount: 399,
        durationDays: 365,
        orderId: "order-uuid-renew",
        packageId: 3,
        packageName: "Pro Annual",
        status: "pending",
        targetPlanName: "Pro",
      }),
    );
    const upgrade = vi.fn().mockResolvedValue(
      wrapSdkworkMembershipResourceResponse({
        amount: 199,
        durationDays: 30,
        orderId: "order-uuid-upgrade",
        packageId: 2,
        packageName: "Pro Monthly",
        status: "pending",
        targetPlanName: "Pro",
      }),
    );
    const orderCreateResponses = [
      { orderId: "order-uuid-purchase", orderNo: "MEMBERSHIP-ORDER-001" },
      { orderId: "order-uuid-upgrade", orderNo: "MEMBERSHIP-ORDER-002" },
      { orderId: "order-uuid-renew", orderNo: "MEMBERSHIP-ORDER-003" },
    ];
    let orderCreateCallIndex = 0;
    configureOrderServiceMock(createOrderAppServiceMock({
      memberships: {
        orders: {
          create: vi.fn().mockImplementation(async () =>
            wrapSdkworkMembershipResourceResponse(
              orderCreateResponses[orderCreateCallIndex++] ?? orderCreateResponses[0],
            ),
          ),
        },
      },
      orders: {
        pay: vi.fn().mockResolvedValue(
          wrapSdkworkMembershipResourceResponse({
            paymentParams: {
              cashierUrl: "https://im.sdkwork.com/cashier?scene=virtual&orderId=MEMBERSHIP-ORDER-001",
            },
          }),
        ),
      },
    }));
    const membershipAppService = createMembershipAppServiceMock({
      memberships: {
        purchases: {
          create: purchase,
          renew,
          upgrade,
        },
      },
    });
    const service = createSdkworkMembershipService({
      membershipAppService,
    });

    await expect(
      service.purchaseMembership({
        packageId: 2,
        paymentMethod: "WECHAT",
      }),
    ).resolves.toMatchObject({
      amountCny: 199,
      orderId: "order-uuid-purchase",
      packageId: 2,
      status: "pending",
    });

    await expect(
      service.upgradeMembership({
        packageId: 2,
        paymentMethod: "WECHAT",
      }),
    ).resolves.toMatchObject({
      amountCny: 199,
      orderId: "order-uuid-upgrade",
      packageId: 2,
      status: "pending",
    });

    await expect(
      service.renewMembership({
        packageId: 3,
        paymentMethod: "ALIPAY",
      }),
    ).resolves.toMatchObject({
      amountCny: 399,
      orderId: "order-uuid-renew",
      packageId: 3,
      status: "pending",
    });

    expect(upgrade).toHaveBeenCalledWith({
      couponId: undefined,
      orderId: "order-uuid-upgrade",
      packageId: 2,
      paymentMethod: "wechat_pay",
      requestNo: "MEMBERSHIP-ORDER-002",
    });
    expect(purchase).toHaveBeenCalledWith({
      couponId: undefined,
      orderId: "order-uuid-purchase",
      packageId: 2,
      paymentMethod: "wechat_pay",
      requestNo: "MEMBERSHIP-ORDER-001",
    });
    expect(renew).toHaveBeenCalledWith({
      couponId: undefined,
      orderId: "order-uuid-renew",
      packageId: 3,
      paymentMethod: "alipay",
      requestNo: "MEMBERSHIP-ORDER-003",
    });
  });

  it("preserves SDK qrCodePayload when the order pay response has no cashier URL", async () => {
    configureOrderServiceMock(createOrderAppServiceMock({
      memberships: {
        orders: {
          create: vi.fn().mockResolvedValue(
            wrapSdkworkMembershipResourceResponse({
              orderId: "order-uuid-qr",
              orderNo: "MEMBERSHIP-ORDER-QR",
            }),
          ),
        },
      },
      orders: {
        pay: vi.fn().mockResolvedValue(
          wrapSdkworkMembershipResourceResponse({
            paymentParams: {},
          }),
        ),
      },
    }));
    const membershipAppService = createMembershipAppServiceMock({
      memberships: {
        purchases: {
          create: vi.fn().mockResolvedValue(
            wrapSdkworkMembershipResourceResponse({
              amount: "68",
              durationDays: 30,
              orderId: "order-uuid-qr",
              packageId: 201,
              packageName: "Monthly Membership",
              qrCodePayload: "weixin://wxpay/bizpayurl?pr=membership",
              status: "pending",
              success: true,
              targetPlanName: "Basic",
            }),
          ),
        },
      },
    });
    const service = createSdkworkMembershipService({
      membershipAppService,
    });

    await expect(
      service.purchaseMembership({
        packageId: 201,
        paymentMethod: "wechat_pay",
      }),
    ).resolves.toMatchObject({
      amountCny: 68,
      orderId: "order-uuid-qr",
      qrCode: "weixin://wxpay/bizpayurl?pr=membership",
      status: "pending",
    });
  });

  it("localizes membership auth and mutation fallback errors at the membership boundary", async () => {
    resetMembershipServiceMockSession();
    const localizedAuthService = createSdkworkMembershipService({
      locale: "zh-CN",
      messages: {
        service: {
          signInRequired: "Please sign in before managing memberships.",
        },
      },
    });

    await expect(
      localizedAuthService.purchaseMembership({
        packageId: 2,
      }),
    ).rejects.toThrow("Please sign in before managing memberships.");

    configureMembershipServiceMockSession({ authToken: "membership-auth-token" });
    configureOrderServiceMock(createOrderAppServiceMock({
      memberships: {
        orders: {
          create: vi.fn().mockResolvedValue(
            wrapSdkworkMembershipResourceResponse({
              orderId: "order-uuid-purchase",
              orderNo: "MEMBERSHIP-ORDER-001",
            }),
          ),
        },
      },
      orders: {
        pay: vi.fn().mockResolvedValue(
          wrapSdkworkMembershipResourceResponse({
            paymentParams: { cashierUrl: "https://example.test/cashier" },
          }),
        ),
      },
    }));
    const localizedMutationService = createSdkworkMembershipService({
      membershipAppService: createMembershipAppServiceMock({
        memberships: {
          purchases: {
            create: vi.fn().mockResolvedValue({
              code: 50001,
              detail: "Membership purchase failed.",
              traceId: "membership-test-trace",
            }),
          },
        },
      }),
      locale: "zh-CN",
      messages: {
        service: {
          purchaseFailed: "Membership purchase failed.",
        },
      },
    });

    await expect(
      localizedMutationService.purchaseMembership({
        packageId: 2,
      }),
    ).rejects.toThrow("Membership purchase failed.");
  });
});
