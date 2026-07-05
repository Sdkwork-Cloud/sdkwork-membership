use std::collections::BTreeMap;

use sdkwork_contract_service::{CommercePaymentStatus, CommerceServiceError};
use sdkwork_utils_rust::SdkWorkPageData;
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::pagination::{
    bounded_sql_page, cursor_page, offset_page, organization_id_text, tenant_id_text,
    MembershipListQuery,
};

use crate::read_model::is_missing_postgres_read_model;
use crate::shared::{
    build_package_group_from_packages, decimal_string,
    map_membership_package_record, normalize_payment_method, parse_points_amount,
    payment_product_for_scan_qr, plan_code_from_rank, plan_rank_from_code,
    privilege_usage_from_benefits, ParsedMembershipPackage, StoredMembershipPlan,
    POINTS_ASSET_TYPE, POINTS_CURRENCY_CODE,
};
use crate::{
    AdminMembershipEntitlementItem, AdminMembershipFuture, AdminMembershipMemberItem,
    AdminMembershipPackageGroupItem, AdminMembershipPackageItem, AdminMembershipPlanItem,
    AdminMembershipStore, AppMembershipBenefitItem, AppMembershipCommandFuture,
    AppMembershipListQuery,
    AppMembershipDailyRewardResponse, AppMembershipDailyRewardStatusResponse,
    AppMembershipInfoResponse, AppMembershipPackageGroupItem, AppMembershipPackageItem,
    AppMembershipPlanItem, AppMembershipPointsBalanceResponse, AppMembershipPointsHistoryItem,
    AppMembershipPointsHistoryQuery, AppMembershipPrivilegeUsageResponse,
    AppMembershipPurchaseOutcome, AppMembershipReadFuture, AppMembershipResult,
    AppMembershipStatusResponse, AppMembershipStore, AppMembershipSubject,
    CreateAdminMembershipPackageCommand, CreateAdminMembershipPackageGroupCommand,
    CreateAdminMembershipPlanCommand, DeleteAdminMembershipPackageCommand,
    DeleteAdminMembershipPackageGroupCommand, DeleteAdminMembershipPlanCommand,
    ListAdminMembershipEntitlementsQuery, ListAdminMembershipMembersQuery,
    ListAdminMembershipPackageGroupsQuery, ListAdminMembershipPackagesQuery,
    ListAdminMembershipPlansQuery, SubmitMembershipPurchaseCommand,
    UpdateAdminMembershipMemberStatusCommand, UpdateAdminMembershipPackageCommand,
    UpdateAdminMembershipPackageGroupCommand, UpdateAdminMembershipPlanCommand,
};

const LOAD_MEMBERSHIP_PACKAGES_BASE: &str = r#"
SELECT
    CAST(p.external_id AS INTEGER) AS external_id,
    p.name,
    p.description,
    CAST(p.price_amount AS TEXT) AS price_amount,
    CAST(COALESCE(p.original_price_amount, '') AS TEXT) AS original_price_amount,
    CAST(COALESCE(p.point_amount, 0) AS INTEGER) AS point_amount,
    CAST(p.duration_days AS INTEGER) AS duration_days,
    CAST(COALESCE(p.sort_weight, 0) AS INTEGER) AS sort_weight,
    CAST(COALESCE(p.recommended, 0) AS INTEGER) AS recommended,
    '[]' AS tags_json,
    p.id AS package_storage_id,
    p.package_group_id AS package_group_storage_id,
    p.plan_id AS plan_storage_id,
    p.sku_id,
    g.external_id AS group_external_id,
    g.name AS group_name,
    g.description AS group_description,
    CAST(COALESCE(g.sort_weight, 0) AS INTEGER) AS group_sort_weight,
    l.plan_no AS plan_no,
    l.name AS plan_name,
    CAST(l.rank AS INTEGER) AS rank
FROM membership_package p
JOIN membership_package_group g
    ON g.id = p.package_group_id
LEFT JOIN membership_plan l
    ON l.id = p.plan_id
WHERE (p.tenant_id = '100001' OR p.tenant_id IS NULL)
  AND (p.organization_id = '0' OR p.organization_id IS NULL)
  AND (g.tenant_id = '100001' OR g.tenant_id IS NULL)
  AND (g.organization_id = '0' OR g.organization_id IS NULL)
  AND p.status = 'active'
  AND g.status = 'active'
"#;

const LOAD_MEMBERSHIP_PACKAGE_BY_ID: &str = r#"
SELECT
    CAST(p.external_id AS INTEGER) AS external_id,
    p.name,
    p.description,
    CAST(p.price_amount AS TEXT) AS price_amount,
    CAST(COALESCE(p.original_price_amount, '') AS TEXT) AS original_price_amount,
    CAST(COALESCE(p.point_amount, 0) AS INTEGER) AS point_amount,
    CAST(p.duration_days AS INTEGER) AS duration_days,
    CAST(COALESCE(p.sort_weight, 0) AS INTEGER) AS sort_weight,
    CAST(COALESCE(p.recommended, 0) AS INTEGER) AS recommended,
    '[]' AS tags_json,
    p.id AS package_storage_id,
    p.package_group_id AS package_group_storage_id,
    p.plan_id AS plan_storage_id,
    p.sku_id,
    g.external_id AS group_external_id,
    g.name AS group_name,
    g.description AS group_description,
    CAST(COALESCE(g.sort_weight, 0) AS INTEGER) AS group_sort_weight,
    l.plan_no AS plan_no,
    l.name AS plan_name,
    CAST(l.rank AS INTEGER) AS rank
FROM membership_package p
JOIN membership_package_group g
    ON g.id = p.package_group_id
LEFT JOIN membership_plan l
    ON l.id = p.plan_id
WHERE p.external_id = $1
  AND (p.tenant_id = '100001' OR p.tenant_id IS NULL)
  AND (p.organization_id = '0' OR p.organization_id IS NULL)
  AND (g.tenant_id = '100001' OR g.tenant_id IS NULL)
  AND (g.organization_id = '0' OR g.organization_id IS NULL)
  AND p.status = 'active'
  AND g.status = 'active'
LIMIT 1
"#;

const LOAD_MEMBERSHIP_PLAN_BY_RANK: &str = r#"
SELECT
    p.id,
    p.plan_no AS plan_no,
    p.name,
    CAST(p.rank AS INTEGER) AS rank,
    p.description,
    b.id AS plan_benefit_id,
    b.benefit_code,
    CAST(b.grant_quantity AS TEXT) AS grant_quantity,
    b.usage_policy,
    d.name AS benefit_name,
    d.benefit_type,
    d.description AS benefit_description
FROM membership_plan p
LEFT JOIN membership_plan_version v
    ON v.plan_id = p.id
   AND v.tenant_id = p.tenant_id
   AND v.lifecycle_status = 'published'
LEFT JOIN membership_plan_benefit b
    ON b.plan_version_id = v.id
   AND b.tenant_id = p.tenant_id
   AND b.status = 'active'
LEFT JOIN benefit_definition d
    ON d.id = b.benefit_id
   AND d.tenant_id = b.tenant_id
WHERE (p.tenant_id = '100001' OR p.tenant_id IS NULL)
  AND (p.organization_id = '0' OR p.organization_id IS NULL)
  AND p.status = 'active'
  AND CAST(p.rank AS INTEGER) = $1
ORDER BY b.sort_weight ASC, b.id ASC
"#;

const LOAD_MEMBERSHIP: &str = r#"
SELECT
    m.id AS membership_id,
    m.plan_id AS plan_storage_id,
    m.status,
    CAST(m.starts_at AS TEXT) AS starts_at,
    CAST(m.expires_at AS TEXT) AS expires_at,
    l.plan_no AS plan_no,
    l.name AS plan_name,
    CAST(l.rank AS INTEGER) AS rank,
    CAST(COALESCE(ab.payable_amount, pi.amount, '0') AS TEXT) AS total_spent
FROM membership_subscription m
LEFT JOIN membership_plan l
    ON l.id = m.plan_id
LEFT JOIN commerce_order_amount_breakdown ab
    ON ab.order_id = m.source_order_id
LEFT JOIN commerce_payment_intent pi
    ON pi.id = m.source_payment_intent_id
WHERE m.tenant_id = CAST($1 AS TEXT)
  AND (m.organization_id IS NULL OR m.organization_id = CAST($2 AS TEXT))
  AND m.subject_type = 'user'
  AND m.subject_id = CAST($3 AS TEXT)
ORDER BY m.created_at DESC, m.id DESC
LIMIT 1
"#;

const LOAD_POINTS_BALANCE: &str = r#"
SELECT
    CAST(available_amount AS TEXT) AS available_amount,
    CAST(frozen_amount AS TEXT) AS frozen_amount
FROM commerce_account
WHERE tenant_id = CAST($1 AS TEXT)
  AND (organization_id IS NULL OR organization_id = CAST($2 AS TEXT))
  AND owner_user_id = CAST($3 AS TEXT)
  AND asset_type = $4
  AND (currency_code = $5 OR currency_code IS NULL)
  AND status = 'active'
ORDER BY updated_at DESC, id DESC
LIMIT 1
"#;

const LOAD_POINTS_HISTORY: &str = r#"
SELECT
    id,
    direction,
    CAST(amount AS TEXT) AS amount,
    CAST(balance_after AS TEXT) AS balance_after,
    business_type,
    source_type,
    remark,
    CAST(created_at AS TEXT) AS created_at
FROM commerce_account_ledger_entry
WHERE tenant_id = CAST($1 AS TEXT)
  AND (organization_id IS NULL OR organization_id = CAST($2 AS TEXT))
  AND owner_user_id = CAST($3 AS TEXT)
  AND asset_type = $4
ORDER BY created_at DESC, id DESC
LIMIT $5 OFFSET $6
"#;

const LOAD_POINTS_HISTORY_CURSOR: &str = r#"
SELECT
    id,
    direction,
    CAST(amount AS TEXT) AS amount,
    CAST(balance_after AS TEXT) AS balance_after,
    business_type,
    source_type,
    remark,
    CAST(created_at AS TEXT) AS created_at
FROM commerce_account_ledger_entry
WHERE tenant_id = CAST($1 AS TEXT)
  AND (organization_id IS NULL OR organization_id = CAST($2 AS TEXT))
  AND owner_user_id = CAST($3 AS TEXT)
  AND asset_type = $4
  AND id < $5
ORDER BY created_at DESC, id DESC
LIMIT $6
"#;

const LOAD_PAYMENT_METHOD: &str = r#"
SELECT
    method_key,
    provider_code
FROM commerce_payment_method
WHERE (tenant_id = CAST($1 AS TEXT) OR tenant_id = '100001')
  AND (organization_id = CAST($2 AS TEXT) OR organization_id = '0')
  AND status = 'active'
  AND LOWER(method_key) = $3
ORDER BY tenant_id DESC, organization_id DESC, sort_order ASC, id ASC
LIMIT 1
"#;

#[derive(Debug, Clone)]
pub struct PostgresCommerceMembershipStore {
    pool: PgPool,
}

impl PostgresCommerceMembershipStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn load_info<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipInfoResponse> {
        <Self as AppMembershipStore>::load_info(self, subject)
    }

    pub fn load_status<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipStatusResponse> {
        <Self as AppMembershipStore>::load_status(self, subject)
    }

    pub fn load_plans<'a>(
        &'a self,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPlanItem>> {
        <Self as AppMembershipStore>::load_plans(self, query)
    }

    pub fn load_benefits<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipBenefitItem>> {
        <Self as AppMembershipStore>::load_benefits(self, subject, plan_id, query)
    }

    pub fn load_packages<'a>(
        &'a self,
        package_group_id: Option<i64>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageItem>> {
        <Self as AppMembershipStore>::load_packages(self, package_group_id, plan_id, query)
    }

    pub fn load_package<'a>(
        &'a self,
        package_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageItem>> {
        <Self as AppMembershipStore>::load_package(self, package_id)
    }

    pub fn load_package_groups<'a>(
        &'a self,
        plan_id: Option<i64>,
        recommended_only: bool,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageGroupItem>> {
        <Self as AppMembershipStore>::load_package_groups(self, plan_id, recommended_only, query)
    }

    pub fn load_package_group<'a>(
        &'a self,
        package_group_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageGroupItem>> {
        <Self as AppMembershipStore>::load_package_group(self, package_group_id)
    }

    pub fn load_points_balance<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPointsBalanceResponse> {
        <Self as AppMembershipStore>::load_points_balance(self, subject)
    }

    pub fn load_points_history<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
        query: AppMembershipPointsHistoryQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPointsHistoryItem>> {
        <Self as AppMembershipStore>::load_points_history(self, subject, query)
    }

    pub fn load_daily_reward_status<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardStatusResponse> {
        <Self as AppMembershipStore>::load_daily_reward_status(self, subject)
    }

    pub fn claim_daily_reward<'a>(
        &'a self,
        subject: AppMembershipSubject,
        requested_at: String,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardResponse> {
        <Self as AppMembershipStore>::claim_daily_reward(self, subject, requested_at)
    }

    pub fn load_privilege_usage<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPrivilegeUsageResponse> {
        <Self as AppMembershipStore>::load_privilege_usage(self, subject)
    }

    pub fn submit_purchase<'a>(
        &'a self,
        command: SubmitMembershipPurchaseCommand,
    ) -> AppMembershipCommandFuture<'a> {
        <Self as AppMembershipStore>::submit_purchase(self, command)
    }
}

impl AppMembershipStore for PostgresCommerceMembershipStore {
    fn load_info<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipInfoResponse> {
        Box::pin(async move { load_info(&self.pool, subject).await })
    }

    fn load_status<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipStatusResponse> {
        Box::pin(async move { load_status(&self.pool, subject).await })
    }

    fn load_plans<'a>(
        &'a self,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPlanItem>> {
        Box::pin(async move { load_plans_page(&self.pool, query).await })
    }

    fn load_benefits<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipBenefitItem>> {
        Box::pin(async move { load_benefits_page(&self.pool, subject, plan_id, query).await })
    }

    fn load_packages<'a>(
        &'a self,
        package_group_id: Option<i64>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageItem>> {
        Box::pin(async move {
            load_package_rows(&self.pool, package_group_id, plan_id, query, false).await
        })
    }

    fn load_package<'a>(
        &'a self,
        package_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageItem>> {
        Box::pin(async move { load_package_by_id(&self.pool, package_id).await })
    }

    fn load_package_groups<'a>(
        &'a self,
        plan_id: Option<i64>,
        recommended_only: bool,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageGroupItem>> {
        Box::pin(async move {
            load_package_groups_page(&self.pool, plan_id, recommended_only, query).await
        })
    }

    fn load_package_group<'a>(
        &'a self,
        package_group_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageGroupItem>> {
        Box::pin(async move { load_package_group_by_id(&self.pool, package_group_id).await })
    }

    fn load_points_balance<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPointsBalanceResponse> {
        Box::pin(async move { load_points_balance(&self.pool, subject).await })
    }

    fn load_points_history<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
        query: AppMembershipPointsHistoryQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPointsHistoryItem>> {
        Box::pin(async move { load_points_history(&self.pool, subject, query).await })
    }

    fn load_daily_reward_status<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardStatusResponse> {
        Box::pin(async move {
            let Some(subject) = subject else {
                return Ok(AppMembershipDailyRewardStatusResponse {
                    can_claim: false,
                    claimed_today: false,
                    consecutive_days: 0,
                    total_days: 0,
                });
            };
            load_daily_reward_status_sqlite(&self.pool, subject).await
        })
    }

    fn claim_daily_reward<'a>(
        &'a self,
        subject: AppMembershipSubject,
        requested_at: String,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardResponse> {
        Box::pin(async move { claim_daily_reward_sqlite(&self.pool, subject, requested_at).await })
    }

    fn load_privilege_usage<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPrivilegeUsageResponse> {
        Box::pin(async move {
            let benefits = load_benefits_page(
                &self.pool,
                subject,
                None,
                AppMembershipListQuery::default(),
            )
            .await?
            .items;
            let mut usage = privilege_usage_from_benefits(&benefits);
            if let Some(subject) = subject {
                if let Ok(actual) = load_privilege_usage_sqlite(&self.pool, subject).await {
                    usage.speed_up_used = actual.speed_up_used;
                    usage.priority_queue_used = actual.priority_queue_used;
                    usage.exclusive_model_used = actual.exclusive_model_used;
                }
            }
            Ok(usage)
        })
    }

    fn consume_speed_up<'a>(
        &'a self,
        subject: AppMembershipSubject,
        requested_at: String,
    ) -> AppMembershipReadFuture<'a, ()> {
        Box::pin(async move { consume_speed_up(&self.pool, subject, requested_at).await })
    }

    fn submit_purchase<'a>(
        &'a self,
        command: SubmitMembershipPurchaseCommand,
    ) -> AppMembershipCommandFuture<'a> {
        Box::pin(async move { submit_purchase(&self.pool, command).await })
    }
}

