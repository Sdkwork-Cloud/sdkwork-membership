# Repository Guidelines

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`
- `../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Read `sdkwork.app.config.json` only when changing Settings application behavior, runtime config, SDK wiring, release metadata, packaging, app-owned capabilities, or deployment metadata.

- Domain: `commerce`
- Capability: `membership`
- Table prefix: `commerce_`
- App API prefix: `/app/v3/api/membership`
- Backend API prefix: `/backend/v3/api/membership`
- PC application root: `apps/sdkwork-membership-pc/`

This is a **T1 commerce capability repository**. This repository is self-contained with its own API server, persistence, IAM middleware, and SDK surfaces.

## Framework Integration Boundaries

### Mandatory Frameworks

- `sdkwork-web-framework`: All HTTP `*-api` surfaces (`open-api`, `app-api`, `backend-api`) MUST integrate `sdkwork-web-core`, `sdkwork-web-axum`, `sdkwork-web-bootstrap`, and route crates through `sdkwork-routes-web-framework-backend-api`. Business repositories MUST NOT fork the standard interceptor chain or request-context framework locally. Authority: `WEB_FRAMEWORK_SPEC.md`.
- `sdkwork-database`: Database lifecycle, migrations, seeds, drift, and SPI orchestration through `sdkwork-database-config`, `sdkwork-database-lifecycle`, `sdkwork-database-spi`, `sdkwork-database-sqlx`. Authority: `DATABASE_FRAMEWORK_SPEC.md`.
- `sdkwork-utils`: Cross-language utility library to reduce duplicate code. Use `sdkwork-utils-rust` for string, datetime, validation, crypto, encoding, collection, http_api operations. Authority: `CODE_STYLE_SPEC.md`.

### Optional Frameworks

- `sdkwork-discovery`: Not integrated. This repository has no RPC services. Add `sdkwork-discovery` integration when RPC services are introduced. Authority: `DISCOVERY_SPEC.md`.
- `sdkwork-drive`: Not integrated. This repository has no file upload surfaces. Add `sdkwork-drive` integration when file upload capabilities are introduced. Authority: `DRIVE_SPEC.md`.

## Local Dictionary Structure

- `AGENTS.md`: agent execution rules and relative spec entrypoint.
- `sdkwork.app.config.json`: application identity, app metadata, release surfaces, and owned capabilities.
- `.sdkwork/`: local skills, plugins, manifests, and repository/application AI workspace metadata.
- `specs/`: repository/application root specs for cross-module machine contracts.
- `docs/`: Canon documentation at `docs/product/prd/PRD.md` and `docs/architecture/tech/TECH_ARCHITECTURE.md`.
- `apis/`: OpenAPI authorities for `app-api`.
- `sdks/`: SDK families, OpenAPI authorities, derived generator inputs, route manifests, SDK assembly, and generated outputs.
- `crates/`: Rust workspace members (service, repository-sqlx, route crates, database-host, service-host, standalone-gateway, gateway-assembly).
- `apps/sdkwork-membership-pc/`: PC web application root with its own `sdkwork.app.config.json`.
- `database/`: DDL baselines, migrations, seeds, and contract registry.
- `package.json`: pnpm scripts and dev dependencies for the workspace root.

## Spec Resolution Order

Standards are resolved in this order:

1. Current or nearest `AGENTS.md`.
2. `sdkwork.app.config.json` when present.
3. Nearest module `specs/README.md` and `specs/component.spec.json` when the task touches an authored module.
4. Repository/application root `specs/` when the task is repository-wide or application-wide.
5. Local `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` when relevant.
6. Global `sdkwork-specs/README.md` through the declared relative path.
7. Task-specific global specs referenced by the task matrix, nearest `AGENTS.md`, or module `canonicalSpecs`.
8. Implementation files.

Local files may narrow the task, but global `sdkwork-specs` remain authoritative.

Loading is dynamic and progressive. Agents MUST load the nearest `AGENTS.md` and dictionary entries first, then only the root specs required by the current task. Agents MUST NOT eagerly load all language, runtime, UI, deployment, or SDK specs for unrelated work. Language-specific specs are on-demand and loaded only when the touched files require them.

## Required Specs By Task Type

Code changes require `../sdkwork-specs/CODE_STYLE_SPEC.md`, `../sdkwork-specs/NAMING_SPEC.md`, and only the language/framework spec for the touched files.

