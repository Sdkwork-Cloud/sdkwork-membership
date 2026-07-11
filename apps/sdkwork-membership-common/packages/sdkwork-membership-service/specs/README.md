# SDKWork Membership Service Specs

This directory is the local standards index for `@sdkwork/membership-service`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `@sdkwork/membership-service` |
| Type | `node-package` |
| Root | `apps/sdkwork-membership-common/packages/sdkwork-membership-service` |
| Domain | `commerce` |
| Capability | `membership-service` |
| Languages | `typescript` |
| Status | `ready` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- This package is an app-side service facade over injected or constructed composed SDK clients.
- It consumes `@sdkwork/membership-app-sdk` and `@sdkwork/order-app-sdk`.
- It does not own order or payment APIs, database tables, generated transport output, or raw HTTP fallbacks.

## Boundary

Membership service composition may call `@sdkwork/order-app-sdk` for membership order creation and order payment requests. Order creation, cashier/payment execution, settlement, and fulfillment saga ownership remain in `sdkwork-order` and `sdkwork-payment`; membership only reserves and fulfills membership entitlements.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [COMPONENT_SPEC.md](../../../../../../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [MODULE_SPEC.md](../../../../../../sdkwork-specs/MODULE_SPEC.md) | Reusable package contract and dependency direction. |
| [APP_SDK_INTEGRATION_SPEC.md](../../../../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md) | App SDK client construction, injection, and dependency SDK composition rules. |
| [SDK_SPEC.md](../../../../../../sdkwork-specs/SDK_SPEC.md) | SDK generation and SDK integration rules. |
| [FRONTEND_SPEC.md](../../../../../../sdkwork-specs/FRONTEND_SPEC.md) | Frontend service to SDK layering rules. |
| [APPLICATION_LAYERED_ARCHITECTURE_SPEC.md](../../../../../../sdkwork-specs/APPLICATION_LAYERED_ARCHITECTURE_SPEC.md) | UI-service-SDK dependency direction and layer ownership rules. |
| [CODE_STYLE_SPEC.md](../../../../../../sdkwork-specs/CODE_STYLE_SPEC.md) | Authored source structure and generated code boundaries. |
| [NAMING_SPEC.md](../../../../../../sdkwork-specs/NAMING_SPEC.md) | Canonical SDKWork naming rules. |
| [TYPESCRIPT_CODE_SPEC.md](../../../../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md) | TypeScript package and public export rules. |
| [TEST_SPEC.md](../../../../../../sdkwork-specs/TEST_SPEC.md) | SDK boundary, service facade, and package verification rules. |

## Public Exports

- `.`

## Verification

- `pnpm typecheck`
- `pnpm test:node`
- `pnpm test:vitest -- apps/sdkwork-membership-pc/packages/sdkwork-membership-pc-membership/tests/membership.service.test.ts`
