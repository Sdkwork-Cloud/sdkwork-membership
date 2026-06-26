pub mod routes;
pub mod web_bootstrap;

pub use routes::build_membership_backend_router_with_framework;

pub async fn gateway_mount(host: Arc<MembershipServiceHost>,) -> Router {
    build_membership_backend_router_with_framework(host).await
}
