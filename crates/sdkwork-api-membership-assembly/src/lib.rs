//! Host-neutral API assembly for sdkwork-membership.

// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{
    assemble_api_router, assemble_api_router_from_env, assemble_api_router_with_pool,
    assemble_app_api_contribution, assemble_app_api_contribution_with_pool,
    assemble_backend_business_router, assemble_backend_business_router_from_env, ApiAssembly,
    ApiAssemblyContribution, BusinessRouterAssembly,
};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
