use std::sync::Arc;

use sdkwork_api_membership_assembly::assemble_api_router;
use sdkwork_membership_service_host::MembershipServiceHost;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(error) = run().await {
        eprintln!("membership gateway failed: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let host = Arc::new(MembershipServiceHost::from_env().await?);
    let business = assemble_api_router(host).await.router;
    // CORS is handled by the sdkwork-web-framework standard interceptor
    // chain (deny-by-default). Do not add permissive CORS here.
    let app = service_router(business, ServiceRouterConfig::default().with_always_ready());
    let addr = std::env::var("MEMBERSHIP_API_BIND").unwrap_or_else(|_| "0.0.0.0:18096".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|error| format!("failed to bind membership gateway on {addr}: {error}"))?;

    tracing::info!("membership gateway listening on {addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|error| format!("membership gateway serve failed: {error}"))?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::warn!("failed to listen for ctrl-c: {error}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        if let Err(error) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install SIGTERM handler")
                .recv()
                .await
        {
            tracing::warn!("failed to listen for SIGTERM: {error}");
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    tracing::info!("membership gateway shutdown signal received");
}
