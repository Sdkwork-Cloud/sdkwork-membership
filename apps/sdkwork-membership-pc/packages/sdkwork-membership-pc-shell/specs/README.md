# SDKWork Membership PC Shell Specs

This directory is the local standards index for `@sdkwork/membership-pc-shell`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `@sdkwork/membership-pc-shell` |
| Type | `react-package` |
| Root | `apps/sdkwork-membership-pc/packages/sdkwork-membership-pc-shell` |
| Domain | `commerce` |
| Capability | `membership-shell` |
| Languages | `typescript`, `react` |
| Status | `ready` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- This package owns PC shell route composition and app-side service bootstrap.
- It does not own business SDK methods, raw HTTP transport, backend-admin APIs, or membership/order persistence.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [COMPONENT_SPEC.md](../../../../../../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [APP_PC_ARCHITECTURE_SPEC.md](../../../../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md) | PC application root package taxonomy and shell responsibilities. |
| [APP_PC_REACT_UI_SPEC.md](../../../../../../sdkwork-specs/APP_PC_REACT_UI_SPEC.md) | App PC React package split and app SDK boundary rules. |
| [UI_ARCHITECTURE_SPEC.md](../../../../../../sdkwork-specs/UI_ARCHITECTURE_SPEC.md) | UI architecture selection and SDK surface boundary rules. |
| [FRONTEND_SPEC.md](../../../../../../sdkwork-specs/FRONTEND_SPEC.md) | Frontend shell, service, and SDK composition rules. |
| [APP_SDK_INTEGRATION_SPEC.md](../../../../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md) | App SDK runtime construction and TokenManager closure rules. |
| [CONFIG_SPEC.md](../../../../../../sdkwork-specs/CONFIG_SPEC.md) | Runtime config and SDK base URL rules. |
| [CODE_STYLE_SPEC.md](../../../../../../sdkwork-specs/CODE_STYLE_SPEC.md) | Authored source structure and generated code boundaries. |
| [NAMING_SPEC.md](../../../../../../sdkwork-specs/NAMING_SPEC.md) | Canonical SDKWork naming rules. |
| [TYPESCRIPT_CODE_SPEC.md](../../../../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md) | TypeScript package and public export rules. |
| [FRONTEND_CODE_SPEC.md](../../../../../../sdkwork-specs/FRONTEND_CODE_SPEC.md) | Frontend authored source structure. |
| [TEST_SPEC.md](../../../../../../sdkwork-specs/TEST_SPEC.md) | Frontend shell and SDK integration verification rules. |

## Public Exports

- `.`

## Verification

- `pnpm typecheck`
- `pnpm test:node`
- `pnpm test:vitest`
