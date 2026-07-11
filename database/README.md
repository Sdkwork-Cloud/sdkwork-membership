# Membership Database

- Owner: sdkwork-membership platform team
- Capability: membership (commerce domain)
- Engines: PostgreSQL, SQLite
- Table prefix: `commerce_` for commerce-scoped membership extension tables
- Compliance level: L2
- Contract version: 1.0.0
- Updated: 2026-07-08

## Related Specs

- `DATABASE_FRAMEWORK_SPEC.md` - lifecycle, directory layout, SPI, drift, seed locale
- `DATABASE_SPEC.md` - table/field/index semantics, pool rules
- `MIGRATION_SPEC.md` - migration governance
- `SUBJECT_ID_SPEC.md` - tenant isolation and subject scope rules
- `specs/commerce-order-membership-boundary.spec.json` - order/payment/membership ownership boundary

## Ownership Boundary

The membership database lifecycle initializes only membership-owned data needed by membership catalog, subscription reservation, entitlement fulfillment, points balance projection, privileges, rewards, and audit flows.

Order creation, token-plan purchase orders, membership-package purchase orders, payment execution, cashier, PSP webhook ingestion, payment settlement, recharge packages, and exchange rules are owned by `sdkwork-order` and `sdkwork-payment`. Those modules run their own database lifecycle and own their tables.

Membership is allowed to store the order identifiers required to reserve and fulfill a membership subscription:

- `membership_subscription.source_order_id`
- `membership_subscription.request_no`
- `membership_period.source_order_id`
- `membership_period.request_no`

Membership must not store payment intent, payment attempt, cashier, provider, or payment-method artifacts in membership-owned tables.

## Initialized Tables

The current baseline and contract initialize 18 membership capability tables.

| Table | Owner | Purpose |
| --- | --- | --- |
| `commerce_product_spu` | sdkwork-membership | Membership catalog SPU rows used by package SKU projection |
| `commerce_product_sku` | sdkwork-membership | Membership package SKU rows and display metadata |
| `membership_plan` | sdkwork-membership | Membership plan/rank definitions |
| `membership_plan_version` | sdkwork-membership | Published plan versions |
| `benefit_definition` | sdkwork-membership | Entitlement/privilege dictionary |
| `membership_plan_benefit` | sdkwork-membership | Plan-version benefit bindings |
| `membership_package_group` | sdkwork-membership | Billing-cycle package groups |
| `membership_package` | sdkwork-membership | Sellable membership packages |
| `membership_subscription` | sdkwork-membership | User subscription reservation and active-state record |
| `membership_period` | sdkwork-membership | Subscription period reservation and activation record |
| `entitlement_account` | sdkwork-membership | Per-user entitlement balances and usage counters |
| `entitlement_grant` | sdkwork-membership | Pending/active entitlement grants from membership fulfillment |
| `entitlement_ledger_entry` | sdkwork-membership | Entitlement grant/use audit ledger |
| `commerce_account` | sdkwork-membership | Membership points account projection |
| `commerce_account_ledger` | sdkwork-membership | Membership points ledger projection |
| `commerce_membership_daily_reward` | sdkwork-membership | Daily check-in reward tracking |
| `commerce_membership_privilege_usage` | sdkwork-membership | Per-period privilege consumption summary |
| `commerce_membership_change_log` | sdkwork-membership | Immutable membership state-change audit log |

The baseline does not create and seeds do not insert any of these external tables:

- order-owned: `commerce_order`, `commerce_order_item`, `commerce_order_amount_breakdown`
- payment-owned: `commerce_payment_intent`, `commerce_payment_attempt`, `commerce_payment_method`, `commerce_payment_provider`, `commerce_payment_provider_account`, `commerce_payment_channel`, `commerce_payment_route_rule`
- order/payment adjacent commerce tables: `commerce_recharge_package`, `commerce_exchange_rule`

## Checkout And Fulfillment Data Flow

Membership checkout uses an order-first flow:

```text
PC service
  -> @sdkwork/order-app-sdk memberships.orders.create({ packageId, paymentMethod })
  -> @sdkwork/membership-app-sdk memberships.purchases.create|renew|upgrade({ packageId, orderId, requestNo, couponId? })
  -> @sdkwork/order-app-sdk orders.payments.create(orderId, { paymentMethod })
  -> sdkwork-order settlement
  -> membership fulfillment port activates subscription and entitlements
```

The membership database only persists the pending membership reservation and later activation state. It does not initialize order rows, payment rows, cashier configuration, PSP webhook rows, token-plan order rows, recharge packages, or exchange rules.

## Baseline And Seeds

- `database/ddl/baseline/postgres/0001_membership_baseline.sql`
- `database/ddl/baseline/sqlite/0001_membership_baseline.sql`
- `database/seeds/common/001_bootstrap.sql`

The baseline is the greenfield initialization snapshot for this pre-launch application. The migration directories are reserved for post-GA incremental changes and are intentionally empty unless a released schema needs expand/contract evolution.

Seed data covers authenticated frontend membership flows for the demo tenant:

- tenant `100001`
- organization `0`
- user `1`
- 4 package groups: yearly, monthly, quarterly, one-time monthly
- 12 sellable packages: external ids `101`-`403`
- active demo subscription, period, entitlement accounts, grants, usage, daily reward, points account, and points ledger

Local checkout tests that need real order/payment rows must start the `sdkwork-order` and `sdkwork-payment` lifecycles separately. This repository must not add order/payment DDL or seed rows for convenience.

## Contract Registry

- `contract/schema.yaml` - 18 table contracts registered with profile, compliance level, and owner
- `contract/table-registry.json` - machine-readable inventory of the 18 initialized tables
- `contract/prefix-registry.json` - `commerce_` prefix registration for commerce-scoped membership extension tables
- `database.manifest.json` - database module manifest used by `sdkwork-database`

Regenerate the contract from the PostgreSQL baseline after DDL changes:

```bash
pnpm run db:materialize:contract
```

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
pnpm run db:bootstrap
```

`db:validate` delegates to `../sdkwork-specs/tools/check-database-framework-standard.mjs`. Runtime lifecycle commands delegate to `sdkwork-database-cli`.

## Verification

Before completing database or repository boundary changes, run:

```bash
pnpm run db:materialize:contract
pnpm run db:validate
cargo test -p sdkwork-membership-repository-sqlx --test membership_sqlx_standard -- --nocapture
node --test --test-reporter=spec --experimental-strip-types tests/static/membership-standards-regression.test.mjs
```