| Task | Required specs |
| --- | --- |
| Agent/workflow rules | `SOUL.md`, `AGENTS_SPEC.md`, `SDKWORK_WORKSPACE_SPEC.md` |
| Any code change | `CODE_STYLE_SPEC.md`, `NAMING_SPEC.md`, plus only the touched language/framework spec |
| Build scripts / dev runners | `CODE_STYLE_SPEC.md` §7, `TYPESCRIPT_CODE_SPEC.md` §5, `PNPM_SCRIPT_SPEC.md` §11 |
| Rust code | `RUST_CODE_SPEC.md` (loaded on demand) |
| TypeScript/Node code | `TYPESCRIPT_CODE_SPEC.md` (loaded on demand) |
| Frontend/UI code | `FRONTEND_CODE_SPEC.md`, `FRONTEND_SPEC.md`, `UI_ARCHITECTURE_SPEC.md` (loaded on demand) |
| API changes | `API_SPEC.md`, `WEB_FRAMEWORK_SPEC.md`, `WEB_BACKEND_SPEC.md`, `SDK_SPEC.md`, `TEST_SPEC.md` |
| Rust HTTP route crates / API servers | `API_SPEC.md`, `SUBJECT_ID_SPEC.md`, `WEB_FRAMEWORK_SPEC.md`, `WEB_BACKEND_SPEC.md`, `RUST_CODE_SPEC.md`, `SECURITY_SPEC.md`, `TEST_SPEC.md` |
| Database changes | `DATABASE_SPEC.md`, `DATABASE_FRAMEWORK_SPEC.md`, `SUBJECT_ID_SPEC.md`, `PRIVACY_SPEC.md`, `TEST_SPEC.md` |
| SDK generation/consumption | `SDK_SPEC.md`, `SDK_WORKSPACE_GENERATION_SPEC.md`, `API_SPEC.md`, `TEST_SPEC.md` |
| App identity/release | `APP_MANIFEST_SPEC.md`, `CONFIG_SPEC.md`, `DEPLOYMENT_SPEC.md` |
| Security/auth | `IAM_SPEC.md`, `SUBJECT_ID_SPEC.md`, `SECURITY_SPEC.md`, `PRIVACY_SPEC.md` |
| Packaging / GitHub workflows | `GITHUB_WORKFLOW_SPEC.md`, `PNPM_SCRIPT_SPEC.md`, `DEPLOYMENT_SPEC.md` |

Language specs are on-demand. Do not require agents to load Rust, TypeScript, and frontend specs for unrelated tasks.

## Code Style Rules

Follow `../sdkwork-specs/CODE_STYLE_SPEC.md` and `../sdkwork-specs/NAMING_SPEC.md`:

- Rust crates use `sdkwork_membership_*` naming (no `sdkwork_commerce_*` aliases).
- SQL repositories emit canonical `SdkWorkApiResponse` / `application/problem+json` envelopes through `sdkwork-web-framework` response mapping.
- Use `sdkwork-utils-rust` helpers (`string`, `datetime`, `validation`, `currency`, `number`) instead of hand-rolled utilities to reduce duplicate code.
- No legacy envelopes (`PlusApiResult`, `AppbaseApiResult`, `StoreApiResult`, `SdkWorkResponse`, per-domain `*ApiResult`) or `requestId` wire fields.

## Build, Test, and Verification

Repository root scripts follow `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`. Standard commands:

```bash
pnpm install
pnpm dev              # Start PC dev server
pnpm build           # Build PC app
pnpm start           # Run standalone gateway binary
pnpm test            # Run node + vitest + cargo tests
pnpm check           # Run app-composition, db, gateway, typecheck checks
pnpm verify          # typecheck + vitest + cargo test + app-composition
pnpm typecheck       # TypeScript typecheck
pnpm format          # cargo fmt --all
pnpm format:rust:check  # cargo fmt --all -- --check
pnpm clean           # Remove reproducible local artifacts
```

Database lifecycle commands (`db:validate`, `db:plan`, `db:init`, `db:migrate`, `db:seed`, `db:status`, `db:drift:check`, `db:bootstrap`) delegate to `sdkwork-database-cli` per `DATABASE_FRAMEWORK_SPEC.md`.

Record commands and outputs. Run `pnpm verify` and `cargo test --workspace` before completing work.

