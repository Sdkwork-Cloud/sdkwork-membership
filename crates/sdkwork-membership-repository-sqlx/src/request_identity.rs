// NOTE: sdkwork_commerce_api_server was deleted
// The original functions with_commerce_app_request_context and with_commerce_backend_request_context
// are no longer available. Providing stub implementations for compatibility.

use axum::Router;

pub(crate) use sdkwork_web_core::resolve_request_id;

// Stub implementations for deleted commerce_api_server functions
// TODO: Replace with proper middleware implementations from sdkwork_web_core

pub(crate) fn with_request_identity(router: Router) -> Router {
    // Stub: return router unchanged (no identity middleware layer)
    router
}

pub(crate) fn with_backend_request_identity(router: Router) -> Router {
    // Stub: return router unchanged (no identity middleware layer)
    router
}
