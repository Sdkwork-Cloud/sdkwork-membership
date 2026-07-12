use std::future::Future;
use std::pin::Pin;

use sdkwork_contract_service::CommerceServiceError;
use sdkwork_utils_rust::{SdkWorkCommandData, SdkWorkPageData};
use serde::{Deserialize, Serialize};

pub use crate::pagination::MembershipListQuery as AppMembershipListQuery;

pub type AppMembershipResult<T> = Result<T, CommerceServiceError>;

pub type AppMembershipReadFuture<'a, T> =
    Pin<Box<dyn Future<Output = AppMembershipResult<T>> + Send + 'a>>;
pub type AppMembershipCommandFuture<'a> =
    Pin<Box<dyn Future<Output = AppMembershipResult<AppMembershipPurchaseOutcome>> + Send + 'a>>;
pub type AppMembershipFulfillmentFuture<'a> = Pin<
    Box<dyn Future<Output = AppMembershipResult<FulfillMembershipPurchaseOutcome>> + Send + 'a>,
>;
pub type AdminMembershipFuture<'a, T> =
    Pin<Box<dyn Future<Output = AppMembershipResult<T>> + Send + 'a>>;

pub trait AppMembershipEntityIdGenerator {
    fn generate_entity_uuid(&self) -> AppMembershipResult<String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppMembershipSubject {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppMembershipPointsHistoryQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub cursor: Option<String>,
}

impl AppMembershipPointsHistoryQuery {
    pub fn limit(&self) -> i64 {
        self.page_size.unwrap_or(20).clamp(1, 200)
    }

    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminMembershipSubject {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub operator_id: i64,
    pub operator_type: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipBenefitItem {
    pub id: i64,
    pub name: String,
    pub benefit_key: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub claimed: bool,
    pub usage_limit: Option<i64>,
    pub used_count: Option<i64>,
    /// Raw text value of the benefit grant quantity. Used for non-numeric
    /// comparison table cells like "2K", "4K/8K", "8折积分", "标准生成通道".
    /// When the grant_quantity is a pure number, display_value is None and
    /// usage_limit holds the parsed integer. When it is a text value,
    /// usage_limit is None and display_value holds the raw text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipPlanItem {
    pub id: i64,
    pub name: String,
    pub rank: i64,
    pub required_points: Option<i64>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub badge: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipInfoResponse {
    pub plan_rank: i64,
    pub plan_name: String,
    pub membership_status: String,
    pub started_at: Option<String>,
    pub expires_at: Option<String>,
    pub remaining_days: Option<i64>,
    pub total_days: Option<i64>,
    pub total_spent: Option<String>,
    pub points: Option<i64>,
    pub growth_value: Option<i64>,
    pub upgrade_growth_value: Option<i64>,
    pub benefits: Vec<AppMembershipBenefitItem>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipStatusResponse {
    pub active: bool,
    pub plan_rank: i64,
    pub expires_at: Option<String>,
    pub point_balance: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipPackageItem {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: String,
    pub original_price: Option<String>,
    pub point_amount: i64,
    pub duration_days: i64,
    pub plan_name: Option<String>,
    pub sort_weight: i64,
    pub recommended: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipPackageGroupItem {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sort_weight: i64,
    pub packages: Vec<AppMembershipPackageItem>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipPointsBalanceResponse {
    pub points: i64,
    pub available_points: i64,
    pub frozen_points: i64,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipPointsHistoryItem {
    pub id: String,
    pub change_type: String,
    pub change_amount: i64,
    pub before_balance: Option<i64>,
    pub after_balance: i64,
    pub source_type: String,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipDailyRewardStatusResponse {
    pub can_claim: bool,
    pub claimed_today: bool,
    pub consecutive_days: i64,
    pub total_days: i64,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipDailyRewardResponse {
    pub reward_points: i64,
    pub claimed_at: Option<String>,
    pub consecutive_days: i64,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipPrivilegeUsageResponse {
    pub speed_up_used: i64,
    pub speed_up_limit: i64,
    pub priority_queue_used: i64,
    pub priority_queue_limit: i64,
    pub exclusive_model_used: i64,
    pub exclusive_model_limit: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitMembershipPurchaseCommand {
    pub subject: AppMembershipSubject,
    pub package_id: i64,
    pub order_uuid: String,
    pub membership_uuid: String,
    pub order_no: String,
    pub idempotency_key: String,
    pub requested_at: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FulfillMembershipPurchaseCommand {
    pub subject: AppMembershipSubject,
    pub order_id: String,
    pub request_no: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FulfillMembershipPurchaseOutcome {
    pub accepted: bool,
    pub replayed: bool,
    pub fulfillment_status: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppMembershipPurchaseOutcome {
    pub request_no: String,
    pub order_id: String,
    pub package_id: i64,
    pub package_name: String,
    pub amount: String,
    pub duration_days: i64,
    pub target_plan_rank: i64,
    pub target_plan_name: String,
    pub status: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdminMembershipPlanItem {
    pub id: String,
    pub code: String,
    pub name: String,
    pub rank: i64,
    pub benefits: Vec<AppMembershipBenefitItem>,
    pub status: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdminMembershipPackageItem {
    pub id: String,
    pub code: String,
    pub package_group_id: String,
    pub plan_id: String,
    pub name: String,
    pub price_amount: String,
    pub currency_code: String,
    pub duration_days: i64,
    pub status: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdminMembershipPackageGroupItem {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub billing_cycle: String,
    pub duration_days: i64,
    pub sort_weight: i64,
    pub status: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdminMembershipMemberItem {
    pub id: String,
    pub owner_user_id: String,
    pub plan_code: String,
    pub status: String,
    pub started_at: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdminMembershipEntitlementItem {
    pub id: String,
    pub code: String,
    pub plan_id: String,
    pub membership_id: String,
    pub quota: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAdminMembershipPlansQuery {
    pub subject: AdminMembershipSubject,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAdminMembershipPackagesQuery {
    pub subject: AdminMembershipSubject,
    pub package_group_id: Option<String>,
    pub plan_id: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAdminMembershipPackageGroupsQuery {
    pub subject: AdminMembershipSubject,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAdminMembershipMembersQuery {
    pub subject: AdminMembershipSubject,
    pub user_id: Option<String>,
    pub plan_id: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAdminMembershipEntitlementsQuery {
    pub subject: AdminMembershipSubject,
    pub plan_id: Option<String>,
    pub membership_id: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminMembershipPlanMutation {
    pub code: String,
    pub name: String,
    pub rank: i64,
    pub benefits: Option<Vec<AppMembershipBenefitItem>>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminMembershipPackageMutation {
    pub code: String,
    pub package_group_id: String,
    pub plan_id: String,
    pub name: String,
    pub price_amount: String,
    pub currency_code: String,
    pub duration_days: i64,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminMembershipPackageGroupMutation {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub billing_cycle: String,
    pub duration_days: i64,
    pub sort_weight: i64,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAdminMembershipPlanCommand {
    pub subject: AdminMembershipSubject,
    pub plan_id: String,
    pub input: AdminMembershipPlanMutation,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateAdminMembershipPlanCommand {
    pub subject: AdminMembershipSubject,
    pub plan_id: String,
    pub input: AdminMembershipPlanMutation,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAdminMembershipPlanCommand {
    pub subject: AdminMembershipSubject,
    pub plan_id: String,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAdminMembershipPackageCommand {
    pub subject: AdminMembershipSubject,
    pub package_id: String,
    pub input: AdminMembershipPackageMutation,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateAdminMembershipPackageCommand {
    pub subject: AdminMembershipSubject,
    pub package_id: String,
    pub input: AdminMembershipPackageMutation,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAdminMembershipPackageCommand {
    pub subject: AdminMembershipSubject,
    pub package_id: String,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAdminMembershipPackageGroupCommand {
    pub subject: AdminMembershipSubject,
    pub package_group_id: String,
    pub input: AdminMembershipPackageGroupMutation,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateAdminMembershipPackageGroupCommand {
    pub subject: AdminMembershipSubject,
    pub package_group_id: String,
    pub input: AdminMembershipPackageGroupMutation,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAdminMembershipPackageGroupCommand {
    pub subject: AdminMembershipSubject,
    pub package_group_id: String,
    pub request_id: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateAdminMembershipMemberStatusCommand {
    pub subject: AdminMembershipSubject,
    pub membership_id: String,
    pub status: String,
    pub request_id: String,
    pub requested_at: String,
}

pub trait AppMembershipStore {
    fn load_info<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipInfoResponse>;

    fn load_status<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipStatusResponse>;

    fn load_plans<'a>(
        &'a self,
        catalog_subject: Option<AppMembershipSubject>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPlanItem>>;

    fn load_benefits<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipBenefitItem>>;

    fn load_packages<'a>(
        &'a self,
        catalog_subject: Option<AppMembershipSubject>,
        package_group_id: Option<i64>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageItem>>;

    fn load_package<'a>(
        &'a self,
        catalog_subject: Option<AppMembershipSubject>,
        package_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageItem>>;

    fn load_package_groups<'a>(
        &'a self,
        catalog_subject: Option<AppMembershipSubject>,
        plan_id: Option<i64>,
        recommended_only: bool,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageGroupItem>>;

    fn load_package_group<'a>(
        &'a self,
        catalog_subject: Option<AppMembershipSubject>,
        package_group_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageGroupItem>>;

    fn load_points_balance<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPointsBalanceResponse>;

    fn load_points_history<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
        query: AppMembershipPointsHistoryQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPointsHistoryItem>>;

    fn load_daily_reward_status<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardStatusResponse>;

    fn claim_daily_reward<'a>(
        &'a self,
        subject: AppMembershipSubject,
        requested_at: String,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardResponse>;

    fn load_privilege_usage<'a>(
        &'a self,
        subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPrivilegeUsageResponse>;

    fn consume_speed_up<'a>(
        &'a self,
        subject: AppMembershipSubject,
        requested_at: String,
    ) -> AppMembershipReadFuture<'a, SdkWorkCommandData>;

    fn submit_purchase<'a>(
        &'a self,
        command: SubmitMembershipPurchaseCommand,
    ) -> AppMembershipCommandFuture<'a>;

    fn fulfill_purchase<'a>(
        &'a self,
        command: FulfillMembershipPurchaseCommand,
    ) -> AppMembershipFulfillmentFuture<'a>;
}

pub trait AdminMembershipStore {
    fn list_admin_membership_plans<'a>(
        &'a self,
        query: ListAdminMembershipPlansQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipPlanItem>>;

    fn create_admin_membership_plan<'a>(
        &'a self,
        command: CreateAdminMembershipPlanCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPlanItem>;

    fn update_admin_membership_plan<'a>(
        &'a self,
        command: UpdateAdminMembershipPlanCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPlanItem>;

    fn delete_admin_membership_plan<'a>(
        &'a self,
        command: DeleteAdminMembershipPlanCommand,
    ) -> AdminMembershipFuture<'a, bool>;

    fn list_admin_membership_packages<'a>(
        &'a self,
        query: ListAdminMembershipPackagesQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipPackageItem>>;

    fn list_admin_membership_package_groups<'a>(
        &'a self,
        query: ListAdminMembershipPackageGroupsQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipPackageGroupItem>>;

    fn create_admin_membership_package_group<'a>(
        &'a self,
        command: CreateAdminMembershipPackageGroupCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageGroupItem>;

    fn update_admin_membership_package_group<'a>(
        &'a self,
        command: UpdateAdminMembershipPackageGroupCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageGroupItem>;

    fn delete_admin_membership_package_group<'a>(
        &'a self,
        command: DeleteAdminMembershipPackageGroupCommand,
    ) -> AdminMembershipFuture<'a, bool>;

    fn create_admin_membership_package<'a>(
        &'a self,
        command: CreateAdminMembershipPackageCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageItem>;

    fn update_admin_membership_package<'a>(
        &'a self,
        command: UpdateAdminMembershipPackageCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipPackageItem>;

    fn delete_admin_membership_package<'a>(
        &'a self,
        command: DeleteAdminMembershipPackageCommand,
    ) -> AdminMembershipFuture<'a, bool>;

    fn list_admin_membership_members<'a>(
        &'a self,
        query: ListAdminMembershipMembersQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipMemberItem>>;

    fn update_admin_membership_member_status<'a>(
        &'a self,
        command: UpdateAdminMembershipMemberStatusCommand,
    ) -> AdminMembershipFuture<'a, AdminMembershipMemberItem>;

    fn list_admin_membership_entitlements<'a>(
        &'a self,
        query: ListAdminMembershipEntitlementsQuery,
    ) -> AdminMembershipFuture<'a, SdkWorkPageData<AdminMembershipEntitlementItem>>;
}
