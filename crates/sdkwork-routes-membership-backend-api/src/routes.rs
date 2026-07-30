use crate::admin_router::admin_membership_router_with_postgres_pool;
use axum::Router;
use sdkwork_membership_service_host::MembershipServiceHost;
use std::sync::Arc;

use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

pub fn build_membership_backend_router(host: Arc<MembershipServiceHost>) -> Router {
    admin_membership_router_with_postgres_pool(host.postgres_pool().clone())
}

pub async fn build_membership_backend_router_with_framework(
    host: Arc<MembershipServiceHost>,
) -> Router {
    wrap_router_with_web_framework_from_env(build_membership_backend_router(host)).await
}
