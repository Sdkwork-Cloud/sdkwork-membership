# Commerce Order And Membership Boundary Spec

Status: active  
Owner: SDKWork maintainers  
Capability: `commerce.membership`  
Updated: 2026-07-06  
Machine contract: `specs/commerce-order-membership-boundary.spec.json`

Authority: `../sdkwork-order/specs/RECHARGE_ORDER_SPEC.md`, `../sdkwork-order/specs/commerce-checkout-topology.spec.json`, `../sdkwork-payment/specs/commerce-dependency-boundary.spec.json`

## 1. Purpose

Eliminate cross-capability confusion by fixing **who creates orders**, **who settles payment**, and **who fulfills membership** when `subject=membership`.

This spec applies to all deployments (standalone gateway, composed mall, platform gateway).

## 2. Capability roles

| Capability | Repository | Owns |
| --- | --- | --- |
| **Order** | `sdkwork-order` | `commerce_order*` headers, checkout, `orders.payments.create`, PSP webhook ingestion, payment settlement orchestration |
| **Payment** | `sdkwork-payment` | Payment execution (`commerce_payment_intent`, attempts, providers, refunds); webhook persistence **via port only** |
| **Membership** | `sdkwork-membership` | Plans, packages, subscriptions, entitlements, points; **fulfillment after order payment success** |

## 3. Mandatory dependency direction

```text
order  →  payment     (in-process ports; payment MUST NOT depend on order)
order  →  membership  (fulfillment port after settlement; membership MUST NOT write commerce_order in production)
client →  order-app-sdk, membership-app-sdk (checkout composition; payment executes behind order/payment ports)
client →  payment-app-sdk (optional cashier read/redirect consumer only; not payment execution)
```

**Answer to the common question:** yes — at **gateway assembly / settlement saga** time, **order depends on membership** through a narrow fulfillment port (same pattern as order → account for `points_recharge`). That is **not** membership depending on order service crates to create orders.

| Direction | Allowed | Notes |
| --- | --- | --- |
| Order → Payment | Yes | `orders.payments.create`, webhook ingest, confirm payment |
| Order → Membership | Yes | `MembershipPurchaseFulfillmentPort` or HTTP adapter after `subject=membership` payment success |
| Order → Account | Yes | `points_recharge` in-process saga (reference implementation) |
| Payment → Order | **No** | Foundation module |
| Payment → Membership | **No** | Foundation module |
| Membership → Order (Rust crate) | **No** | Use `@sdkwork/order-app-sdk` at client/service facade |
| Membership PC UI → `@sdkwork/order-pc-checkout` | Yes | Controlled QR checkout UI only; Membership supplies localized copy and an injected payment-status driver |
| Membership → Payment (Rust crate) | **No** | No payment orchestration in membership service |
| Membership → `@sdkwork/payment-app-sdk` | Yes | Optional cashier read/redirect consumer only; checkout payment requests go through `@sdkwork/order-app-sdk` `orders.payments.create` |

## 4. Create and pay boundary (normative)

Analogous to `recharges.orders.create` in `RECHARGE_ORDER_SPEC.md` §7:

| Step | Owner | Operation |
| --- | --- | --- |
| 1. Create order | **Order** | Checkout session or membership-subject order create writes `commerce_order` + items + breakdown only (`subject=membership`) |
| 2. Reserve subscription | **Membership** | Create or link `membership_subscription` in `pending_activation`, keyed by `order_id` |
| 3. Pay | **Order** | `orders.payments.create(orderId, { paymentMethod })` calls the payment port to create intent/attempt and returns payment parameters |
| 4. PSP notify | **Order** | `POST /app/v3/api/orders/payments/webhooks/{providerCode}` |
| 5. Settle | **Order** | `settle_owner_order_after_payment_success` confirms payment via payment port |
| 6. Fulfill | **Order → Membership port** | Activate subscription, grant entitlements; idempotent on `order_id` |

Order create writes **order domain only**. Payment intent/attempt is created by **`orders.payments.create`**, not at membership purchase API.

## 5. What membership MUST NOT do (production)

- Insert or update `commerce_order`, `commerce_order_item`, or `commerce_order_amount_breakdown`.
- Insert `commerce_payment_intent` or orchestrate PSP webhooks.
- Import `sdkwork-order-service` or `sdkwork-order-repository-sqlx`.
- Expose PSP-facing webhook routes for checkout.

## 6. Production checkout path (normative)

Membership standalone gateway does **not** write `commerce_order`. All purchase flows use:

1. `@sdkwork/order-app-sdk` → `memberships.orders.create`
2. `@sdkwork/membership-app-sdk` → `memberships.purchases.*` (reservation with `orderId` + `requestNo`)
3. `@sdkwork/order-app-sdk` → `orders.payments.create(orderId, { paymentMethod })`
4. `@sdkwork/order-pc-checkout` renders the QR code and polls the injected status driver; it has no Membership, Account, or Payment SDK dependency
5. Order webhook settlement → `MembershipPurchaseFulfillmentPort` → membership backend fulfillments API

## 7. Purchase API contract

`memberships.purchases.*` reserves subscription/entitlement rows only. It **requires** `orderId` and `requestNo` from order create (step 1).

## 8. Related specs

- Order checkout topology: `../sdkwork-order/specs/commerce-checkout-topology.spec.json`
- Order recharge reference flow: `../sdkwork-order/specs/RECHARGE_ORDER_SPEC.md`
- Payment dependency boundary: `../sdkwork-payment/specs/commerce-dependency-boundary.spec.json`
- Architecture narrative: `docs/architecture/tech/TECH_ARCHITECTURE.md` section 7

## 9. Verification

- No `sdkwork_order_*` crate dependency in membership `Cargo.toml` workspace members (production).
- App packages consume `@sdkwork/order-app-sdk` for checkout; no raw order HTTP from membership UI.
- Membership PC package consumes `@sdkwork/order-pc-checkout` only through its public export and supplies the driver from Membership service methods.
- `node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .`
