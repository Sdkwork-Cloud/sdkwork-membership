//! Source-inspection guards for the membership SQLx crate.
//!
//! These tests verify that legacy naming (VIP, level, group_id, compat query
//! aliases, payment method aliases) does not regress into the codebase. They do
//! not require a database and run as part of `cargo test --workspace`.

#[test]
fn commerce_membership_sqlx_exposes_standard_store_names_without_legacy_membership_aliases() {
    let sqlite_source = include_str!("../src/sqlite.rs");
    let postgres_source = include_str!("../src/postgres.rs");
    let lib_source = include_str!("../src/lib.rs");
    let router_source = include_str!("../src/router.rs");
    let admin_router_source = include_str!("../src/admin_router.rs");
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
