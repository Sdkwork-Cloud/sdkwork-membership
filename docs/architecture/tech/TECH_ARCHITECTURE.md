# Membership Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-06-26
Specs: ARCHITECTURE_DECISION_SPEC.md, RUST_CODE_SPEC.md, API_SPEC.md, WEB_FRAMEWORK_SPEC.md, DATABASE_FRAMEWORK_SPEC.md, DRIVE_SPEC.md, DISCOVERY_SPEC.md, SDKWORK_WORKSPACE_SPEC.md

## 1. Architecture Overview

`sdkwork-membership` is a **commerce-domain capability repository** that owns membership plans, packages, entitlements, points, and subscription domain logic. It follows the standard SDKWork repository architecture with self-contained API server, database host, and web framework integration.

```text
sdkwork-specs (L0)
  WEB_FRAMEWORK_SPEC.md, DATABASE_FRAMEWORK_SPEC.md, RUST_CODE_SPEC.md
       -> narrows
sdkwork-web-framework + sdkwork-database + sdkwork-utils (L1 frameworks)
       -> extended by
sdkwork-membership crates (L2 business)
  sdkwork-commerce-membership-service          — domain rules, commands, ports
  sdkwork-commerce-membership-repository-sqlx  — SQLx persistence, seed data
  sdkwork-routes-membership-app-api            — app-api route adapter
  sdkwork-routes-membership-backend-api        — backend-api route adapter
  sdkwork-membership-database-host             — database lifecycle bootstrap
  sdkwork-membership-service-host              — in-process service container
  sdkwork-membership-standalone-gateway                — HTTP process entry point
  sdkwork-membership-gateway-assembly          — route assembly manifest
       -> consumed by
composition applications (sdkwork-mall, etc.) via workspace path dependencies
```

## 2. Technology Choices

- **Rust** domain services and SQLx repositories (`RUST_CODE_SPEC.md`)
- **Axum** HTTP routers integrated via `sdkwork-web-framework` (`WEB_FRAMEWORK_SPEC.md`)
- **sdkwork-database** for connection pools, lifecycle orchestration, and SPI (`DATABASE_FRAMEWORK_SPEC.md`)
- **sdkwork-utils-rust** for cross-language utility functions — eliminates duplicate validation, string, and number helpers (`GENERIC_UTILS_SCOPE.md`)
- **sdkwork-iam-web-adapter** for `WebRequestContext` resolution and IAM token validation
- **Sibling path dependencies** from this repository's `Cargo.toml` — cross-T1 references use `sdkwork_commerce_*` crate names per `sdkwork-<domain>-<capability>-service` naming

## 3. System Boundaries And Modules

| Layer | Owner crate | Responsibility |
| --- | --- | --- |
| Domain commands/queries | `sdkwork-commerce-membership-service` | Business validation, domain models, ports, service contracts |
| SQL repositories | `sdkwork-commerce-membership-repository-sqlx` | Tenant-scoped persistence, row mapping, seed installation |
| App API routes | `sdkwork-routes-membership-app-api` | `/app/v3/api/membership` route adapter with `WebRequestContext` |
| Backend API routes | `sdkwork-routes-membership-backend-api` | `/backend/v3/api/membership` route adapter with `WebRequestContext` |
| Database lifecycle | `sdkwork-membership-database-host` | Pool creation, `DefaultDatabaseModule`, migration orchestration |
| Service container | `sdkwork-membership-service-host` | In-process service container (no HTTP listener) |
| API server | `sdkwork-membership-standalone-gateway` | Process startup, route mounting, HTTP listener, health |
| Gateway assembly | `sdkwork-membership-gateway-assembly` | Route inventory manifest, assembly bootstrap |

## 4. Web Framework Integration

Both app-api and backend-api route crates wrap their routers through `sdkwork-web-framework`:

```text
sdkwork-web-axum::WebFrameworkLayer
  -> with_web_request_context(router, layer)
  -> IamWebRequestContextResolver (dual-token resolution)
  -> WebRequestContext injected into handlers
```

- CORS is deny-by-default, handled by the framework standard interceptor chain.
- `WebRequestContext` is resolved once at the framework boundary.
- Handlers do not parse raw credential, tenant, or request-id headers.
- The API server uses `sdkwork_web_bootstrap::service_router` for final assembly.

## 5. Database Framework Integration

```text
sdkwork-database-sqlx::create_pool_from_config
  -> DatabasePool (Postgres | SQLite)
  -> sdkwork-database-spi::DefaultDatabaseModule::from_app_root
  -> LifecycleOrchestrator (init + optional auto-migrate)
```

- Standard `database/` directory with `database.manifest.json`, contract, migrations, seeds, and drift policy.
- Connection pools created through `sdkwork-database-sqlx`.
- Lifecycle orchestration uses `sdkwork-database-lifecycle`.
- Seed data is installed through repository crate seed functions.

### 5.1 Table Ownership

| Ownership | Tables | Description |
| --- | --- | --- |
| **Membership-owned** | `commerce_membership_daily_reward`, `commerce_membership_privilege_usage`, `commerce_membership_change_log` | Extension tables created by this repo's migration `0001_membership_extension_tables`. Follow `DATABASE_SPEC.md` with `uuid`, `version`, audit fields, and idempotency constraints. |
| **Commerce platform (shared)** | `membership_plan`, `membership_plan_version`, `membership_plan_benefit`, `membership_package_group`, `membership_package`, `membership_subscription`, `membership_period`, `benefit_definition`, `entitlement_account`, `entitlement_grant`, `entitlement_ledger_entry` | Domain tables created by the commerce platform initial migration. This repo reads and writes via repository adapters. Registered in contract for drift detection. |
| **Commerce platform (reference)** | `commerce_product_spu`, `commerce_product_sku`, `commerce_order`, `commerce_order_item`, `commerce_order_amount_breakdown`, `commerce_payment_intent`, `commerce_payment_attempt`, `commerce_account`, `commerce_account_ledger_entry`, `commerce_payment_method`, `commerce_payment_provider`, `commerce_payment_provider_account`, `commerce_payment_channel`, `commerce_payment_route_rule`, `commerce_recharge_package`, `commerce_exchange_rule` | Cross-capability commerce tables accessed via port interfaces. No direct write ownership. |

