//! Gateway assembly for sdkwork-membership.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_api_router, ApiAssembly};

pub async fn assemble_backend_business_router(
    host: std::sync::Arc<sdkwork_membership_service_host::MembershipServiceHost>,
) -> ApiAssembly {
    ApiAssembly {
        router: sdkwork_routes_membership_backend_api::gateway_mount(host).await,
    }
}

pub async fn assemble_api_router_with_process_pool(
    pool: &sdkwork_database_sqlx::DatabasePool,
) -> Result<ApiAssembly, String> {
    let host = std::sync::Arc::new(
        sdkwork_membership_service_host::MembershipServiceHost::from_pool(pool).await?,
    );
    Ok(assemble_api_router(host).await)
}

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let host = std::sync::Arc::new(
        sdkwork_membership_service_host::MembershipServiceHost::from_env().await?,
    );
    Ok(assemble_api_router(host).await)
}

pub async fn assemble_backend_business_router_from_env() -> Result<ApiAssembly, String> {
    let host = std::sync::Arc::new(
        sdkwork_membership_service_host::MembershipServiceHost::from_env().await?,
    );
    Ok(assemble_backend_business_router(host).await)
}

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
