# Membership Database

- Owner: sdkwork-membership platform team
- Capability: membership (commerce domain)
- Engines: PostgreSQL, SQLite
- Table prefix: `commerce_` (shared commerce domain prefix)
- Compliance level: L2
- Contract version: 1.1.0

## Related Specs

- `DATABASE_FRAMEWORK_SPEC.md` — lifecycle, directory, SPI, drift, seed locale
- `DATABASE_SPEC.md` — table/field/index semantics, pool rules
- `MIGRATION_SPEC.md` — migration governance
- `SUBJECT_ID_SPEC.md` — tenant isolation and subject scope rules

## Table Ownership

### Membership-owned extension tables

These tables are created and managed by this repository:

| Table | Profile | Description |
| --- | --- | --- |
| `commerce_membership_daily_reward` | user_entity+event_log | Daily check-in reward tracking with consecutive-day bonus computation |
| `commerce_membership_privilege_usage` | user_entity | Per-period privilege consumption tracker for membership benefits |
| `commerce_membership_change_log` | audit_log | Immutable audit log for membership status transitions and plan changes |

All extension tables follow `DATABASE_SPEC.md` standards:
- `uuid` unique identifier for external reference
- `version` optimistic concurrency control
- `tenant_id` + `organization_id` multi-tenant isolation (BIGINT/INTEGER)
- Idempotency constraints for safe retries
- Check constraints for data integrity

### Commerce platform shared tables

These tables are created by the commerce platform initial migration. The membership repo reads and writes via repository adapters:

- `membership_plan`, `membership_plan_version`, `membership_plan_benefit`
- `membership_package_group`, `membership_package`
- `membership_subscription`, `membership_period`
- `benefit_definition`
- `entitlement_account`, `entitlement_grant`, `entitlement_ledger_entry`

### Commerce platform reference tables

Cross-capability commerce tables accessed via port interfaces (no direct write ownership):

- `commerce_product_spu`, `commerce_product_sku`
- `commerce_order`, `commerce_order_item`, `commerce_order_amount_breakdown`
- `commerce_payment_intent`, `commerce_payment_attempt`
- `commerce_account`, `commerce_account_ledger_entry`
- `commerce_payment_method`, `commerce_payment_provider`, `commerce_payment_provider_account`
- `commerce_payment_channel`, `commerce_payment_route_rule`
- `commerce_recharge_package`, `commerce_exchange_rule`

## Migrations

### 0001_membership_extension_tables

Creates three membership-owned extension tables and adds standard columns to existing commerce platform tables.

**PostgreSQL:**
- Creates `commerce_membership_daily_reward`, `commerce_membership_privilege_usage`, `commerce_membership_change_log`
- Adds `uuid`, `version`, `deleted_at`, `tags`, `auto_renew`, `growth_value` columns via idempotent `DO $$ ... $$` blocks
- Creates performance indexes for high-frequency query patterns

**SQLite:**
- Same table creation with SQLite-compatible types (INTEGER, TEXT)
- Indexes for high-frequency query patterns
- Column additions are handled by the application layer's read-model error tolerance

## Contract Registry

- `contract/schema.yaml` — 30 tables registered with profile, compliance level, write-owner
- `contract/table-registry.json` — machine-readable inventory with migration source and reference flags
- `contract/prefix-registry.json` — `commerce_` prefix registration

## Verification

```bash
pnpm db:validate
pnpm db:status
pnpm db:migrate
pnpm db:seed
pnpm db:drift:check
```

## Bootstrap

```bash
pnpm db:bootstrap
```

This runs `db:migrate` then `db:seed` for local development.

## Initialization state

This module is in **initialization state** for greenfield deployments:

1. **Baseline** — `database/ddl/baseline/{engine}/0001_membership_baseline.sql` contains the full DDL snapshot.
2. **Migrations** — `database/migrations/{engine}/` is reserved for post-GA incremental schema changes only. It is intentionally empty at initialization.
3. **Drift** — run `pnpm db:drift:check` before release.

## Commands

```bash
pnpm run db:validate
pnpm run db:materialize:contract
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```
