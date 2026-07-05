use sdkwork_database_sqlx::DatabasePool;
use sdkwork_membership_database_host::{
    bootstrap_membership_database_from_env, MembershipDatabaseHost,
};

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

    pub fn database_pool(&self) -> &DatabasePool {
        self.database.pool()
    }

    pub fn database_module(&self) -> std::sync::Arc<sdkwork_database_spi::DefaultDatabaseModule> {
        self.database.module()
    }
}
