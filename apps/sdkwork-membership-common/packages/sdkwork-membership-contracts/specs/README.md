# SDKWork Membership Contracts Specs

This directory is the local standards index for `@sdkwork/membership-contracts`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `@sdkwork/membership-contracts` |
| Type | `node-package` |
| Root | `apps/sdkwork-membership-common/packages/sdkwork-membership-contracts` |
| Domain | `commerce` |
| Capability | `membership-contracts` |
| Languages | `typescript` |
| Status | `ready` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- This package owns shared membership TypeScript contracts only.
- It does not construct SDK clients, call HTTP APIs, or own UI/runtime behavior.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [COMPONENT_SPEC.md](../../../../../../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [MODULE_SPEC.md](../../../../../../sdkwork-specs/MODULE_SPEC.md) | Reusable package contract and dependency direction. |
| [CODE_STYLE_SPEC.md](../../../../../../sdkwork-specs/CODE_STYLE_SPEC.md) | Authored source structure and generated code boundaries. |
| [NAMING_SPEC.md](../../../../../../sdkwork-specs/NAMING_SPEC.md) | Canonical SDKWork naming rules. |
| [TYPESCRIPT_CODE_SPEC.md](../../../../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md) | TypeScript package and public export rules. |
| [TEST_SPEC.md](../../../../../../sdkwork-specs/TEST_SPEC.md) | Contract and package verification rules. |

## Public Exports

- `.`

## Verification

- `pnpm typecheck`
- `pnpm test:node`
