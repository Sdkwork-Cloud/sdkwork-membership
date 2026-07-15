import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { configureSdkworkMembershipAppServiceProvider } from "@sdkwork/membership-service";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import {
  createSdkworkMembershipController,
  SdkworkTokenPlanHeaderEntry,
} from "../src";
import {
  configureMembershipServiceMockSession,
  createMembershipAppServiceMock,
  resetMembershipServiceMockSession,
} from "../../../tests/test-utils/membership-service-mock";

function createDashboard() {
  return {
    benefits: [],
    levels: [],
    plans: [
      {
        description: "Best for teams",
        durationDays: 30,
        id: "membership-package-2",
        includedPoints: 5000,
        name: "Pro Monthly",
        originalPriceCny: null,
        packageId: 2,
        priceCny: 199,
        recommended: true,
        tags: ["Popular"],
      },
    ],
    summary: {
      currentLevelName: "Free",
      currentLevelValue: null,
      growthValue: null,
      isAuthenticated: true,
      isMember: false,
      pointBalance: 100,
      remainingDays: null,
      status: "free" as const,
      totalSpent: 0,
      upgradeGrowthValue: 200,
      points: 0,
    },
  };
}

function createEmptyDashboard() {
  return {
    benefits: [],
    levels: [],
    plans: [],
    summary: {
      currentLevelName: "Guest",
      currentLevelValue: null,
      growthValue: null,
      isAuthenticated: false,
      isMember: false,
      pointBalance: null,
      remainingDays: null,
      status: "guest" as const,
      totalSpent: null,
      upgradeGrowthValue: null,
      points: null,
    },
  };
}

describe("sdkwork-membership-pc-membership token plan header entry", () => {
  afterEach(() => {
    cleanup();
    configureSdkworkMembershipAppServiceProvider(null);
    resetMembershipServiceMockSession();
  });

  it("opens the token plan menu and routes selected packages to checkout", async () => {
    configureMembershipServiceMockSession({ accessToken: "membership-header-access-token", authToken: "membership-header-auth-token" });
    configureSdkworkMembershipAppServiceProvider(() => createMembershipAppServiceMock());
    const controller = createSdkworkMembershipController({
      service: {
        getDashboard: vi.fn().mockResolvedValue(createDashboard()),
        getEmptyDashboard: vi.fn().mockReturnValue(createEmptyDashboard()),
      },
    });
    const onNavigate = vi.fn();

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <SdkworkTokenPlanHeaderEntry
          checkoutBasePath="/checkout"
          controller={controller}
          onNavigate={onNavigate}
        />
      </SdkworkThemeProvider>,
    );

    fireEvent.click(await screen.findByRole("button", { name: /open token plan menu/i }));

    expect(await screen.findByText("Token plans")).toBeInTheDocument();
    expect(screen.getByText("Pro Monthly")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /continue to checkout/i }));

    expect(onNavigate).toHaveBeenCalledWith(
      "/checkout?kind=subscription&mode=purchase&packageId=2&sourceId=membership-plan-membership-package-2",
    );
  });

  it("routes to the membership center when requested", async () => {
    configureMembershipServiceMockSession({ accessToken: "membership-header-access-token", authToken: "membership-header-auth-token" });
    configureSdkworkMembershipAppServiceProvider(() => createMembershipAppServiceMock());
    const controller = createSdkworkMembershipController({
      service: {
        getDashboard: vi.fn().mockResolvedValue(createDashboard()),
        getEmptyDashboard: vi.fn().mockReturnValue(createEmptyDashboard()),
      },
    });
    const onOpenCenter = vi.fn();

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <SdkworkTokenPlanHeaderEntry controller={controller} onOpenCenter={onOpenCenter} />
      </SdkworkThemeProvider>,
    );

    fireEvent.click(await screen.findByRole("button", { name: /open token plan menu/i }));
    fireEvent.click(await screen.findByRole("button", { name: /open membership center/i }));

    expect(onOpenCenter).toHaveBeenCalledTimes(1);
  });
});