impl AdminMembershipStore for PostgresCommerceMembershipStore {
    fn list_admin_membership_plans<'a>(
        &'a self,
        query: ListAdminMembershipPlansQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipPlanItem>> {
        Box::pin(async move { list_admin_membership_plans(&self.pool, query).await })
    }

    fn create_admin_membership_plan<'a>(
        &'a self,
        command: CreateAdminMembershipPlanCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPlanItem> {
        Box::pin(async move { create_admin_membership_plan(&self.pool, command).await })
    }

    fn update_admin_membership_plan<'a>(
        &'a self,
        command: UpdateAdminMembershipPlanCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPlanItem> {
        Box::pin(async move { update_admin_membership_plan(&self.pool, command).await })
    }

    fn delete_admin_membership_plan<'a>(
        &'a self,
        command: DeleteAdminMembershipPlanCommand,
    ) -> AdminMembershipFuture<'a, bool> {
        Box::pin(async move { delete_admin_membership_plan(&self.pool, command).await })
    }

    fn list_admin_membership_packages<'a>(
        &'a self,
        query: ListAdminMembershipPackagesQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipPackageItem>> {
        Box::pin(async move { list_admin_membership_packages(&self.pool, query).await })
    }

    fn list_admin_membership_package_groups<'a>(
        &'a self,
        query: ListAdminMembershipPackageGroupsQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipPackageGroupItem>> {
        Box::pin(async move { list_admin_membership_package_groups(&self.pool, query).await })
    }

    fn create_admin_membership_package_group<'a>(
        &'a self,
        command: CreateAdminMembershipPackageGroupCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageGroupItem> {
        Box::pin(async move { create_admin_membership_package_group(&self.pool, command).await })
    }

    fn update_admin_membership_package_group<'a>(
        &'a self,
        command: UpdateAdminMembershipPackageGroupCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageGroupItem> {
        Box::pin(async move { update_admin_membership_package_group(&self.pool, command).await })
    }

    fn delete_admin_membership_package_group<'a>(
        &'a self,
        command: DeleteAdminMembershipPackageGroupCommand,
    ) -> AdminMembershipFuture<'a, bool> {
        Box::pin(async move { delete_admin_membership_package_group(&self.pool, command).await })
    }

    fn create_admin_membership_package<'a>(
        &'a self,
        command: CreateAdminMembershipPackageCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageItem> {
        Box::pin(async move { create_admin_membership_package(&self.pool, command).await })
    }

    fn update_admin_membership_package<'a>(
        &'a self,
        command: UpdateAdminMembershipPackageCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageItem> {
        Box::pin(async move { update_admin_membership_package(&self.pool, command).await })
    }

    fn delete_admin_membership_package<'a>(
        &'a self,
        command: DeleteAdminMembershipPackageCommand,
    ) -> AdminMembershipFuture<'a, bool> {
        Box::pin(async move { delete_admin_membership_package(&self.pool, command).await })
    }

    fn list_admin_membership_members<'a>(
        &'a self,
        query: ListAdminMembershipMembersQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipMemberItem>> {
        Box::pin(async move { list_admin_membership_members(&self.pool, query).await })
    }

    fn update_admin_membership_member_status<'a>(
        &'a self,
        command: UpdateAdminMembershipMemberStatusCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipMemberItem> {
        Box::pin(async move { update_admin_membership_member_status(&self.pool, command).await })
    }

    fn list_admin_membership_entitlements<'a>(
        &'a self,
        query: ListAdminMembershipEntitlementsQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipEntitlementItem>> {
        Box::pin(async move { list_admin_membership_entitlements(&self.pool, query).await })
    }
}

async fn list_admin_membership_plans(
    pool: &PgPool,
    query: ListAdminMembershipPlansQuery,
) -> AppMembershipResult<SdkWorkPageData<AdminMembershipPlanItem>> {
    let params = MembershipListQuery {
        page: query.page,
        page_size: query.page_size,
        cursor: query.cursor.clone(),
    }
    .offset_params();
    let page_size = params.page_size;
    let offset = params.offset;
    let tenant_id = tenant_id_text(query.subject.tenant_id);

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM membership_plan p
        WHERE p.tenant_id = $1
          AND ($2 IS NULL OR p.status = $2)
        "#,
    )
    .bind(&tenant_id)
    .bind(query.status.as_deref())
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let plan_ids: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT p.id
        FROM membership_plan p
        WHERE p.tenant_id = $1
          AND ($2 IS NULL OR p.status = $2)
        ORDER BY p.rank ASC, p.plan_no ASC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(&tenant_id)
    .bind(query.status.as_deref())
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;

    if plan_ids.is_empty() {
        return Ok(offset_page(Vec::new(), total, params));
    }

    let placeholders = plan_ids
        .iter()
        .enumerate()
        .map(|(index, _)| format!("${}", index + 2))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        r#"
        SELECT
            p.id,
            p.plan_no AS plan_no,
            p.name,
            CAST(p.rank AS INTEGER) AS rank,
            p.status,
            b.id AS plan_benefit_id,
            b.benefit_code,
            CAST(b.grant_quantity AS TEXT) AS grant_quantity,
            b.usage_policy,
            d.name AS benefit_name,
            d.benefit_type,
            d.description AS benefit_description
        FROM membership_plan p
        LEFT JOIN membership_plan_version v
            ON v.plan_id = p.id
           AND v.tenant_id = p.tenant_id
           AND v.lifecycle_status = 'published'
        LEFT JOIN membership_plan_benefit b
            ON b.plan_version_id = v.id
           AND b.tenant_id = p.tenant_id
           AND b.status = 'active'
        LEFT JOIN benefit_definition d
            ON d.id = b.benefit_id
           AND d.tenant_id = b.tenant_id
        WHERE p.tenant_id = $1
          AND p.id IN ({placeholders})
        ORDER BY p.rank ASC, p.plan_no ASC, b.sort_weight ASC, b.id ASC
        "#
    );
    let mut db_query = sqlx::query(&sql).bind(&tenant_id);
    for plan_id in &plan_ids {
        db_query = db_query.bind(plan_id);
    }
    let rows = db_query.fetch_all(pool).await.map_err(sql_error)?;
    Ok(offset_page(admin_plans_from_rows(&rows), total, params))
}

async fn create_admin_membership_plan(
    pool: &PgPool,
    command: CreateAdminMembershipPlanCommand,
) -> AppMembershipResult<AdminMembershipPlanItem> {
    let plan_version_id = admin_plan_version_id(&command.plan_id);
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let organization_id = organization_id_text(command.subject.organization_id);
    sqlx::query(
        r#"
        INSERT INTO membership_plan
            (id, tenant_id, organization_id, plan_no, plan_code, name, rank, description, status, created_at, updated_at)
        VALUES
            ($1, $2, $3, $4, $4, $5, $6, NULL, $7, $8, $8)
        "#,
    )
    .bind(&command.plan_id)
    .bind(&tenant_id)
    .bind(&organization_id)
    .bind(&command.input.code)
    .bind(&command.input.name)
    .bind(command.input.rank)
    .bind(&command.input.status)
    .bind(&command.requested_at)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to create membership plan", error))?;
    upsert_admin_membership_plan_version(
        pool,
        &tenant_id,
        &organization_id,
        &command.plan_id,
        &plan_version_id,
        &command.input.name,
        &command.requested_at,
    )
    .await?;
    replace_admin_plan_benefits(
        pool,
        &tenant_id,
        &organization_id,
        &command.plan_id,
        &plan_version_id,
        command.input.benefits.as_deref().unwrap_or(&[]),
        &command.requested_at,
    )
    .await?;
    load_admin_membership_plan(pool, command.subject.tenant_id, &command.plan_id).await
}

async fn update_admin_membership_plan(
    pool: &PgPool,
    command: UpdateAdminMembershipPlanCommand,
) -> AppMembershipResult<AdminMembershipPlanItem> {
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let row = sqlx::query(
        r#"
        SELECT id
        FROM membership_plan
        WHERE tenant_id = $1
          AND (id = $2 OR plan_no = $3)
        ORDER BY CASE WHEN id = $2 THEN 0 ELSE 1 END
        LIMIT 1
        "#,
    )
    .bind(&tenant_id)
    .bind(&command.plan_id)
    .bind(&command.input.code)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?
    .ok_or_else(|| CommerceServiceError::conflict("membership plan was not found"))?;
    let plan_id = string_cell(&row, "id");
    let plan_version_id = ensure_admin_membership_plan_version(
        pool,
        &tenant_id,
        &organization_id_text(command.subject.organization_id),
        &plan_id,
        &command.input.name,
        &command.requested_at,
    )
    .await?;
    sqlx::query(
        r#"
        UPDATE membership_plan
        SET plan_no = $1,
            plan_code = $1,
            name = $2,
            rank = $3,
            status = $4,
            updated_at = $5
        WHERE id = $6
          AND tenant_id = $7
        "#,
    )
    .bind(&command.input.code)
    .bind(&command.input.name)
    .bind(command.input.rank)
    .bind(&command.input.status)
    .bind(&command.requested_at)
    .bind(&plan_id)
    .bind(&tenant_id)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to update membership plan", error))?;
    if let Some(benefits) = command.input.benefits.as_deref() {
        replace_admin_plan_benefits(
            pool,
            &tenant_id,
            &organization_id_text(command.subject.organization_id),
            &plan_id,
            &plan_version_id,
            benefits,
            &command.requested_at,
        )
        .await?;
    }
    load_admin_membership_plan(pool, command.subject.tenant_id, &plan_id).await
}

async fn delete_admin_membership_plan(
    pool: &PgPool,
    command: DeleteAdminMembershipPlanCommand,
) -> AppMembershipResult<bool> {
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let result = sqlx::query(
        r#"
        UPDATE membership_plan
        SET status = 'disabled',
            updated_at = $2
        WHERE tenant_id = $3
          AND (id = $1 OR plan_no = $1)
        "#,
    )
    .bind(&command.plan_id)
    .bind(&command.requested_at)
    .bind(&tenant_id)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to delete membership plan", error))?;
    Ok(result.rows_affected() > 0)
}

async fn list_admin_membership_packages(
    pool: &PgPool,
    query: ListAdminMembershipPackagesQuery,
) -> AppMembershipResult<SdkWorkPageData<AdminMembershipPackageItem>> {
    let params = MembershipListQuery {
        page: query.page,
        page_size: query.page_size,
        cursor: query.cursor.clone(),
    }
    .offset_params();
    let tenant_id = tenant_id_text(query.subject.tenant_id);

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM membership_package
        WHERE tenant_id = $1
          AND ($2 IS NULL OR package_group_id = $2)
          AND ($3 IS NULL OR plan_id = $3)
          AND ($4 IS NULL OR status = $4)
        "#,
    )
    .bind(&tenant_id)
    .bind(query.package_group_id.as_deref())
    .bind(query.plan_id.as_deref())
    .bind(query.status.as_deref())
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let rows = sqlx::query(
        r#"
        SELECT id, package_no, package_group_id AS package_group_id, plan_id AS plan_id, name, CAST(price_amount AS TEXT) AS price_amount,
               currency_code, duration_days AS duration_days, status
        FROM membership_package
        WHERE tenant_id = $1
          AND ($2 IS NULL OR package_group_id = $2)
          AND ($3 IS NULL OR plan_id = $3)
          AND ($4 IS NULL OR status = $4)
        ORDER BY sort_weight ASC, external_id ASC, id ASC
        LIMIT $5 OFFSET $6
        "#,
    )
    .bind(&tenant_id)
    .bind(query.package_group_id.as_deref())
    .bind(query.plan_id.as_deref())
    .bind(query.status.as_deref())
    .bind(params.page_size)
    .bind(params.offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;
    Ok(offset_page(
        rows.iter().map(map_admin_package).collect(),
        total,
        params,
    ))
}

async fn list_admin_membership_package_groups(
    pool: &PgPool,
    query: ListAdminMembershipPackageGroupsQuery,
) -> AppMembershipResult<SdkWorkPageData<AdminMembershipPackageGroupItem>> {
    let params = MembershipListQuery {
        page: query.page,
        page_size: query.page_size,
        cursor: query.cursor.clone(),
    }
    .offset_params();
    let tenant_id = tenant_id_text(query.subject.tenant_id);

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM membership_package_group
        WHERE tenant_id = $1
          AND ($2 IS NULL OR status = $2)
        "#,
    )
    .bind(&tenant_id)
    .bind(query.status.as_deref())
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let rows = sqlx::query(
        r#"
        SELECT id, group_no, name, description, billing_cycle, duration_days,
               sort_weight, status
        FROM membership_package_group
        WHERE tenant_id = $1
          AND ($2 IS NULL OR status = $2)
        ORDER BY sort_weight ASC, external_id ASC, id ASC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(&tenant_id)
    .bind(query.status.as_deref())
    .bind(params.page_size)
    .bind(params.offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;
    Ok(offset_page(
        rows.iter().map(map_admin_package_group).collect(),
        total,
        params,
    ))
}

async fn create_admin_membership_package_group(
    pool: &PgPool,
    command: CreateAdminMembershipPackageGroupCommand,
) -> AppMembershipResult<AdminMembershipPackageGroupItem> {
    let external_id = next_admin_package_group_external_id(pool).await?;
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let organization_id = organization_id_text(command.subject.organization_id);
    sqlx::query(
        r#"
        INSERT INTO membership_package_group
            (id, tenant_id, organization_id, external_id, group_no, name, description, billing_cycle, duration_days, display_channel, sort_weight, status, created_at, updated_at)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'app', $10, $11, $12, $12)
        "#,
    )
    .bind(&command.package_group_id)
    .bind(&tenant_id)
    .bind(&organization_id)
    .bind(external_id)
    .bind(&command.input.code)
    .bind(&command.input.name)
    .bind(command.input.description.as_deref())
    .bind(recurrence_cycle_from_duration(command.input.duration_days))
    .bind(command.input.duration_days)
    .bind(command.input.sort_weight)
    .bind(&command.input.status)
    .bind(&command.requested_at)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to create membership package group", error))?;
    load_admin_membership_package_group(pool, command.subject.tenant_id, &command.package_group_id)
        .await
}

