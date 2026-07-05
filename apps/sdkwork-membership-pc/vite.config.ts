import path from "node:path";
import { fileURLToPath } from "node:url";
import react from "@vitejs/plugin-react";
import { defineConfig, loadEnv } from "vite";

const appRoot = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(appRoot, "../..");
const membershipAppSdkEntry = path.resolve(
  workspaceRoot,
  "sdks/sdkwork-membership-app-sdk/sdkwork-membership-app-sdk-typescript/src/index.ts",
);
const DEFAULT_GATEWAY_TARGET = "http://127.0.0.1:18096";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, appRoot, "");
  const proxyTarget = env.VITE_SDKWORK_MEMBERSHIP_PC_APP_API_BASE_URL?.trim()
    || env.VITE_SDKWORK_MEMBERSHIP_PC_SDK_BASE_URL?.trim()
    || DEFAULT_GATEWAY_TARGET;

  return {
    define: {
      "process.env.SDKWORK_ACCESS_TOKEN": JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ""),
      "process.env.SDKWORK_AUTH_TOKEN": JSON.stringify(env.SDKWORK_AUTH_TOKEN ?? ""),
    },
    plugins: [react()],
    root: appRoot,
    resolve: {
      alias: {
        "@sdkwork/membership-app-sdk": membershipAppSdkEntry,
        "@sdkwork/membership-pc-core": path.resolve(appRoot, "packages/sdkwork-membership-pc-core/src/index.ts"),
        "@sdkwork/membership-pc-shell": path.resolve(appRoot, "packages/sdkwork-membership-pc-shell/src/index.tsx"),
        "@sdkwork/membership-pc-membership": path.resolve(appRoot, "packages/sdkwork-membership-pc-membership/src/index.ts"),
        "@sdkwork/membership-pc-subscription": path.resolve(appRoot, "packages/sdkwork-membership-pc-subscription/src/index.ts"),
        "@sdkwork/membership-service": path.resolve(workspaceRoot, "apps/sdkwork-membership-common/packages/sdkwork-membership-service/src/index.ts"),
        "@sdkwork/membership-sdk-ports": path.resolve(workspaceRoot, "apps/sdkwork-membership-common/packages/sdkwork-membership-sdk-ports/src/index.ts"),
        "@sdkwork/ui-pc-react": path.resolve(workspaceRoot, "../sdkwork-ui/sdkwork-ui-pc-react/src/index.ts"),
        "@sdkwork/ui-pc-react/theme": path.resolve(workspaceRoot, "../sdkwork-ui/sdkwork-ui-pc-react/src/theme/index.ts"),
        "@sdkwork/ui-pc-react/components/ui/button": path.resolve(workspaceRoot, "../sdkwork-ui/sdkwork-ui-pc-react/src/components/ui/button.tsx"),
        "@sdkwork/ui-pc-react/components/ui/feedback/states": path.resolve(workspaceRoot, "../sdkwork-ui/sdkwork-ui-pc-react/src/components/ui/feedback/states.tsx"),
        "@sdkwork/utils": path.resolve(workspaceRoot, "../sdkwork-utils/packages/sdkwork-utils-typescript/src/index.ts"),
        "@sdkwork/promotion-pc-coupon": path.resolve(workspaceRoot, "../sdkwork-promotion/apps/sdkwork-promotion-pc/packages/sdkwork-promotion-pc-coupon/src/index.ts"),
        "@sdkwork/promotion-service": path.resolve(workspaceRoot, "../sdkwork-promotion/apps/sdkwork-promotion-common/packages/sdkwork-promotion-service/src/index.ts"),
        "@sdkwork/payment-pc-payment": path.resolve(workspaceRoot, "../sdkwork-payment/apps/sdkwork-payment-pc/packages/sdkwork-payment-pc-payment/src/index.ts"),
        "@sdkwork/payment-service": path.resolve(workspaceRoot, "../sdkwork-payment/apps/sdkwork-payment-common/packages/sdkwork-payment-service/src/index.ts"),
        "@sdkwork/appbase-pc-react": path.resolve(workspaceRoot, "../sdkwork-appbase/packages/pc-react/foundation/sdkwork-appbase-pc-react/src/index.ts"),
      },
    },
    build: {
      outDir: "dist",
      sourcemap: mode !== "production",
    },
    server: {
      host: "127.0.0.1",
      port: 5186,
      proxy: {
        "/app/v3/api": {
          changeOrigin: true,
          target: proxyTarget,
        },
      },
    },
  };
});
