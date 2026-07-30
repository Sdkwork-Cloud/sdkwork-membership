//! Source-level regression guards for the PostgreSQL membership repository.
//!
//! Live PostgreSQL integration belongs to the database lifecycle/repository gate. These fast
//! checks keep naming, ownership, transaction, and HTTP contract invariants visible in every
//! workspace test run without treating SQLite as authoritative-server evidence.

const POSTGRES_SOURCE: &str = include_str!("../src/postgres.rs");
const LIB_SOURCE: &str = include_str!("../src/lib.rs");
const TYPES_SOURCE: &str = include_str!("../src/types.rs");
const APP_ROUTER_SOURCE: &str =
    include_str!("../../sdkwork-routes-membership-app-api/src/router.rs");
const BACKEND_ROUTER_SOURCE: &str =
    include_str!("../../sdkwork-routes-membership-backend-api/src/admin_router.rs");
const POSTGRES_BASELINE: &str =
    include_str!("../../../database/ddl/baseline/postgres/0001_membership_baseline.sql");

#[test]
fn repository_exposes_only_the_authoritative_postgres_store() {
    assert!(POSTGRES_SOURCE.contains("pub struct PostgresCommerceMembershipStore"));
    assert!(LIB_SOURCE.contains("pub use postgres::PostgresCommerceMembershipStore"));
    assert!(!LIB_SOURCE.to_ascii_lowercase().contains("sqlite"));

    for banned in [
        "AppVip",
        "AdminVip",
        "SubmitVip",
        "SqliteCommerceMembershipStore",
        "PostgresAppVipStore",
        "vip_membership",
        "seed-product-vip-membership",
    ] {
        for (name, source) in [
            ("lib.rs", LIB_SOURCE),
            ("types.rs", TYPES_SOURCE),
            ("postgres.rs", POSTGRES_SOURCE),
            ("app router", APP_ROUTER_SOURCE),
            ("backend router", BACKEND_ROUTER_SOURCE),
        ] {
            assert!(
                !source.contains(banned),
                "{name} must not contain retired membership fragment {banned}",
            );
        }
    }
}

#[test]
fn routers_keep_canonical_paths_and_http_semantics() {
    for source in [APP_ROUTER_SOURCE, BACKEND_ROUTER_SOURCE] {
        assert!(!source.contains("/vip"));
        assert!(!source.contains("/billing"));
        assert!(!source.contains("alias = \"pageSize\""));
        assert!(!source.contains("alias = \"userId\""));
        assert!(!source.contains("alias = \"membershipId\""));
    }

    assert!(APP_ROUTER_SOURCE.contains("/app/v3/api/memberships/package_groups/{packageGroupId}"));
    assert!(APP_ROUTER_SOURCE.contains("/app/v3/api/memberships/packages/{packageId}"));
    assert!(BACKEND_ROUTER_SOURCE.contains("/backend/v3/api/memberships/plans/{planId}"));
    assert!(
        BACKEND_ROUTER_SOURCE.contains("/backend/v3/api/memberships/members/{membershipId}/status")
    );
    assert!(APP_ROUTER_SOURCE.matches("finish_api_created(").count() >= 3);
    assert!(BACKEND_ROUTER_SOURCE.matches("finish_api_created(").count() >= 3);
    assert!(
        BACKEND_ROUTER_SOURCE
            .matches("finish_api_no_content(")
            .count()
            >= 3
    );
}

#[test]
fn fulfillment_stays_order_led_and_transactionally_idempotent() {
    for forbidden in [
        "INSERT INTO commerce_order",
        "INSERT INTO commerce_payment_intent",
        "method_alias",
        "wechatpay",
    ] {
        assert!(!POSTGRES_SOURCE.contains(forbidden));
    }
    assert!(POSTGRES_SOURCE.contains("FROM membership_subscription ms"));
    assert!(POSTGRES_SOURCE.contains("async fn fulfill_purchase_by_order"));
    assert!(POSTGRES_SOURCE.contains("SELECT pg_advisory_xact_lock($1)"));
    assert!(POSTGRES_SOURCE.contains("AND mp.source_order_id = $4"));
    assert!(!BACKEND_ROUTER_SOURCE.contains("purchases/fulfillments"));
    assert!(!BACKEND_ROUTER_SOURCE.contains("MEMBERSHIP_FULFILL_ALLOW_INSECURE"));
}

