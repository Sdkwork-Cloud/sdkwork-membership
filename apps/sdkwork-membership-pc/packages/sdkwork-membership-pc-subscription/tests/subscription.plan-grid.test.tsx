import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import { SdkworkSubscriptionPlanGrid, type SdkworkSubscriptionPackageGroup } from "../src";

const packageGroups: SdkworkSubscriptionPackageGroup[] = [
  {
    id: "membership-package-group-1",
    name: "月度订阅",
    packageGroupId: 1,
    packages: [
      {
        description: "Best for professional creators.",
        durationDays: 30,
        id: "membership-plan-2",
        includedPoints: 5000,
        name: "Pro Monthly",
        originalPriceCny: null,
        packageId: 2,
        priceCny: 199,
        recommended: true,
        tags: ["Popular"],
      },
    ],
    sortWeight: 0,
  },
];

const summary = {
  currentLevelName: "Free",
  currentLevelValue: 1,
  growthValue: 20,
  isAuthenticated: true,
  isMember: false,
  pointBalance: 100,
  remainingDays: null,
  status: "free" as const,
  totalSpent: 0,
  upgradeGrowthValue: 200,
  points: 20,
};

describe("sdkwork-membership-pc-subscription plan grid", () => {
  it("renders the free tier alongside paid plans and selects a plan on click", () => {
    const onSelectPlan = vi.fn();

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <SdkworkSubscriptionPlanGrid
          onSelectPackageGroup={vi.fn()}
          onSelectPlan={onSelectPlan}
          packageGroups={packageGroups}
          selectedPackageGroupId={1}
          selectedPackageId={null}
          summary={summary}
        />
      </SdkworkThemeProvider>,
    );

    expect(
      screen.getByRole("heading", { level: 2, name: /choose a premium package/i }),
    ).toBeInTheDocument();
    expect(screen.getByText(/free membership/i)).toBeInTheDocument();
    expect(screen.getByText("Pro Monthly")).toBeInTheDocument();

    // The recommended paid plan exposes a selection action.
    fireEvent.click(screen.getByRole("button", { name: /activate membership/i }));
    expect(onSelectPlan).toHaveBeenCalledWith(2);
  });
});