async fn update_admin_membership_package_group(
    pool: &PgPool,
    command: UpdateAdminMembershipPackageGroupCommand,
) -> AppMembershipResult<AdminMembershipPackageGroupItem> {
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let package_group_id =
        package_group_external_id_for_update(pool, &tenant_id, &command.package_group_id).await?;
    sqlx::query(
        r#"
        UPDATE membership_package_group
        SET group_no = $1,
            name = $2,
            description = $3,
            billing_cycle = $4,
            duration_days = $5,
            sort_weight = $6,
            status = $7,
            updated_at = $8
        WHERE id = $9
          AND tenant_id = $10
        "#,
    )
    .bind(&command.input.code)
    .bind(&command.input.name)
    .bind(command.input.description.as_deref())
    .bind(recurrence_cycle_from_duration(command.input.duration_days))
    .bind(command.input.duration_days)
    .bind(command.input.sort_weight)
    .bind(&command.input.status)
    .bind(&command.requested_at)
    .bind(&package_group_id)
    .bind(&tenant_id)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to update membership package group", error))?;
    load_admin_membership_package_group(pool, command.subject.tenant_id, &package_group_id).await
}

async fn delete_admin_membership_package_group(
    pool: &PgPool,
    command: DeleteAdminMembershipPackageGroupCommand,
) -> AppMembershipResult<bool> {
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let result = sqlx::query(
        r#"
        UPDATE membership_package_group
        SET status = 'disabled',
            updated_at = $2
        WHERE tenant_id = $3
          AND (id = $1 OR group_no = $1)
        "#,
    )
    .bind(&command.package_group_id)
    .bind(&command.requested_at)
    .bind(&tenant_id)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to delete membership package group", error))?;
    Ok(result.rows_affected() > 0)
}

async fn create_admin_membership_package(
    pool: &PgPool,
    command: CreateAdminMembershipPackageCommand,
) -> AppMembershipResult<AdminMembershipPackageItem> {
    ensure_admin_plan_exists(pool, &command.input.plan_id).await?;
    ensure_admin_package_group_exists(pool, &command.input.package_group_id).await?;
    let external_id = next_admin_package_external_id(pool).await?;
    let sku_id = format!("{}-sku", command.package_id);
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let organization_id = organization_id_text(command.subject.organization_id);
    let plan_version_id = ensure_admin_membership_plan_version(
        pool,
        &tenant_id,
        &organization_id,
        &command.input.plan_id,
        &command.input.name,
        &command.requested_at,
    )
    .await?;
    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at)
        VALUES
            ($1, $2, $3, 'seed-product-membership', $4, $5, $5, $6, NULL, $7, 'membership_activation', 'untracked', $8, '{}', $9, $9)
        ON CONFLICT(id) DO UPDATE SET
            sku_no = excluded.sku_no,
            name = excluded.name,
            title = excluded.title,
            price_amount = excluded.price_amount,
            currency_code = excluded.currency_code,
            status = excluded.status,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&sku_id)
    .bind(&tenant_id)
    .bind(&organization_id)
    .bind(&command.input.code)
    .bind(&command.input.name)
    .bind(&command.input.price_amount)
    .bind(&command.input.currency_code)
    .bind(&command.input.status)
    .bind(&command.requested_at)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to upsert membership package sku", error))?;
    sqlx::query(
        r#"
        INSERT INTO membership_package
            (id, tenant_id, organization_id, external_id, package_no, package_group_id, plan_id, plan_version_id, sku_id, name, description, price_amount, original_price_amount, currency_code, point_amount, duration_days, recurrence_cycle, sort_weight, recommended, status, starts_at, ends_at, created_at, updated_at)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $15, $8, $9, NULL, $10, NULL, $11, 0, $12, $14, $4, 0, $13, NULL, NULL, $16, $16)
        "#,
    )
    .bind(&command.package_id)
    .bind(&tenant_id)
    .bind(&organization_id)
    .bind(external_id)
    .bind(&command.input.code)
    .bind(&command.input.package_group_id)
    .bind(&command.input.plan_id)
    .bind(&sku_id)
    .bind(&command.input.name)
    .bind(&command.input.price_amount)
    .bind(&command.input.currency_code)
    .bind(command.input.duration_days)
    .bind(&command.input.status)
    .bind(recurrence_cycle_from_duration(command.input.duration_days))
    .bind(plan_version_id)
    .bind(&command.requested_at)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to create membership package", error))?;
    load_admin_membership_package(pool, command.subject.tenant_id, &command.package_id).await
}

async fn update_admin_membership_package(
    pool: &PgPool,
    command: UpdateAdminMembershipPackageCommand,
) -> AppMembershipResult<AdminMembershipPackageItem> {
    ensure_admin_plan_exists(pool, &command.input.plan_id).await?;
    ensure_admin_package_group_exists(pool, &command.input.package_group_id).await?;
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let organization_id = organization_id_text(command.subject.organization_id);
    let current = sqlx::query(
        r#"
        SELECT sku_id
        FROM membership_package
        WHERE tenant_id = $1
          AND (id = $2 OR package_no = $2)
        LIMIT 1
        "#,
    )
    .bind(&tenant_id)
    .bind(&command.package_id)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?
    .ok_or_else(|| CommerceServiceError::conflict("membership package was not found"))?;
    let package_id = package_id_for_update(pool, &tenant_id, &command.package_id).await?;
    let sku_id = optional_string_cell(&current, "sku_id")
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| format!("{package_id}-sku"));
    let plan_version_id = ensure_admin_membership_plan_version(
        pool,
        &tenant_id,
        &organization_id,
        &command.input.plan_id,
        &command.input.name,
        &command.requested_at,
    )
    .await?;
    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at)
        VALUES
            ($1, $2, $3, 'seed-product-membership', $4, $5, $5, $6, NULL, $7, 'membership_activation', 'untracked', $8, '{}', $9, $9)
        ON CONFLICT(id) DO UPDATE SET
            sku_no = excluded.sku_no,
            name = excluded.name,
            title = excluded.title,
            price_amount = excluded.price_amount,
            currency_code = excluded.currency_code,
            status = excluded.status,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&sku_id)
    .bind(&tenant_id)
    .bind(&organization_id)
    .bind(&command.input.code)
    .bind(&command.input.name)
    .bind(&command.input.price_amount)
    .bind(&command.input.currency_code)
    .bind(&command.input.status)
    .bind(&command.requested_at)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to upsert membership package sku", error))?;
    sqlx::query(
        r#"
        UPDATE membership_package
        SET package_no = $1,
            package_group_id = $2,
            plan_id = $3,
            plan_version_id = $4,
            sku_id = $5,
            name = $6,
            price_amount = $7,
            currency_code = $8,
            duration_days = $9,
            status = $10,
            updated_at = $11
        WHERE id = $12
          AND tenant_id = $13
        "#,
    )
    .bind(&command.input.code)
    .bind(&command.input.package_group_id)
    .bind(&command.input.plan_id)
    .bind(&plan_version_id)
    .bind(&sku_id)
    .bind(&command.input.name)
    .bind(&command.input.price_amount)
    .bind(&command.input.currency_code)
    .bind(command.input.duration_days)
    .bind(&command.input.status)
    .bind(&command.requested_at)
    .bind(&package_id)
    .bind(&tenant_id)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to update membership package", error))?;
    load_admin_membership_package(pool, command.subject.tenant_id, &package_id).await
}

async fn delete_admin_membership_package(
    pool: &PgPool,
    command: DeleteAdminMembershipPackageCommand,
) -> AppMembershipResult<bool> {
    let tenant_id = tenant_id_text(command.subject.tenant_id);
    let result = sqlx::query(
        r#"
        UPDATE membership_package
        SET status = 'disabled',
            updated_at = $2
        WHERE tenant_id = $3
          AND (id = $1 OR package_no = $1)
        "#,
    )
    .bind(&command.package_id)
    .bind(&command.requested_at)
    .bind(&tenant_id)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to delete membership package", error))?;
    Ok(result.rows_affected() > 0)
}

async fn list_admin_membership_members(
    pool: &PgPool,
    query: ListAdminMembershipMembersQuery,
) -> AppMembershipResult<SdkWorkPageData<AdminMembershipMemberItem>> {
    let params = MembershipListQuery {
        page: query.page,
        page_size: query.page_size,
        cursor: query.cursor.clone(),
    }
    .offset_params();

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM membership_subscription m
        LEFT JOIN membership_plan l ON l.id = m.plan_id
        WHERE m.tenant_id = CAST($1 AS TEXT)
          AND ($2 IS NULL OR m.organization_id IS NULL OR m.organization_id = CAST($2 AS TEXT) OR (CAST($2 AS TEXT) != '0' AND m.organization_id = '0'))
          AND ($3 IS NULL OR m.owner_user_id = $3 OR m.subject_id = $3)
          AND ($4 IS NULL OR m.plan_id = $4 OR l.plan_no = $4)
          AND ($5 IS NULL OR m.status = $5)
        "#,
    )
    .bind(query.subject.tenant_id)
    .bind(query.subject.organization_id)
    .bind(query.user_id.as_deref())
    .bind(query.plan_id.as_deref())
    .bind(query.status.as_deref())
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let rows = sqlx::query(
        r#"
        SELECT m.id, m.owner_user_id, m.status, CAST(m.starts_at AS TEXT) AS starts_at,
               CAST(m.expires_at AS TEXT) AS expires_at, l.plan_no AS plan_no, m.plan_id AS plan_id
        FROM membership_subscription m
        LEFT JOIN membership_plan l ON l.id = m.plan_id
        WHERE m.tenant_id = CAST($1 AS TEXT)
          AND ($2 IS NULL OR m.organization_id IS NULL OR m.organization_id = CAST($2 AS TEXT) OR (CAST($2 AS TEXT) != '0' AND m.organization_id = '0'))
          AND ($3 IS NULL OR m.owner_user_id = $3 OR m.subject_id = $3)
          AND ($4 IS NULL OR m.plan_id = $4 OR l.plan_no = $4)
          AND ($5 IS NULL OR m.status = $5)
        ORDER BY m.created_at DESC, m.id DESC
        LIMIT $6 OFFSET $7
        "#,
    )
    .bind(query.subject.tenant_id)
    .bind(query.subject.organization_id)
    .bind(query.user_id.as_deref())
    .bind(query.plan_id.as_deref())
    .bind(query.status.as_deref())
    .bind(params.page_size)
    .bind(params.offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;
    Ok(offset_page(
        rows.iter().map(map_admin_membership).collect(),
        total,
        params,
    ))
}

async fn update_admin_membership_member_status(
    pool: &PgPool,
    command: UpdateAdminMembershipMemberStatusCommand,
) -> AppMembershipResult<AdminMembershipMemberItem> {
    let result = sqlx::query(
        r#"
        UPDATE membership_subscription
        SET status = $1,
            updated_at = $2
        WHERE tenant_id = CAST($3 AS TEXT)
          AND id = $4
        "#,
    )
    .bind(&command.status)
    .bind(&command.requested_at)
    .bind(command.subject.tenant_id)
    .bind(&command.membership_id)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to update membership membership status", error))?;
    if result.rows_affected() == 0 {
        return Err(CommerceServiceError::conflict(
            "membership membership was not found",
        ));
    }
    load_admin_membership(pool, command.subject.tenant_id, &command.membership_id).await
}

async fn list_admin_membership_entitlements(
    pool: &PgPool,
    query: ListAdminMembershipEntitlementsQuery,
) -> AppMembershipResult<SdkWorkPageData<AdminMembershipEntitlementItem>> {
    let params = MembershipListQuery {
        page: query.page,
        page_size: query.page_size,
        cursor: query.cursor.clone(),
    }
    .offset_params();
    let status_filter = query.status.as_deref();

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM entitlement_account a
        JOIN benefit_definition d
          ON d.id = a.benefit_id
        LEFT JOIN entitlement_grant g
          ON g.benefit_id = a.benefit_id
         AND g.subject_type = a.subject_type
         AND g.subject_id = a.subject_id
         AND g.source_type = 'membership_subscription'
         AND g.tenant_id = a.tenant_id
        LEFT JOIN membership_subscription m
          ON m.id = g.source_id
         AND m.tenant_id = a.tenant_id
        WHERE a.tenant_id = CAST($1 AS TEXT)
          AND ($2 IS NULL OR m.plan_id = $2)
          AND ($3 IS NULL OR g.source_id = $3)
          AND ($4 IS NULL OR (
            CASE
              WHEN CAST(a.total_granted AS REAL) > 0
               AND CAST(a.total_used AS REAL) >= CAST(a.total_granted AS REAL) THEN 'exhausted'
              ELSE 'active'
            END
          ) = $4)
        "#,
    )
    .bind(query.subject.tenant_id)
    .bind(query.plan_id.as_deref())
    .bind(query.membership_id.as_deref())
    .bind(status_filter)
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let rows = sqlx::query(
        r#"
        SELECT a.id, d.benefit_code AS entitlement_code, g.source_id AS membership_id,
               a.total_granted AS granted_quantity, a.total_used AS used_quantity,
               m.plan_id AS plan_id
        FROM entitlement_account a
        JOIN benefit_definition d
          ON d.id = a.benefit_id
        LEFT JOIN entitlement_grant g
          ON g.benefit_id = a.benefit_id
         AND g.subject_type = a.subject_type
         AND g.subject_id = a.subject_id
         AND g.source_type = 'membership_subscription'
         AND g.tenant_id = a.tenant_id
        LEFT JOIN membership_subscription m
          ON m.id = g.source_id
         AND m.tenant_id = a.tenant_id
        WHERE a.tenant_id = CAST($1 AS TEXT)
          AND ($2 IS NULL OR m.plan_id = $2)
          AND ($3 IS NULL OR g.source_id = $3)
          AND ($4 IS NULL OR (
            CASE
              WHEN CAST(a.total_granted AS REAL) > 0
               AND CAST(a.total_used AS REAL) >= CAST(a.total_granted AS REAL) THEN 'exhausted'
              ELSE 'active'
            END
          ) = $4)
        ORDER BY a.created_at DESC, a.id DESC
        LIMIT $5 OFFSET $6
        "#,
    )
    .bind(query.subject.tenant_id)
    .bind(query.plan_id.as_deref())
    .bind(query.membership_id.as_deref())
    .bind(status_filter)
    .bind(params.page_size)
    .bind(params.offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;
    Ok(offset_page(
        rows.iter().map(map_admin_entitlement).collect(),
        total,
        params,
    ))
}

async fn load_admin_membership_plan(
    pool: &PgPool,
    tenant_id: i64,
    plan_id: &str,
) -> AppMembershipResult<AdminMembershipPlanItem> {
    let tenant_id = tenant_id_text(tenant_id);
    let rows = sqlx::query(
        r#"
        SELECT
            p.id,
            p.plan_no AS plan_no,
            p.name,
            CAST(p.rank AS INTEGER) AS rank,
            p.status,
            b.id AS plan_benefit_id,
            b.benefit_code,
            CAST(b.grant_quantity AS TEXT) AS grant_quantity,
            b.usage_policy,
            d.name AS benefit_name,
            d.benefit_type,
            d.description AS benefit_description
        FROM membership_plan p
        LEFT JOIN membership_plan_version v
            ON v.plan_id = p.id
           AND v.tenant_id = p.tenant_id
           AND v.lifecycle_status = 'published'
        LEFT JOIN membership_plan_benefit b
            ON b.plan_version_id = v.id
           AND b.tenant_id = p.tenant_id
           AND b.status = 'active'
        LEFT JOIN benefit_definition d
            ON d.id = b.benefit_id
           AND d.tenant_id = b.tenant_id
        WHERE p.tenant_id = $1
          AND (p.id = $2 OR p.plan_no = $2)
        ORDER BY b.sort_weight ASC, b.id ASC
        "#,
    )
    .bind(&tenant_id)
    .bind(plan_id)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;
    admin_plans_from_rows(&rows)
        .into_iter()
        .next()
        .ok_or_else(|| CommerceServiceError::conflict("membership plan was not found"))
}

