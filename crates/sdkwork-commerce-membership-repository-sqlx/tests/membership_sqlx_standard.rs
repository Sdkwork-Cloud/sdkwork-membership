use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use sdkwork_commerce_api_server::test_http::commerce_test_request;
use sdkwork_commerce_membership_repository_sqlx::{
    admin_membership_router_with_sqlite_pool, app_membership_router_with_sqlite_pool,
    repair_sqlite_commerce_experience_seed, upsert_sqlite_commerce_experience_seed,
    upsert_sqlite_payment_center_seed, AppMembershipSubject, SqliteCommerceMembershipStore,
    SubmitMembershipPurchaseCommand,
};
use sdkwork_commerce_storage_repository_sqlx::{
    commerce_initial_migration_sql, commerce_sqlite_memory_pool,
};
use sdkwork_iam_context_service::{AuthLevel, DeploymentMode, Environment, IamAppContext};
use serde_json::Value;
use sqlx::{Executor, Row, SqlitePool};
use std::collections::BTreeSet;
use tower::ServiceExt;

#[test]
fn commerce_membership_sqlx_exposes_standard_store_names_without_legacy_membership_aliases() {
    let sqlite_source = include_str!("../src/sqlite.rs");
    let postgres_source = include_str!("../src/postgres.rs");
    let lib_source = include_str!("../src/lib.rs");
    let router_source = include_str!("../src/router.rs");
    let admin_router_source = include_str!("../src/admin_router.rs");
    let shared_source = include_str!("../src/shared.rs");
    let seed_source = include_str!("../src/seed.rs");
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
        ("seed.rs", seed_source),
        ("Cargo.toml", manifest_source),
    ] {
        for banned in banned_fragments {
            assert!(
                !source.contains(banned),
                "{name} must not contain legacy membership fragment {banned}"
            );
        }
    }

    let test_source = include_str!("membership_sqlx_standard.rs");
    for banned in [
        concat!("created_", "level", "_id"),
        concat!("delete_", "level"),
        concat!("create_", "level"),
        concat!("update_", "level"),
    ] {
        assert!(
            !test_source.contains(banned),
            "membership SQLx tests must not keep legacy test variable fragment {banned}"
        );
    }
}

#[test]
fn commerce_membership_routers_follow_api_spec_without_compat_query_aliases() {
    let app_router_source = include_str!("../src/router.rs");
    let backend_router_source = include_str!("../src/admin_router.rs");

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
fn commerce_membership_purchase_uses_canonical_payment_method_keys_without_compat_aliases() {
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
            "membership shared helpers",
            include_str!("../src/shared.rs").replace("\r\n", "\n"),
        ),
    ];

    for (label, source) in &checked_sources {
        assert!(
            !source.contains("method_alias"),
            "{label} must query commerce_payment_method.method_key by canonical key only",
        );
        assert!(
            !source.contains("wechatpay"),
            "{label} must not keep legacy compact wechatpay aliases",
        );
        assert!(
            !source.contains("\"wechat_pay\" => \"wechat\""),
            "{label} must preserve the canonical wechat_pay method key",
        );
        assert!(
            !source.contains("LOWER(provider)"),
            "{label} must not treat provider_code as a payment method alias",
        );
        assert!(
            !source.contains("_ => \"wechat_pay\""),
            "{label} must not default unknown payment methods to wechat_pay",
        );
    }

    for (label, source) in checked_sources.iter().take(2) {
        assert!(
            source.contains("SELECT\n    method_key,\n    provider_code"),
            "{label} must load provider_code from commerce_payment_method",
        );
        assert!(
            !source.contains("payment_provider_code(&method_key)"),
            "{label} must not derive provider_code from method_key",
        );
    }
}

#[tokio::test]
async fn sqlite_seed_initializes_membership_catalog_for_app_display() {
    let pool = seeded_pool().await;
    let store = SqliteCommerceMembershipStore::new(pool.clone());

    let groups = store
        .load_package_groups(None, false)
        .await
        .expect("load package groups");
    assert_eq!(2, groups.len());
    assert_eq!(
        vec![1, 2],
        groups.iter().map(|group| group.id).collect::<Vec<_>>()
    );
    assert_eq!(3, groups[0].packages.len());
    assert_eq!(
        vec![301, 302, 303],
        groups[0]
            .packages
            .iter()
            .map(|package| package.id)
            .collect::<Vec<_>>()
    );

    let packages = store
        .load_packages(None, None)
        .await
        .expect("load packages");
    assert_eq!(6, packages.len());
    let monthly_pro = packages
        .iter()
        .find(|package| package.id == 301)
        .expect("monthly pro package exists");
    assert_eq!("69.90", monthly_pro.price);
    assert_eq!(30, monthly_pro.duration_days);
    assert!(monthly_pro.recommended);

    let manifest = sdkwork_commerce_bootstrap_manifest::commerce_experience_seed_manifest();
    assert!(!manifest.payload_json.contains("region_code"));
    assert!(!manifest.payload_json.contains("base_url_template"));
    assert!(!manifest.payload_json.contains("base_url_override"));

    assert!(
        sdkwork_commerce_membership_repository_sqlx::sqlite_commerce_experience_seed_complete(
            &pool
        )
        .await
        .expect("seed complete check")
    );
}

#[tokio::test]
async fn sqlite_seed_integrity_report_is_complete_for_standard_seed() {
    let pool = seeded_pool().await;

    let report =
        sdkwork_commerce_membership_repository_sqlx::sqlite_commerce_experience_seed_integrity_report(&pool)
            .await
            .expect("seed integrity report");

    assert!(report.complete);
    assert_eq!(Vec::<String>::new(), integrity_issue_codes(&report));
}

