# sdkwork-membership

SDKWork commerce **membership** capability building-block repository (domain `commerce`).

- Standards: `../sdkwork-specs/README.md`
- Composition consumer: `../sdkwork-clawrouter/vendor/sdkwork-commerce` (archived transitional platform snapshot)
- Domain service: `crates/sdkwork-commerce-membership-service/`
- Repository SQL: `crates/sdkwork-commerce-membership-repository-sqlx/`
- HTTP API server: `crates/sdkwork-membership-standalone-gateway/`

## Quick start

```bash
cargo test --workspace
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Application Roots

- [apps directory index](apps/README.md)
