# Membership PRD

Status: active
Owner: SDKWork maintainers
Application: membership
Updated: 2026-06-24
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- Platform split alignment (commerce T0): `../sdkwork-commerce/docs/architecture/tech/TECH-2026-06-24-commerce-capability-repo-split-alignment.md`

## 1. Background And Problem

Membership tiers, entitlements, and payment-center seed data need a dedicated capability without composing the entire commerce router surface.

This repository is a **T1 commerce capability building block**. `sdkwork-commerce` remains the T0 composition layer (gateway, IAM wrappers, composed SDK). This repository owns domain logic, persistence, and HTTP route builders for the **membership** capability.

## 2. Target Users

Subscription operators, entitlement administrators, and buyers managing membership benefits.

## 3. Goals And Non-Goals

### Goals

- Own membership service, repository SQL, and membership-specific routes.
- Platform router composition lives in commerce T0 (`sdkwork-commerce-router-composition`).

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
- Phase 2c (complete): platform router composition moved to commerce T0.

## 8. Linked Requirements

- Commerce capability split alignment: `../sdkwork-commerce/docs/architecture/tech/TECH-2026-06-24-commerce-capability-repo-split-alignment.md`
- Component contract: `specs/component.spec.json` (when present)
- Machine contracts: local `specs/`, future `apis/`, and generated `sdks/`

## 9. Open Questions

- Migrate membership HTTP from repository crate to router crates when routes stabilize.
