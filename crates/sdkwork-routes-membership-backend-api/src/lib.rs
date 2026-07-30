use axum::Router;
use sdkwork_membership_service_host::MembershipServiceHost;
use std::sync::Arc;

mod admin_router;
mod http_route_manifest;
pub mod paths;
mod response;
pub mod routes;
mod subject;
pub mod web_bootstrap;

pub use http_route_manifest::backend_route_manifest;
pub use routes::{build_membership_backend_router, build_membership_backend_router_with_framework};
pub use web_bootstrap::wrap_router_with_web_framework_from_env;

pub async fn gateway_mount(host: Arc<MembershipServiceHost>) -> Router {
    build_membership_backend_router(host)
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    backend_route_manifest()
}
