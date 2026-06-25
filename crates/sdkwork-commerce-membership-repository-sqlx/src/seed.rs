use sdkwork_commerce_bootstrap_manifest::{
    commerce_benefit_definition_seeds, commerce_payment_channel_seeds,
    commerce_payment_method_seeds, commerce_payment_provider_account_seeds,
    commerce_payment_provider_seeds, commerce_payment_route_rule_seeds,
    commerce_recharge_package_seeds, commerce_recharge_settings_seeds,
    membership_package_group_seeds, membership_package_seeds, membership_plan_benefit_seeds,
    membership_plan_seeds, membership_plan_version_seeds, CommerceMembershipPackageGroupSeed,
    CommerceMembershipPackageSeed, CommercePaymentMethodSeed, CommerceRechargePackageSeed,
    CommerceRechargeSettingsSeed,
};
use sdkwork_commerce_contract_service::CommerceServiceError;
use sqlx::{PgPool, SqlitePool};

const SEED_TIMESTAMP: &str = "2026-05-19 00:00:00";
const MEMBERSHIP_PRODUCT_ID: &str = "seed-product-membership";
const RECHARGE_PRODUCT_CNY_ID: &str = "seed-product-points-recharge-cny";
const RECHARGE_PRODUCT_NON_CNY_ID: &str = "seed-product-points-recharge-non-cny";
const RECHARGE_PRODUCT_CNY_SPU_NO: &str = "points-recharge-cny";
const RECHARGE_PRODUCT_NON_CNY_SPU_NO: &str = "points-recharge-non-cny";
const PAYMENT_METHOD_SEED_COUNT: i64 = 7;
const PAYMENT_PROVIDER_SEED_COUNT: i64 = 6;
const PAYMENT_PROVIDER_ACCOUNT_SEED_COUNT: i64 = 6;
const PAYMENT_CHANNEL_SEED_COUNT: i64 = 36;
const PAYMENT_ROUTE_RULE_SEED_COUNT: i64 = 36;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommerceExperienceSeedIntegrityReport {
    pub complete: bool,
    pub issues: Vec<CommerceExperienceSeedIntegrityIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommerceExperienceSeedIntegrityIssue {
    pub code: String,
    pub message: String,
    pub expected_count: i64,
    pub actual_count: i64,
}

pub async fn upsert_sqlite_commerce_experience_seed(
    pool: &SqlitePool,
) -> Result<(), CommerceServiceError> {
    upsert_sqlite_seed_products(pool).await?;
    upsert_sqlite_benefit_definitions(pool).await?;
    upsert_sqlite_membership_plans(pool).await?;
    upsert_sqlite_membership_plan_versions(pool).await?;
    upsert_sqlite_membership_plan_benefits(pool).await?;
    upsert_sqlite_membership_package_groups(pool).await?;
    upsert_sqlite_membership_packages(pool).await?;
    upsert_sqlite_recharge_packages(pool).await?;
    upsert_sqlite_recharge_settings(pool).await?;
    upsert_sqlite_payment_center_seed(pool).await?;
    Ok(())
}

pub async fn upsert_postgres_commerce_experience_seed(
    pool: &PgPool,
) -> Result<(), CommerceServiceError> {
    upsert_postgres_seed_products(pool).await?;
    upsert_postgres_benefit_definitions(pool).await?;
    upsert_postgres_membership_plans(pool).await?;
    upsert_postgres_membership_plan_versions(pool).await?;
    upsert_postgres_membership_plan_benefits(pool).await?;
    upsert_postgres_membership_package_groups(pool).await?;
    upsert_postgres_membership_packages(pool).await?;
    upsert_postgres_recharge_packages(pool).await?;
    upsert_postgres_recharge_settings(pool).await?;
    upsert_postgres_payment_center_seed(pool).await?;
    Ok(())
}

pub async fn repair_sqlite_commerce_experience_seed(
    pool: &SqlitePool,
) -> Result<(), CommerceServiceError> {
    let report = sqlite_commerce_experience_seed_integrity_report(pool).await?;
    repair_sqlite_commerce_experience_seed_from_report(pool, &report).await
}

pub async fn repair_sqlite_commerce_experience_seed_from_report(
    pool: &SqlitePool,
    report: &CommerceExperienceSeedIntegrityReport,
) -> Result<(), CommerceServiceError> {
    if report.complete {
        return Ok(());
    }
    let mut upsert_products = false;
    let mut upsert_membership_core = false;
    let mut upsert_membership_groups = false;
    let mut upsert_membership_packages = false;
    let mut upsert_recharge_packages = false;
    let mut upsert_recharge_settings = false;
    let mut upsert_payment_methods = false;
    let mut upsert_payment_providers = false;
    let mut upsert_payment_provider_accounts = false;
    let mut upsert_payment_channels = false;
    let mut upsert_payment_route_rules = false;
    let mut fallback_full_seed = false;

    for issue in &report.issues {
        match issue.code.as_str() {
            "missing_seed_product" => {
                upsert_products = true;
            }
            "missing_membership_plan" => {
                upsert_membership_core = true;
            }
            "missing_membership_package_group" => {
                upsert_membership_groups = true;
            }
            "missing_membership_package"
            | "incomplete_membership_package_group"
            | "missing_membership_sku"
            | "orphan_membership_package_plan"
            | "orphan_membership_package_group"
            | "orphan_membership_package_sku"
            | "invalid_membership_sku_product" => {
                upsert_products = true;
                upsert_membership_core = true;
                upsert_membership_groups = true;
                upsert_membership_packages = true;
            }
            "missing_recharge_package"
            | "missing_recharge_sku"
            | "orphan_recharge_package_sku"
            | "invalid_recharge_sku_product" => {
                upsert_products = true;
                upsert_recharge_packages = true;
            }
            "missing_recharge_settings" => {
                upsert_recharge_settings = true;
            }
            "missing_payment_method" => {
                upsert_payment_methods = true;
            }
            "missing_payment_provider" => {
                upsert_payment_providers = true;
            }
            "missing_payment_provider_account" => {
                upsert_payment_provider_accounts = true;
            }
            "missing_payment_channel" => {
                upsert_payment_channels = true;
            }
            "missing_payment_route_rule" => {
                upsert_payment_route_rules = true;
            }
            _ => {
                fallback_full_seed = true;
            }
        }
    }

    if fallback_full_seed {
        upsert_sqlite_commerce_experience_seed(pool).await?;
        return Ok(());
    }

    if upsert_products {
        upsert_sqlite_seed_products(pool).await?;
    }
    if upsert_membership_core {
        upsert_sqlite_benefit_definitions(pool).await?;
        upsert_sqlite_membership_plans(pool).await?;
        upsert_sqlite_membership_plan_versions(pool).await?;
        upsert_sqlite_membership_plan_benefits(pool).await?;
    }
    if upsert_membership_groups {
        upsert_sqlite_membership_package_groups(pool).await?;
    }
    if upsert_membership_packages {
        upsert_sqlite_membership_packages(pool).await?;
    }
    if upsert_recharge_packages {
        upsert_sqlite_recharge_packages(pool).await?;
    }
    if upsert_recharge_settings {
        upsert_sqlite_recharge_settings(pool).await?;
    }
    if upsert_payment_methods {
        upsert_sqlite_payment_methods(pool).await?;
    }
    if upsert_payment_providers {
        upsert_sqlite_payment_providers(pool).await?;
    }
    if upsert_payment_provider_accounts {
        upsert_sqlite_payment_provider_accounts(pool).await?;
    }
    if upsert_payment_channels {
        upsert_sqlite_payment_channels(pool).await?;
    }
    if upsert_payment_route_rules {
        upsert_sqlite_payment_route_rules(pool).await?;
    }

    if !sqlite_commerce_experience_seed_complete(pool).await? {
        upsert_sqlite_commerce_experience_seed(pool).await?;
    }
    Ok(())
}

async fn upsert_sqlite_seed_products(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    upsert_sqlite_seed_product(
        pool,
        MEMBERSHIP_PRODUCT_ID,
        "membership",
        "Membership",
        "SDKWork membership catalog",
        "Reusable SDKWork commerce membership product.",
        "commerce-membership",
    )
    .await?;
    upsert_sqlite_seed_product(
        pool,
        RECHARGE_PRODUCT_CNY_ID,
        RECHARGE_PRODUCT_CNY_SPU_NO,
        "Points recharge (CNY)",
        "SDKWork points recharge catalog (CNY)",
        "Reusable SDKWork commerce points recharge packages for CNY.",
        "commerce-recharge",
    )
    .await?;
    upsert_sqlite_seed_product(
        pool,
        RECHARGE_PRODUCT_NON_CNY_ID,
        RECHARGE_PRODUCT_NON_CNY_SPU_NO,
        "Points recharge (Non-CNY)",
        "SDKWork points recharge catalog (Non-CNY)",
        "Reusable SDKWork commerce points recharge packages for non-CNY.",
        "commerce-recharge",
    )
    .await?;
    Ok(())
}

async fn upsert_postgres_seed_products(pool: &PgPool) -> Result<(), CommerceServiceError> {
    upsert_postgres_seed_product(
        pool,
        MEMBERSHIP_PRODUCT_ID,
        "membership",
        "Membership",
        "SDKWork membership catalog",
        "Reusable SDKWork commerce membership product.",
        "commerce-membership",
    )
    .await?;
    upsert_postgres_seed_product(
        pool,
        RECHARGE_PRODUCT_CNY_ID,
        RECHARGE_PRODUCT_CNY_SPU_NO,
        "Points recharge (CNY)",
        "SDKWork points recharge catalog (CNY)",
        "Reusable SDKWork commerce points recharge packages for CNY.",
        "commerce-recharge",
    )
    .await?;
    upsert_postgres_seed_product(
        pool,
        RECHARGE_PRODUCT_NON_CNY_ID,
        RECHARGE_PRODUCT_NON_CNY_SPU_NO,
        "Points recharge (Non-CNY)",
        "SDKWork points recharge catalog (Non-CNY)",
        "Reusable SDKWork commerce points recharge packages for non-CNY.",
        "commerce-recharge",
    )
    .await?;
    Ok(())
}

pub async fn upsert_sqlite_payment_center_seed(
    pool: &SqlitePool,
) -> Result<(), CommerceServiceError> {
    upsert_sqlite_payment_methods(pool).await?;
    upsert_sqlite_payment_providers(pool).await?;
    upsert_sqlite_payment_provider_accounts(pool).await?;
    upsert_sqlite_payment_channels(pool).await?;
    upsert_sqlite_payment_route_rules(pool).await?;
    Ok(())
}

pub async fn upsert_postgres_payment_center_seed(
    pool: &PgPool,
) -> Result<(), CommerceServiceError> {
    upsert_postgres_payment_methods(pool).await?;
    upsert_postgres_payment_providers(pool).await?;
    upsert_postgres_payment_provider_accounts(pool).await?;
    upsert_postgres_payment_channels(pool).await?;
    upsert_postgres_payment_route_rules(pool).await?;
    Ok(())
}

pub async fn sqlite_commerce_experience_seed_complete(
    pool: &SqlitePool,
) -> Result<bool, CommerceServiceError> {
    Ok(sqlite_commerce_experience_seed_integrity_report(pool)
        .await?
        .complete)
}

pub async fn sqlite_commerce_experience_seed_integrity_report(
    pool: &SqlitePool,
) -> Result<CommerceExperienceSeedIntegrityReport, CommerceServiceError> {
    let mut issues = Vec::new();
    for check in COMMERCE_EXPERIENCE_SEED_INTEGRITY_CHECKS {
        let actual_count = sqlite_count(pool, check.statement).await?;
        if actual_count != check.expected_count {
            issues.push(check.issue(actual_count));
        }
    }
    Ok(CommerceExperienceSeedIntegrityReport {
        complete: issues.is_empty(),
        issues,
    })
}

pub async fn postgres_commerce_experience_seed_complete(
    pool: &PgPool,
) -> Result<bool, CommerceServiceError> {
    for check in COMMERCE_EXPERIENCE_SEED_INTEGRITY_CHECKS {
        let actual_count = postgres_count(pool, check.statement).await?;
        if actual_count != check.expected_count {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn upsert_sqlite_seed_product(
    pool: &SqlitePool,
    id: &str,
    spu_no: &str,
    title: &str,
    subtitle: &str,
    description: &str,
    category_id: &str,
) -> Result<(), CommerceServiceError> {
    sqlx::query(
        r#"
        INSERT INTO commerce_product_spu
            (id, tenant_id, organization_id, spu_no, title, subtitle, description, product_type, category_id, status, visible_surfaces, created_at, updated_at)
        VALUES
            (?, '100001', '0', ?, ?, ?, ?, ?, ?, 'active', '["app","console","admin"]', ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            tenant_id = excluded.tenant_id,
            organization_id = excluded.organization_id,
            spu_no = excluded.spu_no,
            title = excluded.title,
            subtitle = excluded.subtitle,
            description = excluded.description,
            product_type = excluded.product_type,
            category_id = excluded.category_id,
            status = excluded.status,
            visible_surfaces = excluded.visible_surfaces,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(id)
    .bind(spu_no)
    .bind(title)
    .bind(subtitle)
    .bind(description)
    .bind(seed_product_type(spu_no))
    .bind(category_id)
    .bind(SEED_TIMESTAMP)
    .bind(SEED_TIMESTAMP)
    .execute(pool)
    .await
    .map_err(|error| storage_error("failed to upsert commerce seed product", error))?;
    Ok(())
}

async fn upsert_postgres_seed_product(
    pool: &PgPool,
    id: &str,
    spu_no: &str,
    title: &str,
    subtitle: &str,
    description: &str,
    category_id: &str,
) -> Result<(), CommerceServiceError> {
    sqlx::query(
        r#"
        INSERT INTO commerce_product_spu
            (id, tenant_id, organization_id, spu_no, title, subtitle, description, product_type, category_id, status, visible_surfaces, created_at, updated_at)
        VALUES
            ($1, '100001', '0', $2, $3, $4, $5, $6, $7, 'active', '["app","console","admin"]', $8, $9)
        ON CONFLICT(id) DO UPDATE SET
            tenant_id = excluded.tenant_id,
            organization_id = excluded.organization_id,
            spu_no = excluded.spu_no,
            title = excluded.title,
            subtitle = excluded.subtitle,
            description = excluded.description,
            product_type = excluded.product_type,
            category_id = excluded.category_id,
            status = excluded.status,
            visible_surfaces = excluded.visible_surfaces,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(id)
    .bind(spu_no)
    .bind(title)
    .bind(subtitle)
    .bind(description)
    .bind(seed_product_type(spu_no))
    .bind(category_id)
    .bind(SEED_TIMESTAMP)
    .bind(SEED_TIMESTAMP)
    .execute(pool)
    .await
    .map_err(|error| storage_error("failed to upsert commerce seed product", error))?;
    Ok(())
}

async fn upsert_sqlite_benefit_definitions(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for benefit in commerce_benefit_definition_seeds() {
        sqlx::query(
            r#"
            INSERT INTO benefit_definition
                (id, tenant_id, organization_id, benefit_code, name, benefit_type,
                 value_unit, measurement_type, description, status, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, 'active', ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                benefit_type = excluded.benefit_type,
                value_unit = excluded.value_unit,
                measurement_type = excluded.measurement_type,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(benefit.id)
        .bind(benefit.benefit_code)
        .bind(benefit.name)
        .bind(benefit.benefit_type)
        .bind(benefit.value_unit)
        .bind(benefit.measurement_type)
        .bind(benefit.description)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert benefit definition", error))?;
    }
    Ok(())
}

async fn upsert_postgres_benefit_definitions(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for benefit in commerce_benefit_definition_seeds() {
        sqlx::query(
            r#"
            INSERT INTO benefit_definition
                (id, tenant_id, organization_id, benefit_code, name, benefit_type,
                 value_unit, measurement_type, description, status, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, 'active', $8, $9)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                benefit_type = excluded.benefit_type,
                value_unit = excluded.value_unit,
                measurement_type = excluded.measurement_type,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(benefit.id)
        .bind(benefit.benefit_code)
        .bind(benefit.name)
        .bind(benefit.benefit_type)
        .bind(benefit.value_unit)
        .bind(benefit.measurement_type)
        .bind(benefit.description)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert benefit definition", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_membership_plans(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for plan in membership_plan_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_plan
                (id, tenant_id, organization_id, plan_no, plan_code, name, rank, description,
                 status, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, 'active', ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                plan_code = excluded.plan_code,
                name = excluded.name,
                rank = excluded.rank,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(plan_id_for(plan.plan_no))
        .bind(plan.plan_no)
        .bind(plan.plan_code)
        .bind(plan.name)
        .bind(plan.rank)
        .bind(plan.description)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership plan", error))?;
    }
    Ok(())
}

async fn upsert_postgres_membership_plans(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for plan in membership_plan_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_plan
                (id, tenant_id, organization_id, plan_no, plan_code, name, rank, description,
                 status, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, 'active', $7, $8)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                plan_code = excluded.plan_code,
                name = excluded.name,
                rank = excluded.rank,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(plan_id_for(plan.plan_no))
        .bind(plan.plan_no)
        .bind(plan.plan_code)
        .bind(plan.name)
        .bind(plan.rank)
        .bind(plan.description)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership plan", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_membership_plan_versions(
    pool: &SqlitePool,
) -> Result<(), CommerceServiceError> {
    for version in membership_plan_version_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_plan_version
                (id, tenant_id, organization_id, plan_id, version_no, title, description,
                 lifecycle_status, effective_from, effective_to, published_at, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, NULL, ?, ?, NULL, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                lifecycle_status = excluded.lifecycle_status,
                effective_from = excluded.effective_from,
                published_at = excluded.published_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(version.id)
        .bind(plan_id_for(version.plan_no))
        .bind(version.version_no)
        .bind(version.title)
        .bind(version.lifecycle_status)
        .bind(version.effective_from)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership plan version", error))?;
    }
    Ok(())
}

async fn upsert_postgres_membership_plan_versions(
    pool: &PgPool,
) -> Result<(), CommerceServiceError> {
    for version in membership_plan_version_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_plan_version
                (id, tenant_id, organization_id, plan_id, version_no, title, description,
                 lifecycle_status, effective_from, effective_to, published_at, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, NULL, $5, $6, NULL, $7, $8, $9)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                lifecycle_status = excluded.lifecycle_status,
                effective_from = excluded.effective_from,
                published_at = excluded.published_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(version.id)
        .bind(plan_id_for(version.plan_no))
        .bind(version.version_no)
        .bind(version.title)
        .bind(version.lifecycle_status)
        .bind(version.effective_from)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership plan version", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_membership_plan_benefits(
    pool: &SqlitePool,
) -> Result<(), CommerceServiceError> {
    for benefit in membership_plan_benefit_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_plan_benefit
                (id, tenant_id, organization_id, plan_id, plan_version_id, benefit_id,
                 benefit_code, grant_quantity, grant_period, reset_policy, usage_policy,
                 sort_weight, status, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, ?, ?, ?, 'active', ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                benefit_code = excluded.benefit_code,
                grant_quantity = excluded.grant_quantity,
                grant_period = excluded.grant_period,
                reset_policy = excluded.reset_policy,
                usage_policy = excluded.usage_policy,
                sort_weight = excluded.sort_weight,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(benefit.id)
        .bind(plan_id_for(benefit.plan_no))
        .bind(plan_version_id_for(benefit.plan_no))
        .bind(benefit_definition_id_for(benefit.benefit_code))
        .bind(benefit.benefit_code)
        .bind(benefit.grant_quantity)
        .bind(benefit.grant_period)
        .bind(benefit.reset_policy)
        .bind(benefit.usage_policy)
        .bind(benefit.sort_weight)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership plan benefit", error))?;
    }
    Ok(())
}

async fn upsert_postgres_membership_plan_benefits(
    pool: &PgPool,
) -> Result<(), CommerceServiceError> {
    for benefit in membership_plan_benefit_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_plan_benefit
                (id, tenant_id, organization_id, plan_id, plan_version_id, benefit_id,
                 benefit_code, grant_quantity, grant_period, reset_policy, usage_policy,
                 sort_weight, status, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, $8, $9, $10, 'active', $11, $12)
            ON CONFLICT(id) DO UPDATE SET
                benefit_code = excluded.benefit_code,
                grant_quantity = excluded.grant_quantity,
                grant_period = excluded.grant_period,
                reset_policy = excluded.reset_policy,
                usage_policy = excluded.usage_policy,
                sort_weight = excluded.sort_weight,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(benefit.id)
        .bind(plan_id_for(benefit.plan_no))
        .bind(plan_version_id_for(benefit.plan_no))
        .bind(benefit_definition_id_for(benefit.benefit_code))
        .bind(benefit.benefit_code)
        .bind(benefit.grant_quantity)
        .bind(benefit.grant_period)
        .bind(benefit.reset_policy)
        .bind(benefit.usage_policy)
        .bind(benefit.sort_weight)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership plan benefit", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_membership_package_groups(
    pool: &SqlitePool,
) -> Result<(), CommerceServiceError> {
    for group in membership_package_group_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_package_group
                (id, tenant_id, organization_id, external_id, group_no, name, description,
                 billing_cycle, duration_days, display_channel, sort_weight, status, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, 'app', ?, 'active', ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                external_id = excluded.external_id,
                name = excluded.name,
                description = excluded.description,
                billing_cycle = excluded.billing_cycle,
                duration_days = excluded.duration_days,
                display_channel = excluded.display_channel,
                sort_weight = excluded.sort_weight,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(package_group_id_for(group.package_group_no))
        .bind(group.external_id)
        .bind(group_no_for(group.package_group_no))
        .bind(group.name)
        .bind(group.description)
        .bind(group.billing_cycle)
        .bind(group.duration_days)
        .bind(group.sort_weight)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership package group", error))?;
    }
    Ok(())
}

async fn upsert_postgres_membership_package_groups(
    pool: &PgPool,
) -> Result<(), CommerceServiceError> {
    for group in membership_package_group_seeds() {
        sqlx::query(
            r#"
            INSERT INTO membership_package_group
                (id, tenant_id, organization_id, external_id, group_no, name, description,
                 billing_cycle, duration_days, display_channel, sort_weight, status, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, 'app', $8, 'active', $9, $10)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                external_id = excluded.external_id,
                name = excluded.name,
                description = excluded.description,
                billing_cycle = excluded.billing_cycle,
                duration_days = excluded.duration_days,
                display_channel = excluded.display_channel,
                sort_weight = excluded.sort_weight,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(package_group_id_for(group.package_group_no))
        .bind(group.external_id)
        .bind(group_no_for(group.package_group_no))
        .bind(group.name)
        .bind(group.description)
        .bind(group.billing_cycle)
        .bind(group.duration_days)
        .bind(group.sort_weight)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership package group", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_membership_packages(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for package in membership_package_seeds() {
        upsert_sqlite_membership_package_sku(pool, &package).await?;
        sqlx::query(
            r#"
            INSERT INTO membership_package
                (id, tenant_id, organization_id, external_id, package_no, package_group_id,
                 plan_id, plan_version_id, sku_id, name, description, price_amount,
                 original_price_amount, currency_code, point_amount, duration_days,
                 recurrence_cycle, sort_weight, recommended, status, starts_at, ends_at,
                 created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'active', NULL, NULL, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                external_id = excluded.external_id,
                package_group_id = excluded.package_group_id,
                plan_id = excluded.plan_id,
                plan_version_id = excluded.plan_version_id,
                sku_id = excluded.sku_id,
                name = excluded.name,
                description = excluded.description,
                price_amount = excluded.price_amount,
                original_price_amount = excluded.original_price_amount,
                currency_code = excluded.currency_code,
                point_amount = excluded.point_amount,
                duration_days = excluded.duration_days,
                recurrence_cycle = excluded.recurrence_cycle,
                sort_weight = excluded.sort_weight,
                recommended = excluded.recommended,
                status = excluded.status,
                starts_at = excluded.starts_at,
                ends_at = excluded.ends_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(package.id)
        .bind(package.external_id)
        .bind(package_no_for(package.package_no))
        .bind(package_group_id_for(package.package_group_no))
        .bind(plan_id_for(package.plan_no))
        .bind(plan_version_id_for(package.plan_no))
        .bind(package.sku_id)
        .bind(package.name)
        .bind(package.description)
        .bind(package.price_amount)
        .bind(package.original_price_amount)
        .bind(package.currency_code)
        .bind(package.point_amount)
        .bind(package.duration_days)
        .bind(recurrence_cycle_for(package.package_group_no))
        .bind(package.sort_weight)
        .bind(i64::from(package.recommended))
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership package", error))?;
    }
    Ok(())
}

async fn upsert_postgres_membership_packages(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for package in membership_package_seeds() {
        upsert_postgres_membership_package_sku(pool, &package).await?;
        sqlx::query(
            r#"
            INSERT INTO membership_package
                (id, tenant_id, organization_id, external_id, package_no, package_group_id,
                 plan_id, plan_version_id, sku_id, name, description, price_amount,
                 original_price_amount, currency_code, point_amount, duration_days,
                 recurrence_cycle, sort_weight, recommended, status, starts_at, ends_at,
                 created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, $8, $9, $10,
                 $11, $12, $13, $14, $15, $16, $17, 'active', NULL, NULL, $18, $19)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                external_id = excluded.external_id,
                package_group_id = excluded.package_group_id,
                plan_id = excluded.plan_id,
                plan_version_id = excluded.plan_version_id,
                sku_id = excluded.sku_id,
                name = excluded.name,
                description = excluded.description,
                price_amount = excluded.price_amount,
                original_price_amount = excluded.original_price_amount,
                currency_code = excluded.currency_code,
                point_amount = excluded.point_amount,
                duration_days = excluded.duration_days,
                recurrence_cycle = excluded.recurrence_cycle,
                sort_weight = excluded.sort_weight,
                recommended = excluded.recommended,
                status = excluded.status,
                starts_at = excluded.starts_at,
                ends_at = excluded.ends_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(package.id)
        .bind(package.external_id)
        .bind(package_no_for(package.package_no))
        .bind(package_group_id_for(package.package_group_no))
        .bind(plan_id_for(package.plan_no))
        .bind(plan_version_id_for(package.plan_no))
        .bind(package.sku_id)
        .bind(package.name)
        .bind(package.description)
        .bind(package.price_amount)
        .bind(package.original_price_amount)
        .bind(package.currency_code)
        .bind(package.point_amount)
        .bind(package.duration_days)
        .bind(recurrence_cycle_for(package.package_group_no))
        .bind(package.sort_weight)
        .bind(i64::from(package.recommended))
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert membership package", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_membership_package_sku(
    pool: &SqlitePool,
    package: &CommerceMembershipPackageSeed,
) -> Result<(), CommerceServiceError> {
    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at)
        VALUES
            (?, '100001', '0', ?, ?, ?, ?, ?, ?, ?, 'membership_activation', 'untracked', 'active', ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            tenant_id = excluded.tenant_id,
            organization_id = excluded.organization_id,
            spu_id = excluded.spu_id,
            sku_no = excluded.sku_no,
            name = excluded.name,
            title = excluded.title,
            price_amount = excluded.price_amount,
            original_price_amount = excluded.original_price_amount,
            currency_code = excluded.currency_code,
            fulfillment_type = excluded.fulfillment_type,
            inventory_tracking = excluded.inventory_tracking,
            status = excluded.status,
            spec_json = excluded.spec_json,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(package.sku_id)
    .bind(MEMBERSHIP_PRODUCT_ID)
    .bind(sku_no_for(package.sku_no))
    .bind(package.name)
    .bind(package.title)
    .bind(package.price_amount)
    .bind(package.original_price_amount)
    .bind(package.currency_code)
    .bind(membership_sku_spec_json(package))
    .bind(SEED_TIMESTAMP)
    .bind(SEED_TIMESTAMP)
    .execute(pool)
    .await
    .map_err(|error| storage_error("failed to upsert membership sku", error))?;
    Ok(())
}

async fn upsert_postgres_membership_package_sku(
    pool: &PgPool,
    package: &CommerceMembershipPackageSeed,
) -> Result<(), CommerceServiceError> {
    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at)
        VALUES
            ($1, '100001', '0', $2, $3, $4, $5, $6, $7, $8, 'membership_activation', 'untracked', 'active', $9, $10, $11)
        ON CONFLICT(id) DO UPDATE SET
            tenant_id = excluded.tenant_id,
            organization_id = excluded.organization_id,
            spu_id = excluded.spu_id,
            sku_no = excluded.sku_no,
            name = excluded.name,
            title = excluded.title,
            price_amount = excluded.price_amount,
            original_price_amount = excluded.original_price_amount,
            currency_code = excluded.currency_code,
            fulfillment_type = excluded.fulfillment_type,
            inventory_tracking = excluded.inventory_tracking,
            status = excluded.status,
            spec_json = excluded.spec_json,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(package.sku_id)
    .bind(MEMBERSHIP_PRODUCT_ID)
    .bind(sku_no_for(package.sku_no))
    .bind(package.name)
    .bind(package.title)
    .bind(package.price_amount)
    .bind(package.original_price_amount)
    .bind(package.currency_code)
    .bind(membership_sku_spec_json(package))
    .bind(SEED_TIMESTAMP)
    .bind(SEED_TIMESTAMP)
    .execute(pool)
    .await
    .map_err(|error| storage_error("failed to upsert membership sku", error))?;
    Ok(())
}

async fn upsert_sqlite_recharge_packages(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for package in commerce_recharge_package_seeds() {
        upsert_sqlite_recharge_package_sku(pool, &package).await?;
        sqlx::query(
            r#"
            INSERT INTO commerce_recharge_package
                (id, tenant_id, organization_id, external_id, package_no, sku_id, name, price_amount, currency_code, bonus_points, status, valid_from, valid_to, sort_weight, request_no, idempotency_key, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                external_id = excluded.external_id,
                sku_id = excluded.sku_id,
                name = excluded.name,
                price_amount = excluded.price_amount,
                currency_code = excluded.currency_code,
                bonus_points = excluded.bonus_points,
                status = excluded.status,
                valid_from = excluded.valid_from,
                valid_to = excluded.valid_to,
                sort_weight = excluded.sort_weight,
                request_no = excluded.request_no,
                idempotency_key = excluded.idempotency_key,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(package.package_id)
        .bind(package.external_id)
        .bind(package.package_no)
        .bind(package.sku_id)
        .bind(package.name)
        .bind(package.price_amount)
        .bind(package.currency_code)
        .bind(package.bonus_points)
        .bind(package.status)
        .bind(package.sort_weight)
        .bind(format!("seed-recharge-package-{}", package.package_no))
        .bind(format!("seed-recharge-package-{}", package.package_no))
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert recharge package", error))?;
    }
    Ok(())
}

async fn upsert_postgres_recharge_packages(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for package in commerce_recharge_package_seeds() {
        upsert_postgres_recharge_package_sku(pool, &package).await?;
        sqlx::query(
            r#"
            INSERT INTO commerce_recharge_package
                (id, tenant_id, organization_id, external_id, package_no, sku_id, name, price_amount, currency_code, bonus_points, status, valid_from, valid_to, sort_weight, request_no, idempotency_key, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, $8, $9, NULL, NULL, $10, $11, $12, $13, $14)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                external_id = excluded.external_id,
                sku_id = excluded.sku_id,
                name = excluded.name,
                price_amount = excluded.price_amount,
                currency_code = excluded.currency_code,
                bonus_points = excluded.bonus_points,
                status = excluded.status,
                valid_from = excluded.valid_from,
                valid_to = excluded.valid_to,
                sort_weight = excluded.sort_weight,
                request_no = excluded.request_no,
                idempotency_key = excluded.idempotency_key,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(package.package_id)
        .bind(package.external_id)
        .bind(package.package_no)
        .bind(package.sku_id)
        .bind(package.name)
        .bind(package.price_amount)
        .bind(package.currency_code)
        .bind(package.bonus_points)
        .bind(package.status)
        .bind(package.sort_weight)
        .bind(format!("seed-recharge-package-{}", package.package_no))
        .bind(format!("seed-recharge-package-{}", package.package_no))
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert recharge package", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_recharge_package_sku(
    pool: &SqlitePool,
    package: &CommerceRechargePackageSeed,
) -> Result<(), CommerceServiceError> {
    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at)
        VALUES
            (?, '100001', '0', ?, ?, ?, ?, ?, NULL, ?, 'points_credit', 'untracked', 'active', ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            tenant_id = excluded.tenant_id,
            organization_id = excluded.organization_id,
            spu_id = excluded.spu_id,
            sku_no = excluded.sku_no,
            name = excluded.name,
            title = excluded.title,
            price_amount = excluded.price_amount,
            currency_code = excluded.currency_code,
            fulfillment_type = excluded.fulfillment_type,
            inventory_tracking = excluded.inventory_tracking,
            status = excluded.status,
            spec_json = excluded.spec_json,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(package.sku_id)
    .bind(recharge_product_id_for_currency(package.currency_code))
    .bind(package.sku_no)
    .bind(package.name)
    .bind(package.name)
    .bind(package.price_amount)
    .bind(package.currency_code)
    .bind(recharge_sku_spec_json(package))
    .bind(SEED_TIMESTAMP)
    .bind(SEED_TIMESTAMP)
    .execute(pool)
    .await
    .map_err(|error| storage_error("failed to upsert recharge sku", error))?;
    Ok(())
}

async fn upsert_postgres_recharge_package_sku(
    pool: &PgPool,
    package: &CommerceRechargePackageSeed,
) -> Result<(), CommerceServiceError> {
    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at)
        VALUES
            ($1, '100001', '0', $2, $3, $4, $5, $6, NULL, $7, 'points_credit', 'untracked', 'active', $8, $9, $10)
        ON CONFLICT(id) DO UPDATE SET
            tenant_id = excluded.tenant_id,
            organization_id = excluded.organization_id,
            spu_id = excluded.spu_id,
            sku_no = excluded.sku_no,
            name = excluded.name,
            title = excluded.title,
            price_amount = excluded.price_amount,
            currency_code = excluded.currency_code,
            fulfillment_type = excluded.fulfillment_type,
            inventory_tracking = excluded.inventory_tracking,
            status = excluded.status,
            spec_json = excluded.spec_json,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(package.sku_id)
    .bind(recharge_product_id_for_currency(package.currency_code))
    .bind(package.sku_no)
    .bind(package.name)
    .bind(package.name)
    .bind(package.price_amount)
    .bind(package.currency_code)
    .bind(recharge_sku_spec_json(package))
    .bind(SEED_TIMESTAMP)
    .bind(SEED_TIMESTAMP)
    .execute(pool)
    .await
    .map_err(|error| storage_error("failed to upsert recharge sku", error))?;
    Ok(())
}

async fn upsert_sqlite_recharge_settings(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for setting in commerce_recharge_settings_seeds() {
        let remark_json = recharge_settings_remark_json(&setting);
        sqlx::query(
            r#"
            INSERT INTO commerce_exchange_rule
                (id, tenant_id, organization_id, rule_no, source_asset_type, target_asset_type, rate, status, remark, request_no, idempotency_key, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, 'active', ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                rule_no = excluded.rule_no,
                organization_id = excluded.organization_id,
                source_asset_type = excluded.source_asset_type,
                target_asset_type = excluded.target_asset_type,
                rate = excluded.rate,
                status = excluded.status,
                remark = excluded.remark,
                request_no = excluded.request_no,
                idempotency_key = excluded.idempotency_key,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(setting.rule_id)
        .bind(setting.rule_no)
        .bind(setting.source_asset_type)
        .bind(setting.target_asset_type)
        .bind(setting.rate)
        .bind(&remark_json)
        .bind(format!("seed-recharge-settings-{}", setting.rule_no.to_ascii_lowercase()))
        .bind(format!("seed-recharge-settings-{}", setting.rule_no.to_ascii_lowercase()))
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert recharge settings", error))?;
    }
    Ok(())
}

async fn upsert_postgres_recharge_settings(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for setting in commerce_recharge_settings_seeds() {
        let remark_json = recharge_settings_remark_json(&setting);
        sqlx::query(
            r#"
            INSERT INTO commerce_exchange_rule
                (id, tenant_id, organization_id, rule_no, source_asset_type, target_asset_type, rate, status, remark, request_no, idempotency_key, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, 'active', $6, $7, $8, $9, $10)
            ON CONFLICT(id) DO UPDATE SET
                rule_no = excluded.rule_no,
                organization_id = excluded.organization_id,
                source_asset_type = excluded.source_asset_type,
                target_asset_type = excluded.target_asset_type,
                rate = excluded.rate,
                status = excluded.status,
                remark = excluded.remark,
                request_no = excluded.request_no,
                idempotency_key = excluded.idempotency_key,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(setting.rule_id)
        .bind(setting.rule_no)
        .bind(setting.source_asset_type)
        .bind(setting.target_asset_type)
        .bind(setting.rate)
        .bind(&remark_json)
        .bind(format!("seed-recharge-settings-{}", setting.rule_no.to_ascii_lowercase()))
        .bind(format!("seed-recharge-settings-{}", setting.rule_no.to_ascii_lowercase()))
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert recharge settings", error))?;
    }
    Ok(())
}

fn recharge_settings_remark_json(setting: &CommerceRechargeSettingsSeed) -> String {
    let currency_to_cny_rates = setting
        .currency_to_cny_rates
        .iter()
        .map(|(currency_code, rate)| {
            (
                currency_code.to_string(),
                serde_json::Value::String((*rate).to_string()),
            )
        })
        .collect::<serde_json::Map<String, serde_json::Value>>();
    serde_json::json!({
        "baseCurrencyCode": setting.base_currency_code,
        "currencyToCnyRates": currency_to_cny_rates,
    })
    .to_string()
}

async fn upsert_sqlite_payment_methods(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    let has_legacy_provider = sqlite_payment_method_has_legacy_provider(pool).await?;
    for method in commerce_payment_method_seeds() {
        let request_no = payment_method_request_no(&method);
        if has_legacy_provider {
            sqlx::query(
                r#"
                INSERT INTO commerce_payment_method
                    (id, tenant_id, organization_id, method_key, display_name, provider, provider_code, status, sort_order, request_no, idempotency_key, created_at, updated_at)
                VALUES
                    (?, '100001', '0', ?, ?, ?, ?, 'active', ?, ?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    display_name = excluded.display_name,
                    provider = excluded.provider,
                    provider_code = excluded.provider_code,
                    sort_order = excluded.sort_order,
                    request_no = excluded.request_no,
                    idempotency_key = excluded.idempotency_key,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(method.id)
            .bind(method.method_key)
            .bind(method.display_name)
            .bind(method.provider_code)
            .bind(method.provider_code)
            .bind(method.sort_order)
            .bind(&request_no)
            .bind(&request_no)
            .bind(SEED_TIMESTAMP)
            .bind(SEED_TIMESTAMP)
            .execute(pool)
            .await
            .map_err(|error| storage_error("failed to upsert payment method", error))?;
        } else {
            sqlx::query(
                r#"
                INSERT INTO commerce_payment_method
                    (id, tenant_id, organization_id, method_key, display_name, provider_code, status, sort_order, request_no, idempotency_key, created_at, updated_at)
                VALUES
                    (?, '100001', '0', ?, ?, ?, 'active', ?, ?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    display_name = excluded.display_name,
                    provider_code = excluded.provider_code,
                    sort_order = excluded.sort_order,
                    request_no = excluded.request_no,
                    idempotency_key = excluded.idempotency_key,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(method.id)
            .bind(method.method_key)
            .bind(method.display_name)
            .bind(method.provider_code)
            .bind(method.sort_order)
            .bind(&request_no)
            .bind(&request_no)
            .bind(SEED_TIMESTAMP)
            .bind(SEED_TIMESTAMP)
            .execute(pool)
            .await
            .map_err(|error| storage_error("failed to upsert payment method", error))?;
        }
    }
    Ok(())
}

async fn upsert_postgres_payment_methods(pool: &PgPool) -> Result<(), CommerceServiceError> {
    let has_legacy_provider = postgres_payment_method_has_legacy_provider(pool).await?;
    for method in commerce_payment_method_seeds() {
        let request_no = payment_method_request_no(&method);
        if has_legacy_provider {
            sqlx::query(
                r#"
                INSERT INTO commerce_payment_method
                    (id, tenant_id, organization_id, method_key, display_name, provider, provider_code, status, sort_order, request_no, idempotency_key, created_at, updated_at)
                VALUES
                    ($1, '100001', '0', $2, $3, $4, $5, 'active', $6, $7, $8, $9, $10)
                ON CONFLICT(id) DO UPDATE SET
                    display_name = excluded.display_name,
                    provider = excluded.provider,
                    provider_code = excluded.provider_code,
                    sort_order = excluded.sort_order,
                    request_no = excluded.request_no,
                    idempotency_key = excluded.idempotency_key,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(method.id)
            .bind(method.method_key)
            .bind(method.display_name)
            .bind(method.provider_code)
            .bind(method.provider_code)
            .bind(method.sort_order)
            .bind(&request_no)
            .bind(&request_no)
            .bind(SEED_TIMESTAMP)
            .bind(SEED_TIMESTAMP)
            .execute(pool)
            .await
            .map_err(|error| storage_error("failed to upsert payment method", error))?;
        } else {
            sqlx::query(
                r#"
                INSERT INTO commerce_payment_method
                    (id, tenant_id, organization_id, method_key, display_name, provider_code, status, sort_order, request_no, idempotency_key, created_at, updated_at)
                VALUES
                    ($1, '100001', '0', $2, $3, $4, 'active', $5, $6, $7, $8, $9)
                ON CONFLICT(id) DO UPDATE SET
                    display_name = excluded.display_name,
                    provider_code = excluded.provider_code,
                    sort_order = excluded.sort_order,
                    request_no = excluded.request_no,
                    idempotency_key = excluded.idempotency_key,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(method.id)
            .bind(method.method_key)
            .bind(method.display_name)
            .bind(method.provider_code)
            .bind(method.sort_order)
            .bind(&request_no)
            .bind(&request_no)
            .bind(SEED_TIMESTAMP)
            .bind(SEED_TIMESTAMP)
            .execute(pool)
            .await
            .map_err(|error| storage_error("failed to upsert payment method", error))?;
        }
    }
    Ok(())
}

async fn sqlite_payment_method_has_legacy_provider(
    pool: &SqlitePool,
) -> Result<bool, CommerceServiceError> {
    let column_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM pragma_table_info('commerce_payment_method') WHERE name = 'provider'",
    )
    .fetch_one(pool)
    .await
    .map_err(|error| storage_error("failed to inspect payment method columns", error))?;
    Ok(column_count > 0)
}

async fn postgres_payment_method_has_legacy_provider(
    pool: &PgPool,
) -> Result<bool, CommerceServiceError> {
    sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM information_schema.columns
            WHERE table_schema = current_schema()
              AND table_name = 'commerce_payment_method'
              AND column_name = 'provider'
        )
        "#,
    )
    .fetch_one(pool)
    .await
    .map_err(|error| storage_error("failed to inspect payment method columns", error))
}

async fn upsert_sqlite_payment_providers(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for provider in commerce_payment_provider_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_provider
                (id, tenant_id, organization_id, provider_code, display_name, provider_type, supported_countries, supported_currencies, supported_methods, status, sort_order, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, 'active', ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                display_name = excluded.display_name,
                provider_type = excluded.provider_type,
                supported_countries = excluded.supported_countries,
                supported_currencies = excluded.supported_currencies,
                supported_methods = excluded.supported_methods,
                sort_order = excluded.sort_order,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(provider.id)
        .bind(provider.provider_code)
        .bind(provider.display_name)
        .bind(provider.provider_type)
        .bind(seed_string_array_json(&provider.supported_countries))
        .bind(seed_string_array_json(&provider.supported_currencies))
        .bind(seed_string_array_json(&provider.supported_methods))
        .bind(provider.sort_order)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment provider", error))?;
    }
    Ok(())
}

async fn upsert_postgres_payment_providers(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for provider in commerce_payment_provider_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_provider
                (id, tenant_id, organization_id, provider_code, display_name, provider_type, supported_countries, supported_currencies, supported_methods, status, sort_order, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, 'active', $8, $9, $10)
            ON CONFLICT(id) DO UPDATE SET
                display_name = excluded.display_name,
                provider_type = excluded.provider_type,
                supported_countries = excluded.supported_countries,
                supported_currencies = excluded.supported_currencies,
                supported_methods = excluded.supported_methods,
                sort_order = excluded.sort_order,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(provider.id)
        .bind(provider.provider_code)
        .bind(provider.display_name)
        .bind(provider.provider_type)
        .bind(seed_string_array_json(&provider.supported_countries))
        .bind(seed_string_array_json(&provider.supported_currencies))
        .bind(seed_string_array_json(&provider.supported_methods))
        .bind(provider.sort_order)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment provider", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_payment_provider_accounts(
    pool: &SqlitePool,
) -> Result<(), CommerceServiceError> {
    for account in commerce_payment_provider_account_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_provider_account
                (id, tenant_id, organization_id, account_no, provider_code, merchant_id, environment, country_code, settlement_currency, secret_ref, webhook_secret_ref, certificate_ref, status, rotated_at, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                provider_code = excluded.provider_code,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(account.id)
        .bind(account.account_no)
        .bind(account.provider_code)
        .bind(account.merchant_id)
        .bind(account.environment)
        .bind(account.country_code)
        .bind(account.settlement_currency)
        .bind(account.secret_ref)
        .bind(account.webhook_secret_ref)
        .bind(account.certificate_ref)
        .bind(account.status)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment provider account", error))?;
    }
    Ok(())
}

async fn upsert_postgres_payment_provider_accounts(
    pool: &PgPool,
) -> Result<(), CommerceServiceError> {
    for account in commerce_payment_provider_account_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_provider_account
                (id, tenant_id, organization_id, account_no, provider_code, merchant_id, environment, country_code, settlement_currency, secret_ref, webhook_secret_ref, certificate_ref, status, rotated_at, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NULL, $12, $13)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                provider_code = excluded.provider_code,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(account.id)
        .bind(account.account_no)
        .bind(account.provider_code)
        .bind(account.merchant_id)
        .bind(account.environment)
        .bind(account.country_code)
        .bind(account.settlement_currency)
        .bind(account.secret_ref)
        .bind(account.webhook_secret_ref)
        .bind(account.certificate_ref)
        .bind(account.status)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment provider account", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_payment_channels(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for channel in commerce_payment_channel_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_channel
                (id, tenant_id, organization_id, channel_no, provider_account_id, method_id, scene_code, currency_code, country_code, status, priority, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                provider_account_id = excluded.provider_account_id,
                method_id = excluded.method_id,
                scene_code = excluded.scene_code,
                currency_code = excluded.currency_code,
                country_code = excluded.country_code,
                priority = excluded.priority,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(channel.id)
        .bind(channel.channel_no)
        .bind(channel.provider_account_id)
        .bind(channel.method_id)
        .bind(channel.scene_code)
        .bind(channel.currency_code)
        .bind(channel.country_code)
        .bind(channel.status)
        .bind(channel.priority)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment channel", error))?;
    }
    Ok(())
}

async fn upsert_postgres_payment_channels(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for channel in commerce_payment_channel_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_channel
                (id, tenant_id, organization_id, channel_no, provider_account_id, method_id, scene_code, currency_code, country_code, status, priority, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                provider_account_id = excluded.provider_account_id,
                method_id = excluded.method_id,
                scene_code = excluded.scene_code,
                currency_code = excluded.currency_code,
                country_code = excluded.country_code,
                priority = excluded.priority,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(channel.id)
        .bind(channel.channel_no)
        .bind(channel.provider_account_id)
        .bind(channel.method_id)
        .bind(channel.scene_code)
        .bind(channel.currency_code)
        .bind(channel.country_code)
        .bind(channel.status)
        .bind(channel.priority)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment channel", error))?;
    }
    Ok(())
}

async fn upsert_sqlite_payment_route_rules(pool: &SqlitePool) -> Result<(), CommerceServiceError> {
    for rule in commerce_payment_route_rule_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_route_rule
                (id, tenant_id, organization_id, rule_no, priority, purchase_type, country_code, currency_code, client_platform, amount_min, amount_max, user_segment, risk_level, channel_id, status, starts_at, ends_at, created_at, updated_at)
            VALUES
                (?, '100001', '0', ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, ?, ?, NULL, NULL, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                priority = excluded.priority,
                purchase_type = excluded.purchase_type,
                country_code = excluded.country_code,
                currency_code = excluded.currency_code,
                client_platform = excluded.client_platform,
                amount_min = excluded.amount_min,
                amount_max = excluded.amount_max,
                user_segment = excluded.user_segment,
                risk_level = excluded.risk_level,
                channel_id = excluded.channel_id,
                starts_at = excluded.starts_at,
                ends_at = excluded.ends_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(rule.id)
        .bind(rule.rule_no)
        .bind(rule.priority)
        .bind(rule.purchase_type)
        .bind(rule.country_code)
        .bind(rule.currency_code)
        .bind(rule.client_platform)
        .bind(rule.channel_id)
        .bind(rule.status)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment route rule", error))?;
    }
    Ok(())
}

async fn upsert_postgres_payment_route_rules(pool: &PgPool) -> Result<(), CommerceServiceError> {
    for rule in commerce_payment_route_rule_seeds() {
        sqlx::query(
            r#"
            INSERT INTO commerce_payment_route_rule
                (id, tenant_id, organization_id, rule_no, priority, purchase_type, country_code, currency_code, client_platform, amount_min, amount_max, user_segment, risk_level, channel_id, status, starts_at, ends_at, created_at, updated_at)
            VALUES
                ($1, '100001', '0', $2, $3, $4, $5, $6, $7, NULL, NULL, NULL, NULL, $8, $9, NULL, NULL, $10, $11)
            ON CONFLICT(id) DO UPDATE SET
                organization_id = excluded.organization_id,
                priority = excluded.priority,
                purchase_type = excluded.purchase_type,
                country_code = excluded.country_code,
                currency_code = excluded.currency_code,
                client_platform = excluded.client_platform,
                amount_min = excluded.amount_min,
                amount_max = excluded.amount_max,
                user_segment = excluded.user_segment,
                risk_level = excluded.risk_level,
                channel_id = excluded.channel_id,
                starts_at = excluded.starts_at,
                ends_at = excluded.ends_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(rule.id)
        .bind(rule.rule_no)
        .bind(rule.priority)
        .bind(rule.purchase_type)
        .bind(rule.country_code)
        .bind(rule.currency_code)
        .bind(rule.client_platform)
        .bind(rule.channel_id)
        .bind(rule.status)
        .bind(SEED_TIMESTAMP)
        .bind(SEED_TIMESTAMP)
        .execute(pool)
        .await
        .map_err(|error| storage_error("failed to upsert payment route rule", error))?;
    }
    Ok(())
}

fn membership_sku_spec_json(package: &CommerceMembershipPackageSeed) -> String {
    let group = group_seed_for(package.package_group_no);
    serde_json::json!({
        "kind": "membership_package",
        "packageId": package.id,
        "packageNo": package_no_for(package.package_no),
        "groupCode": group_code(package.package_group_no),
        "groupNo": group_no_for(package.package_group_no),
        "groupName": group.as_ref().map(|item| item.name).unwrap_or(package.package_group_no),
        "planNo": package.plan_no,
        "planCode": package.plan_no,
        "durationDays": package.duration_days,
        "recurrenceCycle": recurrence_cycle_for(package.package_group_no),
        "pointAmount": package.point_amount,
        "recommended": package.recommended,
        "tags": package.tags,
    })
    .to_string()
}

fn recharge_sku_spec_json(package: &CommerceRechargePackageSeed) -> String {
    serde_json::json!({
        "kind": "points_recharge_package",
        "packageId": package.package_id,
        "packageNo": package.package_no,
        "externalId": package.external_id,
        "bonusPoints": package.bonus_points,
    })
    .to_string()
}

fn package_group_id_for(package_group_no: &str) -> &'static str {
    match group_code(package_group_no) {
        "month" => "seed-membership-package-group-month",
        "year" => "seed-membership-package-group-year",
        _ => "seed-membership-package-group-unknown",
    }
}

fn plan_id_for(plan_no: &str) -> &'static str {
    match plan_no {
        "free" => "seed-membership-plan-free",
        "pro" => "seed-membership-plan-pro",
        "max" => "seed-membership-plan-max",
        "vip" => "seed-membership-plan-vip",
        _ => "seed-membership-plan-free",
    }
}

fn plan_version_id_for(plan_no: &str) -> &'static str {
    match plan_no {
        "free" => "seed-membership-plan-version-free-v1",
        "pro" => "seed-membership-plan-version-pro-v1",
        "max" => "seed-membership-plan-version-max-v1",
        "vip" => "seed-membership-plan-version-vip-v1",
        _ => "seed-membership-plan-version-free-v1",
    }
}

fn benefit_definition_id_for(benefit_code: &str) -> &'static str {
    match benefit_code {
        "ai_quota" => "seed-benefit-ai-quota",
        "priority_speed_up" => "seed-benefit-priority-speed-up",
        "member_discount" => "seed-benefit-member-discount",
        "monthly_coupon_grant" => "seed-benefit-monthly-coupon-grant",
        _ => "seed-benefit-ai-quota",
    }
}

fn group_seed_for(group_no: &str) -> Option<CommerceMembershipPackageGroupSeed> {
    membership_package_group_seeds()
        .into_iter()
        .find(|group| group.package_group_no == group_no)
}

fn group_code(group_no: &str) -> &str {
    group_no.strip_prefix("membership-").unwrap_or(group_no)
}

fn group_no_for(group_no: &str) -> String {
    format!("membership-{}", group_code(group_no))
}

fn package_no_for(package_no: &str) -> String {
    package_no.to_owned()
}

fn sku_no_for(sku_no: &str) -> String {
    sku_no.to_owned()
}

fn recurrence_cycle_for(group_no: &str) -> &str {
    match group_code(group_no) {
        "year" => "year",
        _ => "month",
    }
}

fn seed_string_array_json(values: &[&str]) -> String {
    serde_json::to_string(values).unwrap_or_else(|_| "[]".to_owned())
}

fn payment_method_request_no(method: &CommercePaymentMethodSeed) -> String {
    format!("seed-payment-method-{}", method.method_key)
}

fn seed_product_type(spu_no: &str) -> &'static str {
    match spu_no {
        "membership" => "membership",
        RECHARGE_PRODUCT_CNY_SPU_NO | RECHARGE_PRODUCT_NON_CNY_SPU_NO => "points_recharge",
        _ => "virtual",
    }
}

fn is_cny_currency(currency_code: &str) -> bool {
    let normalized = currency_code.trim();
    normalized.is_empty() || normalized.eq_ignore_ascii_case("CNY")
}

fn recharge_product_id_for_currency(currency_code: &str) -> &'static str {
    if is_cny_currency(currency_code) {
        RECHARGE_PRODUCT_CNY_ID
    } else {
        RECHARGE_PRODUCT_NON_CNY_ID
    }
}

async fn sqlite_count(pool: &SqlitePool, statement: &str) -> Result<i64, CommerceServiceError> {
    sqlx::query_scalar::<_, i64>(statement)
        .fetch_one(pool)
        .await
        .map_err(|error| storage_error("failed to count commerce seed rows", error))
}

async fn postgres_count(pool: &PgPool, statement: &str) -> Result<i64, CommerceServiceError> {
    let sql = if statement.contains("::bigint") {
        statement.to_string()
    } else {
        statement.replacen("COUNT(1)", "(COUNT(1))::bigint", 1)
    };
    sqlx::query_scalar::<_, i64>(&sql)
        .fetch_one(pool)
        .await
        .map_err(|error| storage_error("failed to count commerce seed rows", error))
}

struct SeedIntegrityCheck {
    code: &'static str,
    message: &'static str,
    expected_count: i64,
    statement: &'static str,
}

impl SeedIntegrityCheck {
    fn issue(&self, actual_count: i64) -> CommerceExperienceSeedIntegrityIssue {
        CommerceExperienceSeedIntegrityIssue {
            code: self.code.to_owned(),
            message: self.message.to_owned(),
            expected_count: self.expected_count,
            actual_count,
        }
    }
}

const COMMERCE_EXPERIENCE_SEED_INTEGRITY_CHECKS: &[SeedIntegrityCheck] = &[
    SeedIntegrityCheck {
        code: "missing_seed_product",
        message: "expected active membership and grouped points recharge seed products",
        expected_count: 3,
        statement: COMPLETE_PRODUCT_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_membership_plan",
        message: "expected four active membership plans",
        expected_count: 4,
        statement: COMPLETE_MEMBERSHIP_PLAN_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_membership_package_group",
        message: "expected two active membership package groups",
        expected_count: 2,
        statement: COMPLETE_MEMBERSHIP_PACKAGE_GROUP_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_membership_package",
        message: "expected six active membership packages",
        expected_count: 6,
        statement: COMPLETE_MEMBERSHIP_PACKAGE_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "incomplete_membership_package_group",
        message: "expected each seeded membership package group to contain three packages",
        expected_count: 2,
        statement: COMPLETE_MEMBERSHIP_GROUP_PACKAGE_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_membership_sku",
        message: "expected six active membership package SKUs",
        expected_count: 6,
        statement: COMPLETE_MEMBERSHIP_SKU_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_recharge_package",
        message: "expected nine active default points recharge packages",
        expected_count: 9,
        statement: COMPLETE_RECHARGE_PACKAGE_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_recharge_sku",
        message: "expected eighteen seeded points recharge SKUs",
        expected_count: 18,
        statement: COMPLETE_RECHARGE_SKU_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_recharge_settings",
        message: "expected default cash to points recharge settings",
        expected_count: 1,
        statement: COMPLETE_RECHARGE_SETTINGS_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_payment_method",
        message: "expected standard inactive or active payment methods",
        expected_count: PAYMENT_METHOD_SEED_COUNT,
        statement: COMPLETE_PAYMENT_METHOD_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_payment_provider",
        message: "expected standard inactive or active payment providers",
        expected_count: PAYMENT_PROVIDER_SEED_COUNT,
        statement: COMPLETE_PAYMENT_PROVIDER_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_payment_provider_account",
        message: "expected standard inactive or active payment provider accounts",
        expected_count: PAYMENT_PROVIDER_ACCOUNT_SEED_COUNT,
        statement: COMPLETE_PAYMENT_PROVIDER_ACCOUNT_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_payment_channel",
        message: "expected standard inactive or active payment channels",
        expected_count: PAYMENT_CHANNEL_SEED_COUNT,
        statement: COMPLETE_PAYMENT_CHANNEL_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "missing_payment_route_rule",
        message: "expected standard inactive or active payment route rules",
        expected_count: PAYMENT_ROUTE_RULE_SEED_COUNT,
        statement: COMPLETE_PAYMENT_ROUTE_RULE_COUNT_SQL,
    },
    SeedIntegrityCheck {
        code: "orphan_membership_package_plan",
        message: "expected every membership package to reference an active seeded membership plan",
        expected_count: 0,
        statement: ORPHAN_MEMBERSHIP_PACKAGE_PLAN_SQL,
    },
    SeedIntegrityCheck {
        code: "orphan_membership_package_group",
        message: "expected every membership package to reference an active seeded package group",
        expected_count: 0,
        statement: ORPHAN_MEMBERSHIP_PACKAGE_GROUP_SQL,
    },
    SeedIntegrityCheck {
        code: "orphan_membership_package_sku",
        message: "expected every membership package to reference an active membership SKU",
        expected_count: 0,
        statement: ORPHAN_MEMBERSHIP_PACKAGE_SKU_SQL,
    },
    SeedIntegrityCheck {
        code: "invalid_membership_sku_product",
        message: "expected every membership package SKU to belong to the membership seed product",
        expected_count: 0,
        statement: INVALID_MEMBERSHIP_SKU_PRODUCT_SQL,
    },
    SeedIntegrityCheck {
        code: "orphan_recharge_package_sku",
        message:
            "expected every points recharge package to reference an active points recharge SKU",
        expected_count: 0,
        statement: ORPHAN_RECHARGE_PACKAGE_SKU_SQL,
    },
    SeedIntegrityCheck {
        code: "invalid_recharge_sku_product",
        message: "expected every points recharge SKU to belong to the points recharge seed product",
        expected_count: 0,
        statement: INVALID_RECHARGE_SKU_PRODUCT_SQL,
    },
];

const COMPLETE_PRODUCT_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_product_spu
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND spu_no IN ('membership', 'points-recharge-cny', 'points-recharge-non-cny')
  AND status = 'active'
"#;

const COMPLETE_MEMBERSHIP_PLAN_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM membership_plan
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND plan_no IN ('free', 'pro', 'max', 'vip')
  AND status = 'active'
"#;

const COMPLETE_MEMBERSHIP_PACKAGE_GROUP_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM membership_package_group
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND external_id IN (1, 2)
  AND group_no IN ('membership-month', 'membership-year')
  AND status = 'active'
"#;

const COMPLETE_MEMBERSHIP_PACKAGE_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM membership_package
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND external_id IN (301, 302, 303, 401, 402, 403)
  AND package_no LIKE 'membership-%'
  AND status = 'active'
"#;

const COMPLETE_MEMBERSHIP_GROUP_PACKAGE_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM (
    SELECT package_group_id, COUNT(1) AS package_count
    FROM membership_package
    WHERE tenant_id = '100001'
      AND organization_id = '0'
      AND status = 'active'
    GROUP BY package_group_id
    HAVING COUNT(1) = 3
) seeded_groups
"#;

const COMPLETE_MEMBERSHIP_SKU_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_product_sku
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND spu_id = 'seed-product-membership'
  AND sku_no LIKE 'membership-%'
  AND status = 'active'
"#;

const COMPLETE_RECHARGE_PACKAGE_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_recharge_package
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND package_no LIKE 'points-%'
  AND status = 'active'
"#;

const COMPLETE_RECHARGE_SKU_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_product_sku
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND spu_id IN ('seed-product-points-recharge-cny', 'seed-product-points-recharge-non-cny')
  AND sku_no LIKE 'points-recharge-%'
"#;

const COMPLETE_RECHARGE_SETTINGS_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_exchange_rule
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND rule_no = 'CASH_TO_POINTS'
  AND source_asset_type = 'cash'
  AND target_asset_type = 'points'
  AND status = 'active'
"#;

const COMPLETE_PAYMENT_METHOD_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_payment_method
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND method_key IN ('wechat_pay', 'alipay', 'paypal', 'card', 'apple_pay', 'google_pay', 'wallet_balance')
  AND status IN ('active', 'inactive', 'disabled')
"#;

const COMPLETE_PAYMENT_PROVIDER_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_payment_provider
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND provider_code IN ('wechat_pay', 'alipay', 'paypal', 'stripe', 'apple_pay', 'google_pay')
  AND status IN ('active', 'inactive', 'disabled')
"#;

const COMPLETE_PAYMENT_PROVIDER_ACCOUNT_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_payment_provider_account
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND account_no IN ('seed-wechat-pay-sandbox', 'seed-alipay-sandbox', 'seed-paypal-sandbox', 'seed-stripe-sandbox', 'seed-apple-pay-sandbox', 'seed-google-pay-sandbox')
  AND status IN ('active', 'inactive', 'disabled')
"#;

const COMPLETE_PAYMENT_CHANNEL_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_payment_channel
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND id LIKE 'seed-payment-channel-%'
  AND channel_no LIKE 'seed-%'
  AND status IN ('active', 'inactive', 'disabled')
"#;

const COMPLETE_PAYMENT_ROUTE_RULE_COUNT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_payment_route_rule
WHERE tenant_id = '100001'
  AND organization_id = '0'
  AND id LIKE 'seed-payment-route-rule-%'
  AND rule_no LIKE 'route-seed-%'
  AND status IN ('active', 'inactive', 'disabled')
"#;

const ORPHAN_MEMBERSHIP_PACKAGE_PLAN_SQL: &str = r#"
SELECT COUNT(1)
FROM membership_package package
LEFT JOIN membership_plan plan
  ON plan.id = package.plan_id
 AND plan.tenant_id = package.tenant_id
 AND plan.organization_id = package.organization_id
 AND plan.status = 'active'
WHERE package.tenant_id = '100001'
  AND package.organization_id = '0'
  AND package.package_no LIKE 'membership-%'
  AND package.status = 'active'
  AND plan.id IS NULL
"#;

const ORPHAN_MEMBERSHIP_PACKAGE_GROUP_SQL: &str = r#"
SELECT COUNT(1)
FROM membership_package package
LEFT JOIN membership_package_group package_group
  ON package_group.id = package.package_group_id
 AND package_group.tenant_id = package.tenant_id
 AND package_group.organization_id = package.organization_id
 AND package_group.status = 'active'
WHERE package.tenant_id = '100001'
  AND package.organization_id = '0'
  AND package.package_no LIKE 'membership-%'
  AND package.status = 'active'
  AND package_group.id IS NULL
"#;

const ORPHAN_MEMBERSHIP_PACKAGE_SKU_SQL: &str = r#"
SELECT COUNT(1)
FROM membership_package package
LEFT JOIN commerce_product_sku sku
  ON sku.id = package.sku_id
 AND sku.tenant_id = package.tenant_id
 AND sku.organization_id = package.organization_id
 AND sku.status = 'active'
WHERE package.tenant_id = '100001'
  AND package.organization_id = '0'
  AND package.package_no LIKE 'membership-%'
  AND package.status = 'active'
  AND sku.id IS NULL
"#;

const INVALID_MEMBERSHIP_SKU_PRODUCT_SQL: &str = r#"
SELECT COUNT(1)
FROM membership_package package
JOIN commerce_product_sku sku
  ON sku.id = package.sku_id
 AND sku.tenant_id = package.tenant_id
 AND sku.organization_id = package.organization_id
WHERE package.tenant_id = '100001'
  AND package.organization_id = '0'
  AND package.package_no LIKE 'membership-%'
  AND package.status = 'active'
  AND sku.spu_id <> 'seed-product-membership'
"#;

const ORPHAN_RECHARGE_PACKAGE_SKU_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_recharge_package package
LEFT JOIN commerce_product_sku sku
  ON sku.id = package.sku_id
 AND sku.tenant_id = package.tenant_id
 AND sku.organization_id = package.organization_id
 AND sku.status = 'active'
WHERE package.tenant_id = '100001'
  AND package.organization_id = '0'
  AND package.package_no LIKE 'points-%'
  AND package.status = 'active'
  AND sku.id IS NULL
"#;

const INVALID_RECHARGE_SKU_PRODUCT_SQL: &str = r#"
SELECT COUNT(1)
FROM commerce_recharge_package package
JOIN commerce_product_sku sku
  ON sku.id = package.sku_id
 AND sku.tenant_id = package.tenant_id
 AND sku.organization_id = package.organization_id
WHERE package.tenant_id = '100001'
  AND package.organization_id = '0'
  AND package.package_no LIKE 'points-%'
  AND package.status = 'active'
  AND sku.spu_id NOT IN ('seed-product-points-recharge-cny', 'seed-product-points-recharge-non-cny')
"#;

fn storage_error(context: &str, error: sqlx::Error) -> CommerceServiceError {
    CommerceServiceError::storage(format!("{context}: {error}"))
}