#[tokio::test]
async fn sqlite_seed_installs_active_payment_center_defaults_and_preserves_admin_edits() {
    let pool = commerce_sqlite_memory_pool().await;
    install_schema(&pool).await;
    upsert_sqlite_payment_center_seed(&pool)
        .await
        .expect("seed payment center");

    let method_keys = sqlx::query(
        r#"
        SELECT method_key
        FROM commerce_payment_method
        WHERE tenant_id = '100001'
          AND organization_id = '0'
          AND status = 'active'
        ORDER BY method_key
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("load payment methods")
    .into_iter()
    .map(|row| row.get::<String, _>("method_key"))
    .collect::<BTreeSet<_>>();
    assert_eq!(
        sdkwork_commerce_bootstrap_manifest::commerce_payment_method_seeds()
            .into_iter()
            .map(|method| method.method_key.to_owned())
            .collect::<BTreeSet<_>>(),
        method_keys
    );

    let active_provider_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM commerce_payment_provider WHERE tenant_id = '100001' AND organization_id = '0' AND status = 'active'",
    )
    .fetch_one(&pool)
    .await
    .expect("count payment providers");
    assert_eq!(
        sdkwork_commerce_bootstrap_manifest::commerce_payment_provider_seeds().len() as i64,
        active_provider_count
    );

    let active_account_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM commerce_payment_provider_account WHERE tenant_id = '100001' AND organization_id = '0' AND status = 'active'",
    )
    .fetch_one(&pool)
    .await
    .expect("count payment provider accounts");
    assert_eq!(
        sdkwork_commerce_bootstrap_manifest::commerce_payment_provider_account_seeds().len() as i64,
        active_account_count
    );

    let active_channel_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM commerce_payment_channel WHERE tenant_id = '100001' AND organization_id = '0' AND status = 'active'",
    )
    .fetch_one(&pool)
    .await
    .expect("count payment channels");
    assert_eq!(
        sdkwork_commerce_bootstrap_manifest::commerce_payment_channel_seeds().len() as i64,
        active_channel_count
    );

    let active_route_rule_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM commerce_payment_route_rule WHERE tenant_id = '100001' AND organization_id = '0' AND status = 'active'",
    )
    .fetch_one(&pool)
    .await
    .expect("count payment route rules");
    assert_eq!(
        sdkwork_commerce_bootstrap_manifest::commerce_payment_route_rule_seeds().len() as i64,
        active_route_rule_count
    );

    sqlx::query(
        r#"
        UPDATE commerce_payment_provider_account
        SET merchant_id = 'real-stripe-account',
            secret_ref = 'secret://real/stripe',
            webhook_secret_ref = 'secret://real/stripe/webhook',
            environment = 'production',
            status = 'inactive'
        WHERE account_no = 'seed-stripe-sandbox'
        "#,
    )
    .execute(&pool)
    .await
    .expect("edit provider account");
    sqlx::query("UPDATE commerce_payment_method SET status = 'inactive' WHERE method_key = 'card'")
        .execute(&pool)
        .await
        .expect("deactivate card method");
    sqlx::query(
        "UPDATE commerce_payment_channel SET status = 'inactive' WHERE channel_no = 'seed-card-checkout'",
    )
    .execute(&pool)
    .await
    .expect("deactivate card channel");
    sqlx::query(
        "UPDATE commerce_payment_route_rule SET status = 'inactive' WHERE rule_no = 'route-seed-card-checkout'",
    )
    .execute(&pool)
    .await
    .expect("deactivate card route rule");

    upsert_sqlite_payment_center_seed(&pool)
        .await
        .expect("repair payment center seed");

    let account = sqlx::query(
        r#"
        SELECT merchant_id, secret_ref, webhook_secret_ref, environment, status
        FROM commerce_payment_provider_account
        WHERE account_no = 'seed-stripe-sandbox'
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("load edited provider account");
    assert_eq!(
        "real-stripe-account",
        account.get::<String, _>("merchant_id")
    );
    assert_eq!(
        "secret://real/stripe",
        account.get::<String, _>("secret_ref")
    );
    assert_eq!(
        "secret://real/stripe/webhook",
        account.get::<String, _>("webhook_secret_ref")
    );
    assert_eq!("production", account.get::<String, _>("environment"));
    assert_eq!("inactive", account.get::<String, _>("status"));

    let card_method_status: String =
        sqlx::query_scalar("SELECT status FROM commerce_payment_method WHERE method_key = 'card'")
            .fetch_one(&pool)
            .await
            .expect("load card method status");
    assert_eq!("inactive", card_method_status);
    let card_channel_status: String = sqlx::query_scalar(
        "SELECT status FROM commerce_payment_channel WHERE channel_no = 'seed-card-checkout'",
    )
    .fetch_one(&pool)
    .await
    .expect("load card channel status");
    assert_eq!("inactive", card_channel_status);
    let card_rule_status: String = sqlx::query_scalar(
        "SELECT status FROM commerce_payment_route_rule WHERE rule_no = 'route-seed-card-checkout'",
    )
    .fetch_one(&pool)
    .await
    .expect("load card route rule status");
    assert_eq!("inactive", card_rule_status);
}

#[tokio::test]
async fn sqlite_payment_center_seed_populates_legacy_provider_column_when_present() {
    let pool = commerce_sqlite_memory_pool().await;
    install_schema_with_legacy_payment_method_provider(&pool).await;

    upsert_sqlite_payment_center_seed(&pool)
        .await
        .expect("seed payment center with legacy provider column");

    let provider_mismatch_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(1)
        FROM commerce_payment_method
        WHERE provider <> provider_code
           OR provider = ''
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("count legacy provider mismatches");
    assert_eq!(0, provider_mismatch_count);
}

#[tokio::test]
async fn sqlite_seed_integrity_report_keeps_payment_center_complete_after_admin_disables_seed_rows()
{
    let pool = seeded_pool().await;

    for statement in [
        "UPDATE commerce_payment_method SET status = 'disabled' WHERE method_key = 'card'",
        "UPDATE commerce_payment_provider SET status = 'disabled' WHERE provider_code = 'stripe'",
        "UPDATE commerce_payment_provider_account SET status = 'disabled' WHERE account_no = 'seed-stripe-sandbox'",
        "UPDATE commerce_payment_channel SET status = 'disabled' WHERE channel_no = 'seed-card-checkout'",
        "UPDATE commerce_payment_route_rule SET status = 'disabled' WHERE rule_no = 'route-seed-card-checkout'",
    ] {
        sqlx::query(statement)
            .execute(&pool)
            .await
            .unwrap_or_else(|error| panic!("failed to disable payment seed with {statement}: {error}"));
    }

    let report =
        sdkwork_commerce_membership_repository_sqlx::sqlite_commerce_experience_seed_integrity_report(&pool)
            .await
            .expect("seed integrity report");

    assert!(report.complete);
    assert_eq!(Vec::<String>::new(), integrity_issue_codes(&report));
}

#[tokio::test]
async fn sqlite_seed_integrity_report_detects_broken_membership_links() {
    for (statement, expected_code) in [
        (
            "UPDATE membership_plan SET status = 'disabled' WHERE plan_no = 'pro'",
            "missing_membership_plan",
        ),
        (
            "UPDATE membership_package SET plan_id = 'missing-plan' WHERE package_no = 'membership-month-pro'",
            "orphan_membership_package_plan",
        ),
        (
            "UPDATE membership_package SET package_group_id = 'missing-group' WHERE package_no = 'membership-month-pro'",
            "orphan_membership_package_group",
        ),
        (
            "UPDATE membership_package SET sku_id = 'missing-sku' WHERE package_no = 'membership-month-pro'",
            "orphan_membership_package_sku",
        ),
        (
            "UPDATE commerce_product_sku SET spu_id = 'seed-product-points-recharge-cny' WHERE id = 'seed-sku-membership-month-pro'",
            "invalid_membership_sku_product",
        ),
        (
            "UPDATE commerce_recharge_package SET sku_id = 'missing-recharge-sku' WHERE package_no = 'points-cny-5'",
            "orphan_recharge_package_sku",
        ),
        (
            "UPDATE commerce_product_sku SET spu_id = 'seed-product-membership' WHERE id = 'seed-sku-points-recharge-cny-500'",
            "invalid_recharge_sku_product",
        ),
    ] {
        let pool = seeded_pool().await;
        sqlx::query(statement)
            .execute(&pool)
            .await
            .unwrap_or_else(|error| panic!("failed to mutate seed with {statement}: {error}"));

        let report =
            sdkwork_commerce_membership_repository_sqlx::sqlite_commerce_experience_seed_integrity_report(
                &pool,
            )
            .await
            .expect("seed integrity report");

        assert!(!report.complete, "{expected_code} should make report incomplete");
        assert!(
            integrity_issue_codes(&report)
                .iter()
                .any(|code| code == expected_code),
            "{expected_code} missing from {:?}",
            integrity_issue_codes(&report)
        );
    }
}

