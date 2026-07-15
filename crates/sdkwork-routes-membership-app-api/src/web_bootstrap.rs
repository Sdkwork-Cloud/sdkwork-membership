use axum::Router;
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_core::WebRequestContextProfile;

use crate::manifest::APP_API_HTTP_ROUTE_MANIFEST;

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
        .with_security_policy(security_policy)
        .with_route_manifest(APP_API_HTTP_ROUTE_MANIFEST);
    with_web_request_context(router, layer)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;

    fn request_with_origin(origin: &str) -> Request {
        Request::builder()
            .uri("/app/v3/api/memberships/current/status")
            .header("origin", origin)
            .body(Body::empty())
            .expect("request")
    }

    #[test]
    fn development_policy_allows_loopback_on_arbitrary_ports() {
        let policy = sdkwork_web_bootstrap::security_policy_for_environment(
            &sdkwork_web_core::WebEnvironment::Dev,
            Vec::new(),
        );
        for origin in ["http://localhost:3901", "http://127.0.0.1:5173"] {
            policy
                .validate_cors(&request_with_origin(origin))
                .expect("development loopback origin");
        }
    }

    #[test]
    fn production_policy_requires_an_explicit_origin() {
        let allowed = "https://app.sdkwork.com".to_owned();
        let policy = sdkwork_web_bootstrap::security_policy_for_environment(
            &sdkwork_web_core::WebEnvironment::Prod,
            vec![allowed.clone()],
        );
        policy
            .validate_cors(&request_with_origin(&allowed))
            .expect("configured production origin");
        policy
            .validate_cors(&request_with_origin("http://localhost:3901"))
            .expect_err("production must reject implicit loopback origins");
    }
}
