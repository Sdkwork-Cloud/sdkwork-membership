use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_lifecycle::{lifecycle_options_from_env, LifecycleOrchestrator};
use sdkwork_database_spi::{
    DatabaseAssetProvider, DatabaseManifest, DefaultDatabaseModule, SpiError,
};
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};
use std::path::PathBuf;
use std::sync::Arc;

pub struct MembershipDatabaseHost {
    pool: DatabasePool,
    module: Arc<DefaultDatabaseModule>,
}

impl MembershipDatabaseHost {
    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    pub fn module(&self) -> Arc<DefaultDatabaseModule> {
        self.module.clone()
    }
}

/// Returns the membership [`DefaultDatabaseModule`] loaded from the membership
/// repository's `database/` directory.
///
/// # Convention
///
/// Each `*-database-host` crate exports this function so that federated hosts
/// (e.g. ClawRouter) can discover and register database modules through a
/// `DatabaseModuleRegistry` without manual per-capability bootstrap wiring.
/// The host builds a registry from all known `*-database-host` crates and calls
/// `RegistryLifecycleOrchestrator::bootstrap_all_from_env()` once on the shared
/// pool — convention over configuration.
pub fn database_module() -> Result<DefaultDatabaseModule, SpiError> {
    let app_root = std::env::var("SDKWORK_MEMBERSHIP_APP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Canonicalize the CARGO_MANIFEST_DIR + "../.." path so that the
            // resulting app_root does not contain ".." components. The seed
            // security validator rejects paths containing ".." as path traversal.
            let raw = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            std::fs::canonicalize(&raw).unwrap_or(raw)
        });
    DefaultDatabaseModule::from_app_root(&app_root)
}

/// Bootstrap the membership database from the `MEMBERSHIP` env config.
///
/// This creates the pool, runs the DDL baseline (`init`), optionally
/// applies migrations (`auto_migrate`), and optionally applies seeds
/// (`seed_on_boot`). All three toggles are controlled by the database
/// manifest and overridable through `SDKWORK_MEMBERSHIP_DATABASE_*` env vars.
pub async fn bootstrap_membership_database_from_env() -> Result<MembershipDatabaseHost, String> {
    let _ = dotenvy::dotenv();
    let config = DatabaseConfig::from_env("MEMBERSHIP")
        .map_err(|error| format!("read membership database config failed: {error}"))?;
    let pool = create_pool_from_config(config)
        .await
        .map_err(|error| format!("create membership database pool failed: {error}"))?;
    let module = Arc::new(
        database_module()
            .map_err(|error| format!("load membership database module failed: {error}"))?,
    );
    let manifest = DatabaseManifest::from_file(module.manifest_path())
        .map_err(|error| format!("read membership database manifest failed: {error}"))?;
    let options = lifecycle_options_from_env("MEMBERSHIP", &manifest);
    let orchestrator = LifecycleOrchestrator::new(pool.clone(), module.clone())
        .with_applied_by("sdkwork-membership");
    orchestrator.init().await.map_err(|e| format!("{e}"))?;
    if options.auto_migrate {
        orchestrator.migrate().await.map_err(|e| format!("{e}"))?;
    }
    if options.seed_on_boot {
        orchestrator
            .seed(&options.seed_locale, &options.seed_profile)
            .await
            .map_err(|e| format!("{e}"))?;
    }
    Ok(MembershipDatabaseHost { pool, module })
}

/// Bootstrap the membership database schema and seeds using an externally
/// provided pool.
///
/// This is used when membership is integrated as a federated capability
/// inside a host application (e.g. ClawRouter) that already owns a shared
/// database pool. The function loads the membership database module from
/// the membership repository's `database/` assets, runs the DDL baseline,
/// optionally applies migrations, and optionally applies seeds — all
/// controlled by the same manifest/env options as the standalone bootstrap.
pub async fn bootstrap_membership_database_host_with_pool(
    pool: &DatabasePool,
) -> Result<MembershipDatabaseHost, String> {
    let module = Arc::new(
        database_module()
            .map_err(|error| format!("load membership database module failed: {error}"))?,
    );
    let manifest = DatabaseManifest::from_file(module.manifest_path())
        .map_err(|error| format!("read membership database manifest failed: {error}"))?;
    let options = lifecycle_options_from_env("MEMBERSHIP", &manifest);
    let orchestrator = LifecycleOrchestrator::new(pool.clone(), module.clone())
        .with_applied_by("sdkwork-membership");
    orchestrator.init().await.map_err(|e| format!("{e}"))?;
    if options.auto_migrate {
        orchestrator.migrate().await.map_err(|e| format!("{e}"))?;
    }
    if options.seed_on_boot {
        orchestrator
            .seed(&options.seed_locale, &options.seed_profile)
            .await
            .map_err(|e| format!("{e}"))?;
    }
    Ok(MembershipDatabaseHost {
        pool: pool.clone(),
        module,
    })
}

pub async fn bootstrap_membership_database_with_pool(pool: &DatabasePool) -> Result<(), String> {
    bootstrap_membership_database_host_with_pool(pool)
        .await
        .map(|_| ())
}
