use axum::Router;
use sdkwork_membership_service_host::MembershipServiceHost;
use std::sync::Arc;

mod admin_router;
pub mod manifest;
pub mod paths;
mod response;
pub mod routes;
mod subject;
pub mod web_bootstrap;

pub use manifest::BACKEND_API_ROUTE_MANIFEST;
pub use routes::build_membership_backend_router_with_framework;
pub use web_bootstrap::wrap_router_with_web_framework_from_env;

pub async fn gateway_mount(host: Arc<MembershipServiceHost>) -> Router {
    build_membership_backend_router_with_framework(host).await
}