#[tokio::test]
async fn sqlite_seed_repair_restores_only_incomplete_seed_slices() {
    let pool = seeded_pool().await;

    sqlx::query(
        "UPDATE commerce_payment_provider SET display_name = 'Production Stripe' WHERE provider_code = 'stripe'",
    )
    .execute(&pool)
    .await
    .expect("customize provider display name");
    sqlx::query("DELETE FROM membership_package WHERE id = '302'")
        .execute(&pool)
        .await
        .expect("delete one membership package");
    sqlx::query(
        "DELETE FROM commerce_payment_channel WHERE id = 'seed-payment-channel-card-checkout'",
    )
    .execute(&pool)
    .await
    .expect("delete one payment channel");

    let incomplete_report =
        sdkwork_commerce_membership_repository_sqlx::sqlite_commerce_experience_seed_integrity_report(&pool)
            .await
            .expect("seed integrity report");
    assert!(!incomplete_report.complete);
    assert!(integrity_issue_codes(&incomplete_report)
        .iter()
        .any(|code| code == "missing_membership_package"));
    assert!(integrity_issue_codes(&incomplete_report)
        .iter()
        .any(|code| code == "missing_payment_channel"));

    repair_sqlite_commerce_experience_seed(&pool)
        .await
        .expect("repair incomplete seed slices");

    assert!(
        sdkwork_commerce_membership_repository_sqlx::sqlite_commerce_experience_seed_complete(
            &pool
        )
        .await
        .expect("seed complete check")
    );

    let restored_package_count: i64 =
        sqlx::query_scalar("SELECT COUNT(1) FROM membership_package WHERE id = '302'")
            .fetch_one(&pool)
            .await
            .expect("count restored membership package");
    assert_eq!(1, restored_package_count);
    let restored_channel_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM commerce_payment_channel WHERE id = 'seed-payment-channel-card-checkout'",
    )
    .fetch_one(&pool)
    .await
    .expect("count restored payment channel");
    assert_eq!(1, restored_channel_count);

    let provider_display_name: String = sqlx::query_scalar(
        "SELECT display_name FROM commerce_payment_provider WHERE provider_code = 'stripe'",
    )
    .fetch_one(&pool)
    .await
    .expect("load provider display name");
    assert_eq!(
        "Production Stripe", provider_display_name,
        "repair must not replay unrelated payment provider seed slices"
    );
}

#[tokio::test]
async fn sqlite_membership_router_serves_packages_and_groups_without_404() {
    let pool = seeded_pool().await;
    let router = app_membership_router_with_sqlite_pool(pool);

    for uri in [
        "/app/v3/api/memberships/package_groups",
        "/app/v3/api/memberships/package_groups/1",
        "/app/v3/api/memberships/package_groups/1/packages",
        "/app/v3/api/memberships/packages",
        "/app/v3/api/memberships/packages/303",
        "/app/v3/api/memberships/privileges/speed_ups",
    ] {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            StatusCode::NOT_FOUND,
            response.status(),
            "{uri} returned 404"
        );
        assert_ne!(
            StatusCode::NOT_IMPLEMENTED,
            response.status(),
            "{uri} returned not implemented"
        );
    }
}

#[tokio::test]
async fn sqlite_membership_app_router_generates_response_request_id() {
    let pool = seeded_pool().await;
    let router = app_membership_router_with_sqlite_pool(pool);

    let response = router
        .oneshot(membership_test_request(
            Request::builder()
                .method(Method::GET)
                .uri("/app/v3/api/memberships/plans"),
            Body::empty(),
        ))
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    let request_id = response_request_id(&response);
    assert!(
        is_canonical_uuid(&request_id),
        "membership app response returned non-canonical request id {request_id}"
    );
}

#[tokio::test]
async fn sqlite_membership_app_router_overwrites_malformed_upstream_request_id() {
    let pool = seeded_pool().await;
    let router = app_membership_router_with_sqlite_pool(pool);

    let response = router
        .oneshot(membership_test_request(
            Request::builder()
                .method(Method::GET)
                .uri("/app/v3/api/memberships/plans")
                .header("X-Request-Id", "frontend-request-id"),
            Body::empty(),
        ))
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    let request_id = response_request_id(&response);
    assert!(
        is_canonical_uuid(&request_id),
        "membership app response returned non-canonical request id {request_id}"
    );
}

#[tokio::test]
async fn sqlite_admin_membership_router_overwrites_trusted_response_request_id() {
    let pool = seeded_pool().await;
    let router = admin_membership_router_with_sqlite_pool(pool);
    let trusted_request_id = "123e4567-e89b-12d3-a456-426614174000";

    let response = router
        .oneshot(admin_backend_test_request(
            Request::builder()
                .method(Method::GET)
                .uri("/backend/v3/api/memberships/plans")
                .header("X-Request-Id", trusted_request_id),
            Body::empty(),
        ))
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    let response_request_id = response_request_id(&response);
    assert!(is_canonical_uuid(&response_request_id));
    assert_ne!(trusted_request_id, response_request_id);
}

#[tokio::test]
async fn sqlite_admin_membership_router_overwrites_malformed_upstream_request_id() {
    let pool = seeded_pool().await;
    let router = admin_membership_router_with_sqlite_pool(pool);

    let response = router
        .oneshot(admin_backend_test_request(
            Request::builder()
                .method(Method::GET)
                .uri("/backend/v3/api/memberships/plans")
                .header("X-Request-Id", "frontend-request-id"),
            Body::empty(),
        ))
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    let request_id = response_request_id(&response);
    assert!(
        is_canonical_uuid(&request_id),
        "membership admin response returned non-canonical request id {request_id}"
    );
}