async fn load_admin_membership_package(
    pool: &PgPool,
    tenant_id: i64,
    package_id: &str,
) -> AppMembershipResult<AdminMembershipPackageItem> {
    let tenant_id = tenant_id_text(tenant_id);
    let row = sqlx::query(
        r#"
        SELECT id, package_no, package_group_id AS package_group_id, plan_id AS plan_id, name, CAST(price_amount AS TEXT) AS price_amount,
               currency_code, duration_days AS duration_days, status
        FROM membership_package
        WHERE tenant_id = $1
          AND (id = $2 OR package_no = $2)
        LIMIT 1
        "#,
    )
    .bind(&tenant_id)
    .bind(package_id)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?
    .ok_or_else(|| CommerceServiceError::conflict("membership package was not found"))?;
    Ok(map_admin_package(&row))
}

async fn load_admin_membership_package_group(
    pool: &PgPool,
    tenant_id: i64,
    package_group_id: &str,
) -> AppMembershipResult<AdminMembershipPackageGroupItem> {
    let tenant_id = tenant_id_text(tenant_id);
    let row = sqlx::query(
        r#"
        SELECT id, group_no, name, description, billing_cycle, duration_days,
               sort_weight, status
        FROM membership_package_group
        WHERE tenant_id = $1
          AND (id = $2 OR group_no = $2)
        LIMIT 1
        "#,
    )
    .bind(&tenant_id)
    .bind(package_group_id)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?
    .ok_or_else(|| CommerceServiceError::conflict("membership package group was not found"))?;
    Ok(map_admin_package_group(&row))
}

async fn load_admin_membership(
    pool: &PgPool,
    tenant_id: i64,
    membership_id: &str,
) -> AppMembershipResult<AdminMembershipMemberItem> {
    let row = sqlx::query(
        r#"
        SELECT m.id, m.owner_user_id, m.status, CAST(m.starts_at AS TEXT) AS starts_at,
               CAST(m.expires_at AS TEXT) AS expires_at, l.plan_no AS plan_no, m.plan_id AS plan_id
        FROM membership_subscription m
        LEFT JOIN membership_plan l ON l.id = m.plan_id
        WHERE m.tenant_id = CAST($1 AS TEXT) AND m.id = $2
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(membership_id)
    .fetch_optional(pool)
    .await
    .or_else(none_when_read_model_is_missing)?
    .ok_or_else(|| CommerceServiceError::conflict("membership membership was not found"))?;
    Ok(map_admin_membership(&row))
}

async fn ensure_admin_plan_exists(pool: &PgPool, plan_id: &str) -> AppMembershipResult<()> {
    let exists: Option<i64> =
        sqlx::query_scalar("SELECT 1 FROM membership_plan WHERE id = $1 OR plan_no = $1 LIMIT 1")
            .bind(plan_id)
            .fetch_optional(pool)
            .await
            .map_err(sql_error)?;
    exists
        .map(|_| ())
        .ok_or_else(|| CommerceServiceError::conflict("membership target plan was not found"))
}

async fn ensure_admin_membership_plan_version(
    pool: &PgPool,
    tenant_id: &str,
    organization_id: &str,
    plan_id_or_no: &str,
    title: &str,
    requested_at: &str,
) -> AppMembershipResult<String> {
    let row = sqlx::query(
        r#"
        SELECT id
        FROM membership_plan
        WHERE tenant_id = $1
          AND (id = $2 OR plan_no = $2)
        ORDER BY CASE WHEN id = $2 THEN 0 ELSE 1 END
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(plan_id_or_no)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?
    .ok_or_else(|| CommerceServiceError::conflict("membership target plan was not found"))?;
    let plan_id = string_cell(&row, "id");
    let existing: Option<String> = sqlx::query_scalar(
        r#"
        SELECT id
        FROM membership_plan_version
        WHERE plan_id = $1
          AND lifecycle_status = 'published'
        ORDER BY version_no DESC, id DESC
        LIMIT 1
        "#,
    )
    .bind(&plan_id)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?;
    if let Some(version_id) = existing {
        return Ok(version_id);
    }
    let version_id = admin_plan_version_id(&plan_id);
    upsert_admin_membership_plan_version(
        pool,
        tenant_id,
        organization_id,
        &plan_id,
        &version_id,
        title,
        requested_at,
    )
    .await?;
    Ok(version_id)
}

async fn upsert_admin_membership_plan_version(
    pool: &PgPool,
    tenant_id: &str,
    organization_id: &str,
    plan_id: &str,
    plan_version_id: &str,
    title: &str,
    requested_at: &str,
) -> AppMembershipResult<()> {
    sqlx::query(
        r#"
        INSERT INTO membership_plan_version
            (id, tenant_id, organization_id, plan_id, version_no, title, description, lifecycle_status, effective_from, effective_to, published_at, created_at, updated_at)
        VALUES
            ($1, $2, $3, $4, 'v1', $5, NULL, 'published', $6, NULL, $6, $6, $6)
        ON CONFLICT(tenant_id, plan_id, version_no) DO UPDATE SET
            id = excluded.id,
            title = excluded.title,
            lifecycle_status = excluded.lifecycle_status,
            published_at = excluded.published_at,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(plan_version_id)
    .bind(tenant_id)
    .bind(organization_id)
    .bind(plan_id)
    .bind(title)
    .bind(requested_at)
    .execute(pool)
    .await
    .map_err(|error| store_error("failed to upsert membership plan version", error))?;
    Ok(())
}

async fn replace_admin_plan_benefits(
    pool: &PgPool,
    tenant_id: &str,
    organization_id: &str,
    plan_id: &str,
    plan_version_id: &str,
    benefits: &[AppMembershipBenefitItem],
    requested_at: &str,
) -> AppMembershipResult<()> {
    sqlx::query("DELETE FROM membership_plan_benefit WHERE plan_version_id = $1")
        .bind(plan_version_id)
        .execute(pool)
        .await
        .map_err(|error| store_error("failed to clear membership plan benefits", error))?;
    for (index, benefit) in benefits.iter().enumerate() {
        let benefit_code = admin_benefit_code(benefit, index + 1);
        let benefit_id = benefit_definition_id_for_code(&benefit_code);
        let benefit_type = benefit
            .r#type
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("quota");
        sqlx::query(
            r#"
            INSERT INTO benefit_definition
                (id, tenant_id, organization_id, benefit_code, name, benefit_type, value_unit, measurement_type, description, status, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, 'count', 'counter', $7, 'active', $8, $8)
            ON CONFLICT(tenant_id, organization_id, benefit_code) DO UPDATE SET
                id = excluded.id,
                name = excluded.name,
                benefit_type = excluded.benefit_type,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&benefit_id)
        .bind(tenant_id)
        .bind(organization_id)
        .bind(&benefit_code)
        .bind(&benefit.name)
        .bind(benefit_type)
        .bind(benefit.description.as_deref())
        .bind(requested_at)
        .execute(pool)
        .await
        .map_err(|error| store_error("failed to upsert benefit definition", error))?;

        let plan_benefit_id = format!("{plan_version_id}-benefit-{}", index + 1);
        let grant_quantity = benefit.usage_limit.unwrap_or(0).max(0).to_string();
        sqlx::query(
            r#"
            INSERT INTO membership_plan_benefit
                (id, tenant_id, organization_id, plan_id, plan_version_id, benefit_id, benefit_code, grant_quantity, grant_period, reset_policy, usage_policy, sort_weight, status, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, 'membership_period', NULL, $9, $10, 'active', $11, $11)
            "#,
        )
        .bind(&plan_benefit_id)
        .bind(tenant_id)
        .bind(organization_id)
        .bind(plan_id)
        .bind(plan_version_id)
        .bind(&benefit_id)
        .bind(&benefit_code)
        .bind(grant_quantity)
        .bind(benefit_type)
        .bind((index + 1) as i64)
        .bind(requested_at)
        .execute(pool)
        .await
        .map_err(|error| store_error("failed to upsert membership plan benefit", error))?;
    }
    Ok(())
}

async fn ensure_admin_package_group_exists(
    pool: &PgPool,
    package_group_id: &str,
) -> AppMembershipResult<()> {
    let exists: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM membership_package_group WHERE id = $1 OR group_no = $1 LIMIT 1",
    )
    .bind(package_group_id)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?;
    exists
        .map(|_| ())
        .ok_or_else(|| CommerceServiceError::conflict("membership package group was not found"))
}

async fn next_admin_package_external_id(pool: &PgPool) -> AppMembershipResult<i64> {
    let max_id: Option<i64> = sqlx::query_scalar("SELECT MAX(external_id) FROM membership_package")
        .fetch_one(pool)
        .await
        .map_err(sql_error)?;
    Ok(max_id.unwrap_or(0) + 1)
}

async fn next_admin_package_group_external_id(pool: &PgPool) -> AppMembershipResult<i64> {
    let max_id: Option<i64> =
        sqlx::query_scalar("SELECT MAX(external_id) FROM membership_package_group")
            .fetch_one(pool)
            .await
            .map_err(sql_error)?;
    Ok(max_id.unwrap_or(0) + 1)
}

async fn package_id_for_update(
    pool: &PgPool,
    tenant_id: &str,
    value: &str,
) -> AppMembershipResult<String> {
    let row = sqlx::query(
        r#"
        SELECT id
        FROM membership_package
        WHERE tenant_id = $1
          AND (id = $2 OR package_no = $2)
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(value)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?
    .ok_or_else(|| CommerceServiceError::conflict("membership package was not found"))?;
    Ok(string_cell(&row, "id"))
}

async fn package_group_external_id_for_update(
    pool: &PgPool,
    tenant_id: &str,
    value: &str,
) -> AppMembershipResult<String> {
    let row = sqlx::query(
        r#"
        SELECT id
        FROM membership_package_group
        WHERE tenant_id = $1
          AND (id = $2 OR group_no = $2)
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(value)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?
    .ok_or_else(|| CommerceServiceError::conflict("membership package group was not found"))?;
    Ok(string_cell(&row, "id"))
}

fn admin_plans_from_rows(rows: &[sqlx::postgres::PgRow]) -> Vec<AdminMembershipPlanItem> {
    let mut grouped = BTreeMap::<String, AdminMembershipPlanItem>::new();
    for row in rows.iter() {
        let id = string_cell(row, "id");
        let code = string_cell(row, "plan_no");
        let rank = integer_cell(row, "rank");
        let plan = grouped
            .entry(id.clone())
            .or_insert_with(|| AdminMembershipPlanItem {
                id,
                code: code.clone(),
                name: string_cell(row, "name"),
                rank: if rank == 0 {
                    plan_rank_from_code(&code)
                } else {
                    rank
                },
                benefits: Vec::new(),
                status: string_cell(row, "status"),
            });
        if let Some(benefit) = plan_benefit_from_row(row, (plan.benefits.len() + 1) as i64) {
            if !plan
                .benefits
                .iter()
                .any(|item| item.benefit_key.as_deref() == benefit.benefit_key.as_deref())
            {
                plan.benefits.push(benefit);
            }
        }
    }
    let mut plans = grouped.into_values().collect::<Vec<_>>();
    plans.sort_by_key(|plan| (plan.rank, plan.code.clone(), plan.id.clone()));
    plans
}

fn map_admin_package(row: &sqlx::postgres::PgRow) -> AdminMembershipPackageItem {
    AdminMembershipPackageItem {
        id: string_cell(row, "id"),
        code: string_cell(row, "package_no"),
        package_group_id: string_cell(row, "package_group_id"),
        plan_id: string_cell(row, "plan_id"),
        name: string_cell(row, "name"),
        price_amount: decimal_string(
            &string_cell(row, "price_amount"),
            "membership package price",
        )
        .unwrap_or_else(|_| string_cell(row, "price_amount")),
        currency_code: string_cell(row, "currency_code"),
        duration_days: integer_cell(row, "duration_days"),
        status: string_cell(row, "status"),
    }
}

fn map_admin_package_group(row: &sqlx::postgres::PgRow) -> AdminMembershipPackageGroupItem {
    AdminMembershipPackageGroupItem {
        id: string_cell(row, "id"),
        code: string_cell(row, "group_no"),
        name: string_cell(row, "name"),
        description: optional_string_cell(row, "description"),
        billing_cycle: string_cell(row, "billing_cycle"),
        duration_days: integer_cell(row, "duration_days"),
        sort_weight: integer_cell(row, "sort_weight"),
        status: string_cell(row, "status"),
    }
}

fn map_admin_membership(row: &sqlx::postgres::PgRow) -> AdminMembershipMemberItem {
    AdminMembershipMemberItem {
        id: string_cell(row, "id"),
        owner_user_id: string_cell(row, "owner_user_id"),
        plan_code: optional_string_cell(row, "plan_no")
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| string_cell(row, "plan_id")),
        status: admin_membership_status(&string_cell(row, "status")).to_owned(),
        started_at: string_cell(row, "starts_at"),
        expires_at: string_cell(row, "expires_at"),
    }
}

fn map_admin_entitlement(row: &sqlx::postgres::PgRow) -> AdminMembershipEntitlementItem {
    let granted = parse_points_amount(&string_cell(row, "granted_quantity"));
    let used = parse_points_amount(&string_cell(row, "used_quantity"));
    AdminMembershipEntitlementItem {
        id: string_cell(row, "id"),
        code: string_cell(row, "entitlement_code"),
        plan_id: string_cell(row, "plan_id"),
        membership_id: string_cell(row, "membership_id"),
        quota: granted.to_string(),
        status: if granted > 0 && used >= granted {
            "exhausted".to_owned()
        } else {
            "active".to_owned()
        },
    }
}

fn admin_membership_status(status: &str) -> &'static str {
    match status.trim().to_ascii_lowercase().as_str() {
        "active" => "active",
        "expired" => "expired",
        "suspended" => "suspended",
        "cancelled" => "cancelled",
        _ => "inactive",
    }
}

fn recurrence_cycle_from_duration(duration_days: i64) -> &'static str {
    match duration_days {
        365.. => "year",
        30..=364 => "month",
        7..=29 => "week",
        _ => "day",
    }
}

async fn load_info(
    pool: &PgPool,
    subject: Option<AppMembershipSubject>,
) -> AppMembershipResult<AppMembershipInfoResponse> {
    let membership = match subject {
        Some(subject) => load_current_membership(pool, subject).await?,
        None => None,
    };
    let points = load_points_balance(pool, subject).await?;
    match membership {
        Some(membership) => Ok(AppMembershipInfoResponse {
            plan_rank: membership.rank,
            plan_name: membership.plan_name,
            membership_status: membership.status,
            started_at: Some(membership.starts_at),
            expires_at: Some(membership.expires_at.clone()),
            remaining_days: remaining_days(&membership.expires_at),
            total_days: None,
            total_spent: Some(membership.total_spent),
            points: Some(points.available_points),
            growth_value: Some(points.available_points),
            upgrade_growth_value: None,
            benefits: membership.benefits,
        }),
        None => {
            let benefits = load_benefits_list(pool, subject, Some(0))
                .await
                .unwrap_or_default();
            Ok(AppMembershipInfoResponse {
                plan_rank: 0,
                plan_name: "Free".to_owned(),
                membership_status: "free".to_owned(),
                started_at: None,
                expires_at: None,
                remaining_days: None,
                total_days: None,
                total_spent: Some("0.00".to_owned()),
                points: Some(points.available_points),
                growth_value: Some(points.available_points),
                upgrade_growth_value: None,
                benefits,
            })
        }
    }
}

async fn load_status(
    pool: &PgPool,
    subject: Option<AppMembershipSubject>,
) -> AppMembershipResult<AppMembershipStatusResponse> {
    let membership = match subject {
        Some(subject) => load_current_membership(pool, subject).await?,
        None => None,
    };
    let points = load_points_balance(pool, subject).await?;
    Ok(AppMembershipStatusResponse {
        active: membership
            .as_ref()
            .map(|item| item.rank > 0 && item.status != "expired")
            .unwrap_or(false),
        plan_rank: membership.as_ref().map(|item| item.rank).unwrap_or(0),
        expires_at: membership.map(|item| item.expires_at),
        point_balance: Some(points.available_points),
    })
}

async fn load_benefits_list(
    pool: &PgPool,
    subject: Option<AppMembershipSubject>,
    plan_id: Option<i64>,
) -> AppMembershipResult<Vec<AppMembershipBenefitItem>> {
    let rank = resolve_plan_rank(pool, subject, plan_id).await?;
    Ok(load_stored_plan_by_rank(pool, rank)
        .await?
        .map(|plan| plan.benefits)
        .unwrap_or_default())
}

async fn load_benefits_page(
    pool: &PgPool,
    subject: Option<AppMembershipSubject>,
    plan_id: Option<i64>,
    query: MembershipListQuery,
) -> AppMembershipResult<SdkWorkPageData<AppMembershipBenefitItem>> {
    let rank = resolve_plan_rank(pool, subject, plan_id).await?;
    let params = query.offset_params();
    let page_size = params.page_size;
    let offset = params.offset;

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM membership_plan p
        JOIN membership_plan_version v
            ON v.plan_id = p.id
           AND v.tenant_id = p.tenant_id
           AND v.lifecycle_status = 'published'
        JOIN membership_plan_benefit b
            ON b.plan_version_id = v.id
           AND b.tenant_id = p.tenant_id
           AND b.status = 'active'
        WHERE (p.tenant_id = '100001' OR p.tenant_id IS NULL)
          AND (p.organization_id = '0' OR p.organization_id IS NULL)
          AND p.status = 'active'
          AND CAST(p.rank AS INTEGER) = $1
        "#,
    )
    .bind(rank)
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let rows = sqlx::query(
        r#"
        SELECT
            b.id AS plan_benefit_id,
            b.benefit_code,
            CAST(b.grant_quantity AS TEXT) AS grant_quantity,
            b.usage_policy,
            d.name AS benefit_name,
            d.benefit_type,
            d.description AS benefit_description
        FROM membership_plan p
        JOIN membership_plan_version v
            ON v.plan_id = p.id
           AND v.tenant_id = p.tenant_id
           AND v.lifecycle_status = 'published'
        JOIN membership_plan_benefit b
            ON b.plan_version_id = v.id
           AND b.tenant_id = p.tenant_id
           AND b.status = 'active'
        LEFT JOIN benefit_definition d
            ON d.id = b.benefit_id
           AND d.tenant_id = b.tenant_id
        WHERE (p.tenant_id = '100001' OR p.tenant_id IS NULL)
          AND (p.organization_id = '0' OR p.organization_id IS NULL)
          AND p.status = 'active'
          AND CAST(p.rank AS INTEGER) = $1
        ORDER BY b.sort_weight ASC, b.id ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(rank)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;
    let items = rows
        .iter()
        .enumerate()
        .filter_map(|(index, row)| plan_benefit_from_row(row, (index + 1) as i64))
        .collect();
    Ok(offset_page(items, total, params))
}

async fn resolve_plan_rank(
    pool: &PgPool,
    subject: Option<AppMembershipSubject>,
    plan_id: Option<i64>,
) -> AppMembershipResult<i64> {
    match plan_id {
        Some(value) => Ok(value),
        None => match subject {
            Some(subject) => Ok(load_current_membership(pool, subject)
                .await?
                .map(|membership| membership.rank)
                .unwrap_or(0)),
            None => Ok(0),
        },
    }
}

async fn load_plans_page(
    pool: &PgPool,
    query: MembershipListQuery,
) -> AppMembershipResult<SdkWorkPageData<AppMembershipPlanItem>> {
    let params = query.offset_params();
    let page_size = params.page_size;
    let offset = params.offset;

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM membership_plan p
        WHERE (p.tenant_id = '100001' OR p.tenant_id IS NULL)
          AND (p.organization_id = '0' OR p.organization_id IS NULL)
          AND p.status = 'active'
        "#,
    )
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let plan_ids: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT p.id
        FROM membership_plan p
        WHERE (p.tenant_id = '100001' OR p.tenant_id IS NULL)
          AND (p.organization_id = '0' OR p.organization_id IS NULL)
          AND p.status = 'active'
        ORDER BY p.rank ASC, p.plan_no ASC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;

    if plan_ids.is_empty() {
        return Ok(offset_page(Vec::new(), total, params));
    }

    let placeholders = plan_ids
        .iter()
        .enumerate()
        .map(|(index, _)| format!("${}", index + 1))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        r#"
        SELECT
            p.id,
            p.plan_no AS plan_no,
            p.name,
            CAST(p.rank AS INTEGER) AS rank,
            p.description,
            b.id AS plan_benefit_id,
            b.benefit_code,
            CAST(b.grant_quantity AS TEXT) AS grant_quantity,
            b.usage_policy,
            d.name AS benefit_name,
            d.benefit_type,
            d.description AS benefit_description
        FROM membership_plan p
        LEFT JOIN membership_plan_version v
            ON v.plan_id = p.id
           AND v.tenant_id = p.tenant_id
           AND v.lifecycle_status = 'published'
        LEFT JOIN membership_plan_benefit b
            ON b.plan_version_id = v.id
           AND b.tenant_id = p.tenant_id
           AND b.status = 'active'
        LEFT JOIN benefit_definition d
            ON d.id = b.benefit_id
           AND d.tenant_id = b.tenant_id
        WHERE p.id IN ({placeholders})
        ORDER BY p.rank ASC, p.plan_no ASC, b.sort_weight ASC, b.id ASC
        "#
    );
    let mut db_query = sqlx::query(&sql);
    for plan_id in &plan_ids {
        db_query = db_query.bind(plan_id);
    }
    let rows = db_query.fetch_all(pool).await.map_err(sql_error)?;
    Ok(offset_page(
        plan_items(stored_plans_from_rows(&rows)),
        total,
        params,
    ))
}

