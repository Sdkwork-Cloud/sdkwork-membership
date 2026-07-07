import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  configureMembershipServiceMockSession,
  createMembershipAppServiceMock,
  resetMembershipServiceMockSession,
} from "../../../tests/test-utils/membership-service-mock";
import { createSdkworkSubscriptionCatalogService } from "../src/subscription-catalog-service";

describe("sdkwork-membership-pc-subscription catalog service", () => {
  beforeEach(() => {
    configureMembershipServiceMockSession({ authToken: "catalog-auth-token" });
  });

  afterEach(() => {
    resetMembershipServiceMockSession();
  });

  it("maps package groups into billing cycles and plan cards", async () => {
    const membershipAppService = createMembershipAppServiceMock({
      memberships: {
        packageGroups: {
          list: vi.fn().mockResolvedValue({
            code: 0,
            data: {
              items: [
                {
                  description: "6折",
                  id: 2,
                  name: "连续包月",
                  packages: [
                    {
                      description: "连续包月，到期自动续费",
                      durationDays: 30,
                      id: 201,
                      name: "基础会员·月卡",
                      originalPrice: "73",
                      planName: "基础会员",
                      pointAmount: 725,
                      price: "68",
                      recommended: false,
                      sortWeight: 1,
                      tags: ["首月60"],
                    },
                  ],
                  sortWeight: 2,
                },
              ],
              pageInfo: { mode: "offset", page: 1, pageSize: 200, total: 1 },
            },
            traceId: "trace-catalog-1",
          }),
        },
        plans: {
          list: vi.fn().mockResolvedValue({
            code: 0,
            data: {
              items: [
                { id: 1, name: "基础会员", rank: 1 },
                { id: 2, name: "标准会员", rank: 2 },
                { id: 3, name: "高级会员", rank: 3 },
              ],
              pageInfo: { mode: "offset", page: 1, pageSize: 200, total: 3 },
            },
            traceId: "trace-catalog-2",
          }),
        },
        benefits: {
          list: vi.fn().mockResolvedValue({
            code: 0,
            data: {
              items: [],
              pageInfo: { mode: "offset", page: 1, pageSize: 200, total: 0 },
            },
            traceId: "trace-catalog-3",
          }),
        },
        current: {
          status: {
            retrieve: vi.fn().mockResolvedValue({
              code: 0,
              data: { active: false, planRank: 0 },
              traceId: "trace-catalog-4",
            }),
          },
        },
      },
    });

    const service = createSdkworkSubscriptionCatalogService({
      membershipAppService,
      translate: (_, defaultValue) => defaultValue ?? "",
    });

    const catalog = await service.getCatalog();
    const viewModel = service.getViewModel(catalog, 0);

    expect(viewModel.billingCycles[0]?.label).toBe("连续包月");
    expect(viewModel.planCards[0]?.id).toBe("201");
    expect(viewModel.planCards[0]?.priceLabel).toBe("68");
    expect(viewModel.tierColumns[0]?.packageNumericId).toBe(201);
    expect(viewModel.planCards.at(-1)?.disabled).toBe(true);
  });
});
