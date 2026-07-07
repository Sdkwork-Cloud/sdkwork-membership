import { useCallback, useState } from "react";
import { StatusNotice } from "@sdkwork/ui-pc-react";
import { sdkworkSubscriptionCatalogHostComponents } from "../components/subscription-catalog-host-components";
import { SdkworkSubscriptionCatalogPage } from "./SubscriptionCatalogPage";

function CatalogNotifyOutlet() {
  return null;
}

function resolveNoticeTone(tone: "error" | "info" | "success"): "danger" | "default" | "success" {
  if (tone === "error") {
    return "danger";
  }
  if (tone === "success") {
    return "success";
  }
  return "default";
}

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
        components={sdkworkSubscriptionCatalogHostComponents}
        notifyOutlet={CatalogNotifyOutlet}
        onMembershipTierUpdated={() => undefined}
        onNotify={handleNotify}
      />
    </>
  );
}
