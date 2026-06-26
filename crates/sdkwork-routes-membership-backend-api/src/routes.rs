use axum::Router;
use sdkwork_commerce_membership_repository_sqlx::{
    admin_membership_router_with_postgres_pool, admin_membership_router_with_sqlite_pool,
};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_membership_service_host::MembershipServiceHost;
use std::sync::Arc;

use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

pub fn build_membership_backend_router(host: Arc<MembershipServiceHost>) -> Router {
    match host.database_pool() {
        DatabasePool::Postgres(pool, _) => {
            admin_membership_router_with_postgres_pool(pool.clone())
        }
        DatabasePool::Sqlite(pool, _) => {
            admin_membership_router_with_sqlite_pool(pool.clone())
        }
    }
}

pub async fn build_membership_backend_router_with_framework(
    host: Arc<MembershipServiceHost>,
) -> Router {
    wrap_router_with_web_framework_from_env(build_membership_backend_router(host)).await
}
