import { describe, expect, it, vi } from "vitest";

import { createSdkworkMembershipBackendService } from "../src/backend";

function createClient(overrides: Record<string, unknown> = {}) {
  return {
    memberships: {
      plans: {
        list: vi.fn(),
        create: vi.fn(),
        update: vi.fn(),
        delete: vi.fn(),
      },
      packageGroups: {
        list: vi.fn(),
        create: vi.fn(),
        update: vi.fn(),
        delete: vi.fn(),
      },
      packages: {
        list: vi.fn(),
        create: vi.fn(),
        update: vi.fn(),
        delete: vi.fn(),
      },
      members: {
        list: vi.fn(),
        retrieve: vi.fn(),
        status: { update: vi.fn() },
      },
      entitlements: { list: vi.fn() },
      ...overrides,
    },
  };
}

describe("createSdkworkMembershipBackendService", () => {
  it("normalizes filters and preserves server pagination", async () => {
    const client = createClient();
    client.memberships.members.list.mockResolvedValue({
      items: [{ id: "membership-1", ownerUserId: "42" }],
      pageInfo: {
        mode: "offset",
        page: 2,
        pageSize: 50,
        totalItems: "51",
        totalPages: 2,
      },
    });
    const service = createSdkworkMembershipBackendService(client as never);

    const page = await service.listMembers({
      page: 2,
      pageSize: 50,
      planId: " plan-pro ",
      status: " active ",
      userId: " 42 ",
    });

    expect(client.memberships.members.list).toHaveBeenCalledWith({
      page: 2,
      pageSize: 50,
      planId: "plan-pro",
      status: "active",
      userId: "42",
    });
    expect(page).toEqual({
      items: [{ id: "membership-1", ownerUserId: "42" }],
      pageInfo: {
        mode: "offset",
        page: 2,
        pageSize: 50,
        totalItems: 51,
        totalPages: 2,
        nextCursor: undefined,
        hasMore: undefined,
      },
    });
  });

  it("caps list page size at the platform maximum", async () => {
    const client = createClient();
    client.memberships.plans.list.mockResolvedValue({
      items: [],
      pageInfo: { mode: "offset", page: 1, pageSize: 200, totalItems: 0 },
    });
    const service = createSdkworkMembershipBackendService(client as never);

    await service.listPlans({ page: -1, pageSize: 1000 });

    expect(client.memberships.plans.list).toHaveBeenCalledWith({
      page: 1,
      pageSize: 200,
      status: undefined,
    });
  });

  it("delegates catalog mutations and member status updates", async () => {
    const client = createClient();
    client.memberships.plans.create.mockResolvedValue({ id: "plan-1" });
    client.memberships.members.status.update.mockResolvedValue({ id: "membership-1", status: "active" });
    client.memberships.members.retrieve.mockResolvedValue({ id: "membership-1", status: "active" });
    const service = createSdkworkMembershipBackendService(client as never);
    const plan = { code: "pro", name: "Pro", rank: "1", status: "active" };

    await service.createPlan(plan);
    await service.getMember("membership-1");
    await service.updateMemberStatus("membership-1", { status: "active" });

    expect(client.memberships.plans.create).toHaveBeenCalledWith(plan);
    expect(client.memberships.members.status.update).toHaveBeenCalledWith(
      "membership-1",
      { status: "active" },
    );
    expect(client.memberships.members.retrieve).toHaveBeenCalledWith("membership-1");
  });

  it("fails closed when a backend list response is malformed", async () => {
    const client = createClient();
    client.memberships.entitlements.list.mockResolvedValue({ items: [] });
    const service = createSdkworkMembershipBackendService(client as never);

    await expect(service.listEntitlements()).rejects.toThrow("Invalid membership backend pageInfo");
  });

  it("does not expose service-only fulfillment", () => {
    const service = createSdkworkMembershipBackendService(createClient() as never);

    expect(service).not.toHaveProperty("fulfillMembershipPurchase");
  });
});
