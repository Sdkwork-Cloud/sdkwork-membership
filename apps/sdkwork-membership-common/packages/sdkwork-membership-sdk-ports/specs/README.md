# SDKWork Membership SDK Ports Specs

This directory is the local standards index for `@sdkwork/membership-sdk-ports`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `@sdkwork/membership-sdk-ports` |
| Type | `node-package` |
| Root | `apps/sdkwork-membership-common/packages/sdkwork-membership-sdk-ports` |
| Domain | `commerce` |
| Capability | `membership-sdk-ports` |
| Languages | `typescript` |
| Status | `ready` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- This package owns typed service ports for injected membership app SDK clients.
- It does not create clients, call raw HTTP, or accept legacy response envelopes.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [COMPONENT_SPEC.md](../../../../../../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [MODULE_SPEC.md](../../../../../../sdkwork-specs/MODULE_SPEC.md) | Reusable package contract and dependency direction. |
| [APP_SDK_INTEGRATION_SPEC.md](../../../../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md) | App SDK port injection and composed consumer rules. |
| [SDK_SPEC.md](../../../../../../sdkwork-specs/SDK_SPEC.md) | SDK generation and SDK integration rules. |
| [CODE_STYLE_SPEC.md](../../../../../../sdkwork-specs/CODE_STYLE_SPEC.md) | Authored source structure and generated code boundaries. |
| [NAMING_SPEC.md](../../../../../../sdkwork-specs/NAMING_SPEC.md) | Canonical SDKWork naming rules. |
| [TYPESCRIPT_CODE_SPEC.md](../../../../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md) | TypeScript package and public export rules. |
| [TEST_SPEC.md](../../../../../../sdkwork-specs/TEST_SPEC.md) | SDK boundary and service-port verification rules. |

## Public Exports

- `.`

## Verification

- `pnpm typecheck`
- `pnpm test:node`
