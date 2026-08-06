use sdkwork_database_sqlx::DatabasePool;
use sdkwork_membership_database_host::{
    bootstrap_membership_database_from_env, bootstrap_membership_database_host_with_pool,
    MembershipDatabaseHost,
};
use sdkwork_membership_repository_sqlx::{AppMembershipStore, PostgresCommerceMembershipStore};
use std::sync::Once;

pub struct MembershipServiceHost {
    database: MembershipDatabaseHost,
}

impl MembershipServiceHost {
    pub async fn new() -> Result<Self, String> {
        Self::from_env().await
    }

    pub async fn from_env() -> Result<Self, String> {
        let database = bootstrap_membership_database_from_env().await?;
        Ok(Self { database })
    }

    pub async fn from_pool(pool: &DatabasePool) -> Result<Self, String> {
        let database = bootstrap_membership_database_host_with_pool(pool).await?;
        Ok(Self { database })
    }

    pub fn database_pool(&self) -> &DatabasePool {
        self.database.pool()
    }

    pub fn postgres_pool(&self) -> &sqlx::PgPool {
        self.database.postgres_pool()
    }

    pub fn database_module(&self) -> std::sync::Arc<sdkwork_database_spi::DefaultDatabaseModule> {
        self.database.module()
    }

    /// 会员订阅生命周期 worker：定期扫描到期订阅并执行过期处理。
    /// advisory lock 防多实例并发；Once 防同一进程重复 spawn。
    #[allow(irrefutable_let_patterns)]
    pub fn spawn_membership_lifecycle_worker(&self) {
        // 服务端权威持久化仅支持 PostgreSQL（DATABASE_SPEC：authoritative-server）；
        // 在联合工作区（如 cloudrouter）中 Sqlite 变体仍可能因 client-local feature 存在，故保留守卫。
        let DatabasePool::Postgres(pool, _) = self.database_pool() else {
            tracing::warn!("membership lifecycle worker requires a PostgreSQL pool; skipped");
            return;
        };
        let pool = pool.clone();
        let interval_seconds = std::env::var("MEMBERSHIP_LIFECYCLE_SWEEP_INTERVAL_SECONDS")
            .ok()
            .and_then(|value| value.trim().parse::<u64>().ok())
            .unwrap_or(DEFAULT_LIFECYCLE_SWEEP_INTERVAL_SECONDS);
        static SPAWNED: Once = Once::new();
        SPAWNED.call_once(|| {
            tracing::info!(interval_seconds, "spawning membership lifecycle worker");
            tokio::spawn(async move {
                let store = PostgresCommerceMembershipStore::new(pool.clone());
                loop {
                    if let Err(error) = store.expire_due_memberships().await {
                        tracing::warn!(
                            error = error.message(),
                            "membership lifecycle sweep failed"
                        );
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(interval_seconds)).await;
                }
            });
        });
    }
}

/// 会员订阅生命周期扫描默认间隔（秒）：300（5 分钟）。
const DEFAULT_LIFECYCLE_SWEEP_INTERVAL_SECONDS: u64 = 300;
