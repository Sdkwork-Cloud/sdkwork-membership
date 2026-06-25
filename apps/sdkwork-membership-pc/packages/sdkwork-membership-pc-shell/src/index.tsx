import { sdkworkMembershipPcRuntimeIdentity } from "@sdkwork/membership-pc-core";

export function MembershipAppShell() {
  return (
    <main className="membership-shell">
      <section className="membership-card">
        <h1>SDKWork Membership</h1>
        <p>{sdkworkMembershipPcRuntimeIdentity.appKey}</p>
        <p>Membership tiers, entitlements, and upgrade flows capability PC surface — aligned with sdkwork-specs building-block model.</p>
      </section>
    </main>
  );
}
