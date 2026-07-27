use sdkwork_membership_repository_sqlx::{
    AppMembershipStore, AppMembershipSubject, ConsumeSubscriptionQuotaCommand,
    SqliteCommerceMembershipStore,
};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

#[tokio::test]
async fn coupon_subscription_quota_enforces_daily_total_and_idempotency_limits() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("sqlite pool");
    install_quota_schema(&pool).await;
    seed_coupon_quota(&pool).await;
    let store = SqliteCommerceMembershipStore::new(pool.clone());

    let first = store
        .consume_subscription_quota(command(7, "request-1", "idem-1", "2026-07-26 08:00:00"))
        .await
        .expect("first quota consumption");
    assert!(first.accepted);
    assert!(!first.replayed);
    assert_eq!(7, first.used_daily_quota);
    assert_eq!(3, first.remaining_daily_quota);
    assert_eq!(18, first.remaining_total_quota);

    let replay = store
        .consume_subscription_quota(command(7, "request-1", "idem-1", "2026-07-26 08:00:00"))
        .await
        .expect("idempotent replay");
    assert!(replay.replayed);
    assert_eq!(7, replay.used_daily_quota);
    assert_eq!(18, replay.remaining_total_quota);

    let daily_error = store
        .consume_subscription_quota(command(4, "request-2", "idem-2", "2026-07-26 09:00:00"))
        .await
        .expect_err("daily quota must reject overflow");
    assert_eq!("conflict", daily_error.code());
    assert_eq!(18, account_balance(&pool).await);

    let daily_completion = store
        .consume_subscription_quota(command(3, "request-3", "idem-3", "2026-07-26 10:00:00"))
        .await
        .expect("complete first day quota");
    assert_eq!(10, daily_completion.used_daily_quota);
    assert_eq!(0, daily_completion.remaining_daily_quota);
    assert_eq!(15, daily_completion.remaining_total_quota);

    let next_day = store
        .consume_subscription_quota(command(10, "request-4", "idem-4", "2026-07-27 08:00:00"))
        .await
        .expect("next day quota consumption");
    assert_eq!(10, next_day.used_daily_quota);
    assert_eq!(5, next_day.remaining_total_quota);

    let total_error = store
        .consume_subscription_quota(command(6, "request-5", "idem-5", "2026-07-28 08:00:00"))
        .await
        .expect_err("total quota must reject overflow");
    assert_eq!("conflict", total_error.code());
    assert_eq!(5, account_balance(&pool).await);

    let ledger_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM entitlement_ledger_entry WHERE business_type = 'coupon_subscription_quota_usage'",
    )
    .fetch_one(&pool)
    .await
    .expect("ledger count");
    let usage_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM commerce_membership_privilege_usage")
            .fetch_one(&pool)
            .await
            .expect("usage count");
    assert_eq!(3, ledger_count);
    assert_eq!(2, usage_count);
}

#[tokio::test]
async fn coupon_subscription_quota_rejects_an_inactive_subscription() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("sqlite pool");
    install_quota_schema(&pool).await;
    seed_coupon_quota(&pool).await;
    sqlx::query("UPDATE membership_subscription SET status = 'pending'")
        .execute(&pool)
        .await
        .expect("mark subscription pending");
    let store = SqliteCommerceMembershipStore::new(pool.clone());

    let error = store
        .consume_subscription_quota(command(
            1,
            "request-pending",
            "idem-pending",
            "2026-07-26 08:00:00",
        ))
        .await
        .expect_err("pending subscription quota must not be consumable");

    assert_eq!("conflict", error.code());
    assert_eq!(25, account_balance(&pool).await);
    let ledger_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM entitlement_ledger_entry")
        .fetch_one(&pool)
        .await
        .expect("ledger count");
    assert_eq!(0, ledger_count);
}

fn command(
    amount: i64,
    request_no: &str,
    idempotency_key: &str,
    requested_at: &str,
) -> ConsumeSubscriptionQuotaCommand {
    ConsumeSubscriptionQuotaCommand {
        subject: AppMembershipSubject {
            tenant_id: 100001,
            organization_id: 0,
            user_id: 200001,
        },
        amount,
        request_no: request_no.to_owned(),
        idempotency_key: idempotency_key.to_owned(),
        requested_at: requested_at.to_owned(),
    }
}

async fn account_balance(pool: &SqlitePool) -> i64 {
    let row = sqlx::query("SELECT balance FROM entitlement_account WHERE id = 'account-ai-quota'")
        .fetch_one(pool)
        .await
        .expect("quota account");
    row.try_get::<String, _>("balance")
        .expect("balance text")
        .parse()
        .expect("balance integer")
}

