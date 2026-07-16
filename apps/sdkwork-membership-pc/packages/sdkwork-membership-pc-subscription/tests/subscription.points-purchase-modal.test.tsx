import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { configureSdkworkMembershipSessionTokenProvider } from "@sdkwork/membership-service";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import { SdkworkSubscriptionCatalogPage } from "../src/pages/SubscriptionCatalogPage";
import { createSdkworkSubscriptionCatalogController } from "../src/subscription-catalog-controller";
import type {
  SdkworkSubscriptionCatalogHostComponents,
  SdkworkSubscriptionCatalogModalProps,
} from "../src/subscription-catalog-host";

const emptyCatalog = {
  benefitsByRank: {},
  billingCycles: [],
  comparisonCategories: [],
  memberSummary: null,
  packageGroupIds: [],
  packageGroups: [],
  plans: [],
};

function PointsPurchaseModal({ isOpen }: SdkworkSubscriptionCatalogModalProps) {
  return isOpen ? <div>points purchase open</div> : null;
}

const hostComponents: SdkworkSubscriptionCatalogHostComponents = {
  checkoutModal: () => null,
  pointsDetailsModal: () => null,
  pointsPurchaseModal: PointsPurchaseModal,
  redeemModal: () => null,
};

afterEach(() => {
  configureSdkworkMembershipSessionTokenProvider(null);
});

describe("subscription catalog points purchase", () => {
  it("opens the Order-owned recharge modal without redirecting anonymous users", async () => {
    configureSdkworkMembershipSessionTokenProvider(() => ({}));
    const onLoginRequired = vi.fn();
    const controller = createSdkworkSubscriptionCatalogController({
      service: {
        getCatalog: vi.fn().mockResolvedValue(emptyCatalog),
      },
    });
    await controller.bootstrap();

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <SdkworkSubscriptionCatalogPage
          catalogController={controller}
          components={hostComponents}
          onLoginRequired={onLoginRequired}
        />
      </SdkworkThemeProvider>,
    );

    fireEvent.click(screen.getByRole("button", { name: /购买算力元|buy compute credits/i }));

    expect(screen.getByText("points purchase open")).toBeInTheDocument();
    expect(onLoginRequired).not.toHaveBeenCalled();
  });
});
