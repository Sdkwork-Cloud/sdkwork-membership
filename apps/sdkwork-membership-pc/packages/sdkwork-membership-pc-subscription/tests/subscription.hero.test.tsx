import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import { SdkworkSubscriptionHero } from "../src";

const summary = {
  currentLevelName: "Pro",
  currentLevelValue: 3,
  growthValue: 180,
  isAuthenticated: true,
  isMember: true,
  pointBalance: 2400,
  remainingDays: 88,
  status: "active" as const,
  totalSpent: 399,
  upgradeGrowthValue: 500,
  points: 3200,
};

describe("sdkwork-membership-pc-subscription hero", () => {
  it("renders the hero shell with the member level and action switcher", () => {
    const onActionChange = vi.fn();

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <SdkworkSubscriptionHero
          activeAction="upgrade"
          onActionChange={onActionChange}
          summary={summary}
        />
      </SdkworkThemeProvider>,
    );

    // The hero surfaces the member's current level name in the heading.
    const heading = screen.getByRole("heading", { level: 1 });
    expect(heading.textContent).toContain("Pro");

    // Members see renew and upgrade actions only.
    const labels = screen
      .getAllByRole("button")
      .map((button) => button.textContent);
    expect(labels).toEqual(["Renew", "Upgrade"]);

    // Selecting an action propagates the change to the parent.
    fireEvent.click(screen.getByRole("button", { name: "Renew" }));
    expect(onActionChange).toHaveBeenCalledWith("renew");
  });

  it("shows purchase only for guests", () => {
    render(
      <SdkworkThemeProvider defaultTheme="light">
        <SdkworkSubscriptionHero
          activeAction="purchase"
          onActionChange={vi.fn()}
          summary={{
            ...summary,
            currentLevelName: "Free",
            currentLevelValue: 0,
            isMember: false,
            remainingDays: null,
            totalSpent: 0,
          }}
        />
      </SdkworkThemeProvider>,
    );

    const labels = screen
      .getAllByRole("button")
      .map((button) => button.textContent);
    expect(labels).toEqual(["Purchase"]);
  });
});
