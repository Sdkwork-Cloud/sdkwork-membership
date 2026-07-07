use sdkwork_contract_service::{CommerceServiceError, CommerceSurfaceProfile};
use sdkwork_membership_service::{
    membership_service_contract, EntitlementGrantDraft, MembershipActivationDraft,
    MembershipBillingCycle, MembershipPackageDraft, MembershipPackageGroupDraft,
    MembershipPackageGroupDraftInput, MembershipPackageGroupListQuery, MembershipPackageListQuery,
    MembershipPlanDraft, MembershipPortRequirement, MembershipPurchaseDraft,
    MembershipRepositoryCommand, MembershipStatus, MembershipTransition,
};

#[test]
fn membership_domain_contract_uses_standard_plan_and_package_terms() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let checked_sources = [
        "Cargo.toml",
        "src/lib.rs",
        "src/domain/mod.rs",
        "src/commands/mod.rs",
        "src/queries/mod.rs",
        "src/ports/mod.rs",
        "src/service/mod.rs",
    ];
    let banned_fragments = [
        concat!("V", "ip"),
        concat!("v", "ip"),
        concat!("V", "IP"),
        concat!("Membership", "Level"),
        concat!("level", "_id"),
        concat!("\"", "group", "_id", "\""),
        concat!("membership", "_duration_days"),
        concat!("CreateMembership", "Level"),
    ];

    let mut violations = Vec::new();
    for relative_path in checked_sources {
        let contents = std::fs::read_to_string(crate_root.join(relative_path))
            .unwrap_or_else(|error| panic!("failed to read {relative_path}: {error}"));
        for banned in banned_fragments {
            if contents.contains(banned) {
                violations.push(format!("{relative_path} contains `{banned}`"));
            }
        }
        for line in contents.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with(concat!("pub group", "_id:"))
                || trimmed.starts_with(concat!("group", "_id:"))
            {
                violations.push(format!("{relative_path} contains legacy `{trimmed}`"));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "membership domain must use plan/package terminology without legacy fragments:\n{}",
        violations.join("\n")
    );
}

#[test]
fn validates_membership_plan_definition_and_surface_visibility() {
    let plan = MembershipPlanDraft::new(
        "100001",
        "membership-plan-pro",
        "Pro member",
        30,
        vec![CommerceSurfaceProfile::App, CommerceSurfaceProfile::Console],
    )
    .unwrap();

    assert_eq!(plan.plan_id, "membership-plan-pro");
    assert_eq!(plan.validity_days, 30);
    assert!(plan.visible_surfaces.contains(&CommerceSurfaceProfile::App));
    assert!(MembershipPlanDraft::new(
        "100001",
        "membership-plan-pro",
        "Pro member",
        0,
        vec![CommerceSurfaceProfile::App]
    )
    .is_err());
}

#[test]
fn validates_membership_status_lifecycle() {
    assert_eq!(
        MembershipStatus::PendingActivation.as_storage_str(),
        "pending_activation"
    );
    assert_eq!(MembershipStatus::Active.as_storage_str(), "active");
    assert_eq!(
        MembershipStatus::GracePeriod.as_storage_str(),
        "grace_period"
    );
    assert_eq!(MembershipStatus::Expired.as_storage_str(), "expired");
    assert_eq!(MembershipStatus::Cancelled.as_storage_str(), "cancelled");

    assert_eq!(
        MembershipTransition::new(
            MembershipStatus::PendingActivation,
            MembershipStatus::Active
        )
        .validate(),
        Ok(()),
    );
    assert_eq!(
        MembershipTransition::new(MembershipStatus::Active, MembershipStatus::Expired).validate(),
        Ok(()),
    );
    assert_eq!(
        MembershipTransition::new(MembershipStatus::Expired, MembershipStatus::Active).validate(),
        Err(CommerceServiceError::invalid_state(
            "invalid membership status transition"
        )),
    );
}