async fn load_stored_plan_by_rank(
    pool: &PgPool,
    rank: i64,
) -> AppMembershipResult<Option<StoredMembershipPlan>> {
    let rows = sqlx::query(LOAD_MEMBERSHIP_PLAN_BY_RANK)
        .bind(rank)
        .fetch_all(pool)
        .await
        .map_err(sql_error)?;
    Ok(stored_plans_from_rows(&rows).into_iter().next())
}

fn stored_plans_from_rows(rows: &[sqlx::postgres::PgRow]) -> Vec<StoredMembershipPlan> {
    let mut grouped = BTreeMap::<String, StoredMembershipPlan>::new();
    for row in rows.iter() {
        let id_text = string_cell(row, "id");
        let plan_no = string_cell(row, "plan_no");
        let rank = integer_cell(row, "rank");
        let plan = grouped
            .entry(id_text.clone())
            .or_insert_with(|| StoredMembershipPlan {
                id: rank,
                storage_id: id_text,
                plan_no: plan_no.clone(),
                item: AppMembershipPlanItem {
                    id: rank,
                    name: string_cell(row, "name"),
                    rank,
                    required_points: Some(plan_required_points(&plan_no)),
                    description: optional_string_cell(row, "description"),
                    icon: None,
                    badge: Some(plan_badge(&plan_no).to_owned()),
                },
                benefits: Vec::new(),
                rank,
            });
        if let Some(benefit) = plan_benefit_from_row(row, (plan.benefits.len() + 1) as i64) {
            if !plan
                .benefits
                .iter()
                .any(|item| item.benefit_key.as_deref() == benefit.benefit_key.as_deref())
            {
                plan.benefits.push(benefit);
            }
        }
    }
    let mut plans = grouped.into_values().collect::<Vec<_>>();
    plans.sort_by_key(|plan| (plan.rank, plan.id));
    plans
}

fn plan_benefit_from_row(
    row: &sqlx::postgres::PgRow,
    fallback_id: i64,
) -> Option<AppMembershipBenefitItem> {
    let benefit_code = optional_string_cell(row, "benefit_code")?;
    let grant_quantity =
        optional_string_cell(row, "grant_quantity").map(|value| parse_points_amount(&value));
    Some(AppMembershipBenefitItem {
        id: numeric_suffix(&string_cell(row, "plan_benefit_id")).unwrap_or(fallback_id),
        name: optional_string_cell(row, "benefit_name").unwrap_or_else(|| benefit_code.clone()),
        benefit_key: Some(benefit_code),
        r#type: optional_string_cell(row, "benefit_type")
            .or_else(|| optional_string_cell(row, "usage_policy")),
        description: optional_string_cell(row, "benefit_description"),
        icon: None,
        claimed: false,
        usage_limit: grant_quantity,
        used_count: Some(0),
    })
}

fn plan_required_points(plan_no: &str) -> i64 {
    match plan_no {
        "pro" => 5_000,
        "max" => 12_000,
        "vip" => 20_000,
        _ => 0,
    }
}

fn plan_badge(plan_no: &str) -> &'static str {
    match plan_no {
        "pro" => "Pro",
        "max" => "Max",
        "vip" => "VIP",
        _ => "Free",
    }
}

async fn load_package_rows(
    pool: &PgPool,
    package_group_id: Option<i64>,
    plan_id: Option<i64>,
    query: MembershipListQuery,
    recommended_only: bool,
) -> AppMembershipResult<SdkWorkPageData<AppMembershipPackageItem>> {
    let params = query.offset_params();
    let page_size = params.page_size as usize;
    let offset = params.offset as usize;
    let limit = (page_size + 1) as i64;

    let mut sql = String::from(LOAD_MEMBERSHIP_PACKAGES_BASE);
    if package_group_id.is_some() {
        sql.push_str("  AND g.external_id = ?\n");
    }
    if plan_id.is_some() {
        sql.push_str("  AND l.rank = ?\n");
    }
    if recommended_only {
        sql.push_str("  AND p.recommended != 0\n");
    }
    sql.push_str("ORDER BY g.sort_weight ASC, p.sort_weight ASC, p.external_id ASC\n");
    sql.push_str("LIMIT ? OFFSET ?\n");

    let mut db_query = sqlx::query(&sql);
    if let Some(group_id) = package_group_id {
        db_query = db_query.bind(group_id);
    }
    if let Some(rank) = plan_id {
        db_query = db_query.bind(rank);
    }
    db_query = db_query.bind(limit).bind(offset as i64);
    let rows = db_query.fetch_all(pool).await.map_err(sql_error)?;
    let packages: Vec<AppMembershipPackageItem> = rows
        .iter()
        .filter_map(map_package)
        .map(|package| package.item)
        .collect();
    Ok(bounded_sql_page(packages, page_size, offset))
}

async fn load_package_by_id(
    pool: &PgPool,
    package_id: i64,
) -> AppMembershipResult<Option<AppMembershipPackageItem>> {
    let row = sqlx::query(LOAD_MEMBERSHIP_PACKAGE_BY_ID)
        .bind(package_id)
        .fetch_optional(pool)
        .await
        .map_err(sql_error)?;
    Ok(row.as_ref().and_then(map_package).map(|package| package.item))
}

fn map_package(row: &sqlx::postgres::PgRow) -> Option<ParsedMembershipPackage> {
    let id = integer_cell(row, "external_id");
    let price = decimal_string(
        &string_cell(row, "price_amount"),
        "membership package price",
    )
    .ok()?;
    let original_price = optional_string_cell(row, "original_price_amount")
        .filter(|value| !value.trim().is_empty())
        .and_then(|value| decimal_string(&value, "membership package original price").ok());
    map_membership_package_record(
        id,
        string_cell(row, "name"),
        optional_string_cell(row, "description"),
        price,
        original_price,
        integer_cell(row, "point_amount"),
        integer_cell(row, "duration_days"),
        optional_string_cell(row, "plan_name"),
        integer_cell(row, "sort_weight"),
        integer_cell(row, "recommended") != 0,
        &string_cell(row, "tags_json"),
        integer_cell(row, "group_external_id"),
        string_cell(row, "group_name"),
        optional_string_cell(row, "group_description"),
        integer_cell(row, "group_sort_weight"),
        optional_string_cell(row, "plan_no"),
        integer_cell(row, "rank"),
        optional_string_cell(row, "sku_id"),
    )
}

fn plan_items(plans: Vec<StoredMembershipPlan>) -> Vec<AppMembershipPlanItem> {
    plans.into_iter().map(|plan| plan.item).collect()
}

async fn load_package_groups_page(
    pool: &PgPool,
    plan_id: Option<i64>,
    recommended_only: bool,
    query: MembershipListQuery,
) -> AppMembershipResult<SdkWorkPageData<AppMembershipPackageGroupItem>> {
    let params = query.offset_params();
    let page_size = params.page_size;
    let offset = params.offset;

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM membership_package_group g
        WHERE (g.tenant_id = '100001' OR g.tenant_id IS NULL)
          AND (g.organization_id = '0' OR g.organization_id IS NULL)
          AND g.status = 'active'
        "#,
    )
    .fetch_one(pool)
    .await
    .map_err(sql_error)?;

    let group_rows = sqlx::query(
        r#"
        SELECT
            CAST(g.external_id AS INTEGER) AS external_id,
            g.name,
            g.description,
            CAST(COALESCE(g.sort_weight, 0) AS INTEGER) AS sort_weight
        FROM membership_package_group g
        WHERE (g.tenant_id = '100001' OR g.tenant_id IS NULL)
          AND (g.organization_id = '0' OR g.organization_id IS NULL)
          AND g.status = 'active'
        ORDER BY g.sort_weight ASC, g.external_id ASC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(sql_error)?;

    let mut groups = Vec::new();
    for row in group_rows {
        let group_external_id = integer_cell(&row, "external_id");
        let packages = load_package_rows(
            pool,
            Some(group_external_id),
            plan_id,
            MembershipListQuery {
                page: Some(1),
                page_size: Some(200),
                cursor: None,
            },
            recommended_only,
        )
        .await?
        .items;
        if recommended_only && packages.is_empty() {
            continue;
        }
        groups.push(build_package_group_from_packages(
            group_external_id,
            string_cell(&row, "name"),
            optional_string_cell(&row, "description"),
            integer_cell(&row, "sort_weight"),
            packages,
        ));
    }
    Ok(offset_page(groups, total, params))
}

