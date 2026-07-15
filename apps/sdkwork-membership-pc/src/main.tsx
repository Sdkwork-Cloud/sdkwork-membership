import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { MembershipAppShell } from "@sdkwork/membership-pc-shell";
import "@sdkwork/ui-pc-react/styles.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <MembershipAppShell />
  </StrictMode>,
);