#[tokio::test]
async fn sqlite_purchase_creates_order_payment_membership_and_entitlements() {
    let pool = seeded_pool().await;
    activate_payment_method_for_purchase(&pool, "wechat_pay").await;
    let store = SqliteCommerceMembershipStore::new(pool.clone());

    let outcome = store
        .submit_purchase(SubmitMembershipPurchaseCommand {
            subject: AppMembershipSubject {
                tenant_id: 100001,
                organization_id: 0,
                user_id: 1,
            },
            package_id: 301,
            order_uuid: "order-membership-301".to_owned(),
            order_item_uuid: "order-item-membership-301".to_owned(),
            payment_uuid: "payment-membership-301".to_owned(),
            attempt_uuid: "payment-attempt-membership-301".to_owned(),
            membership_uuid: "membership-301".to_owned(),
            order_no: "MEMBERSHIP20260519001".to_owned(),
            out_trade_no: "MEMBERSHIPTRADE20260519001".to_owned(),
            requested_at: "2026-05-19 00:00:00".to_owned(),
            expire_at: "2026-05-19 00:30:00".to_owned(),
            action: "purchase".to_owned(),
        })
        .await
        .expect("submit membership purchase");

    assert_eq!("MEMBERSHIP20260519001", outcome.order_id);
    assert!(outcome.success);
    assert_eq!("MEMBERSHIP20260519001", outcome.request_no);
    assert_eq!("wechat_pay", outcome.provider_code);
    assert_eq!("wechat_pay", outcome.payment_method);
    assert_eq!("wechat_native", outcome.payment_product);
    assert_eq!("payment-membership-301", outcome.payment_id);
    assert_eq!(
        "https://im.sdkwork.com/cashier?scene=membership&orderId=MEMBERSHIP20260519001&paymentId=payment-membership-301",
        outcome.qr_code_payload
    );
    assert_eq!(
        "https://im.sdkwork.com/cashier?scene=membership&orderId=MEMBERSHIP20260519001&paymentId=payment-membership-301",
        outcome.cashier_url
    );
    assert_eq!(None, outcome.qr_code_image_url);
    assert_eq!(None, outcome.request_payment_payload);
    assert_eq!("scan_qr", outcome.next_action);
    assert_eq!(301, outcome.package_id);
    assert_eq!("69.90", outcome.amount);
    assert_eq!(30, outcome.duration_days);
    assert_eq!(1, outcome.target_plan_rank);

    let membership: (String, String, String, String, String, String, String) = sqlx::query_as(
        r#"
        SELECT tenant_id, organization_id, owner_user_id, plan_id, package_id, status, expires_at
        FROM membership_subscription
        WHERE id = 'membership-301'
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("membership row");
    assert_eq!("100001", membership.0);
    assert_eq!("0", membership.1);
    assert_eq!("1", membership.2);
    assert_eq!("seed-membership-plan-pro", membership.3);
    assert_eq!("301", membership.4);
    assert_eq!("pending_activation", membership.5);
    assert_eq!("2026-06-18 00:00:00", membership.6);

    let order_status: String =
        sqlx::query_scalar("SELECT status FROM commerce_order WHERE id = 'order-membership-301'")
            .fetch_one(&pool)
            .await
            .expect("order row");
    assert_eq!("pending_payment", order_status);

    let payment_status: String = sqlx::query_scalar(
        "SELECT status FROM commerce_payment_intent WHERE id = 'payment-membership-301'",
    )
    .fetch_one(&pool)
    .await
    .expect("payment row");
    assert_eq!("pending", payment_status);

    let payment_fact: (String, String) = sqlx::query_as(
        "SELECT payment_method, provider_code FROM commerce_payment_intent WHERE id = 'payment-membership-301'",
    )
    .fetch_one(&pool)
    .await
    .expect("payment fact method and provider");
    assert_eq!("wechat_pay", payment_fact.0);
    assert_eq!("wechat_pay", payment_fact.1);

    let entitlement_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM entitlement_grant WHERE source_type = 'membership_subscription' AND source_id = 'membership-301'",
    )
    .fetch_one(&pool)
    .await
    .expect("entitlement count");
    assert!(entitlement_count > 0);
}

#[tokio::test]
async fn sqlite_purchase_accumulates_existing_entitlement_accounts_for_same_subject() {
    let pool = seeded_pool().await;
    activate_payment_method_for_purchase(&pool, "wechat_pay").await;
    let store = SqliteCommerceMembershipStore::new(pool.clone());

    store
        .submit_purchase(SubmitMembershipPurchaseCommand {
            subject: AppMembershipSubject {
                tenant_id: 100001,
                organization_id: 0,
                user_id: 1,
            },
            package_id: 301,
            order_uuid: "order-membership-first-301".to_owned(),
            order_item_uuid: "order-item-membership-first-301".to_owned(),
            payment_uuid: "payment-membership-first-301".to_owned(),
            attempt_uuid: "payment-attempt-membership-first-301".to_owned(),
            membership_uuid: "membership-first-301".to_owned(),
            order_no: "MEMBERSHIP20260519011".to_owned(),
            out_trade_no: "MEMBERSHIPTRADE20260519011".to_owned(),
            requested_at: "2026-05-19 00:00:00".to_owned(),
            expire_at: "2026-05-19 00:30:00".to_owned(),
            action: "purchase".to_owned(),
        })
        .await
        .expect("submit first membership purchase");

    let first_accounts = membership_entitlement_accounts_for_subject(&pool).await;
    assert!(
        !first_accounts.is_empty(),
        "first purchase must create entitlement accounts"
    );

    store
        .submit_purchase(SubmitMembershipPurchaseCommand {
            subject: AppMembershipSubject {
                tenant_id: 100001,
                organization_id: 0,
                user_id: 1,
            },
            package_id: 301,
            order_uuid: "order-membership-second-301".to_owned(),
            order_item_uuid: "order-item-membership-second-301".to_owned(),
            payment_uuid: "payment-membership-second-301".to_owned(),
            attempt_uuid: "payment-attempt-membership-second-301".to_owned(),
            membership_uuid: "membership-second-301".to_owned(),
            order_no: "MEMBERSHIP20260519012".to_owned(),
            out_trade_no: "MEMBERSHIPTRADE20260519012".to_owned(),
            requested_at: "2026-05-20 00:00:00".to_owned(),
            expire_at: "2026-05-20 00:30:00".to_owned(),
            action: "purchase".to_owned(),
        })
        .await
        .expect("submit second membership purchase");

    let second_accounts = membership_entitlement_accounts_for_subject(&pool).await;
    assert_eq!(first_accounts.len(), second_accounts.len());
    for (first, second) in first_accounts.iter().zip(second_accounts.iter()) {
        assert_eq!(first.0, second.0);
        assert_eq!(first.1 * 2, second.1);
        assert_eq!(first.2 * 2, second.2);
    }

    let second_grant_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM entitlement_grant WHERE source_type = 'membership_subscription' AND source_id = 'membership-second-301'",
    )
    .fetch_one(&pool)
    .await
    .expect("second entitlement grant count");
    assert_eq!(first_accounts.len() as i64, second_grant_count);

    let ledger_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM entitlement_ledger_entry WHERE tenant_id = '100001' AND subject_type = 'user' AND subject_id = '1' AND business_type = 'membership_grant'",
    )
    .fetch_one(&pool)
    .await
    .expect("membership entitlement ledger count");
    assert_eq!((first_accounts.len() * 2) as i64, ledger_count);
}

