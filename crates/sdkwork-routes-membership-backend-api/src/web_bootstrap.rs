use axum::Router;
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_core::WebRequestContextProfile;

pub async fn wrap_router_with_web_framework_from_env(router: Router) -> Router {
    let resolver = sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env().await;
    let environment = sdkwork_web_bootstrap::web_environment_from_env(&[
        "SDKWORK_MEMBERSHIP_ENVIRONMENT",
        "SDKWORK_LIFECYCLE_ENVIRONMENT",
        "SDKWORK_ENVIRONMENT",
        "SDKWORK_ENV",
    ]);
    let security_policy = sdkwork_web_bootstrap::security_policy_for_environment(
        &environment,
        sdkwork_web_bootstrap::cors_allowed_origins_from_env(&[
            "SDKWORK_MEMBERSHIP_CORS_ALLOWED_ORIGINS",
            "SDKWORK_CORS_ALLOWED_ORIGINS",
        ]),
    );
    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(WebRequestContextProfile {
            public_path_prefixes: sdkwork_web_bootstrap::infra_public_path_prefixes(),
            environment,
            ..WebRequestContextProfile::default()
        })
        .with_security_policy(security_policy);
    with_web_request_context(router, layer)
}
