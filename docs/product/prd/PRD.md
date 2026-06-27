# Membership PRD

Status: active
Owner: SDKWork maintainers
Application: membership
Updated: 2026-06-24
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- Commerce repository dissolution: `../sdkwork-specs/MIGRATION_SPEC.md` §8

## 1. Background And Problem

Membership tiers, entitlements, and payment-center seed data need a dedicated capability without composing the entire commerce router surface.

This repository is a **T1 commerce capability building block**. The `sdkwork-commerce` monolith has been dissolved; this repository is self-contained with its own domain logic, persistence, HTTP route builders, API server, and IAM middleware for the **membership** capability.

## 2. Target Users

Subscription operators, entitlement administrators, and buyers managing membership benefits.

## 3. Goals And Non-Goals

### Goals

- Own membership service, repository SQL, and membership-specific routes.
- Platform router composition lives in the T1 `sdkwork-membership-standalone-gateway`.

### Non-Goals

- Owning non-membership commerce routes in membership repository crate.

## 4. Scope

- Membership service and repository (migrated).
- Membership app and admin HTTP routes.

Primary API prefixes:

- App: `/app/v3/api/membership`
- Backend: `/backend/v3/api/membership`

Migration status: **complete**.

## 5. User Scenarios

- An operator configures membership tiers and entitlements for a tenant.
- Integration tests seed payment-center data through membership repository helpers.

## 6. Success Metrics

- Membership repository tests pass without local duplicate in commerce storage.
- Membership repository exports membership-only routers and seed helpers.

## 7. Phases

- Phase 1 (complete): service + repository in sibling repo.
- Phase 2c (complete): platform router composition moved to T1 `*-standalone-gateway`.

## 8. Linked Requirements

- Commerce repository dissolution: `../sdkwork-specs/MIGRATION_SPEC.md` §8
- Component contract: `specs/component.spec.json` (when present)
- Machine contracts: local `specs/`, future `apis/`, and generated `sdks/`

## 9. Open Questions

- Migrate membership HTTP from repository crate to router crates when routes stabilize.
