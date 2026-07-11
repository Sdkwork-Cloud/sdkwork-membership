//! Source-inspection guards for the membership SQLx crate.
//!
//! These tests verify that legacy naming (VIP, level, group_id, compat query
//! aliases, payment artifacts) does not regress into the codebase. They do
//! not require a database and run as part of `cargo test --workspace`.

#[test]
fn commerce_membership_sqlx_exposes_standard_store_names_without_legacy_membership_aliases() {
    let sqlite_source = include_str!("../src/sqlite.rs");
    let postgres_source = include_str!("../src/postgres.rs");
    let lib_source = include_str!("../src/lib.rs");
    let router_source = include_str!("../../sdkwork-routes-membership-app-api/src/router.rs");
    let admin_router_source =
        include_str!("../../sdkwork-routes-membership-backend-api/src/admin_router.rs");
    let shared_source = include_str!("../src/shared.rs");
    let types_source = include_str!("../src/types.rs");
    let manifest_source = include_str!("../Cargo.toml");

    assert!(sqlite_source.contains("pub struct SqliteCommerceMembershipStore"));
    assert!(postgres_source.contains("pub struct PostgresCommerceMembershipStore"));
    assert!(lib_source.contains("pub use postgres::PostgresCommerceMembershipStore"));
    assert!(lib_source.contains("pub use sqlite::SqliteCommerceMembershipStore"));
    assert!(!lib_source.contains("pub use types::{\n    AppVip"));
    assert!(!lib_source.contains("SubmitVipPurchaseCommand as SubmitMembershipPurchaseCommand"));

    let banned_fragments = [
        "AppVip",
        "AdminVip",
        "SubmitVip",
        "CreateAdminVip",
        "UpdateAdminVip",
        "DeleteAdminVip",
        "ListAdminVip",
        "SqliteAppVipStore",
        "PostgresAppVipStore",
        "ParsedVip",
        "StoredVip",
        "VipPaymentMethod",
        "vip_membership",
        "seed-product-vip-membership",
        concat!("\"basic\"", " =>"),
        concat!("\"advanced\"", " =>"),
        concat!("\"premium\"", " =>"),
        concat!("\"ultimate\"", " =>"),
        "| \"basic\"",
        "| \"advanced\"",
        "| \"premium\"",
        "| \"ultimate\"",
        concat!("level", "_id"),
        concat!("AS group", "_id"),
        concat!("\"group", "_id\""),
        concat!("ensure_admin_", "level", "_exists"),
        concat!("level", "_id_for_storage"),
        concat!("build_", "group", "_from_packages"),
        concat!("fn group", "_id_for("),
        concat!("list_", "level", "s"),
        concat!("create_", "level"),
        concat!("update_", "level"),
        concat!("delete_", "level"),
        concat!("map_admin_", "level"),
        concat!("load_", "level", "_for_package"),
        concat!("level", "_name"),
    ];

    for (name, source) in [
        ("lib.rs", lib_source),
        ("types.rs", types_source),
        ("router.rs", router_source),
        ("admin_router.rs", admin_router_source),
        ("shared.rs", shared_source),
        ("sqlite.rs", sqlite_source),
        ("postgres.rs", postgres_source),
        ("Cargo.toml", manifest_source),
    ] {
        for banned in banned_fragments {
            assert!(
                !source.contains(banned),
                "{name} must not contain legacy membership fragment {banned}"
            );
        }
    }
}

#[test]
fn commerce_membership_routers_follow_api_spec_without_compat_query_aliases() {
    let app_router_source = include_str!("../../sdkwork-routes-membership-app-api/src/router.rs");
    let backend_router_source =
        include_str!("../../sdkwork-routes-membership-backend-api/src/admin_router.rs");

    for source in [app_router_source, backend_router_source] {
        assert!(!source.contains("/vip"));
        assert!(!source.contains("/billing"));
        assert!(!source.contains("alias = \"pageSize\""));
        assert!(!source.contains("alias = \"userId\""));
        assert!(!source.contains("alias = \"membershipId\""));
    }

    assert!(app_router_source.contains("/app/v3/api/memberships/package_groups/{packageGroupId}"));
    assert!(app_router_source.contains("/app/v3/api/memberships/packages/{packageId}"));
    assert!(backend_router_source.contains("/backend/v3/api/memberships/plans/{planId}"));
    assert!(
        backend_router_source.contains("/backend/v3/api/memberships/members/{membershipId}/status")
    );
}