#[tokio::test]
async fn sqlite_speed_up_consumes_priority_entitlement_once() {
    let pool = seeded_pool().await;
    seed_speed_up_membership(&pool).await;
    let router = app_membership_router_with_sqlite_pool(pool.clone());

    let response = router
        .clone()
        .oneshot(signed_request(
            "POST",
            "/app/v3/api/memberships/privileges/speed_ups",
            "{}",
        ))
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());
    let payload = json_payload(response).await;
    assert_eq!("2000", payload["code"]);

    let used_quantity: String = sqlx::query_scalar(
        "SELECT total_used FROM entitlement_account WHERE id = 'membership-entitlement-speed-up'",
    )
    .fetch_one(&pool)
    .await
    .expect("speed up entitlement usage");
    assert_eq!("1", used_quantity);

    let usage_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM entitlement_ledger_entry WHERE account_id = 'membership-entitlement-speed-up'",
    )
    .fetch_one(&pool)
    .await
    .expect("speed up usage record count");
    assert_eq!(1, usage_count);

    let exhausted_response = router
        .oneshot(signed_request(
            "POST",
            "/app/v3/api/memberships/privileges/speed_ups",
            "{}",
        ))
        .await
        .unwrap();
    assert_eq!(StatusCode::CONFLICT, exhausted_response.status());
}

#[tokio::test]
async fn sqlite_membership_points_history_honors_pagination_query() {
    let pool = seeded_pool().await;
    seed_points_history(&pool).await;
    let router = app_membership_router_with_sqlite_pool(pool);

    let payload = request_json(
        router,
        signed_request(
            "GET",
            "/app/v3/api/memberships/points/history?page=2&page_size=1",
            "",
        ),
    )
    .await;

    assert_eq!("2000", payload["code"]);
    let items = payload["data"].as_array().expect("history item array");
    assert_eq!(1, items.len());
    assert_eq!("ledger-2", items[0]["id"]);
}

