use axum::Router;
use sdkwork_commerce_membership_repository_sqlx::{
    app_membership_router_with_postgres_pool, app_membership_router_with_sqlite_pool,
};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_membership_service_host::MembershipServiceHost;
use std::sync::Arc;

use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

pub fn build_membership_app_router(host: Arc<MembershipServiceHost>) -> Router {
    match host.database_pool() {
        DatabasePool::Postgres(pool, _) => {
            app_membership_router_with_postgres_pool(pool.clone())
        }
        DatabasePool::Sqlite(pool, _) => app_membership_router_with_sqlite_pool(pool.clone()),
    }
}

pub async fn build_membership_app_router_with_framework(
    host: Arc<MembershipServiceHost>,
) -> Router {
    wrap_router_with_web_framework_from_env(build_membership_app_router(host)).await
}
