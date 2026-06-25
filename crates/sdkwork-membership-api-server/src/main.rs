use axum::Router;
use sdkwork_router_membership_app_api::build_membership_app_router_with_framework;
use sdkwork_router_membership_backend_api::build_membership_backend_router_with_framework;
use sdkwork_membership_api_server::membership_health_router;
use sdkwork_membership_service_host::MembershipServiceHost;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let host = Arc::new(MembershipServiceHost::new().await);
    let app = Router::new()
        .merge(membership_health_router())
        .merge(build_membership_app_router_with_framework(host.clone()).await)
        .merge(build_membership_backend_router_with_framework(host).await)
        .layer(CorsLayer::permissive());
    let addr = std::env::var("MEMBERSHIP_API_BIND").unwrap_or_else(|_| "0.0.0.0:18096".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