### 5.2 Migration Strategy

- Migration `0001_membership_extension_tables` creates three new membership-owned tables and adds standard columns (`uuid`, `version`, `deleted_at`, `tags`, `auto_renew`, `growth_value`) to existing commerce platform tables via idempotent `ALTER TABLE` statements.
- Performance indexes are created for high-frequency query patterns: tenant-scoped status lookups, subscription expiration scanning, and entitlement account balance queries.
- All migrations support both PostgreSQL and SQLite engines.
- Down migrations safely drop extension tables; ALTER TABLE additions are preserved to avoid data loss.

### 5.3 Contract Registry

- `contract/schema.yaml` registers all 30 tables with table profile, compliance level, system-of-record flag, and write-owner attribution.
- `contract/table-registry.json` provides machine-readable table inventory with migration source and `referenceOnly` flags.
- `contract/prefix-registry.json` registers the `commerce_` prefix owned by `sdkwork-membership`.
- Drift policy enforces error-level severity for missing tables, columns, constraints, and migration checksums.

## 6. sdkwork-utils Integration

The service crate uses `sdkwork-utils-rust` for:
- `string::is_blank` — empty/blank validation
- `number::parse_int` — numeric context parsing
- Centralized `normalize_optional_text` in `validation` module eliminates duplicate implementations across `domain` and `queries` modules.

## 7. Discovery And RPC

This repository currently exposes only HTTP APIs and has no gRPC/RPC services. `sdkwork-discovery` is not required. When RPC services are added, they MUST follow `DISCOVERY_SPEC.md` and register through `sdkwork-discovery`.

## 8. Drive And File Storage

The current membership capability does not involve file upload or object storage. If file upload is needed in the future (e.g., membership card icons, benefit images), it MUST use `sdkwork-drive` integration:
- Client uploads via `sdkwork-drive-app-sdk` `client.uploader.*`
- Server-side Rust uploads via `sdkwork_drive_uploader_service`
- Business tables store only `drive_space_id`, `drive_node_id`, or `drive_uri` references

## 9. Directory And Package Layout

```text
sdkwork-membership/
  database/                          — standard database lifecycle assets
    database.manifest.json
    contract/schema.yaml
    contract/prefix-registry.json
    contract/table-registry.json
    migrations/postgres/
    migrations/sqlite/
    seeds/seed.manifest.json
    seeds/common/
    seeds/locales/zh-CN/
    drift/policy.yaml
    fixtures/
  crates/
    sdkwork-commerce-membership-service/       — domain rules
    sdkwork-commerce-membership-repository-sqlx/ — SQLx persistence
    sdkwork-routes-membership-app-api/         — app-api routes
    sdkwork-routes-membership-backend-api/     — backend-api routes
    sdkwork-membership-database-host/          — database bootstrap
    sdkwork-membership-service-host/           — service container
    sdkwork-membership-standalone-gateway/             — HTTP process
    sdkwork-membership-gateway-assembly/       — route assembly
  packages/common/membership/                  — TypeScript contracts and SDK ports
  apps/sdkwork-membership-pc/                  — PC application root
  docs/                                        — documentation canon
```

## 10. API Surface

- App API prefix: `/app/v3/api/membership`
- Backend API prefix: `/backend/v3/api/membership`
- Table prefix: `commerce_` for capability-owned tables (commerce domain)
- Membership-owned extension tables: `commerce_membership_daily_reward`, `commerce_membership_privilege_usage`, `commerce_membership_change_log`
- Public SDK consumption: generated commerce SDK families; do not hand-craft raw HTTP

## 11. Security, Privacy, And Observability

- Authentication and tenant context are resolved through `sdkwork-web-framework` standard interceptor chain.
- Both app-api and backend-api surfaces run the full `WebRequestContext` pipeline (request identity, surface classification, CORS, auth, authorization, tenant isolation, logging).
- Write routes require idempotency and request-hash headers where applicable.
- Ledger, payment, and account mutations fail closed on validation errors.
- Daily reward claims enforce per-user-per-day uniqueness via database constraint and idempotency key.
- Privilege usage tracking enforces per-period uniqueness to prevent double-counting.
- Structured errors use `CommerceServiceError` contracts; internal SQL details are not leaked to clients.
- Subject scope projection follows `SUBJECT_ID_SPEC.md` — `tenant_id` and `user_id` are positive integers, `organization_id` is non-negative with 0 meaning tenant-level scope.
- Missing-table errors degrade gracefully: extension table queries return empty results or conflict errors instead of leaking SQL details.

## 12. Deployment And Runtime Topology

- Local development: `cargo test --workspace` in this repository.
- Independent deployment: `sdkwork-membership-standalone-gateway` binary `membership-server`.
- Platform composition: composition applications (sdkwork-mall, etc.) consume per-T1 SDKs via workspace paths. The `sdkwork-commerce` monolith has been dissolved.
- Environment variables follow `ENVIRONMENT_SPEC.md` with `MEMBERSHIP` service code prefix.

## 13. Verification

```bash
pnpm install
pnpm verify
cargo test --workspace
```
