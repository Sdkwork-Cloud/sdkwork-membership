use axum::Router;
use sdkwork_membership_service_host::MembershipServiceHost;
use std::sync::Arc;

pub mod manifest;
pub mod paths;
mod response;
mod router;
pub mod routes;
mod subject;
pub mod web_bootstrap;

pub use manifest::{APP_API_HTTP_ROUTE_MANIFEST, APP_API_ROUTE_MANIFEST};
pub use router::{
    app_membership_router_with_postgres_pool, app_membership_router_with_sqlite_pool,
};
pub use routes::build_membership_app_router_with_framework;
pub use web_bootstrap::wrap_router_with_web_framework_from_env;

pub async fn gateway_mount(host: Arc<MembershipServiceHost>) -> Router {
    build_membership_app_router_with_framework(host).await
}
