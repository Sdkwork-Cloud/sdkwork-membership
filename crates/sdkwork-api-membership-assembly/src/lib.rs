//! Gateway assembly for sdkwork-membership.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{
    assemble_api_router_from_env, assemble_api_router,
    assemble_backend_business_router, assemble_backend_business_router_from_env,
    ApiAssembly,
};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