#[tokio::test]
async fn sqlite_admin_membership_router_manages_standard_membership_catalog() {
    let pool = seeded_pool().await;
    seed_membership_for_admin(&pool).await;
    let router = admin_membership_router_with_sqlite_pool(pool.clone());

    let plans = request_json(
        router.clone(),
        signed_request("GET", "/backend/v3/api/memberships/plans", ""),
    )
    .await;
    assert_eq!("2000", plans["code"]);
    assert_eq!("seed-membership-plan-pro", plans["data"]["items"][1]["id"]);
    assert_eq!("pro", plans["data"]["items"][1]["code"]);
    assert_eq!(1, plans["data"]["items"][1]["rank"]);

    let create_plan = request_json(
        router.clone(),
        signed_request(
            "POST",
            "/backend/v3/api/memberships/plans",
            r#"{"code":"team","name":"Team member","rank":40,"status":"active"}"#,
        ),
    )
    .await;
    assert_eq!("Team member", create_plan["data"]["item"]["name"]);
    assert_eq!("team", create_plan["data"]["item"]["code"]);
    assert_eq!(40, create_plan["data"]["item"]["rank"]);
    let created_plan_id = create_plan["data"]["item"]["id"]
        .as_str()
        .expect("created plan id")
        .to_owned();

    let update_plan = request_json(
        router.clone(),
        signed_request(
            "PUT",
            &format!("/backend/v3/api/memberships/plans/{created_plan_id}"),
            r#"{"code":"team","name":"Team workspace","rank":41,"status":"inactive"}"#,
        ),
    )
    .await;
    assert_eq!("Team workspace", update_plan["data"]["item"]["name"]);
    assert_eq!(41, update_plan["data"]["item"]["rank"]);
    assert_eq!("inactive", update_plan["data"]["item"]["status"]);
    assert_eq!(
        0,
        update_plan["data"]["item"]["benefits"]
            .as_array()
            .expect("updated plan benefits array")
            .len()
    );

    let update_plan_benefits = request_json(
        router.clone(),
        signed_request(
            "PUT",
            &format!("/backend/v3/api/memberships/plans/{created_plan_id}"),
            r#"{"code":"team","name":"Team workspace","rank":41,"status":"active","benefits":[{"id":9001,"name":"Priority render","benefitKey":"priority_render","type":"quota","description":"Priority rendering quota","icon":"sparkles","usageLimit":100,"usedCount":0,"claimed":false}]}"#,
        ),
    )
    .await;
    assert_eq!(
        "Priority render",
        update_plan_benefits["data"]["item"]["benefits"][0]["name"]
    );
    assert_eq!(
        "priority_render",
        update_plan_benefits["data"]["item"]["benefits"][0]["benefitKey"]
    );
    assert_eq!(
        100,
        update_plan_benefits["data"]["item"]["benefits"][0]["usageLimit"]
    );

    let update_seed_plan = request_json(
        router.clone(),
        signed_request(
            "PUT",
            "/backend/v3/api/memberships/plans/seed-membership-plan-pro",
            r#"{"code":"pro","name":"Pro member","rank":20,"status":"active"}"#,
        ),
    )
    .await;
    assert_eq!("Pro member", update_seed_plan["data"]["item"]["name"]);
    assert_eq!(20, update_seed_plan["data"]["item"]["rank"]);

    let team_app_benefits = request_json(
        app_membership_router_with_sqlite_pool(pool.clone()),
        signed_request("GET", "/app/v3/api/memberships/benefits?plan_id=41", ""),
    )
    .await;
    assert_eq!("2000", team_app_benefits["code"]);
    assert_eq!(
        "priority_render",
        team_app_benefits["data"][0]["benefitKey"]
    );

    let rewrite_plan_benefits = request_json(
        router.clone(),
        signed_request(
            "PUT",
            &format!("/backend/v3/api/memberships/plans/{created_plan_id}"),
            r#"{"code":"team","name":"Team workspace","rank":41,"status":"active","benefits":[{"id":9001,"name":"Priority queue","benefitKey":"priority_queue","type":"quota","description":"Updated priority queue quota","icon":"sparkles","usageLimit":80,"usedCount":0,"claimed":false}]}"#,
        ),
    )
    .await;
    assert_eq!(
        "Priority queue",
        rewrite_plan_benefits["data"]["item"]["benefits"][0]["name"]
    );
    assert_eq!(
        "priority_queue",
        rewrite_plan_benefits["data"]["item"]["benefits"][0]["benefitKey"]
    );

    let clear_plan_benefits = request_json(
        router.clone(),
        signed_request(
            "PUT",
            &format!("/backend/v3/api/memberships/plans/{created_plan_id}"),
            r#"{"code":"team","name":"Team workspace","rank":41,"status":"active","benefits":[]}"#,
        ),
    )
    .await;
    assert_eq!(
        0,
        clear_plan_benefits["data"]["item"]["benefits"]
            .as_array()
            .expect("cleared plan benefits array")
            .len()
    );

    let cleared_team_app_benefits = request_json(
        app_membership_router_with_sqlite_pool(pool.clone()),
        signed_request("GET", "/app/v3/api/memberships/benefits?plan_id=41", ""),
    )
    .await;
    assert_eq!("2000", cleared_team_app_benefits["code"]);
    assert_eq!(
        0,
        cleared_team_app_benefits["data"]
            .as_array()
            .expect("cleared app benefits array")
            .len()
    );

    let create_group = request_json(
        router.clone(),
        signed_request(
            "POST",
            "/backend/v3/api/memberships/package_groups",
            r#"{"code":"membership-team","name":"Team bundles","description":"Team workspace membership plans","billingCycle":"team","durationDays":30,"sortWeight":15,"status":"active"}"#,
        ),
    )
    .await;
    assert_eq!("Team bundles", create_group["data"]["item"]["name"]);
    assert_eq!("membership-team", create_group["data"]["item"]["code"]);
    assert_eq!(30, create_group["data"]["item"]["durationDays"]);
    let created_group_id = create_group["data"]["item"]["id"]
        .as_str()
        .expect("created package group id")
        .to_owned();

    let update_group = request_json(
        router.clone(),
        signed_request(
            "PUT",
            &format!("/backend/v3/api/memberships/package_groups/{created_group_id}"),
            r#"{"code":"membership-team","name":"Team bundles updated","description":"Updated team workspace membership plans","billingCycle":"team","durationDays":60,"sortWeight":16,"status":"inactive"}"#,
        ),
    )
    .await;
    assert_eq!("Team bundles updated", update_group["data"]["item"]["name"]);
    assert_eq!(60, update_group["data"]["item"]["durationDays"]);
    assert_eq!(16, update_group["data"]["item"]["sortWeight"]);
    assert_eq!("inactive", update_group["data"]["item"]["status"]);

    let create_package = request_json(
        router.clone(),
        signed_request(
            "POST",
            "/backend/v3/api/memberships/packages",
            r#"{"code":"membership-month-pro-plus","packageGroupId":"seed-membership-package-group-year","planId":"seed-membership-plan-pro","name":"Pro Plus Monthly","priceAmount":"89.90","currencyCode":"CNY","durationDays":30,"status":"active"}"#,
        ),
    )
    .await;
    assert_eq!("Pro Plus Monthly", create_package["data"]["item"]["name"]);
    assert_eq!(
        "membership-month-pro-plus",
        create_package["data"]["item"]["code"]
    );
    assert_eq!(
        "seed-membership-package-group-year",
        create_package["data"]["item"]["packageGroupId"]
    );
    let package_id = create_package["data"]["item"]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let package_group_id_after_create: String =
        sqlx::query_scalar("SELECT package_group_id FROM membership_package WHERE id = ?1")
            .bind(&package_id)
            .fetch_one(&pool)
            .await
            .expect("created package group id");
    assert_eq!(
        "seed-membership-package-group-year",
        package_group_id_after_create
    );

    let update_package = request_json(
        router.clone(),
        signed_request(
            "PUT",
            &format!("/backend/v3/api/memberships/packages/{package_id}"),
            r#"{"code":"membership-month-pro-plus","packageGroupId":"seed-membership-package-group-month","planId":"seed-membership-plan-pro","name":"Pro Plus Monthly Updated","priceAmount":"99.90","currencyCode":"CNY","durationDays":60,"status":"inactive"}"#,
        ),
    )
    .await;
    assert_eq!(
        "Pro Plus Monthly Updated",
        update_package["data"]["item"]["name"]
    );
    assert_eq!("inactive", update_package["data"]["item"]["status"]);
    assert_eq!(
        "seed-membership-package-group-month",
        update_package["data"]["item"]["packageGroupId"]
    );
    let package_group_id_after_update: String =
        sqlx::query_scalar("SELECT package_group_id FROM membership_package WHERE id = ?1")
            .bind(&package_id)
            .fetch_one(&pool)
            .await
            .expect("updated package group id");
    assert_eq!(
        "seed-membership-package-group-month",
        package_group_id_after_update
    );

    let packages = request_json(
        router.clone(),
        signed_request(
            "GET",
            "/backend/v3/api/memberships/packages?status=inactive",
            "",
        ),
    )
    .await;
    assert!(packages["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["id"] == package_id));
    assert!(packages["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["packageGroupId"] == "seed-membership-package-group-month"));

    let package_groups = request_json(
        router.clone(),
        signed_request(
            "GET",
            "/backend/v3/api/memberships/package_groups?status=active",
            "",
        ),
    )
    .await;
    assert_eq!("2000", package_groups["code"]);
    assert!(package_groups["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| {
            item["id"] == "seed-membership-package-group-month"
                && item["code"] == "membership-month"
                && item["durationDays"] == 30
        }));

    let memberships = request_json(
        router.clone(),
        signed_request("GET", "/backend/v3/api/memberships/members?user_id=1", ""),
    )
    .await;
    assert_eq!("membership-admin", memberships["data"]["items"][0]["id"]);
    assert_eq!("1", memberships["data"]["items"][0]["ownerUserId"]);
    assert_eq!("pro", memberships["data"]["items"][0]["planCode"]);
    assert_eq!(Value::Null, memberships["data"]["items"][0]["levelCode"]);

    let membership_status = request_json(
        router.clone(),
        signed_request(
            "PATCH",
            "/backend/v3/api/memberships/members/membership-admin/status",
            r#"{"status":"suspended"}"#,
        ),
    )
    .await;
    assert_eq!("suspended", membership_status["data"]["item"]["status"]);

    let entitlements = request_json(
        router.clone(),
        signed_request(
            "GET",
            "/backend/v3/api/memberships/entitlements?membership_id=membership-admin",
            "",
        ),
    )
    .await;
    assert_eq!(
        "membership-entitlement-admin",
        entitlements["data"]["items"][0]["id"]
    );
    assert_eq!("ai_quota", entitlements["data"]["items"][0]["code"]);
    assert_eq!("10", entitlements["data"]["items"][0]["quota"]);

    let delete_package = request_json(
        router.clone(),
        signed_request(
            "DELETE",
            &format!("/backend/v3/api/memberships/packages/{package_id}"),
            "",
        ),
    )
    .await;
    assert_eq!(true, delete_package["data"]["deleted"]);
    assert_eq!(package_id, delete_package["data"]["packageId"]);

    let delete_plan = request_json(
        router.clone(),
        signed_request(
            "DELETE",
            &format!("/backend/v3/api/memberships/plans/{created_plan_id}"),
            "",
        ),
    )
    .await;
    assert_eq!(true, delete_plan["data"]["deleted"]);
    assert_eq!(created_plan_id, delete_plan["data"]["planId"]);

    let delete_group = request_json(
        router,
        signed_request(
            "DELETE",
            &format!("/backend/v3/api/memberships/package_groups/{created_group_id}"),
            "",
        ),
    )
    .await;
    assert_eq!(true, delete_group["data"]["deleted"]);
    assert_eq!(created_group_id, delete_group["data"]["packageGroupId"]);
}

async fn seeded_pool() -> SqlitePool {
    let pool = commerce_sqlite_memory_pool().await;
    install_schema(&pool).await;
    upsert_sqlite_commerce_experience_seed(&pool)
        .await
        .expect("seed commerce experience");
    pool
}

async fn activate_payment_method_for_purchase(pool: &SqlitePool, method_key: &str) {
    sqlx::query("UPDATE commerce_payment_method SET status = 'active' WHERE method_key = ?1")
        .bind(method_key)
        .execute(pool)
        .await
        .expect("activate payment method");
    sqlx::query(
        "UPDATE commerce_payment_provider_account SET status = 'active' WHERE provider_code = ?1",
    )
    .bind(method_key)
    .execute(pool)
    .await
    .expect("activate payment provider account");
    sqlx::query("UPDATE commerce_payment_channel SET status = 'active' WHERE method_id IN (SELECT id FROM commerce_payment_method WHERE method_key = ?1)")
        .bind(method_key)
        .execute(pool)
        .await
        .expect("activate payment channels");
    sqlx::query("UPDATE commerce_payment_route_rule SET status = 'active' WHERE channel_id IN (SELECT id FROM commerce_payment_channel WHERE method_id IN (SELECT id FROM commerce_payment_method WHERE method_key = ?1))")
        .bind(method_key)
        .execute(pool)
        .await
        .expect("activate payment route rules");
}

async fn seed_membership_for_admin(pool: &SqlitePool) {
    sqlx::query(
        r#"
        INSERT INTO membership_subscription
            (id, tenant_id, organization_id, subscription_no, subject_type, subject_id, owner_user_id, plan_id, plan_version_id, package_id, current_period_id, source_order_id, source_payment_intent_id, status, starts_at, expires_at, grace_until, cancel_at_period_end, request_no, idempotency_key, created_at, updated_at)
        VALUES
            ('membership-admin', '100001', '0', 'membership-admin', 'user', '1', '1', 'seed-membership-plan-pro', 'seed-membership-plan-version-pro-v1', '301', 'membership-admin-period-1', 'membership-order-admin', 'membership-payment-admin', 'active', '2026-05-01 00:00:00', '2026-06-01 00:00:00', NULL, 0, 'membership-admin', 'membership-admin', '2026-05-01 00:00:00', '2026-05-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await
    .expect("insert admin membership fixture");
    sqlx::query(
        r#"
        INSERT INTO entitlement_grant
            (id, tenant_id, organization_id, grant_no, benefit_id, subject_type, subject_id, source_type, source_id, grant_policy, granted_quantity, status, starts_at, expires_at, request_no, idempotency_key, created_at, updated_at)
        VALUES
            ('membership-entitlement-admin-grant', '100001', '0', 'membership-entitlement-admin-grant', 'seed-benefit-ai-quota', 'user', '1', 'membership_subscription', 'membership-admin', 'membership_plan', '10', 'active', '2026-05-01 00:00:00', '2026-06-01 00:00:00', 'membership-entitlement-admin', 'membership-entitlement-admin', '2026-05-01 00:00:00', '2026-05-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await
    .expect("insert admin entitlement grant fixture");
    sqlx::query(
        r#"
        INSERT INTO entitlement_account
            (id, tenant_id, organization_id, account_no, benefit_id, subject_type, subject_id, total_granted, total_used, balance, status, expires_at, version, created_at, updated_at)
        VALUES
            ('membership-entitlement-admin', '100001', '0', 'membership-entitlement-admin', 'seed-benefit-ai-quota', 'user', '1', '10', '0', '10', 'active', '2026-06-01 00:00:00', 0, '2026-05-01 00:00:00', '2026-05-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await
    .expect("insert admin entitlement account fixture");
}

async fn seed_speed_up_membership(pool: &SqlitePool) {
    sqlx::query(
        r#"
        INSERT INTO membership_subscription
            (id, tenant_id, organization_id, subscription_no, subject_type, subject_id, owner_user_id, plan_id, plan_version_id, package_id, current_period_id, source_order_id, source_payment_intent_id, status, starts_at, expires_at, grace_until, cancel_at_period_end, request_no, idempotency_key, created_at, updated_at)
        VALUES
            ('membership-speed-up', '100001', '0', 'membership-speed-up', 'user', '1', '1', 'seed-membership-plan-pro', 'seed-membership-plan-version-pro-v1', '301', 'membership-speed-up-period-1', 'membership-order-speed-up', 'membership-payment-speed-up', 'active', '2026-05-01 00:00:00', '2026-06-01 00:00:00', NULL, 0, 'membership-speed-up', 'membership-speed-up', '2026-05-01 00:00:00', '2026-05-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await
    .expect("insert speed up membership fixture");
    sqlx::query(
        r#"
        INSERT INTO entitlement_grant
            (id, tenant_id, organization_id, grant_no, benefit_id, subject_type, subject_id, source_type, source_id, grant_policy, granted_quantity, status, starts_at, expires_at, request_no, idempotency_key, created_at, updated_at)
        VALUES
            ('membership-entitlement-speed-up-grant', '100001', '0', 'membership-entitlement-speed-up-grant', 'seed-benefit-priority-speed-up', 'user', '1', 'membership_subscription', 'membership-speed-up', 'membership_plan', '1', 'active', '2026-05-01 00:00:00', '2026-06-01 00:00:00', 'membership-entitlement-speed-up', 'membership-entitlement-speed-up', '2026-05-01 00:00:00', '2026-05-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await
    .expect("insert speed up entitlement grant fixture");
    sqlx::query(
        r#"
        INSERT INTO entitlement_account
            (id, tenant_id, organization_id, account_no, benefit_id, subject_type, subject_id, total_granted, total_used, balance, status, expires_at, version, created_at, updated_at)
        VALUES
            ('membership-entitlement-speed-up', '100001', '0', 'membership-entitlement-speed-up', 'seed-benefit-priority-speed-up', 'user', '1', '1', '0', '1', 'active', '2026-06-01 00:00:00', 0, '2026-05-01 00:00:00', '2026-05-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await
    .expect("insert speed up entitlement account fixture");
}

async fn seed_points_history(pool: &SqlitePool) {
    sqlx::query(
        r#"
        INSERT INTO commerce_account
            (id, tenant_id, organization_id, owner_user_id, asset_type, currency_code, available_amount, frozen_amount, version, status, created_at, updated_at)
        VALUES
            ('membership-points-account-history', '100001', '0', '1', 'points', 'PTS', '60', '0', 1, 'active', '2026-05-01 00:00:00', '2026-05-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await
    .expect("insert points account fixture");

    for (id, amount, balance_after, created_at) in [
        ("ledger-1", "10", "60", "2026-05-03 00:00:00"),
        ("ledger-2", "20", "50", "2026-05-02 00:00:00"),
        ("ledger-3", "30", "30", "2026-05-01 00:00:00"),
    ] {
        sqlx::query(
            r#"
            INSERT INTO commerce_account_ledger_entry
                (id, tenant_id, organization_id, account_id, owner_user_id, asset_type, direction, amount, balance_after, business_type, transaction_no, request_no, idempotency_key, source_type, source_id, remark, created_at)
            VALUES
                (?1, '100001', '0', 'membership-points-account-history', '1', 'points', 'credit', ?2, ?3, 'membership_points', ?1, ?1, ?1, 'membership', ?1, ?1, ?4)
            "#,
        )
        .bind(id)
        .bind(amount)
        .bind(balance_after)
        .bind(created_at)
        .execute(pool)
        .await
        .expect("insert points history fixture");
    }
}

async fn membership_entitlement_accounts_for_subject(pool: &SqlitePool) -> Vec<(String, i64, i64)> {
    sqlx::query(
        r#"
        SELECT benefit_id, total_granted, balance
        FROM entitlement_account
        WHERE tenant_id = '100001'
          AND subject_type = 'user'
          AND subject_id = '1'
        ORDER BY benefit_id
        "#,
    )
    .fetch_all(pool)
    .await
    .expect("membership entitlement accounts")
    .into_iter()
    .map(|row| {
        let benefit_id: String = row.try_get("benefit_id").expect("benefit id");
        let total_granted: String = row.try_get("total_granted").expect("total granted");
        let balance: String = row.try_get("balance").expect("balance");
        (
            benefit_id,
            total_granted.parse().expect("numeric total granted"),
            balance.parse().expect("numeric balance"),
        )
    })
    .collect()
}

fn signed_request(method: &str, uri: &str, body: &str) -> Request<Body> {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .header("X-Request-Id", "123e4567-e89b-12d3-a456-426614174000");
    if matches!(method, "POST" | "PUT" | "PATCH" | "DELETE") {
        builder = builder
            .header("Idempotency-Key", "membership-test-idem")
            .header("Sdkwork-Request-No", "membership-test-req-1");
    }
    if uri.starts_with("/backend/") {
        admin_backend_test_request(builder, Body::from(body.to_owned()))
    } else {
        membership_test_request(builder, Body::from(body.to_owned()))
    }
}

fn membership_test_request(builder: axum::http::request::Builder, body: Body) -> Request<Body> {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        std::env::set_var("SDKWORK_ENV", "test");
        std::env::set_var("SDKWORK_DEPLOYMENT_MODE", "local");
    });
    commerce_test_request(builder, Some(&standard_context()), body)
}

fn admin_backend_test_request(builder: axum::http::request::Builder, body: Body) -> Request<Body> {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        std::env::set_var("SDKWORK_ENV", "test");
        std::env::set_var("SDKWORK_DEPLOYMENT_MODE", "local");
    });
    commerce_test_request(builder, Some(&admin_backend_context()), body)
}

fn standard_context() -> IamAppContext {
    IamAppContext::new(
        "100001",
        None,
        "1",
        "session-30",
        "app-1",
        Environment::Test,
        DeploymentMode::Local,
        AuthLevel::Password,
        vec!["tenant:100001".to_owned()],
        vec!["commerce.*".to_owned()],
    )
}

fn admin_backend_context() -> IamAppContext {
    IamAppContext::new(
        "100001",
        Some("1"),
        "1",
        "session-30",
        "app-1",
        Environment::Test,
        DeploymentMode::Local,
        AuthLevel::Password,
        vec!["tenant:100001".to_owned(), "organization:1".to_owned()],
        vec!["commerce.*".to_owned()],
    )
}

async fn request_json(router: axum::Router, request: Request<Body>) -> Value {
    let response = router.oneshot(request).await.unwrap();
    assert_eq!(StatusCode::OK, response.status());
    json_payload(response).await
}

fn response_request_id(response: &axum::response::Response) -> String {
    response
        .headers()
        .get("X-Request-Id")
        .expect("response request id header")
        .to_str()
        .expect("request id header text")
        .to_owned()
}

fn is_canonical_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => *byte == b'-',
            _ => matches!(*byte, b'0'..=b'9' | b'a'..=b'f'),
        })
}

async fn json_payload(response: axum::response::Response) -> Value {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&body).unwrap()
}

async fn install_schema(pool: &SqlitePool) {
    for statement in sqlite_schema_statements() {
        pool.execute(statement.as_str())
            .await
            .unwrap_or_else(|error| panic!("failed schema statement: {statement}\n{error}"));
    }
}

async fn install_schema_with_legacy_payment_method_provider(pool: &SqlitePool) {
    for statement in sqlite_schema_statements_with_legacy_payment_method_provider() {
        pool.execute(statement.as_str())
            .await
            .unwrap_or_else(|error| panic!("failed legacy schema statement: {statement}\n{error}"));
    }
}

fn sqlite_schema_statements() -> Vec<String> {
    commerce_initial_migration_sql()
        .split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .map(|statement| {
            statement
                .replace("TIMESTAMPTZ", "TEXT")
                .replace("JSONB", "TEXT")
                .replace("BIGINT", "INTEGER")
                .replace("BOOLEAN", "INTEGER")
                .replace("VARCHAR", "TEXT")
        })
        .collect()
}

fn sqlite_schema_statements_with_legacy_payment_method_provider() -> Vec<String> {
    sqlite_schema_statements()
        .into_iter()
        .map(|statement| {
            if statement.contains("CREATE TABLE IF NOT EXISTS commerce_payment_method") {
                statement.replace(
                    "provider_code TEXT NOT NULL,",
                    "provider TEXT NOT NULL,\n  provider_code TEXT NOT NULL,",
                )
            } else {
                statement
            }
        })
        .collect()
}

fn integrity_issue_codes(
    report: &sdkwork_commerce_membership_repository_sqlx::CommerceExperienceSeedIntegrityReport,
) -> Vec<String> {
    report
        .issues
        .iter()
        .map(|issue| issue.code.clone())
        .collect()
}
