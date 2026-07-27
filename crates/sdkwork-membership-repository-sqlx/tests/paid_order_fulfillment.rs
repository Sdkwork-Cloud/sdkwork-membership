use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_history::execute_sql_script;
use sdkwork_database_sqlx::{DatabasePool, PoolContext};
use sdkwork_membership_repository_sqlx::{
    AppMembershipStore, AppMembershipSubject, FulfillPaidMembershipPurchaseCommand,
    SqliteCommerceMembershipStore,
};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

const SQLITE_BASELINE: &str =
    include_str!("../../../database/ddl/baseline/sqlite/0001_membership_baseline.sql");
const CATALOG_SEED: &str = include_str!("../../../database/seeds/common/001_catalog.sql");

#[tokio::test]
async fn paid_orders_atomically_purchase_renew_upgrade_and_replay_platform_packages() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("sqlite pool");
    install_membership_fixture(&pool).await;
    let store = SqliteCommerceMembershipStore::new(pool.clone());

    let purchase = store
        .fulfill_paid_purchase(command(
            201,
            "purchase",
            "order-purchase",
            "MB-PURCHASE",
            "2026-07-26T08:00:00Z",
        ))
        .await
        .expect("paid purchase fulfillment");
    assert!(purchase.accepted);
    assert!(!purchase.replayed);
    assert_eq!(purchase.fulfillment_status, "active");
    let purchase_grants = count(&pool, "entitlement_grant").await;
    assert!(purchase_grants > 0);
    let purchase_starts_at = single_text(
        &pool,
        "SELECT starts_at FROM membership_subscription LIMIT 1",
        "starts_at",
    )
    .await;
    let purchase_expires_at = single_text(
        &pool,
        "SELECT expires_at FROM membership_subscription LIMIT 1",
        "expires_at",
    )
    .await;

    let replay = store
        .fulfill_paid_purchase(command(
            201,
            "purchase",
            "order-purchase",
            "MB-PURCHASE",
            "2026-07-26T08:00:00Z",
        ))
        .await
        .expect("paid purchase replay");
    assert!(replay.accepted);
    assert!(replay.replayed);

    let renew = store
        .fulfill_paid_purchase(command(
            201,
            "renew",
            "order-renew",
            "MB-RENEW",
            "2026-07-27 08:00:00",
        ))
        .await
        .expect("paid renewal fulfillment");
    assert!(renew.accepted);
    assert!(!renew.replayed);
    let renewal_grants = count(&pool, "entitlement_grant").await;
    assert!(renewal_grants > purchase_grants);
    let renewal_starts_at = single_text(
        &pool,
        "SELECT starts_at FROM membership_subscription LIMIT 1",
        "starts_at",
    )
    .await;
    let renewal_expires_at = single_text(
        &pool,
        "SELECT expires_at FROM membership_subscription LIMIT 1",
        "expires_at",
    )
    .await;
    let renewal_period_starts_at = single_text(
        &pool,
        "SELECT starts_at FROM membership_period WHERE source_order_id = 'order-renew'",
        "starts_at",
    )
    .await;
    let renewal_period_ends_at = single_text(
        &pool,
        "SELECT ends_at FROM membership_period WHERE source_order_id = 'order-renew'",
        "ends_at",
    )
    .await;
    assert_eq!(renewal_starts_at, purchase_starts_at);
    assert_eq!(renewal_period_starts_at, purchase_expires_at);
    assert_eq!(renewal_period_ends_at, renewal_expires_at);
    assert_eq!(
        count_where(
            &pool,
            "entitlement_account",
            "expires_at <> (SELECT expires_at FROM membership_subscription LIMIT 1)",
        )
        .await,
        0
    );

    let upgrade = store
        .fulfill_paid_purchase(command(
            202,
            "upgrade",
            "order-upgrade",
            "MB-UPGRADE",
            "2026-07-28T16:00:00+08:00",
        ))
        .await
        .expect("paid upgrade fulfillment");
    assert!(upgrade.accepted);
    assert!(!upgrade.replayed);

    assert_eq!(count(&pool, "membership_subscription").await, 1);
    assert_eq!(count(&pool, "membership_period").await, 3);
    assert_eq!(active_count(&pool, "membership_subscription").await, 1);
    assert_eq!(active_count(&pool, "membership_period").await, 3);
    assert!(count(&pool, "entitlement_grant").await > 0);
    assert_eq!(
        distinct_count(&pool, "entitlement_grant").await,
        count(&pool, "entitlement_grant").await
    );

    let historical_replay = store
        .fulfill_paid_purchase(command(
            201,
            "purchase",
            "order-purchase",
            "MB-PURCHASE",
            "2026-07-26T08:00:00Z",
        ))
        .await
        .expect("historical order replay after renewal and upgrade");
    assert!(historical_replay.replayed);
    assert_eq!(count(&pool, "membership_period").await, 3);
}

fn command(
    package_id: i64,
    action: &str,
    order_id: &str,
    order_no: &str,
    paid_at: &str,
) -> FulfillPaidMembershipPurchaseCommand {
    FulfillPaidMembershipPurchaseCommand {
        subject: AppMembershipSubject {
            tenant_id: 200002,
            organization_id: 0,
            user_id: 300003,
        },
        package_id,
        order_id: order_id.to_owned(),
        membership_id: format!("membership-{order_id}"),
        order_no: order_no.to_owned(),
        request_no: format!("request-{order_id}"),
        idempotency_key: format!("membership-purchase:fulfill:{order_id}"),
        paid_at: paid_at.to_owned(),
        action: action.to_owned(),
    }
}

async fn install_membership_fixture(pool: &SqlitePool) {
    let database_pool = DatabasePool::Sqlite(
        pool.clone(),
        PoolContext {
            config: DatabaseConfig::default(),
        },
    );
    execute_sql_script(&database_pool, SQLITE_BASELINE)
        .await
        .expect("membership baseline");
    execute_sql_script(&database_pool, CATALOG_SEED)
        .await
        .expect("membership catalog seed");
}

async fn count(pool: &SqlitePool, table: &str) -> i64 {
    let query = format!("SELECT COUNT(*) AS count_value FROM {table}");
    sqlx::query(&query)
        .fetch_one(pool)
        .await
        .expect("row count")
        .try_get("count_value")
        .expect("count value")
}

async fn active_count(pool: &SqlitePool, table: &str) -> i64 {
    let query = format!("SELECT COUNT(*) AS count_value FROM {table} WHERE status = 'active'");
    sqlx::query(&query)
        .fetch_one(pool)
        .await
        .expect("active row count")
        .try_get("count_value")
        .expect("active count value")
}

async fn distinct_count(pool: &SqlitePool, table: &str) -> i64 {
    let query = format!("SELECT COUNT(DISTINCT id) AS count_value FROM {table}");
    sqlx::query(&query)
        .fetch_one(pool)
        .await
        .expect("distinct row count")
        .try_get("count_value")
        .expect("distinct count value")
}

async fn count_where(pool: &SqlitePool, table: &str, predicate: &str) -> i64 {
    let query = format!("SELECT COUNT(*) AS count_value FROM {table} WHERE {predicate}");
    sqlx::query(&query)
        .fetch_one(pool)
        .await
        .expect("filtered row count")
        .try_get("count_value")
        .expect("filtered count value")
}

async fn single_text(pool: &SqlitePool, query: &str, column: &str) -> String {
    sqlx::query(query)
        .fetch_one(pool)
        .await
        .expect("single text row")
        .try_get(column)
        .expect("single text value")
}