#[test]
fn commerce_membership_purchase_respects_order_boundary_without_local_commerce_writes() {
    let checked_sources = [
        (
            "membership sqlite repository",
            include_str!("../src/sqlite.rs").replace("\r\n", "\n"),
        ),
        (
            "membership postgres repository",
            include_str!("../src/postgres.rs").replace("\r\n", "\n"),
        ),
        (
            "membership admin router",
            include_str!("../../sdkwork-routes-membership-backend-api/src/admin_router.rs")
                .replace("\r\n", "\n"),
        ),
    ];

    for (label, source) in &checked_sources {
        assert!(
            !source.contains("INSERT INTO commerce_order"),
            "{label} must not create commerce_order rows during membership purchase",
        );
        assert!(
            !source.contains("INSERT INTO commerce_payment_intent"),
            "{label} must not create commerce_payment_intent rows during membership purchase",
        );
        assert!(
            !source.contains("method_alias"),
            "{label} must not keep legacy payment method alias queries",
        );
        assert!(
            !source.contains("wechatpay"),
            "{label} must not keep legacy compact wechatpay aliases",
        );
    }

    for (label, source) in checked_sources.iter().take(2) {
        assert!(
            source.contains("FROM membership_subscription ms"),
            "{label} must resolve purchase idempotency from membership_subscription",
        );
        assert!(
            source.contains("async fn fulfill_purchase_by_order"),
            "{label} must expose order-led membership fulfillment",
        );
        assert!(
            source.contains("'pending'"),
            "{label} must reserve entitlements in pending status before fulfillment",
        );
    }

    assert!(
        checked_sources[2]
            .1
            .contains("/backend/v3/api/memberships/purchases/fulfillments"),
        "membership admin router must expose purchase fulfillment endpoint",
    );
}

#[test]
fn commerce_membership_routers_use_standard_create_and_delete_http_statuses() {
    let app_router_source = include_str!("../../sdkwork-routes-membership-app-api/src/router.rs");
    let backend_router_source =
        include_str!("../../sdkwork-routes-membership-backend-api/src/admin_router.rs");

    for handler in ["purchase", "claim_daily_reward", "create_speed_up"] {
        assert!(
            app_router_source.contains(&format!("async fn {handler}(")),
            "app router must keep {handler} handler"
        );
    }
    assert!(
        app_router_source.matches("finish_api_created(").count() >= 3,
        "app create command handlers must return HTTP 201 through finish_api_created"
    );

    for handler in [
        "create_plan",
        "create_package",
        "create_package_group",
        "fulfill_membership_purchase",
    ] {
        assert!(
            backend_router_source.contains(&format!("async fn {handler}(")),
            "backend router must keep {handler} handler"
        );
    }
    assert!(
        backend_router_source.matches("finish_api_created(").count() >= 4,
        "backend create handlers must return HTTP 201 through finish_api_created"
    );
    assert!(
        backend_router_source
            .matches("finish_api_no_content(")
            .count()
            >= 3,
        "backend delete handlers must return HTTP 204 through finish_api_no_content"
    );
}

#[test]
fn commerce_membership_seed_covers_authenticated_frontend_flows_without_local_order_rows() {
    let seed_source =
        include_str!("../../../database/seeds/common/001_bootstrap.sql").replace("\r\n", "\n");

    for table in [
        "membership_subscription",
        "membership_period",
        "commerce_account",
        "commerce_account_ledger",
        "entitlement_account",
        "entitlement_grant",
        "entitlement_ledger_entry",
        "commerce_membership_privilege_usage",
    ] {
        assert!(
            seed_source.contains(&format!("INSERT OR IGNORE INTO {table}")),
            "database seed must initialize {table} for authenticated frontend membership flows",
        );
    }

    for benefit_code in ["priority_speed_up", "priority_queue", "ai_quota"] {
        assert!(
            seed_source.contains(benefit_code),
            "database seed must include canonical privilege benefit {benefit_code}",
        );
    }

    assert!(
        seed_source.contains("'100001'") && seed_source.contains("'1'"),
        "database seed must cover the demo tenant 100001 and bootstrap user 1",
    );

    for table in [
        "commerce_order",
        "commerce_order_item",
        "commerce_order_amount_breakdown",
        "commerce_payment_method",
        "commerce_payment_intent",
        "commerce_payment_attempt",
    ] {
        assert!(
            !seed_source.contains(&format!("INSERT OR IGNORE INTO {table}"))
                && !seed_source.contains(&format!("INSERT INTO {table}")),
            "membership seed must not initialize {table}; order/payment own that domain",
        );
    }
}

