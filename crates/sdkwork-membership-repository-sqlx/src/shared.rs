use std::collections::BTreeSet;

use sdkwork_contract_service::{CommerceMoney, CommerceServiceError};
use serde_json::Value;

use crate::{
    AppMembershipBenefitItem, AppMembershipPackageGroupItem, AppMembershipPackageItem,
    AppMembershipPlanItem, AppMembershipPrivilegeUsageResponse,
};

pub(crate) const POINTS_ASSET_TYPE: &str = "points";
pub(crate) const POINTS_CURRENCY_CODE: &str = "POINT";

pub(crate) fn normalize_payment_method(method: &str) -> String {
    method.trim().to_ascii_lowercase()
}

pub(crate) fn payment_product_for_scan_qr(
    method: &str,
) -> Result<&'static str, CommerceServiceError> {
    match method.trim().to_ascii_lowercase().as_str() {
        "wechat_pay" => Ok("wechat_native"),
        "alipay" => Ok("alipay_page"),
        "paypal" => Ok("paypal_checkout"),
        "card" => Ok("card"),
        "apple_pay" => Ok("apple_pay"),
        "google_pay" => Ok("google_pay"),
        "wallet_balance" => Ok("wallet_balance"),
        _ => Err(CommerceServiceError::conflict(
            "membership payment method is unavailable",
        )),
    }
}

pub(crate) fn plan_rank_from_code(plan_no: &str) -> i64 {
    match plan_no.trim().to_ascii_lowercase().as_str() {
        "free" => 0,
        "pro" => 1,
        "max" => 2,
        "vip" => 3,
        _ => 0,
    }
}

pub(crate) fn plan_code_from_rank(rank: i64) -> &'static str {
    match rank {
        1 => "pro",
        2 => "max",
        3 => "vip",
        _ => "free",
    }
}

pub(crate) fn default_plan_name(rank: i64) -> &'static str {
    match rank {
        1 => "Pro member",
        2 => "Max member",
        3 => "VIP member",
        _ => "Free",
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn map_membership_package_record(
    id: i64,
    name: String,
    description: Option<String>,
    price: String,
    original_price: Option<String>,
    point_amount: i64,
    duration_days: i64,
    plan_name: Option<String>,
    sort_weight: i64,
    recommended: bool,
    tags_json: &str,
    group_external_id: i64,
    group_name: String,
    group_description: Option<String>,
    group_sort_weight: i64,
    plan_no: Option<String>,
    rank: i64,
    sku_id: Option<String>,
) -> Option<ParsedMembershipPackage> {
    if id <= 0 || group_external_id <= 0 {
        return None;
    }
    let plan_no = plan_no.unwrap_or_else(|| plan_code_from_rank(rank).to_owned());
    let rank = if rank == 0 {
        plan_rank_from_code(&plan_no)
    } else {
        rank
    };
    let item = AppMembershipPackageItem {
        id,
        name,
        description,
        price,
        original_price,
        point_amount: point_amount.max(0),
        duration_days: duration_days.max(0),
        plan_name: plan_name.or_else(|| Some(default_plan_name(rank).to_owned())),
        sort_weight,
        recommended,
        tags: string_array_from_json(tags_json),
    };
    Some(ParsedMembershipPackage {
        group_external_id,
        group_name,
        group_description,
        group_sort_weight,
        plan_no,
        rank,
        sku_id,
        item,
    })
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedMembershipPackage {
    pub group_external_id: i64,
    pub group_name: String,
    pub group_description: Option<String>,
    pub group_sort_weight: i64,
    pub plan_no: String,
    pub rank: i64,
    pub sku_id: Option<String>,
    pub item: AppMembershipPackageItem,
}

pub(crate) fn build_package_group_from_packages(
    package_group_id: i64,
    name: String,
    description: Option<String>,
    sort_weight: i64,
    packages: Vec<AppMembershipPackageItem>,
) -> AppMembershipPackageGroupItem {
    AppMembershipPackageGroupItem {
        id: package_group_id,
        name,
        description,
        sort_weight,
        packages,
    }
}

#[allow(dead_code)]
pub(crate) fn default_free_plan() -> AppMembershipPlanItem {
    AppMembershipPlanItem {
        id: 0,
        name: "Free".to_owned(),
        rank: 0,
        required_points: Some(0),
        description: Some(
            "Basic model catalog access, public routes, and a small trial quota.".to_owned(),
        ),
        icon: None,
        badge: Some("Free".to_owned()),
    }
}

pub(crate) fn benefits_for_plan(
    plans: &[StoredMembershipPlan],
    rank: i64,
) -> Vec<AppMembershipBenefitItem> {
    plans
        .iter()
        .find(|item| item.rank == rank)
        .or_else(|| {
            if matches!(rank, 0..=3) {
                let code = plan_code_from_rank(rank);
                return plans.iter().find(|item| item.plan_no == code);
            }
            None
        })
        .map(|item| item.benefits.clone())
        .unwrap_or_default()
}

#[derive(Debug, Clone)]
pub(crate) struct StoredMembershipPlan {
    pub id: i64,
    pub storage_id: String,
    pub plan_no: String,
    pub item: AppMembershipPlanItem,
    pub benefits: Vec<AppMembershipBenefitItem>,
    pub rank: i64,
}

pub(crate) fn privilege_usage_from_benefits(
    benefits: &[AppMembershipBenefitItem],
) -> AppMembershipPrivilegeUsageResponse {
    AppMembershipPrivilegeUsageResponse {
        speed_up_used: 0,
        speed_up_limit: benefit_limit(benefits, &["priority_speed_up"]),
        priority_queue_used: 0,
        priority_queue_limit: benefit_limit(benefits, &["priority_speed_up"]),
        exclusive_model_used: 0,
        exclusive_model_limit: benefit_limit(benefits, &["ai_quota"]),
    }
}

fn benefit_limit(benefits: &[AppMembershipBenefitItem], keys: &[&str]) -> i64 {
    let key_set = keys.iter().copied().collect::<BTreeSet<_>>();
    benefits
        .iter()
        .find_map(|item| {
            let key = item.benefit_key.as_deref()?;
            if key_set.contains(key) {
                item.usage_limit
            } else {
                None
            }
        })
        .unwrap_or(0)
}

pub(crate) fn decimal_string(
    value: &str,
    field_name: &str,
) -> Result<String, CommerceServiceError> {
    CommerceMoney::new(value)
        .map(|amount| amount.as_str().to_owned())
        .map_err(|_| CommerceServiceError::storage(format!("invalid {field_name}: {value}")))
}

pub(crate) fn parse_points_amount(value: &str) -> i64 {
    let normalized = value.trim();
    if normalized.is_empty() {
        return 0;
    }
    let unsigned = normalized.trim_start_matches('+');
    let Some(integer_part) = unsigned.split('.').next() else {
        return 0;
    };
    integer_part.parse::<i64>().unwrap_or(0)
}

fn string_array_from_json(value: &str) -> Vec<String> {
    serde_json::from_str::<Value>(value)
        .ok()
        .and_then(|parsed| parsed.as_array().cloned())
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}