#[test]
fn membership_activation_requires_paid_order_and_idempotency() {
    let activation = MembershipActivationDraft::new(
        "100001",
        "order-1",
        "payment-1",
        "user-1",
        "membership-plan-pro",
        "idem-activate-1",
    )
    .unwrap();

    assert_eq!(activation.owner_user_id, "user-1");
    assert_eq!(activation.payment_id, "payment-1");
    assert!(MembershipActivationDraft::new(
        "100001",
        "order-1",
        "",
        "user-1",
        "membership-plan-pro",
        "idem-activate-1"
    )
    .is_err());
}

#[test]
fn entitlement_grants_are_bound_to_membership_and_quota() {
    let grant = EntitlementGrantDraft::new("100001", "membership-1", "model-quota", 1000).unwrap();

    assert_eq!(grant.entitlement_code, "model-quota");
    assert_eq!(grant.quantity, 1000);
    assert!(EntitlementGrantDraft::new("100001", "membership-1", "model-quota", 0).is_err());
}

#[test]
fn membership_repository_contract_exposes_required_commands() {
    assert_eq!(
        MembershipPortRequirement::standard_commands(),
        vec![
            MembershipRepositoryCommand::CreateMembershipPlan,
            MembershipRepositoryCommand::UpsertMembershipPackageGroup,
            MembershipRepositoryCommand::UpsertMembershipPackage,
            MembershipRepositoryCommand::ActivateMembership,
            MembershipRepositoryCommand::GrantEntitlement,
            MembershipRepositoryCommand::RenewMembership,
            MembershipRepositoryCommand::ExpireMembership,
            MembershipRepositoryCommand::CreateMembershipPurchase,
            MembershipRepositoryCommand::RenewMembershipPurchase,
            MembershipRepositoryCommand::UpgradeMembershipPurchase,
        ],
    );
}

#[test]
fn validates_membership_package_group_catalog_shape() {
    let group = MembershipPackageGroupDraft::from_input(MembershipPackageGroupDraftInput {
        tenant_id: "100001".to_string(),
        organization_id: "0".to_string(),
        external_id: 1,
        package_group_no: "membership-month".to_string(),
        name: "Monthly purchase".to_string(),
        description: Some("Monthly membership packages".to_string()),
        billing_cycle: MembershipBillingCycle::Month,
        duration_days: 30,
        sort_weight: 10,
    })
    .unwrap();

    assert_eq!(group.external_id, 1);
    assert_eq!(group.package_group_no, "membership-month");
    assert_eq!(group.billing_cycle.as_storage_str(), "month");
    assert_eq!(group.duration_days, 30);
    assert!(
        MembershipPackageGroupDraft::from_input(MembershipPackageGroupDraftInput {
            tenant_id: "100001".to_string(),
            organization_id: "0".to_string(),
            external_id: 0,
            package_group_no: "membership-month".to_string(),
            name: "Monthly purchase".to_string(),
            description: None,
            billing_cycle: MembershipBillingCycle::Month,
            duration_days: 30,
            sort_weight: 10,
        })
        .is_err()
    );
    assert!(
        MembershipPackageGroupDraft::from_input(MembershipPackageGroupDraftInput {
            tenant_id: "100001".to_string(),
            organization_id: "0".to_string(),
            external_id: 1,
            package_group_no: String::new(),
            name: "Monthly purchase".to_string(),
            description: None,
            billing_cycle: MembershipBillingCycle::Month,
            duration_days: 30,
            sort_weight: 10,
        })
        .is_err()
    );
}