#[test]
fn commerce_membership_baselines_create_seeded_frontend_flow_tables() {
    let sqlite_baseline =
        include_str!("../../../database/ddl/baseline/sqlite/0001_membership_baseline.sql")
            .replace("\r\n", "\n");
    let postgres_baseline =
        include_str!("../../../database/ddl/baseline/postgres/0001_membership_baseline.sql")
            .replace("\r\n", "\n");

    for table in [
        "commerce_product_spu",
        "commerce_product_sku",
        "membership_plan",
        "membership_plan_version",
        "benefit_definition",
        "membership_plan_benefit",
        "membership_package_group",
        "membership_package",
        "membership_subscription",
        "membership_period",
        "commerce_account",
        "commerce_account_ledger",
        "entitlement_account",
        "entitlement_grant",
        "entitlement_ledger_entry",
        "commerce_membership_privilege_usage",
        "commerce_membership_daily_reward",
    ] {
        let create_statement = format!("CREATE TABLE IF NOT EXISTS {table}");
        assert!(
            sqlite_baseline.contains(&create_statement),
            "sqlite baseline must create {table} because the bootstrap seed inserts it",
        );
        assert!(
            postgres_baseline.contains(&create_statement),
            "postgres baseline must create {table} because the bootstrap seed inserts it",
        );
    }

    for baseline in [sqlite_baseline, postgres_baseline] {
        for table in [
            "commerce_order",
            "commerce_order_item",
            "commerce_order_amount_breakdown",
        ] {
            assert!(
                !baseline.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")),
                "membership baseline must not create {table}; order owns order persistence",
            );
        }
    }
}

#[test]
fn commerce_membership_baselines_exclude_order_and_payment_owned_tables() {
    let sqlite_baseline =
        include_str!("../../../database/ddl/baseline/sqlite/0001_membership_baseline.sql")
            .replace("\r\n", "\n");
    let postgres_baseline =
        include_str!("../../../database/ddl/baseline/postgres/0001_membership_baseline.sql")
            .replace("\r\n", "\n");

    for (label, baseline) in [
        ("sqlite baseline", sqlite_baseline),
        ("postgres baseline", postgres_baseline),
    ] {
        for table in [
            "commerce_order",
            "commerce_order_item",
            "commerce_order_amount_breakdown",
            "commerce_payment_method",
            "commerce_payment_intent",
            "commerce_payment_attempt",
        ] {
            assert!(
                !baseline.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")),
                "{label} must not create {table}; the owning order/payment modules initialize it",
            );
        }
    }
}

#[test]
fn commerce_membership_points_queries_use_account_baseline_tables() {
    for (label, source) in [
        ("sqlite repository", include_str!("../src/sqlite.rs")),
        ("postgres repository", include_str!("../src/postgres.rs")),
    ] {
        assert!(
            source.contains("FROM commerce_account\n"),
            "{label} must read point balances from the account baseline table",
        );
        assert!(
            source.contains("FROM commerce_account_ledger\n"),
            "{label} must read point history from the account ledger baseline table",
        );
        assert!(
            source.contains("owner_type = 'USER'"),
            "{label} must scope point reads to user-owned accounts",
        );
        assert!(
            source.contains("owner_id = CAST("),
            "{label} must use account baseline owner_id instead of legacy owner_user_id",
        );
        assert!(
            source.contains("asset_code ="),
            "{label} must use account baseline asset_code instead of legacy asset_type",
        );
        assert!(
            !source.contains("commerce_account_ledger_entry"),
            "{label} must not depend on legacy commerce_account_ledger_entry",
        );
    }

    let sqlite_source = include_str!("../src/sqlite.rs");
    assert!(
        sqlite_source.contains("tenant_id = CAST(?1 AS INTEGER)")
            && sqlite_source.contains("organization_id = CAST(?2 AS INTEGER)")
            && sqlite_source.contains("owner_id = CAST(?3 AS INTEGER)"),
        "sqlite points queries must compare account baseline integer subject columns as integers",
    );

    let postgres_source = include_str!("../src/postgres.rs");
    assert!(
        postgres_source.contains("tenant_id = CAST($1 AS BIGINT)")
            && postgres_source.contains("organization_id = CAST($2 AS BIGINT)")
            && postgres_source.contains("owner_id = CAST($3 AS BIGINT)"),
        "postgres points queries must compare account baseline BIGINT subject columns as BIGINT",
    );
}
