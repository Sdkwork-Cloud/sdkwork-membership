import { useCallback, useState } from "react";
import { StatusNotice } from "@sdkwork/ui-pc-react";
import { SdkworkSubscriptionCatalogPage } from "./SubscriptionCatalogPage";

function resolveNoticeTone(tone: "error" | "info" | "success"): "danger" | "default" | "success" {
  if (tone === "error") {
    return "danger";
  }
  if (tone === "success") {
    return "success";
  }
  return "default";
}

/**
 * A self-contained subscription catalog screen that can be mounted with zero
 * props.  It wires default host components, notification UI, and no-op
 * callbacks internally, so external hosts (e.g. ClawRouter) can embed it
 * with a single `<SdkworkSubscriptionCatalogScreen />`.
 *
 * The component automatically:
 * - Bootstraps the catalog from the configured SDK service provider
 * - Shows loading and error states with retry
 * - Displays plan cards, tier comparison, and billing cycle tabs
 * - Handles checkout via a built-in confirmation modal
 */
export function SdkworkSubscriptionCatalogScreen() {
  const [notice, setNotice] = useState<{ message: string; tone: "error" | "info" | "success" } | null>(null);

  const handleNotify = useCallback((message: string, tone: "error" | "info" | "success") => {
    setNotice({ message, tone });
  }, []);

  return (
    <>
      {notice ? (
        <div className="px-6 pt-4">
          <StatusNotice tone={resolveNoticeTone(notice.tone)}>
            {notice.message}
          </StatusNotice>
        </div>
      ) : null}
      <SdkworkSubscriptionCatalogPage
        onNotify={handleNotify}
      />
    </>
  );
}
