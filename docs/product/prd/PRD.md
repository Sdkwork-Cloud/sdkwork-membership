# Membership PRD

Status: active
Owner: SDKWork maintainers
Application: membership
Updated: 2026-07-01
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Background And Problem

Membership tiers, entitlements, and payment-center seed data need a dedicated capability without composing the entire commerce router surface.

This repository is a **T1 commerce capability building block**. This repository is self-contained with its own domain logic, persistence, HTTP route builders, API server, and IAM middleware for the **membership** capability.

## 2. Target Users

Subscription operators, entitlement administrators, and buyers managing membership benefits.

## 3. Goals And Non-Goals

### Goals

- Own membership service, repository SQL, and membership-specific routes.
- Platform router composition lives in the T1 `sdkwork-membership-standalone-gateway`.

### Non-Goals

- Owning non-membership commerce routes in membership repository crate.

## 4. Scope

- Membership service and repository.
- Membership app and admin HTTP routes.

Primary API prefixes:

- App: `/app/v3/api/membership`
- Backend: `/backend/v3/api/membership`

## 5. User Scenarios

- An operator configures membership tiers and entitlements for a tenant.
- Integration tests seed payment-center data through `sdkwork-database-cli` seed mechanism.

## 6. Success Metrics

- Membership repository tests pass.
- Membership repository exports membership-only routers and store adapters.

## 7. Phases

- Phase 1 (complete): service + repository in this repo.
- Phase 2c (complete): platform router composition moved to T1 `*-standalone-gateway`.

## 8. Linked Requirements

- Component contract: `specs/component.spec.json` (when present)
- Machine contracts: local `specs/`, future `apis/`, and generated `sdks/`

## 9. Open Questions

- Migrate membership HTTP from repository crate to router crates when routes stabilize.
