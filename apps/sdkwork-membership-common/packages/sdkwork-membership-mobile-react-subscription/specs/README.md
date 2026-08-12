# SDKWork Membership Mobile Subscription Specs

This directory is the local standards index for `@sdkwork/membership-mobile-react-subscription`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `@sdkwork/membership-mobile-react-subscription` |
| Type | `node-package` |
| Root | `apps/sdkwork-membership-common/packages/sdkwork-membership-mobile-react-subscription` |
| Domain | `commerce` |
| Capability | `membership-mobile-subscription` |
| Languages | `typescript`, `tsx` |
| Status | `ready` |

## Contract Manifest

- `component.spec.json` — machine-readable component contract (exports, ports, verification).
- `package.json` — package identity and public entrypoint (`package.json#exports`).

## Ownership

This package renders mobile membership subscription surfaces (VIP subscription and Token recharge pages). Membership catalog data is owned by `sdkwork-membership`; order creation and payment settlement remain owned by `sdkwork-order` and `sdkwork-payment` per `specs/COMMERCE_ORDER_BOUNDARY_SPEC.md` in the repository root.