#[test]
fn validates_membership_package_catalog_shape_and_prices() {
    let package = MembershipPackageDraft::new(
        "100001",
        "0",
        303,
        "membership-month-pro",
        "membership-package-group-month",
        "membership-plan-pro",
        Some("sku-membership-month-pro"),
        "Monthly Pro",
        Some("Advanced model access"),
        "6990",
        Some("12900"),
        "CNY",
        45_000,
        30,
        303,
        true,
        vec!["monthly".to_owned(), "advanced".to_owned()],
    )
    .unwrap();

    assert_eq!(package.external_id, 303);
    assert_eq!(package.package_group_id, "membership-package-group-month");
    assert_eq!(package.plan_id, "membership-plan-pro");
    assert_eq!(package.price_amount, "6990");
    assert_eq!(package.duration_days, 30);
    assert!(package.recommended);
    assert_eq!(package.tags, vec!["monthly", "advanced"]);

    assert!(MembershipPackageDraft::new(
        "100001",
        "0",
        303,
        "membership-month-pro",
        "membership-package-group-month",
        "membership-plan-pro",
        None,
        "Monthly Pro",
        None,
        "-100",
        None,
        "CNY",
        45_000,
        30,
        303,
        true,
        Vec::new(),
    )
    .is_err());
    assert!(MembershipPackageDraft::new(
        "100001",
        "0",
        303,
        "membership-month-pro",
        "membership-package-group-month",
        "membership-plan-pro",
        None,
        "Monthly Pro",
        None,
        "6990",
        None,
        "CNY",
        45_000,
        0,
        303,
        true,
        Vec::new(),
    )
    .is_err());
}

#[test]
fn membership_package_queries_keep_group_and_region_independent() {
    let group_query =
        MembershipPackageGroupListQuery::new("100001", Some("0"), Some("month")).unwrap();
    let package_query = MembershipPackageListQuery::new(
        "100001",
        Some("0"),
        Some("membership-package-group-month"),
        Some("membership-plan-pro"),
    )
    .unwrap();

    assert_eq!(group_query.billing_cycle.as_deref(), Some("month"));
    assert_eq!(
        package_query.package_group_id.as_deref(),
        Some("membership-package-group-month")
    );
    assert_eq!(
        package_query.plan_id.as_deref(),
        Some("membership-plan-pro")
    );
}

#[test]
fn membership_purchase_uses_package_id_and_payment_method_only() {
    let purchase =
        MembershipPurchaseDraft::new("100001", "0", "user-1", 303, Some("wechat_pay"), None)
            .unwrap();

    assert_eq!(purchase.package_id, 303);
    assert_eq!(purchase.payment_method.as_deref(), Some("wechat_pay"));
    assert!(
        MembershipPurchaseDraft::new("100001", "0", "user-1", 0, Some("wechat_pay"), None).is_err()
    );
    for legacy_method in ["wechat", "wechatpay", "stripe"] {
        assert!(
            MembershipPurchaseDraft::new("100001", "0", "user-1", 303, Some(legacy_method), None)
                .is_err(),
            "membership purchase must reject legacy payment method alias {legacy_method}"
        );
    }
}

#[test]
fn membership_service_contract_exposes_membership_package_catalog_operations() {
    let contract = membership_service_contract();

    for query in [
        "memberships.current.retrieve",
        "memberships.current.status.retrieve",
        "memberships.benefits.list",
        "memberships.packageGroups.list",
        "memberships.packageGroups.retrieve",
        "memberships.packageGroups.packages.list",
        "memberships.packages.list",
        "memberships.packages.retrieve",
        "memberships.points.balance.retrieve",
        "memberships.points.history.list",
        "memberships.points.dailyRewards.status.retrieve",
        "memberships.privileges.usage.retrieve",
    ] {
        assert!(
            contract.read_queries.contains(&query),
            "membership contract must expose read query {query}",
        );
    }

    for command in [
        "memberships.purchases.create",
        "memberships.purchases.renew",
        "memberships.purchases.upgrade",
        "memberships.points.dailyRewards.create",
        "memberships.privileges.speedUps.create",
    ] {
        assert!(
            contract.write_commands.contains(&command),
            "membership contract must expose write command {command}",
        );
    }
}
