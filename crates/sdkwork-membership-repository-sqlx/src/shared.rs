use std::collections::BTreeSet;

use chrono::{TimeZone, Utc};
use sdkwork_contract_service::{CommerceMoney, CommerceServiceError};
use serde_json::Value;

use crate::{
    AppMembershipBenefitItem, AppMembershipPackageGroupItem, AppMembershipPackageItem,
    AppMembershipPlanItem, AppMembershipPrivilegeUsageResponse, SubmitMembershipPurchaseCommand,
};

/// Trim and drop empty optional query/header values. Shared by app and admin routers.
pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.map(|v| v.trim().to_owned()).filter(|v| !v.is_empty())
}

/// Current UTC timestamp formatted as `%Y-%m-%d %H:%M:%S`. Shared by app and admin routers.
pub fn current_timestamp_string() -> String {
    format_unix_timestamp(Utc::now().timestamp())
}

/// Format a unix timestamp (seconds) as `%Y-%m-%d %H:%M:%S`. Shared by app and admin routers.
pub fn format_unix_timestamp(seconds: i64) -> String {
    Utc.timestamp_opt(seconds, 0)
        .single()
        .map(|ts| ts.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| format!("{seconds}"))
}

pub(crate) const POINTS_ASSET_CODE: &str = "points";
pub(crate) const POINTS_CURRENCY_CODE: &str = "POINT";

/// Default catalog tenant used by membership catalog queries (plans, packages,
/// package groups) whose trait signatures do not carry a request subject.
/// Replaces the previously inline `'100001'` literal scattered across SQL.
pub(crate) const DEFAULT_CATALOG_TENANT_ID: i64 = 100001;

/// Default catalog organization used by membership catalog queries. Replaces
/// the previously inline `'0'` literal scattered across SQL.
pub(crate) const DEFAULT_CATALOG_ORGANIZATION_ID: i64 = 0;

/// Resolve tenant scope for catalog reads from the authenticated subject, or the
/// seeded demo tenant for guest browsing.
pub(crate) fn resolve_catalog_scope(
    catalog_subject: Option<crate::AppMembershipSubject>,
) -> (i64, i64) {
    catalog_subject
        .map(|subject| (subject.tenant_id, subject.organization_id))
        .unwrap_or((DEFAULT_CATALOG_TENANT_ID, DEFAULT_CATALOG_ORGANIZATION_ID))
}

pub(crate) fn validate_membership_purchase_action(
    action: &str,
    membership_active: bool,
    current_rank: i64,
    target_plan_rank: i64,
) -> Result<(), CommerceServiceError> {
    match action.trim().to_ascii_lowercase().as_str() {
        "purchase" => Ok(()),
        "renew" => {
            if !membership_active {
                return Err(CommerceServiceError::validation(
                    "membership renew requires an active membership",
                ));
            }
            if target_plan_rank != current_rank {
                return Err(CommerceServiceError::validation(
                    "membership renew requires the same plan rank",
                ));
            }
            Ok(())
        }
        "upgrade" => {
            if !membership_active {
                return Err(CommerceServiceError::validation(
                    "membership upgrade requires an active membership",
                ));
            }
            if target_plan_rank <= current_rank {
                return Err(CommerceServiceError::validation(
                    "membership upgrade requires a higher plan rank",
                ));
            }
            Ok(())
        }
        _ => Err(CommerceServiceError::validation(
            "membership purchase action is invalid",
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
    _group_name: String,
    _group_description: Option<String>,
    _group_sort_weight: i64,
    plan_no: Option<String>,
    rank: i64,
    _sku_id: Option<String>,
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
        plan_no,
        rank,
        item,
    })
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedMembershipPackage {
    pub plan_no: String,
    pub rank: i64,
    pub item: AppMembershipPackageItem,
}

#[derive(Debug, Clone)]
pub(crate) struct CurrentMembershipSnapshot {
    pub membership_id: String,
    pub _rank: i64,
    pub _status: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MembershipPurchasePersistenceMode {
    New,
    Renew,
    Upgrade,
}

#[derive(Debug, Clone)]
pub(crate) struct MembershipPurchaseBinding {
    pub membership_uuid: String,
    pub period_starts_at: String,
    pub persistence_mode: MembershipPurchasePersistenceMode,
}

pub(crate) fn resolve_membership_purchase_binding(
    command: &SubmitMembershipPurchaseCommand,
    current: Option<CurrentMembershipSnapshot>,
    membership_active: bool,
) -> MembershipPurchaseBinding {
    let action = command.action.trim().to_ascii_lowercase();
    match action.as_str() {
        "renew" if membership_active => {
            let current = current.expect("renew requires active membership");
            MembershipPurchaseBinding {
                membership_uuid: current.membership_id.clone(),
                period_starts_at: later_membership_timestamp(
                    &current.expires_at,
                    &command.requested_at,
                ),
                persistence_mode: MembershipPurchasePersistenceMode::Renew,
            }
        }
        "upgrade" if membership_active => {
            let current = current.expect("upgrade requires active membership");
            MembershipPurchaseBinding {
                membership_uuid: current.membership_id.clone(),
                period_starts_at: command.requested_at.clone(),
                persistence_mode: MembershipPurchasePersistenceMode::Upgrade,
            }
        }
        _ => MembershipPurchaseBinding {
            membership_uuid: command.membership_uuid.clone(),
            period_starts_at: command.requested_at.clone(),
            persistence_mode: MembershipPurchasePersistenceMode::New,
        },
    }
}

pub(crate) fn later_membership_timestamp(current: &str, requested_at: &str) -> String {
    match (
        parse_membership_timestamp_seconds(current),
        parse_membership_timestamp_seconds(requested_at),
    ) {
        (Some(left), Some(right)) if left >= right => current.trim().to_owned(),
        (Some(_), Some(_)) => requested_at.trim().to_owned(),
        (Some(_), None) => current.trim().to_owned(),
        _ => requested_at.trim().to_owned(),
    }
}

fn parse_membership_timestamp_seconds(timestamp: &str) -> Option<i64> {
    let (date, time) = timestamp.trim().split_once(' ')?;
    let mut date_parts = date.split('-');
    let year = date_parts.next()?.parse::<i64>().ok()?;
    let month = date_parts.next()?.parse::<i64>().ok()?;
    let day = date_parts.next()?.parse::<i64>().ok()?;
    let mut time_parts = time.split(':');
    let hour = time_parts.next()?.parse::<i64>().ok()?;
    let minute = time_parts.next()?.parse::<i64>().ok()?;
    let second = time_parts.next()?.parse::<i64>().ok()?;
    Some(
        membership_days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second,
    )
}

fn membership_days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
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
        priority_queue_limit: benefit_limit(benefits, &["priority_queue"]),
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
