import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";
import {
  configureSdkworkMembershipAppServiceProvider,
  configureSdkworkMembershipSessionTokenProvider,
} from "@sdkwork/membership-service";

afterEach(() => {
  cleanup();
  configureSdkworkMembershipAppServiceProvider(null);
  configureSdkworkMembershipSessionTokenProvider(null);
});
