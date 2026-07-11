# SDKWork Membership PC Core Specs

This directory is the local standards index for `@sdkwork/membership-pc-core`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `@sdkwork/membership-pc-core` |
| Type | `typescript-package` |
| Root | `apps/sdkwork-membership-pc/packages/sdkwork-membership-pc-core` |
| Domain | `membership` |
| Capability | `core` |
| Languages | `typescript` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- This package owns PC runtime identity, SDK inventory, host/session contracts, and composition helpers.
- It must not depend on membership feature packages or backend-admin SDK wrappers.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [COMPONENT_SPEC.md](../../../../../../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [APP_PC_ARCHITECTURE_SPEC.md](../../../../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md) | PC core package responsibilities. |
| [APP_SDK_INTEGRATION_SPEC.md](../../../../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md) | SDK inventory and app SDK composition rules. |
| [FRONTEND_SPEC.md](../../../../../../sdkwork-specs/FRONTEND_SPEC.md) | Frontend core and service layering rules. |
| [CODE_STYLE_SPEC.md](../../../../../../sdkwork-specs/CODE_STYLE_SPEC.md) | Authored source structure and generated code boundaries. |
| [NAMING_SPEC.md](../../../../../../sdkwork-specs/NAMING_SPEC.md) | Canonical SDKWork naming rules. |
| [TYPESCRIPT_CODE_SPEC.md](../../../../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md) | TypeScript package and public export rules. |

## Public Exports

- `.`
- `./sdk`
- `./modules`
- `./host`
- `./session`
- `./composition`

## Verification

- `pnpm typecheck`
- `pnpm test:node`
