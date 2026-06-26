pub mod routes;
pub mod web_bootstrap;

pub use routes::build_membership_app_router_with_framework;
pub use web_bootstrap::wrap_router_with_web_framework_from_env;

pub async fn gateway_mount(host: Arc<MembershipServiceHost>,) -> Router {
    build_membership_app_router_with_framework(host).await
}