async fn load_package_group_by_id(
    pool: &PgPool,
    package_group_id: i64,
) -> AppMembershipResult<Option<AppMembershipPackageGroupItem>> {
    let row = sqlx::query(
        r#"
        SELECT
            CAST(g.external_id AS INTEGER) AS external_id,
            g.name,
            g.description,
            CAST(COALESCE(g.sort_weight, 0) AS INTEGER) AS sort_weight
        FROM membership_package_group g
        WHERE g.external_id = $1
          AND (g.tenant_id = '100001' OR g.tenant_id IS NULL)
          AND (g.organization_id = '0' OR g.organization_id IS NULL)
          AND g.status = 'active'
        LIMIT 1
        "#,
    )
    .bind(package_group_id)
    .fetch_optional(pool)
    .await
    .map_err(sql_error)?;
    let Some(row) = row else {
        return Ok(None);
    };
    let group_external_id = integer_cell(&row, "external_id");
    let packages = load_package_rows(
        pool,
        Some(group_external_id),
        None,
        MembershipListQuery {
            page: Some(1),
            page_size: Some(200),
            cursor: None,
        },
        false,
    )
    .await?
    .items;
    Ok(Some(build_package_group_from_packages(
        group_external_id,
        string_cell(&row, "name"),
        optional_string_cell(&row, "description"),
        integer_cell(&row, "sort_weight"),
        packages,
    )))
}

async fn load_points_balance(
    pool: &PgPool,
    subject: Option<AppMembershipSubject>,
) -> AppMembershipResult<AppMembershipPointsBalanceResponse> {
    let Some(subject) = subject else {
        return Ok(AppMembershipPointsBalanceResponse::default());
    };
    let row = sqlx::query(LOAD_POINTS_BALANCE)
        .bind(subject.tenant_id)
        .bind(subject.organization_id)
        .bind(subject.user_id)
        .bind(POINTS_ASSET_TYPE)
        .bind(POINTS_CURRENCY_CODE)
        .fetch_optional(pool)
        .await
        .map_err(sql_error)?;
    let available_points = row
        .as_ref()
        .map(|row| parse_points_amount(&string_cell(row, "available_amount")))
        .unwrap_or(0);
    let frozen_points = row
        .as_ref()
        .map(|row| parse_points_amount(&string_cell(row, "frozen_amount")))
        .unwrap_or(0);
    Ok(AppMembershipPointsBalanceResponse {
        points: available_points + frozen_points,
        available_points,
        frozen_points,
    })
}

async fn load_points_history(
    pool: &PgPool,
    subject: Option<AppMembershipSubject>,
    query: AppMembershipPointsHistoryQuery,
) -> AppMembershipResult<SdkWorkPageData<AppMembershipPointsHistoryItem>> {
    let Some(subject) = subject else {
        return Err(CommerceServiceError::unauthenticated(
            "membership points history requires an authenticated subject",
        ));
    };
    let page_size = query.limit() as usize;
    let fetch_limit = page_size.saturating_add(1) as i64;
    let rows = if let Some(cursor) = query
        .cursor
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        sqlx::query(LOAD_POINTS_HISTORY_CURSOR)
            .bind(subject.tenant_id)
            .bind(subject.organization_id)
            .bind(subject.user_id)
            .bind(POINTS_ASSET_TYPE)
            .bind(cursor)
            .bind(fetch_limit)
            .fetch_all(pool)
            .await
    } else {
        let offset = query.offset();
        sqlx::query(LOAD_POINTS_HISTORY)
            .bind(subject.tenant_id)
            .bind(subject.organization_id)
            .bind(subject.user_id)
            .bind(POINTS_ASSET_TYPE)
            .bind(fetch_limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }
    .map_err(sql_error)?;
    let has_more = rows.len() > page_size;
    let items: Vec<_> = rows
        .iter()
        .take(page_size)
        .map(map_points_history_item)
        .collect();
    let next_cursor = has_more
        .then(|| items.last().map(|item| item.id.clone()))
        .flatten();
    Ok(cursor_page(items, page_size, next_cursor, has_more))
}

fn map_points_history_item(row: &sqlx::postgres::PgRow) -> AppMembershipPointsHistoryItem {
    let amount = parse_points_amount(&string_cell(row, "amount"));
    let after_balance = parse_points_amount(&string_cell(row, "balance_after"));
    let direction = string_cell(row, "direction").to_ascii_lowercase();
    let signed_amount = if direction == "debit" || direction == "out" {
        -amount
    } else {
        amount
    };
    AppMembershipPointsHistoryItem {
        id: string_cell(row, "id"),
        change_type: string_cell(row, "business_type"),
        change_amount: signed_amount,
        before_balance: Some(after_balance - signed_amount),
        after_balance,
        source_type: string_cell(row, "source_type"),
        remark: optional_string_cell(row, "remark"),
        created_at: optional_string_cell(row, "created_at"),
    }
}

#[derive(Debug, Clone)]
struct CurrentMembership {
    rank: i64,
    plan_name: String,
    status: String,
    starts_at: String,
    expires_at: String,
    total_spent: String,
    benefits: Vec<AppMembershipBenefitItem>,
}

async fn load_current_membership(
    pool: &PgPool,
    subject: AppMembershipSubject,
) -> AppMembershipResult<Option<CurrentMembership>> {
    let row = sqlx::query(LOAD_MEMBERSHIP)
        .bind(subject.tenant_id)
        .bind(subject.organization_id)
        .bind(subject.user_id)
        .fetch_optional(pool)
        .await
        .or_else(none_when_read_model_is_missing)?;
    let Some(row) = row else {
        return Ok(None);
    };
    let mut membership = map_membership(&row);
    membership.benefits = load_stored_plan_by_rank(pool, membership.rank)
        .await?
        .map(|plan| plan.benefits)
        .unwrap_or_default();
    Ok(Some(membership))
}

fn map_membership(row: &sqlx::postgres::PgRow) -> CurrentMembership {
    let plan_no = string_cell(row, "plan_no");
    let rank = integer_cell(row, "rank").max(plan_rank_from_code(&plan_no));
    CurrentMembership {
        rank,
        plan_name: string_cell(row, "plan_name"),
        status: membership_status_label(&string_cell(row, "status")).to_owned(),
        starts_at: string_cell(row, "starts_at"),
        expires_at: string_cell(row, "expires_at"),
        total_spent: decimal_string(
            &string_cell(row, "total_spent"),
            "membership membership total spent",
        )
        .unwrap_or_else(|_| "0.00".to_owned()),
        benefits: Vec::new(),
    }
}

async fn submit_purchase(
    pool: &PgPool,
    command: SubmitMembershipPurchaseCommand,
) -> AppMembershipResult<AppMembershipPurchaseOutcome> {
    if let Some(outcome) = load_purchase_outcome_by_idempotency(pool, &command).await? {
        return Ok(outcome);
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|error| store_error("failed to begin membership purchase transaction", error))?;
    let package = load_package_for_purchase(&mut tx, command.package_id).await?;
    let method = load_payment_method(&mut tx, &command).await?;
    let plan = load_plan_for_package(&mut tx, &package).await?;
    let membership_expires_at =
        add_days_to_timestamp(&command.requested_at, package.item.duration_days);

    insert_order(&mut tx, &command, &package).await?;
    insert_order_item(&mut tx, &command, &package).await?;
    insert_order_amount_breakdown(&mut tx, &command, &package).await?;
    insert_payment(&mut tx, &command, &package, &method).await?;
    insert_membership(&mut tx, &command, &package, &plan, &membership_expires_at).await?;
    insert_entitlements(&mut tx, &command, &plan, &membership_expires_at).await?;
    tx.commit()
        .await
        .map_err(|error| store_error("failed to commit membership purchase transaction", error))?;

    Ok(AppMembershipPurchaseOutcome {
        success: true,
        request_no: command.order_no.clone(),
        order_id: command.order_no.clone(),
        provider_code: method.provider_code.clone(),
        payment_method: method.method_key.clone(),
        payment_product: method.payment_product.clone(),
        next_action: "scan_qr".to_owned(),
        payment_id: command.payment_uuid.clone(),
        cashier_url: membership_payment_qr_code_payload(&command.payment_uuid, &command.order_no),
        qr_code_payload: membership_payment_qr_code_payload(
            &command.payment_uuid,
            &command.order_no,
        ),
        qr_code_image_url: None,
        request_payment_payload: None,
        package_id: package.item.id,
        package_name: package.item.name,
        amount: package.item.price,
        duration_days: package.item.duration_days,
        target_plan_rank: plan.rank,
        target_plan_name: plan.item.name,
        status: "pending".to_owned(),
    })
}

fn membership_payment_qr_code_payload(payment_id: &str, order_id: &str) -> String {
    format!(
        "https://im.sdkwork.com/cashier?scene=membership&orderId={order_id}&paymentId={payment_id}"
    )
}

async fn load_purchase_outcome_by_idempotency(
    pool: &PgPool,
    command: &SubmitMembershipPurchaseCommand,
) -> AppMembershipResult<Option<AppMembershipPurchaseOutcome>> {
    let row = sqlx::query(
        r#"
        SELECT
            o.order_no,
            o.id AS order_uuid,
            o.payment_status,
            pi.id AS payment_uuid,
            pi.provider_code,
            pi.payment_method,
            pi.amount,
            pi.status AS payment_status_detail
        FROM commerce_order o
        LEFT JOIN commerce_payment_intent pi
            ON pi.order_id = o.id
           AND pi.tenant_id = o.tenant_id
        WHERE o.tenant_id = CAST($1 AS TEXT)
          AND o.owner_user_id = CAST($2 AS TEXT)
          AND o.idempotency_key = $3
        ORDER BY o.created_at DESC
        LIMIT 1
        "#,
    )
    .bind(command.subject.tenant_id)
    .bind(command.subject.user_id)
    .bind(&command.idempotency_key)
    .fetch_optional(pool)
    .await
    .map_err(|error| store_error("failed to load membership purchase idempotency record", error))?;

    let Some(row) = row else {
        return Ok(None);
    };

    let package = load_package_for_purchase_by_pool(pool, command.package_id).await?;
    let plan = load_plan_for_package_by_pool(pool, &package).await?;
    let order_no = string_cell(&row, "order_no");
    let payment_uuid = optional_string_cell(&row, "payment_uuid")
        .unwrap_or_else(|| command.payment_uuid.clone());
    let provider_code = optional_string_cell(&row, "provider_code")
        .unwrap_or_else(|| "membership".to_owned());
    let payment_method = optional_string_cell(&row, "payment_method")
        .map(|value| normalize_payment_method(&value))
        .unwrap_or_else(|| MEMBERSHIP_PAYMENT_METHOD.to_owned());
    let payment_status = string_cell(&row, "payment_status").to_ascii_lowercase();
    let status = if payment_status == "paid" || payment_status == "completed" {
        "completed".to_owned()
    } else if payment_status == "failed" || payment_status == "cancelled" {
        "failed".to_owned()
    } else {
        "pending".to_owned()
    };

    Ok(Some(AppMembershipPurchaseOutcome {
        success: status != "failed",
        request_no: order_no.clone(),
        order_id: order_no.clone(),
        provider_code,
        payment_method: payment_method.clone(),
        payment_product: payment_product_for_scan_qr(&payment_method)?.to_owned(),
        next_action: "scan_qr".to_owned(),
        payment_id: payment_uuid.clone(),
        cashier_url: membership_payment_qr_code_payload(&payment_uuid, &order_no),
        qr_code_payload: membership_payment_qr_code_payload(&payment_uuid, &order_no),
        qr_code_image_url: None,
        request_payment_payload: None,
        package_id: package.item.id,
        package_name: package.item.name,
        amount: package.item.price,
        duration_days: package.item.duration_days,
        target_plan_rank: plan.rank,
        target_plan_name: plan.item.name,
        status,
    }))
}

async fn load_package_for_purchase_by_pool(
    pool: &PgPool,
    package_id: i64,
) -> AppMembershipResult<ParsedMembershipPackage> {
    let row = sqlx::query(LOAD_MEMBERSHIP_PACKAGE_BY_ID)
        .bind(package_id)
        .fetch_optional(pool)
        .await
        .map_err(|error| store_error("failed to load membership packages", error))?;
    row.as_ref()
        .and_then(map_package)
        .ok_or_else(|| CommerceServiceError::conflict("membership package is unavailable"))
}

async fn load_plan_for_package_by_pool(
    pool: &PgPool,
    package: &ParsedMembershipPackage,
) -> AppMembershipResult<StoredMembershipPlan> {
    let rows = sqlx::query(LOAD_MEMBERSHIP_PLAN_BY_RANK)
        .bind(package.rank)
        .fetch_all(pool)
        .await
        .map_err(|error| store_error("failed to load membership plans for purchase", error))?;
    stored_plans_from_rows(&rows)
        .into_iter()
        .find(|plan| plan.plan_no == package.plan_no || plan.rank == package.rank)
        .ok_or_else(|| CommerceServiceError::conflict("membership target plan is unavailable"))
}

async fn load_package_for_purchase(
    tx: &mut Transaction<'_, Postgres>,
    package_id: i64,
) -> AppMembershipResult<ParsedMembershipPackage> {
    let row = sqlx::query(LOAD_MEMBERSHIP_PACKAGE_BY_ID)
        .bind(package_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|error| store_error("failed to load membership packages", error))?;
    row.as_ref()
        .and_then(map_package)
        .ok_or_else(|| CommerceServiceError::conflict("membership package is unavailable"))
}

#[derive(Debug, Clone)]
struct MembershipPaymentMethod {
    method_key: String,
    provider_code: String,
    payment_product: String,
}

const MEMBERSHIP_PAYMENT_METHOD: &str = "wechat_pay";

async fn load_payment_method(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
) -> AppMembershipResult<MembershipPaymentMethod> {
    let row = sqlx::query(LOAD_PAYMENT_METHOD)
        .bind(command.subject.tenant_id)
        .bind(command.subject.organization_id)
        .bind(MEMBERSHIP_PAYMENT_METHOD)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|error| store_error("failed to load membership payment method", error))?
        .ok_or_else(|| {
            CommerceServiceError::conflict("membership payment method is unavailable")
        })?;
    let method_key = normalize_payment_method(&string_cell(&row, "method_key"));
    Ok(MembershipPaymentMethod {
        provider_code: normalize_payment_method(&string_cell(&row, "provider_code")),
        payment_product: payment_product_for_scan_qr(&method_key)?.to_owned(),
        method_key,
    })
}

async fn load_plan_for_package(
    tx: &mut Transaction<'_, Postgres>,
    package: &ParsedMembershipPackage,
) -> AppMembershipResult<StoredMembershipPlan> {
    let rows = sqlx::query(LOAD_MEMBERSHIP_PLAN_BY_RANK)
        .bind(package.rank)
        .fetch_all(&mut **tx)
        .await
        .map_err(|error| store_error("failed to load membership plans for purchase", error))?;
    stored_plans_from_rows(&rows)
        .into_iter()
        .find(|plan| plan.plan_no == package.plan_no || plan.rank == package.rank)
        .ok_or_else(|| CommerceServiceError::conflict("membership target plan is unavailable"))
}

async fn membership_package_id_for_storage(
    tx: &mut Transaction<'_, Postgres>,
    external_id: i64,
) -> AppMembershipResult<String> {
    let row = sqlx::query(
        r#"
        SELECT id
        FROM membership_package
        WHERE external_id = ?
          AND (tenant_id = '100001' OR tenant_id IS NULL)
          AND (organization_id = '0' OR organization_id IS NULL)
          AND status = 'active'
        ORDER BY id ASC
        LIMIT 1
        "#,
    )
    .bind(external_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|error| store_error("failed to load membership package storage id", error))?
    .ok_or_else(|| CommerceServiceError::conflict("membership package is unavailable"))?;
    Ok(string_cell(&row, "id"))
}

