# Commerce Order And Membership Boundary Spec

Status: active
Owner: SDKWork maintainers
Capability: `commerce.membership`
Updated: 2026-07-17
Machine contract: `specs/commerce-order-membership-boundary.spec.json`

## 1. Purpose

Fix the dependency direction for membership purchase flows. `sdkwork-membership` owns membership catalog, state, subscriptions, entitlements, and fulfillment results. It does not own order creation, payment status, or checkout UI. Those capabilities belong to `sdkwork-order`.

## 2. Capability Roles

| Capability | Repository | Owns |
| --- | --- | --- |
| Order | `sdkwork-order` | `commerce_order*`, membership-subject order creation, checkout UI/service, payment status, settlement orchestration |
| Payment | `sdkwork-payment` | Payment execution, attempts, providers, refunds, and webhook persistence behind order-owned ports |
| Membership | `sdkwork-membership` | Plans, packages, membership state, subscriptions, entitlements, and idempotent fulfillment after payment success |
| Application composition | Product application root such as `sdkwork-clawrouter` | Composes membership and order packages and injects the order-owned checkout implementation into membership UI |

## 3. Mandatory Dependency Direction

```text
sdkwork-order -> sdkwork-payment
sdkwork-order -> sdkwork-membership fulfillment port
application composition root -> sdkwork-membership + sdkwork-order
sdkwork-membership -> membership SDK only
```

The following dependencies are forbidden in `sdkwork-membership` application, service, shell, and UI packages:

- Any `@sdkwork/order-*` package, including app SDK, service, checkout, and recharge packages.
- Any `@sdkwork/payment-*` package used for checkout or payment execution.
- Any `sdkwork-order-*` or `sdkwork-payment-*` Rust crate.
- Raw order/payment HTTP, manual auth headers, or local order transport facades.
- Direct writes to `commerce_order*` or `commerce_payment*` tables.

## 4. Checkout Composition Contract

Membership frontend packages may define and consume a domain-neutral, host-injected `SdkworkMembershipCheckoutPort`. The port carries membership intent and normalized checkout results only:

- `createCheckout({ action, packageId, couponId?, paymentMethod? })`
- `getCheckoutStatus(orderId)`

The port is implemented outside `sdkwork-membership`. A product application composition root creates the implementation from `sdkwork-order` and injects it into the membership catalog. The checkout modal is also supplied by the composition root and owned by `sdkwork-order`.

The standalone membership application intentionally has no default ordering capability. Without a host-provided port it must return an explicit configuration error instead of bootstrapping an order SDK.

## 5. Canonical Purchase Flow

1. Membership UI selects a package and delegates a membership checkout intent through the injected port.
2. The order-owned checkout service calls `memberships.orders.create` and returns normalized payment data.
3. The order-owned checkout UI renders payment state and polls `orders.paymentSuccess.retrieve` through the same order-owned service.
4. Order settles payment through payment ports.
5. Order calls the membership fulfillment port after successful settlement.
6. Membership activates the subscription and grants entitlements idempotently.

## 6. Verification

- `rg '@sdkwork/order|@sdkwork/payment' apps packages` returns no active membership consumer dependency.
- `rg 'sdkwork_order|commerce_order' crates/**/Cargo.toml crates` returns no membership-to-order production dependency or write path.
- The membership workspace and app manifest contain no order SDK package or order base URL.
- The product composition root proves injection of an order-owned checkout service and checkout UI.
- `node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .`

## 7. Related Specs

- `../sdkwork-order/specs/commerce-checkout-topology.spec.json`
- `../sdkwork-order/specs/RECHARGE_ORDER_SPEC.md`
- `../sdkwork-order/docs/architecture/commerce/COMMERCE_CHECKOUT_ARCHITECTURE.md`
- `../sdkwork-payment/specs/commerce-dependency-boundary.spec.json`
