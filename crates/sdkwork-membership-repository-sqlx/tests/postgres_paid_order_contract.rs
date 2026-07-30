const POSTGRES_SOURCE: &str = include_str!("../src/postgres.rs");

#[test]
fn postgres_paid_order_fulfillment_keeps_concurrency_and_replay_guards() {
    assert!(POSTGRES_SOURCE.contains("SELECT pg_advisory_xact_lock($1)"));
    assert!(POSTGRES_SOURCE.contains("AND mp.source_order_id = $4"));
    assert!(POSTGRES_SOURCE.contains("starts_at = CASE WHEN $6 THEN starts_at ELSE $7 END"));
}

#[test]
fn postgres_platform_catalog_and_entitlement_identity_are_stable() {
    assert!(POSTGRES_SOURCE
        .contains("p.tenant_id = CAST($1 AS TEXT) OR p.tenant_id = CAST($4 AS TEXT)"));
    assert!(POSTGRES_SOURCE.contains(".bind(DEFAULT_CATALOG_TENANT_ID)"));
    assert!(POSTGRES_SOURCE.contains(
        "let period_id = membership_period_id(&binding.membership_uuid, &command.order_no);"
    ));
    assert!(POSTGRES_SOURCE
        .contains("let grant_id = format!(\"{}-entitlement-grant-{}\", period_id, index + 1);"));
    assert!(POSTGRES_SOURCE
        .contains("let ledger_id = format!(\"{}-entitlement-ledger-{}\", period_id, index + 1);"));
}
