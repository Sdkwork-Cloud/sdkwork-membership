use axum::Router;
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_core::WebRequestContextProfile;

use crate::http_route_manifest::backend_route_manifest;

pub async fn wrap_router_with_web_framework_from_env(router: Router) -> Router {
    let resolver = sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env().await;
    let (environment, security_policy) =
        sdkwork_web_bootstrap::application_security_policy_from_env(
            &[
                "SDKWORK_ENVIRONMENT",
                "SDKWORK_MEMBERSHIP_ENVIRONMENT",
                "SDKWORK_LIFECYCLE_ENVIRONMENT",
                "SDKWORK_ENV",
            ],
            &["SDKWORK_CORS_ALLOWED_ORIGINS"],
        );
    let route_manifest = backend_route_manifest();
    let public_path_prefixes = sdkwork_web_bootstrap::infra_public_path_prefixes();
    route_manifest
        .validate_public_path_prefixes(&public_path_prefixes)
        .expect("membership backend-api public prefixes must not cover protected manifest routes");
    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(WebRequestContextProfile {
            public_path_prefixes,
            environment,
            ..WebRequestContextProfile::default()
        })
        .with_security_policy(security_policy)
        .with_route_manifest(route_manifest);
    with_web_request_context(router, layer)
}