#[test]
fn postgres_baseline_covers_membership_tables_only() {
    for table in [
        "membership_product_spu",
        "membership_product_sku",
        "membership_plan",
        "membership_plan_version",
        "membership_benefit_definition",
        "membership_plan_benefit",
        "membership_package_group",
        "membership_package",
        "membership_subscription",
        "membership_period",
        "membership_points_account",
        "membership_points_ledger",
        "membership_entitlement_account",
        "membership_entitlement_grant",
        "membership_entitlement_ledger_entry",
        "membership_privilege_usage",
        "membership_daily_reward",
    ] {
        assert!(
            POSTGRES_BASELINE.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")),
            "PostgreSQL baseline must create {table}",
        );
    }

    for table in [
        "commerce_order",
        "commerce_order_item",
        "commerce_order_amount_breakdown",
        "commerce_payment_method",
        "commerce_payment_intent",
        "commerce_payment_attempt",
    ] {
        assert!(
            !POSTGRES_BASELINE.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")),
            "Membership baseline must not create external table {table}",
        );
    }
}

#[test]
fn seeds_cover_membership_flows_without_order_or_payment_rows() {
    let seeds = format!(
        "{}\n{}",
        include_str!("../../../database/seeds/common/001_catalog.sql"),
        include_str!("../../../database/fixtures/002_dev_demo.sql"),
    );
    for table in [
        "membership_subscription",
        "membership_period",
        "membership_points_account",
        "membership_points_ledger",
        "membership_entitlement_account",
        "membership_entitlement_grant",
        "membership_entitlement_ledger_entry",
        "membership_privilege_usage",
    ] {
        assert!(seeds.contains(&format!("INSERT INTO {table}")));
    }
    for table in [
        "commerce_order",
        "commerce_order_item",
        "commerce_payment_method",
        "commerce_payment_intent",
    ] {
        assert!(!seeds.contains(&format!("INSERT INTO {table}")));
    }
}

#[test]
fn point_queries_use_tenant_scoped_account_tables() {
    assert!(POSTGRES_SOURCE.contains("FROM membership_points_account\n"));
    assert!(POSTGRES_SOURCE.contains("FROM membership_points_ledger\n"));
    assert!(POSTGRES_SOURCE.contains("owner_type = 'USER'"));
    assert!(POSTGRES_SOURCE.contains("tenant_id = CAST($1 AS BIGINT)"));
    assert!(POSTGRES_SOURCE.contains("organization_id = CAST($2 AS BIGINT)"));
    assert!(POSTGRES_SOURCE.contains("owner_id = CAST($3 AS BIGINT)"));
    assert!(!POSTGRES_SOURCE.contains("membership_points_ledger_entry"));
}

#[test]
fn plan_rank_and_storage_id_queries_keep_distinct_bindings() {
    assert!(POSTGRES_SOURCE.contains("AND CAST(p.rank AS INTEGER) = $3"));
    assert!(POSTGRES_SOURCE.contains("WHERE p.id = $1"));
    assert!(POSTGRES_SOURCE.contains(
        "let rows = sqlx::query(LOAD_MEMBERSHIP_PLAN_BY_RANK)\n        .bind(DEFAULT_CATALOG_TENANT_ID)\n        .bind(DEFAULT_CATALOG_ORGANIZATION_ID)\n        .bind(rank)",
    ));
    assert!(POSTGRES_SOURCE.contains(
        "let rows = sqlx::query(LOAD_MEMBERSHIP_PLAN_BY_STORAGE_ID)\n        .bind(&package.plan_storage_id)",
    ));
}
