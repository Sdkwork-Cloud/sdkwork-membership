use axum::Router;
use sdkwork_membership_gateway_assembly::assemble_application_router;
use sdkwork_membership_service_host::MembershipServiceHost;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let host = Arc::new(MembershipServiceHost::new().await);
    let business = assemble_application_router(host).await.router
        .layer(CorsLayer::permissive());
    let app = service_router(business, ServiceRouterConfig::default().with_always_ready());
    let addr = std::env::var("MEMBERSHIP_API_BIND").unwrap_or_else(|_| "0.0.0.0:18096".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
