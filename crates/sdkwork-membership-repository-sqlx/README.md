# sdkwork-membership-repository-sqlx

Domain: commerce
Capability: membership
Package type: rust-crate
Status: stable

This README is the SDKWork module entrypoint for `sdkwork_membership_repository_sqlx`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Public API

- `PostgresCommerceMembershipStore` - tenant-scoped authoritative membership persistence.
- `app_membership_router_with_*`, `admin_membership_router_with_*` — Axum routers emitting canonical `SdkWorkApiResponse` / `application/problem+json` envelopes.
- `TimestampMembershipEntityIdGenerator` — default entity id generator for command surfaces.

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Configuration keys, runtime entrypoints, and integration contracts are declared in `specs/component.spec.json`. Shared modules must receive configuration through typed bootstrap or service boundaries rather than reading host-local environment state directly.

## Deployment Behavior

This repository is an `authoritative-server` module and accepts PostgreSQL only. Standalone and
cloud hosts inject the process-shared pool; SQLite is not a server fallback or test substitute.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API and SDK access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test --manifest-path crates/sdkwork-membership-repository-sqlx/Cargo.toml`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
