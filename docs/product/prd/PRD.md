# Membership PRD

Status: active
Owner: SDKWork maintainers
Application: membership
Updated: 2026-07-05
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Background And Problem

Membership tiers, entitlements, and subscription checkout need a dedicated commerce capability with production-grade pagination, tenant isolation, and a runnable PC surface for operators and buyers.

This repository is a **T1 commerce capability building block**. It is self-contained with domain logic, SQLx persistence, HTTP route builders, API server, IAM middleware, and an embeddable PC React application.

## 2. Target Users

- Subscription operators configuring tiers, packages, and entitlements per tenant
- Buyers comparing plans, completing checkout, and managing membership benefits
- Platform integrators embedding membership UI and APIs into mall/commerce hosts

## 3. Goals And Non-Goals

### Goals

- Own membership service contracts, SQLx repositories, app/backend HTTP routes, and PC UI packages.
- Emit canonical `SdkWorkApiResponse` envelopes with `SdkWorkPageData` (`items` + `pageInfo`) for all list/search APIs.
- Enforce tenant-scoped admin mutations and SQL-level pagination (no in-process full collect + slice).
- Provide a runnable PC shell (`/membership`, `/subscription/checkout`) with SDK bootstrap and pending-payment UX.
- Ship database catalog seeds for tenant `100001` demo/bootstrap flows.

### Non-Goals

- Owning non-membership commerce routes (orders, wallet, promotions) in this repository.
- Replacing the payment provider; membership integrates payment rails through composed SDK boundaries.

## 4. Scope

- Membership SQLx repository and HTTP adapters (`/app/v3/api/memberships`, `/backend/v3/api/memberships`).
- Standalone gateway process and gateway assembly manifest.
- PC packages: `membership-pc-membership`, `membership-pc-subscription`, `membership-pc-shell`.
- OpenAPI authority and SDK generation inputs under `apis/` and `sdks/`.

## 5. User Scenarios

- An operator lists and mutates membership plans/packages for their tenant through backend APIs with pagination and tenant isolation.
- A signed-in buyer opens the PC membership page, compares tiers from live API data, and completes checkout with pending-payment handoff.
- Integration tests and local dev bootstrap catalog data through `database/seeds/common/001_bootstrap.sql`.

## 6. Success Metrics

- `pnpm test:vitest` and `cargo test --workspace` pass.
- Governance checks pass: API envelope, pagination heuristic, SDK consumer imports.
- List APIs return `data.items` + `data.pageInfo`; admin mutations are tenant-scoped.
- PC shell routes render membership and subscription flows without placeholder-only content.

## 7. Phases

- **Phase 1 (complete):** service contracts + SQLx repository + route crates.
- **Phase 2 (complete):** standalone gateway assembly and database framework integration.
- **Phase 3 (complete):** pagination/tenant-isolation hardening, catalog seeds, PC shell, pending-payment UX, composed `@sdkwork/membership-app-sdk`, purchase idempotency via server `requestId`, OpenAPI list envelope alignment.

## 8. Linked Requirements

- Component contract: `specs/component.spec.json` (when present)
- Machine contracts: local `specs/`, `apis/`, and `sdks/`
- Canon docs: `docs/product/prd/PRD.md`, `docs/architecture/tech/TECH_ARCHITECTURE.md`

## 9. Open Questions

- Wire payment provider callbacks (`sdkwork-payment`) so `pending` purchases activate membership after paid confirmation.
- Add HTTP integration tests against sqlite test database for purchase and admin CRUD paths.
- Integrate IAM PC login bootstrap for production auth (replace env-token dev bootstrap).
