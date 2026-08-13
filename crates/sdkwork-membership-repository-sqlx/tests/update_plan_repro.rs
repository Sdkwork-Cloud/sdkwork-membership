//! Temporary reproduction test for the PUT /plans/{id} 50301 regression.
//!
//! Simulates the admin "edit plan and save" flow against a real PostgreSQL
//! database (SDKWORK_DATABASE_URL or the canonical local dev URL) using the
//! exact command path the backend API executes. Run with:
//!
//! ```text
//! cargo test -p sdkwork-membership-repository-sqlx --test update_plan_repro -- --nocapture
//! ```

use sdkwork_membership_repository_sqlx::{
    AdminMembershipPlanMutation, AdminMembershipStore, AdminMembershipSubject,
    PostgresCommerceMembershipStore, UpdateAdminMembershipPlanCommand,
};

fn database_url() -> String {
    std::env::var("SDKWORK_DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev".to_owned()
    })
}

fn subject() -> AdminMembershipSubject {
    AdminMembershipSubject {
        tenant_id: 100_001,
        organization_id: 0,
        operator_id: 1,
        operator_type: 1,
    }
}

#[tokio::test]
#[ignore = "requires a local PostgreSQL database; run with --ignored"]
async fn update_plan_circle_membership_without_benefits_succeeds() {
    let pool = sqlx::PgPool::connect(&database_url())
        .await
        .expect("connect database");
    let store = PostgresCommerceMembershipStore::new(pool);
    let item = store
        .update_admin_membership_plan(UpdateAdminMembershipPlanCommand {
            subject: subject(),
            plan_id: "plan-circle-membership".to_owned(),
            input: AdminMembershipPlanMutation {
                category: "token".to_owned(),
                code: "plan-circle-membership".to_owned(),
                name: "plan-circle-membership".to_owned(),
                rank: 0,
                benefits: None,
                status: "active".to_owned(),
            },
            request_id: "repro-update-empty-benefits".to_owned(),
            requested_at: "2026-08-13T00:00:00.000Z".to_owned(),
        })
        .await;
    assert!(
        item.is_ok(),
        "update without benefits must succeed, got: {item:?}"
    );
    eprintln!("update (no benefits) OK -> {:?}", item.unwrap());
}

#[tokio::test]
#[ignore = "requires a local PostgreSQL database; run with --ignored"]
async fn update_plan_with_legacy_discount_benefit_normalises_and_succeeds() {
    let pool = sqlx::PgPool::connect(&database_url())
        .await
        .expect("connect database");
    let store = PostgresCommerceMembershipStore::new(pool);
    let item = store
        .update_admin_membership_plan(UpdateAdminMembershipPlanCommand {
            subject: subject(),
            plan_id: "plan-circle-membership".to_owned(),
            input: AdminMembershipPlanMutation {
                category: "token".to_owned(),
                code: "plan-circle-membership".to_owned(),
                name: "plan-circle-membership".to_owned(),
                rank: 0,
                benefits: Some(vec![
                    sdkwork_membership_repository_sqlx::AppMembershipBenefitItem {
                        id: 1,
                        name: "Legacy discount benefit".to_owned(),
                        benefit_key: Some("legacy_discount".to_owned()),
                        r#type: Some("discount".to_owned()),
                        description: None,
                        icon: None,
                        claimed: false,
                        usage_limit: Some(1),
                        used_count: Some(0),
                        display_value: None,
                    },
                ]),
                status: "active".to_owned(),
            },
            request_id: "repro-update-discount-benefit".to_owned(),
            requested_at: "2026-08-13T00:00:00.000Z".to_owned(),
        })
        .await;
    assert!(
        item.is_ok(),
        "update with legacy discount benefit must succeed (normalised), got: {item:?}"
    );
    eprintln!("update (discount benefit) OK -> {item:?}");
}