async fn install_quota_schema(pool: &SqlitePool) {
    let statements = [
        r#"CREATE TABLE benefit_definition (
            id TEXT NOT NULL,
            tenant_id TEXT NOT NULL,
            benefit_code TEXT NOT NULL,
            PRIMARY KEY (id)
        )"#,
        r#"CREATE TABLE membership_subscription (
            id TEXT NOT NULL,
            tenant_id TEXT NOT NULL,
            status TEXT NOT NULL,
            expires_at TEXT,
            PRIMARY KEY (id)
        )"#,
        r#"CREATE TABLE entitlement_account (
            id TEXT NOT NULL,
            tenant_id TEXT NOT NULL,
            organization_id TEXT,
            subject_type TEXT NOT NULL,
            subject_id TEXT NOT NULL,
            benefit_id TEXT NOT NULL,
            total_granted TEXT NOT NULL,
            total_used TEXT NOT NULL,
            balance TEXT NOT NULL,
            status TEXT NOT NULL,
            expires_at TEXT,
            version INTEGER NOT NULL DEFAULT 0,
            updated_at TEXT NOT NULL,
            PRIMARY KEY (id)
        )"#,
        r#"CREATE TABLE entitlement_grant (
            id TEXT NOT NULL,
            tenant_id TEXT NOT NULL,
            organization_id TEXT,
            subject_type TEXT NOT NULL,
            subject_id TEXT NOT NULL,
            benefit_id TEXT NOT NULL,
            source_type TEXT NOT NULL,
            source_id TEXT NOT NULL,
            grant_policy TEXT,
            granted_quantity TEXT NOT NULL,
            status TEXT NOT NULL,
            starts_at TEXT,
            expires_at TEXT,
            created_at TEXT NOT NULL,
            PRIMARY KEY (id)
        )"#,
        r#"CREATE TABLE entitlement_ledger_entry (
            id TEXT NOT NULL,
            tenant_id TEXT NOT NULL,
            organization_id TEXT,
            ledger_no TEXT NOT NULL,
            account_id TEXT NOT NULL,
            grant_id TEXT,
            benefit_id TEXT NOT NULL,
            subject_type TEXT NOT NULL,
            subject_id TEXT NOT NULL,
            direction TEXT NOT NULL,
            amount TEXT NOT NULL,
            balance_after TEXT NOT NULL,
            business_type TEXT NOT NULL,
            source_type TEXT NOT NULL,
            source_id TEXT NOT NULL,
            request_no TEXT,
            idempotency_key TEXT,
            occurred_at TEXT NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (id)
        )"#,
        r#"CREATE TABLE commerce_membership_privilege_usage (
            id INTEGER NOT NULL,
            uuid TEXT NOT NULL,
            tenant_id INTEGER NOT NULL,
            organization_id INTEGER NOT NULL DEFAULT 0,
            user_id INTEGER NOT NULL,
            subscription_id INTEGER,
            benefit_code TEXT NOT NULL,
            period_start TEXT NOT NULL,
            period_end TEXT NOT NULL,
            used_count INTEGER NOT NULL DEFAULT 0,
            usage_limit INTEGER NOT NULL DEFAULT 0,
            last_used_at TEXT,
            version INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            PRIMARY KEY (id),
            UNIQUE (tenant_id, user_id, benefit_code, period_start)
        )"#,
    ];
    for statement in statements {
        sqlx::query(statement)
            .execute(pool)
            .await
            .expect("install quota schema");
    }
}

async fn seed_coupon_quota(pool: &SqlitePool) {
    sqlx::query(
        "INSERT INTO benefit_definition (id, tenant_id, benefit_code) VALUES ('benefit-ai-quota', '100001', 'ai_quota')",
    )
    .execute(pool)
    .await
    .expect("benefit");
    sqlx::query(
        "INSERT INTO membership_subscription (id, tenant_id, status, expires_at) VALUES ('subscription-month', '100001', 'active', '2026-08-25 00:00:00')",
    )
    .execute(pool)
    .await
    .expect("subscription");
    sqlx::query(
        r#"INSERT INTO entitlement_account
            (id, tenant_id, organization_id, subject_type, subject_id, benefit_id,
             total_granted, total_used, balance, status, expires_at, updated_at)
           VALUES
            ('account-ai-quota', '100001', '0', 'user', '200001', 'benefit-ai-quota',
             '25', '0', '25', 'active', '2026-08-25 00:00:00', '2026-07-26 00:00:00')"#,
    )
    .execute(pool)
    .await
    .expect("entitlement account");
    sqlx::query(
        r#"INSERT INTO entitlement_grant
            (id, tenant_id, organization_id, subject_type, subject_id, benefit_id,
             source_type, source_id, grant_policy, granted_quantity, status, starts_at, expires_at, created_at)
           VALUES
            ('grant-month', '100001', '0', 'user', '200001', 'benefit-ai-quota',
             'membership_subscription', 'subscription-month',
             '{"kind":"coupon_subscription_quota","couponOrderId":"order-month","period":"month","dailyQuota":10,"totalQuota":25}',
             '25', 'active', '2026-07-26 00:00:00', '2026-08-25 00:00:00', '2026-07-26 00:00:00')"#,
    )
    .execute(pool)
    .await
    .expect("entitlement grant");
}
