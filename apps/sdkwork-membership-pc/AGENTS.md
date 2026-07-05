# Application Guidelines

## SDKWORK Soul

Read `../../../sdkwork-specs/SOUL.md` before executing tasks in this application root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this application root:

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/AGENTS_SPEC.md`
- `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md`
- `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`
- `../../../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../../../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this application. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Read `sdkwork.app.config.json` only when changing PC application behavior, runtime config, SDK wiring, release metadata, packaging, or app-owned capabilities.

- App key: `sdkwork-membership-pc`
- App type: `APP_REACT`
- Domain: `commerce`
- Capability: `membership`
- Runtime: `web` / `react`
- Delivery modes: `WEB_URL`
- Supported deployment profiles: `standalone`, `cloud`
- Default deployment profile: `standalone`

This is the PC web application for the SDKWork Membership capability. It consumes the membership `app-api` and renders membership tiers, entitlements, and upgrade flows.

## Local Dictionary Structure

- `AGENTS.md`: agent execution rules for this PC application root.
- `sdkwork.app.config.json`: PC application identity, metadata, and runtime config.
- `packages/`: React packages for membership UI surfaces (`sdkwork-membership-pc-membership`, `sdkwork-membership-pc-common`).
- `specs/`: application-level spec systems when present.
- `docs/`: application-local documentation when present.

## Spec Resolution Order

Standards are resolved in this order:

1. Current or nearest `AGENTS.md` (this file).
2. `sdkwork.app.config.json` when present.
3. Nearest module `specs/README.md` and `specs/component.spec.json` when the task touches an authored package.
4. Repository root `AGENTS.md` at `../../AGENTS.md` for repository-wide rules.
5. Global `sdkwork-specs/README.md` through the declared relative path.
6. Task-specific global specs referenced by the task matrix, nearest `AGENTS.md`, or module `canonicalSpecs`.
7. Implementation files.

Local files may narrow the task, but global `sdkwork-specs` remain authoritative.

Loading is dynamic and progressive. Agents MUST load the nearest `AGENTS.md` and dictionary entries first, then only the root specs required by the current task. Agents MUST NOT eagerly load all language, runtime, UI, deployment, or SDK specs for unrelated work. Language-specific specs are on-demand and loaded only when the touched files require them.

## Required Specs By Task Type

Code changes require `../../../sdkwork-specs/CODE_STYLE_SPEC.md`, `../../../sdkwork-specs/NAMING_SPEC.md`, and only the language/framework spec for the touched files.

| Task | Required specs |
| --- | --- |
| Agent/workflow rules | `SOUL.md`, `AGENTS_SPEC.md`, `SDKWORK_WORKSPACE_SPEC.md` |
| Any code change | `CODE_STYLE_SPEC.md`, `NAMING_SPEC.md`, plus only the touched language/framework spec |
| TypeScript/React code | `TYPESCRIPT_CODE_SPEC.md` (loaded on demand) |
| Frontend/UI code | `FRONTEND_CODE_SPEC.md`, `FRONTEND_SPEC.md`, `UI_ARCHITECTURE_SPEC.md`, `APP_PC_REACT_UI_SPEC.md` (loaded on demand) |
| API integration | `API_SPEC.md`, `SDK_SPEC.md`, `FRONTEND_SPEC.md` |
| App identity/release | `APP_MANIFEST_SPEC.md`, `CONFIG_SPEC.md`, `DEPLOYMENT_SPEC.md` |
| Packaging / GitHub workflows | `GITHUB_WORKFLOW_SPEC.md`, `PNPM_SCRIPT_SPEC.md`, `DEPLOYMENT_SPEC.md` |

Language specs are on-demand. Do not require agents to load TypeScript and frontend specs for unrelated tasks.

## Code Style Rules

Follow `../../../sdkwork-specs/CODE_STYLE_SPEC.md` and `../../../sdkwork-specs/NAMING_SPEC.md`:

- React packages use `@sdkwork/membership-pc-*` naming.
- Consume membership `app-api` through generated SDK clients, not hand-rolled fetch calls.
- Do not introduce legacy API envelopes or `requestId` wire fields in frontend code.
- Use `sdkwork-utils` TypeScript helpers when available to reduce duplicate code.

## Build, Test, and Verification

Application scripts follow `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md`. Standard commands run from repository root:

```bash
pnpm --filter @sdkwork/membership-pc-membership typecheck
pnpm dev
pnpm build
```

Record commands and outputs. Run `pnpm verify` from the repository root before completing work.

## Agent Execution Rules

- Load this `AGENTS.md` and `sdkwork.app.config.json` first, then only the task-specific specs from `../../../sdkwork-specs/`.
- Stop and report when relative `sdkwork-specs` paths do not resolve.
- Consume the membership `app-api` through generated SDK clients; do not hand-roll API calls.
- Do not introduce legacy envelopes or `requestId` wire fields in frontend code.
- Run the relevant verification scripts before completing work:
  - `node ../../../sdkwork-specs/tools/check-api-response-envelope.mjs`
  - `node ../../../sdkwork-specs/tools/check-agent-workflow-standard.mjs --root ../../..`

## App SDK Consumer Imports



Application, feature, shell, and service packages `MUST` consume HTTP SDKs through scoped composed consumer packages, not generator transport package names.



- App API clients: `@sdkwork/<application-code>-app-sdk`

- Backend API clients (`backend-admin` only): `@sdkwork/<application-code>-backend-sdk`

- Federated Claw Router domain surfaces: `@sdkwork/clawrouter-app-sdk/domains` and `@sdkwork/clawrouter-backend-sdk/domains`

- Open/domain API clients: `@sdkwork/<domain>-sdk`



Canonical examples (IAM):



