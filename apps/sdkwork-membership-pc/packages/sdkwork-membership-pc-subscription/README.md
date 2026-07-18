# @sdkwork/membership-pc-subscription

## Purpose

Subscription checkout, coupon application, and premium membership selection.

## Placement

- Architecture: `pc-react`
- Domain: `commerce`
- Capability: `subscription`
- Status: `ready`

## Depends on

- `@sdkwork/ui-pc-react` for shared UI primitives and patterns
- `@sdkwork/membership-service` for generated app/backend SDK boundaries, session checks, and response normalization
- Lower-level foundation host packages only

## Ownership

This package is implemented as an independent SDKWork commerce capability. It owns its public React/service contracts and consumes commerce data through injected service boundaries with wallet and membership ownership kept separate.

## Runtime boundary

All remote commerce access goes through `@sdkwork/membership-service` or through sibling commerce services that use the same boundary. Generated SDK clients remain behind the shared service contract.

Subscription catalog reads and purchases are orchestrated by:

- `subscription-catalog-service` — `memberships.packageGroups.list`, `plans.list`, `benefits.list`, purchase delegation
- `subscription-service` — checkout dashboard, coupons, payment methods, `memberships.purchases.*`
- `subscription-catalog-controller` — page lifecycle for `SubscriptionCatalogPage`

Static fixtures in `subscription-catalog-content.ts` are offline fallbacks only; production paths load data from app-api via SDK.

**Commerce checkout boundary:** The subscription package does not import order or payment SDKs, services, or UI packages. It delegates purchase intent and status reads through a host-injected `SdkworkMembershipCheckoutPort`. A product application composition root supplies the order-owned service and checkout UI. After payment success, order calls the membership fulfillment port so `pending_activation` subscriptions become active.

## Verification

Use the package `typecheck` script and focused Vitest coverage for service, controller, and UI behavior when changing this package.

## SDKWork Documentation Contract

Domain: commerce
Capability: subscription
Package type: react-package
Status: ready

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `pnpm --filter @sdkwork/membership-pc-subscription typecheck`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
