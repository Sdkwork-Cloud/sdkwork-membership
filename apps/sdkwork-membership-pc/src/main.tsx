import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { MembershipAppShell } from "@sdkwork/membership-pc-shell";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <MembershipAppShell />
  </StrictMode>,
);
