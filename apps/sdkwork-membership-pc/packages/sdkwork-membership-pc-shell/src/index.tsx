import { useEffect, useMemo } from "react";
import {
  BrowserRouter,
  Link,
  Navigate,
  Route,
  Routes,
  useNavigate,
} from "react-router-dom";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import { bootstrapSdkworkMembershipAppService, bootstrapSdkworkOrderAppService } from "@sdkwork/membership-service";
import { SdkworkMembershipPage } from "@sdkwork/membership-pc-membership";
import { SdkworkSubscriptionCatalogScreen, SdkworkSubscriptionPage } from "@sdkwork/membership-pc-subscription";
import { sdkworkMembershipPcRuntimeIdentity } from "@sdkwork/membership-pc-core";

const env = (import.meta as ImportMeta & { env?: Record<string, string | undefined> }).env;

function resolveMembershipApiBaseUrl(): string {
  const configured = env?.VITE_SDKWORK_MEMBERSHIP_PC_APP_API_BASE_URL?.trim();
  if (configured) {
    return configured;
  }
  return typeof window !== "undefined" ? window.location.origin : "http://127.0.0.1:5186";
}

function MembershipRoutes() {
  const navigate = useNavigate();

  return (
    <Routes>
      <Route
        path="/"
        element={(
          <Navigate replace to="/membership" />
        )}
      />
      <Route
        path="/membership"
        element={(
          <SdkworkMembershipPage
            checkoutBasePath="/subscription/checkout"
            onNavigate={(route) => navigate(route)}
          />
        )}
      />
      <Route
        path="/subscription/catalog"
        element={<SdkworkSubscriptionCatalogScreen />}
      />
      <Route
        path="/subscription/checkout"
        element={<SdkworkSubscriptionPage />}
      />
      <Route
        path="*"
        element={(
          <Navigate replace to="/membership" />
        )}
      />
    </Routes>
  );
}

export function MembershipAppShell() {
  const apiBaseUrl = useMemo(() => resolveMembershipApiBaseUrl(), []);

  useEffect(() => {
    bootstrapSdkworkMembershipAppService({
      accessToken: env?.SDKWORK_ACCESS_TOKEN || env?.VITE_SDKWORK_ACCESS_TOKEN,
      authToken: env?.SDKWORK_AUTH_TOKEN || env?.VITE_SDKWORK_AUTH_TOKEN,
      baseUrl: apiBaseUrl,
    });
    bootstrapSdkworkOrderAppService({
      accessToken: env?.SDKWORK_ACCESS_TOKEN || env?.VITE_SDKWORK_ACCESS_TOKEN,
      authToken: env?.SDKWORK_AUTH_TOKEN || env?.VITE_SDKWORK_AUTH_TOKEN,
      baseUrl: apiBaseUrl,
    });
  }, [apiBaseUrl]);

  return (
    <SdkworkThemeProvider defaultTheme="light">
      <BrowserRouter>
        <div className="min-h-screen bg-[var(--sdk-color-surface-base)] text-[var(--sdk-color-text-primary)]">
          <header className="border-b border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)] px-6 py-4">
            <div className="mx-auto flex max-w-[1280px] items-center justify-between gap-4">
              <div>
                <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
                  SDKWork Membership
                </div>
                <h1 className="mt-1 text-lg font-semibold">
                  {sdkworkMembershipPcRuntimeIdentity.appKey}
                </h1>
              </div>
              <nav className="flex items-center gap-3 text-sm">
                <Link className="text-[var(--sdk-color-text-secondary)] hover:text-[var(--sdk-color-text-primary)]" to="/membership">
                  Membership
                </Link>
                <Link className="text-[var(--sdk-color-text-secondary)] hover:text-[var(--sdk-color-text-primary)]" to="/subscription/catalog">
                  Catalog
                </Link>
                <Link className="text-[var(--sdk-color-text-secondary)] hover:text-[var(--sdk-color-text-primary)]" to="/subscription/checkout">
                  Checkout
                </Link>
              </nav>
            </div>
          </header>
          <main className="min-h-[calc(100vh-5rem)]">
            <MembershipRoutes />
          </main>
        </div>
      </BrowserRouter>
    </SdkworkThemeProvider>
  );
}