async fn insert_order(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
    package: &ParsedMembershipPackage,
) -> AppMembershipResult<()> {
    sqlx::query(
        r#"
        INSERT INTO commerce_order
            (id, tenant_id, organization_id, owner_user_id, order_no, status, payment_status, fulfillment_status, refund_status, subject, currency_code, request_no, idempotency_key, created_at, paid_at, cancelled_at, expired_at, updated_at)
        VALUES
            ($1, CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, 'pending_payment', 'pending', 'unfulfilled', 'none', 'membership', 'CNY', $6, $7, $8, NULL, NULL, $9, $10)
        "#,
    )
    .bind(&command.order_uuid)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.subject.user_id)
    .bind(&command.order_no)
    .bind(&command.order_no)
    .bind(&command.idempotency_key)
    .bind(&command.requested_at)
    .bind(&command.expire_at)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert membership order", error))?;
    let _ = package;
    Ok(())
}

async fn insert_order_item(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
    package: &ParsedMembershipPackage,
) -> AppMembershipResult<()> {
    sqlx::query(
        r#"
        INSERT INTO commerce_order_item
            (id, tenant_id, order_id, sku_id, sku_snapshot_json, title, quantity, unit_price_amount, total_amount, fulfillment_status, refund_status, created_at)
        VALUES
            (?, CAST(? AS TEXT), ?, CAST(? AS TEXT), ?, ?, 1, ?, ?, 'unfulfilled', 'none', ?)
        "#,
    )
    .bind(&command.order_item_uuid)
    .bind(command.subject.tenant_id)
    .bind(&command.order_uuid)
    .bind(package.sku_id.as_deref().unwrap_or(""))
    .bind(membership_order_item_snapshot_json(package))
    .bind(&package.item.name)
    .bind(&package.item.price)
    .bind(&package.item.price)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert membership order item", error))?;
    Ok(())
}

async fn insert_order_amount_breakdown(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
    package: &ParsedMembershipPackage,
) -> AppMembershipResult<()> {
    sqlx::query(
        r#"
        INSERT INTO commerce_order_amount_breakdown
            (id, tenant_id, order_id, original_amount, discount_amount, payable_amount, currency_code, created_at)
        VALUES
            (?, CAST(? AS TEXT), ?, ?, '0.00', ?, 'CNY', ?)
        "#,
    )
    .bind(format!("{}-amount", command.order_uuid))
    .bind(command.subject.tenant_id)
    .bind(&command.order_uuid)
    .bind(package.item.original_price.as_ref().unwrap_or(&package.item.price))
    .bind(&package.item.price)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert membership order amount breakdown", error))?;
    Ok(())
}

async fn insert_payment(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
    package: &ParsedMembershipPackage,
    method: &MembershipPaymentMethod,
) -> AppMembershipResult<()> {
    sqlx::query(
        r#"
        INSERT INTO commerce_payment_intent
            (id, tenant_id, organization_id, owner_user_id, order_id, payment_intent_no, payment_method, provider_code, amount, currency_code, status, request_no, idempotency_key, created_at, updated_at)
        VALUES
            (?, CAST(? AS TEXT), CAST(? AS TEXT), CAST(? AS TEXT), ?, ?, ?, ?, ?, 'CNY', ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&command.payment_uuid)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.subject.user_id)
    .bind(&command.order_uuid)
    .bind(format!("PAY-{}", command.order_no))
    .bind(&method.method_key)
    .bind(&method.provider_code)
    .bind(&package.item.price)
    .bind(CommercePaymentStatus::Pending.as_str())
    .bind(&command.order_no)
    .bind(&command.out_trade_no)
    .bind(&command.requested_at)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert membership payment intent", error))?;
    sqlx::query(
        r#"
        INSERT INTO commerce_payment_attempt
            (id, tenant_id, organization_id, owner_user_id, payment_intent_id, order_id, payment_method, provider_code, out_trade_no, amount, currency_code, status, callback_payload, created_at, paid_at, updated_at)
        VALUES
            (?, CAST(? AS TEXT), CAST(? AS TEXT), CAST(? AS TEXT), ?, ?, ?, ?, ?, ?, 'CNY', ?, ?, ?, NULL, ?)
        "#,
    )
    .bind(&command.attempt_uuid)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.subject.user_id)
    .bind(&command.payment_uuid)
    .bind(&command.order_uuid)
    .bind(&method.method_key)
    .bind(&method.provider_code)
    .bind(&command.out_trade_no)
    .bind(&package.item.price)
    .bind(CommercePaymentStatus::Pending.as_str())
    .bind(format!(
        r#"{{"subject":"membership","packageId":{},"action":"{}"}}"#,
        package.item.id, command.action
    ))
    .bind(&command.requested_at)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert membership payment attempt", error))?;
    Ok(())
}