```typescript

import { createClient, type SdkworkAppClient } from '@sdkwork/iam-app-sdk';

import type { SdkworkBackendClient } from '@sdkwork/iam-backend-sdk'; // backend-admin only

import { createClient as createClawRouterDomainsClient } from '@sdkwork/clawrouter-app-sdk/domains';

```



Forbidden in application `apps/`, `packages/`, bootstrap, services, UI, contract tests, and composed SDK `src/**` outside generator ownership:



- `sdkwork-*-app-sdk-generated-typescript`, `sdkwork-*-backend-sdk-generated-typescript`, and other generator transport names as consumer imports

- `@sdkwork/commerce-app-sdk`, `@sdkwork/commerce-backend-sdk`, `@sdkwork/clawrouter-*-domain-transport-sdk`

- filesystem paths containing `domain-transport-typescript`, `domain-transport-sdk`, or sibling `*-typescript/generated` hops from composed `src/**`

- deep imports into `generated/server-openapi/src/*` from consumers when a composed facade exists



Allowed:



- Composed facade entry imports such as `@sdkwork/iam-app-sdk`, `@sdkwork/knowledgebase-app-sdk`, and `@sdkwork/clawrouter-app-sdk/domains`

- Composed re-exports that import only from `../generated/**` within the same `*-sdk-typescript` family root

- Generated transport ownership inside `sdks/**/generated/**` only



Each SDK family `MUST` expose the composed TypeScript facade at `sdks/<sdk-family>/<sdk-family>-typescript/src/index.ts` (and optional subpath exports such as `./domains`) with `package.json#name` equal to the scoped consumer package.



Before completing SDK integration or frontend service work, run:



```bash

node <sdkwork-specs>/tools/check-app-sdk-consumer-imports.mjs --workspace <workspace-root>

```



Authority: `APP_SDK_INTEGRATION_SPEC.md` section 9, `SDK_SPEC.md` package naming table, `SDK_WORKSPACE_GENERATION_SPEC.md` composed facade rules.



## HTTP API Response Envelope

All SDKWork HTTP `app-api` contracts consumed by this application `MUST` follow `../../../sdkwork-specs/API_SPEC.md` section 4.5, section 14, and section 15:

- **Success output:** `SdkWorkApiResponse` with `{ "code": 0, "data": <payload>, "traceId": "<server-uuid>" }`.
- **Error output:** HTTP 4xx/5xx `application/problem+json` (`ProblemDetail`) with numeric `code` and `traceId`.
- Success `code` is numeric `int32`; HTTP 2xx JSON bodies `MUST` use `0` only.
- Platform error codes are numeric non-zero values per section 15.3 (`40001`, `40101`, `40401`, …).
- Single resource: `data.item`
- Lists: `data.items` + `data.pageInfo` (`PageInfo.mode` is `offset` or `cursor`)
- Commands: `data.accepted` plus optional `resourceId` / `status`

Forbidden legacy envelopes and fields: `PlusApiResult`, `AppbaseApiResult`, `StoreApiResult`, `SdkWorkResponse`, per-domain `*ApiResult`, wire field `requestId`, bare domain DTOs at the HTTP root, and top-level `{ items, pageInfo, traceId }` without `data`.

Generated HTTP SDKs (`--standard-profile sdkwork-v3`) unwrap `data` by default and expose typed numeric `ProblemDetail.code` / `traceId` on errors; use `.raw` when the full envelope is required.

Authority: `../../../sdkwork-specs/API_SPEC.md` section 4.5 and sections 14–16, `../../../sdkwork-specs/SDK_SPEC.md` section 4.2, `../../../sdkwork-specs/FRONTEND_SPEC.md`, `../../../sdkwork-specs/APP_PC_REACT_UI_SPEC.md`.

## List And Search Pagination

All L2+ list/search APIs and their backing services, repositories, SDK consumers, and interactive frontend lists `MUST` follow `PAGINATION_SPEC.md`:

- **Input:** standard `SdkWorkListQuery` or query params (`page`/`page_size` or `cursor`/`page_size` per `API_SPEC.md` §14.1); default `page_size` `20`; max `200` unless a documented exception exists.
- **Output:** `SdkWorkApiResponse.data.items` + `data.pageInfo` with `PageInfo.mode` (`offset` or `cursor`) per `API_SPEC.md` §16.
- **Store-level pagination:** push filtering, sorting, and page selection to SQL `LIMIT`/keyset or incrementally maintained indexes — never unbounded collect then `skip`/`take`/`slice` in process memory (`PAGINATION_SPEC.md` §2).
- **SDK and frontend:** interactive lists request one page at a time from the server; no default `listAll*` on P0/P1 paths; no client-side `slice` pagination over full downloads.

Before completing list/search API, repository, SDK list helper, projection read model, or paginated UI work, run:

```bash
node <sdkwork-specs>/tools/check-pagination.mjs --workspace <workspace-root>
```

Authority: `PAGINATION_SPEC.md`, `API_SPEC.md` §14.1/§16, `DATABASE_SPEC.md` §20.5, `WEB_BACKEND_SPEC.md` §12, `SDK_SPEC.md` §4.2/§6, `FRONTEND_SPEC.md`, `APP_SDK_INTEGRATION_SPEC.md` §9.

## Human Review Rules

Follow `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md` for packaging and release workflow changes:

- Require human review before merging API contract changes that affect frontend consumption.
- Require human review before introducing new SDK dependencies or altering the app runtime config.
- Do not commit secrets, live tokens, or app-local credential handling.
- Run `pnpm verify` from the repository root before requesting review.
- Update `docs/product/prd/PRD.md` and `docs/architecture/tech/TECH_ARCHITECTURE.md` at the repository root when application behavior changes.