## Agent Execution Rules

- Load `AGENTS.md` and `sdkwork.app.config.json` first, then only the task-specific specs from `../sdkwork-specs/`.
- Stop and report when relative `sdkwork-specs` paths do not resolve.
- Do not fork `sdkwork-web-framework` interceptor chains or request-context framework locally.
- Do not introduce legacy envelopes or `requestId` wire fields.
- Use `sdkwork-utils-rust` helpers instead of duplicating utility code.
- Run the relevant verification scripts before completing work:
  - `node ../sdkwork-specs/tools/check-api-response-envelope.mjs`
  - `node ../sdkwork-specs/tools/verify-repo.mjs --root .`
  - `node ../sdkwork-specs/tools/check-database-framework-standard.mjs --root .`
  - `node ../sdkwork-specs/tools/check-agent-workflow-standard.mjs --root .`
  - `node ../sdkwork-specs/tools/check-pnpm-script-standard.mjs --root .`

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

All L2+ `app-api`, `backend-api`, and SDKWork-owned business `open-api` HTTP contracts `MUST` follow `../sdkwork-specs/API_SPEC.md` section 4.5, section 14, and section 15:

- **Input:** typed request bodies, section 14.1 list/search/command input, `SdkWorkListQuery`, and `q` for free-text search.
- **Success output:** `SdkWorkApiResponse` with `{ "code": 0, "data": <payload>, "traceId": "<server-uuid>" }`.
- **Error output:** HTTP 4xx/5xx `application/problem+json` (`ProblemDetail`) with numeric `code` and `traceId`.
- Success `code` is numeric `int32`; HTTP 2xx JSON bodies `MUST` use `0` only. REST semantics remain on HTTP status (`201`, `202`, etc.).
- Platform error codes are numeric non-zero values per section 15.3 (`40001`, `40101`, `40401`, …).
- Single resource: `data.item`
- Lists: `data.items` + `data.pageInfo` (`PageInfo.mode` is `offset` or `cursor`)
- Commands: `data.accepted` plus optional `resourceId` / `status`
- Async accept (`202`): `data.operationId`, `data.status`, optional `pollUrl`

Vendor compatibility `open-api` routes that mirror upstream tool or provider wire (for example OpenAI `/v1/*`, Claude Code, Codex) `MAY` opt out only when every exempt operation declares `x-sdkwork-wire-protocol: external` and `x-sdkwork-external-protocol-id` per `API_SPEC.md` section 4.5.2. SDKWork-owned business `open-api` operations `MUST NOT` opt out.

Errors `MUST` use HTTP 4xx/5xx with `application/problem+json` (`ProblemDetail`) including required numeric `code` and `traceId`. Business failures `MUST NOT` use HTTP 2xx with non-zero `code`, string wire codes, `success`, or human `message`.

Forbidden legacy envelopes and fields: `PlusApiResult`, `AppbaseApiResult`, `StoreApiResult`, `SdkWorkResponse`, per-domain `*ApiResult`, wire field `requestId`, bare domain DTOs at the HTTP root, and top-level `{ items, pageInfo, traceId }` without `data`.

Handlers `MUST` serialize success and map errors through `sdkwork-web-framework` response mapping. Generated HTTP SDKs (`--standard-profile sdkwork-v3`) unwrap `data` by default and expose typed numeric `ProblemDetail.code` / `traceId` on errors; use `.raw` when the full envelope is required.

Before completing API contract, SDK generation, or frontend service work, run:

```bash
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace ..
```

Authority: `../sdkwork-specs/API_SPEC.md` section 4.5 and sections 14–16, `../sdkwork-specs/SDK_SPEC.md` section 4.2, `../sdkwork-specs/FRONTEND_SPEC.md`, `../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`.

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

Follow `../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md` for packaging and release workflow changes:

- Require human review before merging API contract, database migration, SDK generation, or deployment manifest changes.
- Require human review before introducing new framework dependencies or altering the standard interceptor chain.
- Do not commit secrets, live tokens, or app-local credential handling. Protected API and SDK access must use the generated SDK or approved service boundary.
- Run `pnpm verify` and all relevant check scripts before requesting review.
- Document breaking changes in `docs/architecture/decisions/` ADRs and update `docs/product/prd/PRD.md` and `docs/architecture/tech/TECH_ARCHITECTURE.md` accordingly.