async fn insert_membership(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
    package: &ParsedMembershipPackage,
    plan: &StoredMembershipPlan,
    expires_at: &str,
) -> AppMembershipResult<()> {
    let period_id = membership_period_id(command);
    let package_id = membership_package_id_for_storage(tx, package.item.id).await?;
    sqlx::query(
        r#"
        INSERT INTO membership_subscription
            (id, tenant_id, organization_id, subscription_no, subject_type, subject_id,
             owner_user_id, plan_id, plan_version_id, package_id, current_period_id,
             source_order_id, source_payment_intent_id, status, starts_at, expires_at,
             grace_until, cancel_at_period_end, request_no, idempotency_key, created_at, updated_at)
        VALUES
            (?, CAST(? AS TEXT), CAST(? AS TEXT), ?, 'user', CAST(? AS TEXT),
             CAST(? AS TEXT), ?, ?, ?, ?, ?, ?, 'pending_activation', ?, ?,
             NULL, 0, ?, ?, ?, ?)
        "#,
    )
    .bind(&command.membership_uuid)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(&command.membership_uuid)
    .bind(command.subject.user_id)
    .bind(command.subject.user_id)
    .bind(plan_id_for_storage(plan))
    .bind(plan_version_id_for_storage(plan))
    .bind(&package_id)
    .bind(&period_id)
    .bind(&command.order_uuid)
    .bind(&command.payment_uuid)
    .bind(&command.requested_at)
    .bind(expires_at)
    .bind(&command.order_no)
    .bind(&command.out_trade_no)
    .bind(&command.requested_at)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert membership subscription", error))?;

    sqlx::query(
        r#"
        INSERT INTO membership_period
            (id, tenant_id, organization_id, period_no, subscription_id, subject_type,
             subject_id, plan_id, plan_version_id, starts_at, ends_at, status,
             source_order_id, source_payment_intent_id, request_no, idempotency_key,
             created_at, updated_at)
        VALUES
            (?, CAST(? AS TEXT), CAST(? AS TEXT), ?, ?, 'user',
             CAST(? AS TEXT), ?, ?, ?, ?, 'pending_activation',
             ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&period_id)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(&period_id)
    .bind(&command.membership_uuid)
    .bind(command.subject.user_id)
    .bind(plan_id_for_storage(plan))
    .bind(plan_version_id_for_storage(plan))
    .bind(&command.requested_at)
    .bind(expires_at)
    .bind(&command.order_uuid)
    .bind(&command.payment_uuid)
    .bind(&command.order_no)
    .bind(&command.out_trade_no)
    .bind(&command.requested_at)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert membership period", error))?;
    Ok(())
}

async fn insert_entitlements(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
    plan: &StoredMembershipPlan,
    expires_at: &str,
) -> AppMembershipResult<()> {
    for (index, benefit) in plan.benefits.iter().enumerate() {
        let benefit_code = benefit
            .benefit_key
            .clone()
            .unwrap_or_else(|| format!("membership-benefit-{}", benefit.id));
        let benefit_id = benefit_definition_id_for_code(&benefit_code);
        let quantity = benefit.usage_limit.unwrap_or(0).max(0).to_string();
        let account_id = format!(
            "{}-entitlement-account-{}",
            command.membership_uuid,
            index + 1
        );
        let grant_id = format!(
            "{}-entitlement-grant-{}",
            command.membership_uuid,
            index + 1
        );
        let ledger_id = format!(
            "{}-entitlement-ledger-{}",
            command.membership_uuid,
            index + 1
        );
        sqlx::query(
            r#"
            INSERT INTO entitlement_grant
                (id, tenant_id, organization_id, grant_no, benefit_id, subject_type, subject_id,
                 source_type, source_id, grant_policy, granted_quantity, status, starts_at,
                 expires_at, request_no, idempotency_key, created_at, updated_at)
            VALUES
                (?, CAST(? AS TEXT), CAST(? AS TEXT), ?, ?, 'user', CAST(? AS TEXT),
                 'membership_subscription', ?, 'membership_plan', ?, 'active', ?, ?,
                 ?, ?, ?, ?)
            "#,
        )
        .bind(&grant_id)
        .bind(command.subject.tenant_id)
        .bind(command.subject.organization_id)
        .bind(&grant_id)
        .bind(&benefit_id)
        .bind(command.subject.user_id)
        .bind(&command.membership_uuid)
        .bind(&quantity)
        .bind(&command.requested_at)
        .bind(expires_at)
        .bind(format!("{}-grant-{}", command.order_no, index + 1))
        .bind(&command.out_trade_no)
        .bind(&command.requested_at)
        .bind(&command.requested_at)
        .execute(&mut **tx)
        .await
        .map_err(|error| store_error("failed to insert entitlement grant", error))?;

        let account = upsert_entitlement_account(
            tx,
            command,
            &account_id,
            &benefit_id,
            &quantity,
            expires_at,
        )
        .await?;

        sqlx::query(
            r#"
            INSERT INTO entitlement_ledger_entry
                (id, tenant_id, organization_id, ledger_no, account_id, grant_id, benefit_id,
                 subject_type, subject_id, direction, amount, balance_after, business_type,
                 source_type, source_id, request_no, idempotency_key, occurred_at, created_at)
            VALUES
                (?, CAST(? AS TEXT), CAST(? AS TEXT), ?, ?, ?, ?,
                 'user', CAST(? AS TEXT), 'credit', ?, ?, 'membership_grant',
                 'membership_subscription', ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&ledger_id)
        .bind(command.subject.tenant_id)
        .bind(command.subject.organization_id)
        .bind(&ledger_id)
        .bind(&account.account_id)
        .bind(&grant_id)
        .bind(&benefit_id)
        .bind(command.subject.user_id)
        .bind(&quantity)
        .bind(&account.balance_after)
        .bind(&command.membership_uuid)
        .bind(format!("{}-ledger-{}", command.order_no, index + 1))
        .bind(&command.out_trade_no)
        .bind(&command.requested_at)
        .bind(&command.requested_at)
        .execute(&mut **tx)
        .await
        .map_err(|error| store_error("failed to insert entitlement ledger", error))?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct EntitlementAccountBalance {
    account_id: String,
    balance_after: String,
}

async fn upsert_entitlement_account(
    tx: &mut Transaction<'_, Postgres>,
    command: &SubmitMembershipPurchaseCommand,
    account_id: &str,
    benefit_id: &str,
    quantity: &str,
    expires_at: &str,
) -> AppMembershipResult<EntitlementAccountBalance> {
    sqlx::query(
        r#"
        INSERT INTO entitlement_account
            (id, tenant_id, organization_id, account_no, benefit_id, subject_type,
             subject_id, total_granted, total_used, balance, status, expires_at,
             version, created_at, updated_at)
        VALUES
            (?, CAST(? AS TEXT), CAST(? AS TEXT), ?, ?, 'user',
             CAST(? AS TEXT), ?, '0', ?, 'active', ?, 0, ?, ?)
        ON CONFLICT(tenant_id, subject_type, subject_id, benefit_id) DO UPDATE SET
            total_granted = CAST((CAST(entitlement_account.total_granted AS INTEGER) + CAST(excluded.total_granted AS INTEGER)) AS TEXT),
            balance = CAST((CAST(entitlement_account.balance AS INTEGER) + CAST(excluded.balance AS INTEGER)) AS TEXT),
            status = 'active',
            expires_at = CASE
                WHEN entitlement_account.expires_at IS NULL OR excluded.expires_at > entitlement_account.expires_at THEN excluded.expires_at
                ELSE entitlement_account.expires_at
            END,
            version = entitlement_account.version + 1,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(account_id)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(account_id)
    .bind(benefit_id)
    .bind(command.subject.user_id)
    .bind(quantity)
    .bind(quantity)
    .bind(expires_at)
    .bind(&command.requested_at)
    .bind(&command.requested_at)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to insert entitlement account", error))?;

    let row = sqlx::query(
        r#"
        SELECT id, balance
        FROM entitlement_account
        WHERE tenant_id = CAST(? AS TEXT)
          AND subject_type = 'user'
          AND subject_id = CAST(? AS TEXT)
          AND benefit_id = ?
        "#,
    )
    .bind(command.subject.tenant_id)
    .bind(command.subject.user_id)
    .bind(benefit_id)
    .fetch_one(&mut **tx)
    .await
    .map_err(|error| store_error("failed to load entitlement account", error))?;

    Ok(EntitlementAccountBalance {
        account_id: string_cell(&row, "id"),
        balance_after: string_cell(&row, "balance"),
    })
}

async fn consume_speed_up(
    pool: &PgPool,
    subject: AppMembershipSubject,
    requested_at: String,
) -> AppMembershipResult<()> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|error| store_error("failed to begin membership speed up transaction", error))?;
    let row = sqlx::query(
        r#"
        SELECT
            a.id,
            d.benefit_code AS entitlement_code,
            COALESCE(g.source_id, '') AS membership_id,
            g.id AS grant_id,
            a.total_granted AS granted_quantity,
            a.total_used AS used_quantity,
            a.balance AS balance
        FROM entitlement_account a
        JOIN benefit_definition d
          ON d.id = a.benefit_id
        LEFT JOIN entitlement_grant g
          ON g.benefit_id = a.benefit_id
         AND g.subject_type = a.subject_type
         AND g.subject_id = a.subject_id
         AND g.source_type = 'membership_subscription'
         AND g.tenant_id = a.tenant_id
        LEFT JOIN membership_subscription m
          ON m.id = g.source_id
         AND m.tenant_id = a.tenant_id
        WHERE a.tenant_id = CAST($1 AS TEXT)
          AND (a.organization_id IS NULL OR a.organization_id = CAST($2 AS TEXT))
          AND a.subject_type = 'user'
          AND a.subject_id = CAST($3 AS TEXT)
          AND a.status = 'active'
          AND d.benefit_code = 'priority_speed_up'
          AND CAST(a.balance AS INTEGER) > 0
        ORDER BY a.expires_at DESC, a.updated_at ASC, a.created_at ASC, a.id ASC
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(subject.user_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|error| store_error("failed to load membership speed up entitlement", error))?
    .ok_or_else(|| {
        CommerceServiceError::conflict("membership speed up privilege is unavailable")
    })?;

    let account_id = string_cell(&row, "id");
    let membership_id = string_cell(&row, "membership_id");
    let grant_id = optional_string_cell(&row, "grant_id");
    let granted_quantity = parse_points_amount(&string_cell(&row, "granted_quantity")).max(0);
    let used_quantity = parse_points_amount(&string_cell(&row, "used_quantity")).max(0);
    let balance = parse_points_amount(&string_cell(&row, "balance")).max(0);
    if balance <= 0 || used_quantity >= granted_quantity || granted_quantity <= 0 {
        return Err(CommerceServiceError::conflict(
            "membership speed up privilege is exhausted",
        ));
    }

    let updated_rows = sqlx::query(
        r#"
        UPDATE entitlement_account
        SET total_used = CAST(CAST(total_used AS INTEGER) + 1 AS TEXT),
            balance = CAST(CAST(balance AS INTEGER) - 1 AS TEXT),
            version = version + 1,
            updated_at = $2
        WHERE id = $1
          AND CAST(balance AS INTEGER) > 0
        "#,
    )
    .bind(&account_id)
    .bind(&requested_at)
    .execute(&mut *tx)
    .await
    .map_err(|error| store_error("failed to update membership speed up entitlement", error))?
    .rows_affected();
    if updated_rows == 0 {
        return Err(CommerceServiceError::conflict(
            "membership speed up privilege is exhausted",
        ));
    }

    let usage_id = format!("{account_id}-ledger-debit-{}", used_quantity + 1);
    let request_no = format!(
        "membership-speed-up-{}-{}",
        subject.user_id,
        used_quantity + 1
    );
    sqlx::query(
        r#"
        INSERT INTO entitlement_ledger_entry
            (id, tenant_id, organization_id, ledger_no, account_id, grant_id, benefit_id,
             subject_type, subject_id, direction, amount, balance_after, business_type,
             source_type, source_id, request_no, idempotency_key, occurred_at, created_at)
        SELECT
            $1, CAST($2 AS TEXT), CAST($3 AS TEXT), $1, a.id, $4, a.benefit_id,
            'user', CAST($5 AS TEXT), 'debit', '1', CAST($6 AS TEXT), 'membership_speed_up',
            'membership_subscription', $7, $8, $8, $9, $9
        FROM entitlement_account a
        WHERE a.id = $10
        "#,
    )
    .bind(&usage_id)
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(grant_id.as_deref())
    .bind(subject.user_id)
    .bind((balance - 1).max(0).to_string())
    .bind(if membership_id.trim().is_empty() {
        "membership-speed-up"
    } else {
        membership_id.as_str()
    })
    .bind(&request_no)
    .bind(&requested_at)
    .bind(&account_id)
    .execute(&mut *tx)
    .await
    .map_err(|error| store_error("failed to insert membership speed up usage", error))?;

    tx.commit()
        .await
        .map_err(|error| store_error("failed to commit membership speed up transaction", error))?;
    Ok(())
}

fn plan_id_for_storage(plan: &StoredMembershipPlan) -> String {
    if !plan.storage_id.trim().is_empty() {
        plan.storage_id.clone()
    } else if plan.plan_no.trim().is_empty() {
        format!("membership-plan-{}", plan_code_from_rank(plan.rank))
    } else {
        format!("membership-plan-{}", plan.plan_no.trim())
    }
}

fn plan_version_id_for_storage(plan: &StoredMembershipPlan) -> String {
    match plan.plan_no.as_str() {
        "free" => "seed-membership-plan-version-free-v1".to_owned(),
        "pro" => "seed-membership-plan-version-pro-v1".to_owned(),
        "max" => "seed-membership-plan-version-max-v1".to_owned(),
        "vip" => "seed-membership-plan-version-vip-v1".to_owned(),
        _ => format!("{}-version-v1", plan_id_for_storage(plan)),
    }
}

fn membership_period_id(command: &SubmitMembershipPurchaseCommand) -> String {
    format!("{}-period-1", command.membership_uuid)
}

fn membership_order_item_snapshot_json(package: &ParsedMembershipPackage) -> String {
    serde_json::json!({
        "skuId": package.sku_id.as_deref().unwrap_or(""),
        "packageId": package.item.id,
        "packageName": &package.item.name,
        "durationDays": package.item.duration_days,
        "planName": package.item.plan_name.as_deref(),
    })
    .to_string()
}

fn admin_plan_version_id(plan_id: &str) -> String {
    format!("{}-version-v1", storage_key(plan_id))
}

fn admin_benefit_code(benefit: &AppMembershipBenefitItem, fallback_index: usize) -> String {
    benefit
        .benefit_key
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().to_ascii_lowercase())
        .unwrap_or_else(|| format!("membership_benefit_{fallback_index}"))
}

fn benefit_definition_id_for_code(benefit_code: &str) -> String {
    match benefit_code {
        "ai_quota" => "seed-benefit-ai-quota".to_owned(),
        "priority_speed_up" => "seed-benefit-priority-speed-up".to_owned(),
        "member_discount" => "seed-benefit-member-discount".to_owned(),
        "monthly_coupon_grant" => "seed-benefit-monthly-coupon-grant".to_owned(),
        value => format!("benefit-definition-{}", storage_key(value)),
    }
}

fn storage_key(value: &str) -> String {
    let key = value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    let key = key.trim_matches('-').to_owned();
    if key.is_empty() {
        "standard".to_owned()
    } else {
        key
    }
}

fn membership_status_label(status: &str) -> &'static str {
    match status.trim().to_ascii_lowercase().as_str() {
        "active" => "active",
        "pending_activation" | "pending" => "pending",
        "expired" => "expired",
        _ => "free",
    }
}

fn remaining_days(_expires_at: &str) -> Option<i64> {
    None
}

fn add_days_to_timestamp(timestamp: &str, days: i64) -> String {
    let Some(seconds) = parse_timestamp(timestamp) else {
        return timestamp.to_owned();
    };
    format_unix_timestamp(seconds + days.max(0) * 86_400)
}

fn parse_timestamp(timestamp: &str) -> Option<i64> {
    let (date, time) = timestamp.trim().split_once(' ')?;
    let mut date_parts = date.split('-');
    let year = date_parts.next()?.parse::<i64>().ok()?;
    let month = date_parts.next()?.parse::<i64>().ok()?;
    let day = date_parts.next()?.parse::<i64>().ok()?;
    let mut time_parts = time.split(':');
    let hour = time_parts.next()?.parse::<i64>().ok()?;
    let minute = time_parts.next()?.parse::<i64>().ok()?;
    let second = time_parts.next()?.parse::<i64>().ok()?;
    Some(days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second)
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn format_unix_timestamp(seconds: i64) -> String {
    let days = seconds.div_euclid(86_400);
    let seconds_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}")
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

fn numeric_suffix(value: &str) -> Option<i64> {
    value.rsplit('-').next()?.parse::<i64>().ok()
}

fn optional_string_cell(row: &sqlx::postgres::PgRow, column: &str) -> Option<String> {
    row.try_get::<Option<String>, _>(column).ok().flatten()
}

fn string_cell(row: &sqlx::postgres::PgRow, column: &str) -> String {
    optional_string_cell(row, column).unwrap_or_default()
}

fn integer_cell(row: &sqlx::postgres::PgRow, column: &str) -> i64 {
    row.try_get::<i64, _>(column)
        .or_else(|_| row.try_get::<i32, _>(column).map(i64::from))
        .unwrap_or(0)
}

fn sql_error(error: sqlx::Error) -> CommerceServiceError {
    CommerceServiceError::storage(error.to_string())
}

fn store_error(context: &str, error: sqlx::Error) -> CommerceServiceError {
    CommerceServiceError::storage(format!("{context}: {error}"))
}

fn empty_rows_when_read_model_is_missing(
    error: sqlx::Error,
) -> Result<Vec<sqlx::postgres::PgRow>, CommerceServiceError> {
    Err(sql_error(error))
}

fn none_when_read_model_is_missing(
    error: sqlx::Error,
) -> Result<Option<sqlx::postgres::PgRow>, CommerceServiceError> {
    if is_missing_postgres_read_model(&error) {
        Ok(None)
    } else {
        Err(sql_error(error))
    }
}

// ── Daily reward helpers ──

const DAILY_REWARD_BASE_POINTS: i64 = 10;
const DAILY_REWARD_WEEKLY_BONUS: i64 = 50;
const DAILY_REWARD_BIWEEKLY_BONUS: i64 = 100;
const DAILY_REWARD_MONTHLY_BONUS: i64 = 500;

fn daily_reward_points(consecutive_days: i64) -> i64 {
    let day = consecutive_days.max(1);
    if day % 30 == 0 {
        DAILY_REWARD_MONTHLY_BONUS
    } else if day % 14 == 0 {
        DAILY_REWARD_BIWEEKLY_BONUS
    } else if day % 7 == 0 {
        DAILY_REWARD_WEEKLY_BONUS
    } else {
        DAILY_REWARD_BASE_POINTS
    }
}

async fn load_daily_reward_status_sqlite(
    pool: &PgPool,
    subject: AppMembershipSubject,
) -> AppMembershipResult<AppMembershipDailyRewardStatusResponse> {
    let row = sqlx::query(
        r#"
        SELECT
            reward_date,
            CAST(consecutive_days AS INTEGER) AS consecutive_days,
            CAST(total_days AS INTEGER) AS total_days,
            date('now') AS today,
            CASE WHEN reward_date = date('now') THEN 1 ELSE 0 END AS is_today
        FROM commerce_membership_daily_reward
        WHERE tenant_id = $1
          AND (organization_id = 0 OR organization_id = $2)
          AND user_id = $3
          AND reward_date >= date('now', '-2 days')
        ORDER BY reward_date DESC
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(subject.user_id)
    .fetch_optional(pool)
    .await
    .or_else(none_when_read_model_is_missing)?;

    let Some(row) = row else {
        return Ok(AppMembershipDailyRewardStatusResponse {
            can_claim: true,
            claimed_today: false,
            consecutive_days: 0,
            total_days: 0,
        });
    };

    let is_today: i64 = row.try_get("is_today").unwrap_or(0);
    let consecutive_days = integer_cell(&row, "consecutive_days");
    let total_days = integer_cell(&row, "total_days");

    Ok(AppMembershipDailyRewardStatusResponse {
        can_claim: is_today == 0,
        claimed_today: is_today != 0,
        consecutive_days,
        total_days,
    })
}

async fn claim_daily_reward_sqlite(
    pool: &PgPool,
    subject: AppMembershipSubject,
    requested_at: String,
) -> AppMembershipResult<AppMembershipDailyRewardResponse> {
    let last_row = sqlx::query(
        r#"
        SELECT
            reward_date,
            CAST(consecutive_days AS INTEGER) AS consecutive_days,
            CAST(total_days AS INTEGER) AS total_days,
            date('now') AS today
        FROM commerce_membership_daily_reward
        WHERE tenant_id = $1
          AND (organization_id = 0 OR organization_id = $2)
          AND user_id = $3
          AND reward_date >= date('now', '-1 day')
        ORDER BY reward_date DESC
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(subject.user_id)
    .fetch_optional(pool)
    .await
    .or_else(none_when_read_model_is_missing)?;

    let today = current_date_sqlite(pool).await;
    let (prev_consecutive, prev_total, prev_date) = last_row
        .as_ref()
        .map(|row| {
            (
                integer_cell(row, "consecutive_days"),
                integer_cell(row, "total_days"),
                string_cell(row, "reward_date"),
            )
        })
        .unwrap_or((0, 0, String::new()));

    if prev_date == today {
        return Err(CommerceServiceError::conflict(
            "membership daily reward has already been claimed today",
        ));
    }

    let yesterday = yesterday_date_sqlite(pool).await;
    let new_consecutive = if prev_date == yesterday {
        prev_consecutive + 1
    } else {
        1
    };
    let new_total = prev_total + 1;
    let reward_points = daily_reward_points(new_consecutive);
    let reward_id = format!(
        "daily-reward-{}-{}-{}",
        subject.tenant_id, subject.user_id, today
    );
    let reward_uuid = format!("dr-{}-{}-{}", subject.tenant_id, subject.user_id, today);
    let idempotency_key = format!(
        "daily-reward-{}-{}-{}",
        subject.tenant_id, subject.user_id, today
    );

    sqlx::query(
        r#"
        INSERT INTO commerce_membership_daily_reward
            (id, uuid, tenant_id, organization_id, user_id, reward_date,
             reward_points, consecutive_days, total_days, status, idempotency_key, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'claimed', ?, ?)
        ON CONFLICT (tenant_id, user_id, reward_date) DO NOTHING
        "#,
    )
    .bind(&reward_id)
    .bind(&reward_uuid)
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(subject.user_id)
    .bind(&today)
    .bind(reward_points)
    .bind(new_consecutive)
    .bind(new_total)
    .bind(&idempotency_key)
    .bind(&requested_at)
    .execute(pool)
    .await
    .map_err(|error| {
        if is_missing_postgres_read_model(&error) {
            CommerceServiceError::conflict(
                "membership daily reward is unavailable without reward table migration",
            )
        } else {
            store_error("failed to insert daily reward", error)
        }
    })?;

    Ok(AppMembershipDailyRewardResponse {
        reward_points,
        claimed_at: Some(requested_at),
        message: format!("claimed {reward_points} points for day {new_consecutive}"),
        consecutive_days: new_consecutive,
    })
}

async fn current_date_sqlite(pool: &PgPool) -> String {
    sqlx::query_scalar::<_, String>("SELECT date('now')")
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| format_timestamp_date(std::time::SystemTime::now()))
}

async fn yesterday_date_sqlite(pool: &PgPool) -> String {
    sqlx::query_scalar::<_, String>("SELECT date('now', '-1 day')")
        .fetch_one(pool)
        .await
        .unwrap_or_default()
}

fn format_timestamp_date(time: std::time::SystemTime) -> String {
    let secs = time
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86400;
    let (year, month, day) = epoch_days_to_ymd(days as i64);
    format!("{year:04}-{month:02}-{day:02}")
}

fn epoch_days_to_ymd(days_since_epoch: i64) -> (i64, u32, u32) {
    let mut days = days_since_epoch;
    let mut year = 1970i64;
    loop {
        let leap = is_leap_year(year);
        let year_days = if leap { 366 } else { 365 };
        if days >= year_days {
            days -= year_days;
            year += 1;
        } else {
            break;
        }
    }
    let leap = is_leap_year(year);
    let month_days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 0u32;
    let mut day = days as u32;
    while (month as usize) < 12 && day >= month_days[month as usize] {
        day -= month_days[month as usize];
        month += 1;
    }
    (year, month + 1, day + 1)
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

// ── Privilege usage helpers ──

async fn load_privilege_usage_sqlite(
    pool: &PgPool,
    subject: AppMembershipSubject,
) -> AppMembershipResult<AppMembershipPrivilegeUsageResponse> {
    let rows = sqlx::query(
        r#"
        SELECT
            benefit_code,
            CAST(used_count AS INTEGER) AS used_count,
            CAST(usage_limit AS INTEGER) AS usage_limit
        FROM commerce_membership_privilege_usage
        WHERE tenant_id = $1
          AND (organization_id = 0 OR organization_id = $2)
          AND user_id = $3
          AND period_end >= date('now')
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(subject.user_id)
    .fetch_all(pool)
    .await
    .or_else(empty_rows_when_read_model_is_missing)?;

    let mut response = AppMembershipPrivilegeUsageResponse::default();
    for row in &rows {
        let benefit_code = string_cell(row, "benefit_code");
        let used = integer_cell(row, "used_count");
        match benefit_code.as_str() {
            "priority_speed_up" => {
                response.speed_up_used = used;
            }
            "priority_queue" => {
                response.priority_queue_used = used;
            }
            "ai_quota" | "exclusive_model" => {
                response.exclusive_model_used = used;
            }
            _ => {}
        }
    }
    Ok(response)
}
