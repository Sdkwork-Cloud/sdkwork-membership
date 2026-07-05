//! App-API HTTP adapters for the membership surface (`/app/v3/api/memberships/*`).
//!
//! Handlers receive the canonical `WebRequestContext` injected by the
//! `sdkwork-web-framework` interceptor chain and emit responses through the
//! standard `SdkWorkApiResponse` / `application/problem+json` envelopes defined
//! in `API_SPEC.md` §15–§16. No handler hand-builds a wire envelope.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use sdkwork_contract_service::CommerceServiceError;
use serde::Deserialize;
use sqlx::{PgPool, SqlitePool};

use crate::catalog::{
    builtin_benefits, builtin_daily_reward_claim, builtin_daily_reward_status, builtin_info,
    builtin_package, builtin_package_group, builtin_package_groups, builtin_packages,
    builtin_plans, builtin_points_balance, builtin_points_history, builtin_privilege_usage,
    builtin_status,
};
use crate::pagination::{cursor_page, offset_page};
use crate::response::{finish_api_json, ApiProblem, ApiResult};
use crate::shared::{current_timestamp_string, format_unix_timestamp, normalize_optional_text};
use crate::subject::numeric_runtime_subject_from_context;
use sdkwork_utils_rust::SdkWorkPageData;
use crate::{
    AppMembershipBenefitItem, AppMembershipCommandFuture, AppMembershipDailyRewardResponse,
    AppMembershipDailyRewardStatusResponse, AppMembershipEntityIdGenerator,
    AppMembershipInfoResponse, AppMembershipListQuery, AppMembershipPackageGroupItem,
    AppMembershipPackageItem, AppMembershipPlanItem, AppMembershipPointsBalanceResponse,
    AppMembershipPointsHistoryItem, AppMembershipPointsHistoryQuery, AppMembershipPurchaseOutcome,
    AppMembershipPrivilegeUsageResponse, AppMembershipReadFuture, AppMembershipResult,
    AppMembershipStatusResponse, AppMembershipStore, AppMembershipSubject,
    PostgresCommerceMembershipStore, SqliteCommerceMembershipStore, SubmitMembershipPurchaseCommand,
};
use sdkwork_web_core::WebRequestContext;

const PAYMENT_EXPIRE_SECONDS: i64 = 1_800;

#[derive(Clone)]
struct AppMembershipState {
    store: Arc<dyn AppMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
    require_subject: bool,
}

#[derive(Debug, Deserialize)]
struct MembershipCatalogQuery {
    plan_id: Option<i64>,
    recommended_only: Option<bool>,
    page: Option<i64>,
    #[serde(rename = "page_size")]
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MembershipBenefitQuery {
    plan_id: Option<i64>,
    page: Option<i64>,
    #[serde(rename = "page_size")]
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MembershipPackagesQuery {
    package_group_id: Option<i64>,
    plan_id: Option<i64>,
    page: Option<i64>,
    #[serde(rename = "page_size")]
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MembershipPointsHistoryQuery {
    page: Option<i64>,
    #[serde(rename = "page_size")]
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubmitMembershipPurchaseRequest {
    package_id: i64,
    coupon_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct TimestampMembershipEntityIdGenerator {
    sequence: AtomicU64,
}

impl AppMembershipEntityIdGenerator for TimestampMembershipEntityIdGenerator {
    fn generate_entity_uuid(&self) -> AppMembershipResult<String> {
        let now = current_unix_timestamp();
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        Ok(format!("membership-{now}-{sequence:016x}"))
    }
}

fn list_query_from_params(
    page: Option<i64>,
    page_size: Option<i64>,
    cursor: Option<String>,
) -> AppMembershipListQuery {
    AppMembershipListQuery {
        page,
        page_size,
        cursor: normalize_optional_text(cursor),
    }
}

fn empty_list_page<T>(query: AppMembershipListQuery) -> SdkWorkPageData<T> {
    offset_page(Vec::new(), 0, query.offset_params())
}

struct EmptyAppMembershipStore;

impl AppMembershipStore for EmptyAppMembershipStore {
    fn load_info<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipInfoResponse> {
        Box::pin(async { Ok(AppMembershipInfoResponse::default()) })
    }

    fn load_status<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipStatusResponse> {
        Box::pin(async { Ok(AppMembershipStatusResponse::default()) })
    }

    fn load_plans<'a>(
        &'a self,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPlanItem>> {
        Box::pin(async move { Ok(empty_list_page(query)) })
    }

    fn load_benefits<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
        _plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipBenefitItem>> {
        Box::pin(async move { Ok(empty_list_page(query)) })
    }

    fn load_packages<'a>(
        &'a self,
        _package_group_id: Option<i64>,
        _plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageItem>> {
        Box::pin(async move { Ok(empty_list_page(query)) })
    }

    fn load_package<'a>(
        &'a self,
        _package_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageItem>> {
        Box::pin(async { Ok(None) })
    }

    fn load_package_groups<'a>(
        &'a self,
        _plan_id: Option<i64>,
        _recommended_only: bool,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageGroupItem>> {
        Box::pin(async move { Ok(empty_list_page(query)) })
    }

    fn load_package_group<'a>(
        &'a self,
        _package_group_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageGroupItem>> {
        Box::pin(async { Ok(None) })
    }

    fn load_points_balance<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPointsBalanceResponse> {
        Box::pin(async { Ok(AppMembershipPointsBalanceResponse::default()) })
    }

    fn load_points_history<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
        query: AppMembershipPointsHistoryQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPointsHistoryItem>> {
        Box::pin(async move {
            Ok(empty_list_page(AppMembershipListQuery {
                page: query.page,
                page_size: query.page_size,
                cursor: query.cursor,
            }))
        })
    }

    fn load_daily_reward_status<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardStatusResponse> {
        Box::pin(async { Ok(AppMembershipDailyRewardStatusResponse::default()) })
    }

    fn claim_daily_reward<'a>(
        &'a self,
        _subject: AppMembershipSubject,
        _requested_at: String,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardResponse> {
        Box::pin(async {
            Err(CommerceServiceError::storage(
                "membership command store is unavailable without database configuration",
            ))
        })
    }

    fn load_privilege_usage<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPrivilegeUsageResponse> {
        Box::pin(async { Ok(AppMembershipPrivilegeUsageResponse::default()) })
    }

    fn consume_speed_up<'a>(
        &'a self,
        _subject: AppMembershipSubject,
        _requested_at: String,
    ) -> AppMembershipReadFuture<'a, ()> {
        Box::pin(async {
            Err(CommerceServiceError::storage(
                "membership command store is unavailable without database configuration",
            ))
        })
    }

    fn submit_purchase<'a>(
        &'a self,
        _command: SubmitMembershipPurchaseCommand,
    ) -> AppMembershipCommandFuture<'a> {
        Box::pin(async {
            Err(CommerceServiceError::storage(
                "membership command store is unavailable without database configuration",
            ))
        })
    }
}

/// In-memory catalog used only by `app_membership_router_with_builtin_catalog()` for unit tests.
/// Production gateways MUST use the DB-backed store from `app_membership_router()`.
struct CatalogAppMembershipStore;

impl AppMembershipStore for CatalogAppMembershipStore {
    fn load_info<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipInfoResponse> {
        Box::pin(async { Ok(builtin_info()) })
    }

    fn load_status<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipStatusResponse> {
        Box::pin(async { Ok(builtin_status()) })
    }

    fn load_plans<'a>(
        &'a self,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPlanItem>> {
        Box::pin(async move {
            let params = query.offset_params();
            let items = builtin_plans();
            let total = items.len() as i64;
            let offset = params.offset as usize;
            let page_size = params.page_size as usize;
            let page_items = items
                .into_iter()
                .skip(offset)
                .take(page_size)
                .collect();
            Ok(offset_page(page_items, total, params))
        })
    }

    fn load_benefits<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipBenefitItem>> {
        Box::pin(async move {
            let params = query.offset_params();
            let items = builtin_benefits(plan_id);
            let total = items.len() as i64;
            let offset = params.offset as usize;
            let page_size = params.page_size as usize;
            let page_items = items
                .into_iter()
                .skip(offset)
                .take(page_size)
                .collect();
            Ok(offset_page(page_items, total, params))
        })
    }

    fn load_packages<'a>(
        &'a self,
        package_group_id: Option<i64>,
        plan_id: Option<i64>,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageItem>> {
        Box::pin(async move {
            let params = query.offset_params();
            let items = builtin_packages(package_group_id, plan_id);
            let total = items.len() as i64;
            let offset = params.offset as usize;
            let page_size = params.page_size as usize;
            let page_items = items
                .into_iter()
                .skip(offset)
                .take(page_size)
                .collect();
            Ok(offset_page(page_items, total, params))
        })
    }

    fn load_package<'a>(
        &'a self,
        package_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageItem>> {
        Box::pin(async move { Ok(builtin_package(package_id)) })
    }

    fn load_package_groups<'a>(
        &'a self,
        _plan_id: Option<i64>,
        _recommended_only: bool,
        query: AppMembershipListQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPackageGroupItem>> {
        Box::pin(async move {
            let params = query.offset_params();
            let items = builtin_package_groups();
            let total = items.len() as i64;
            let offset = params.offset as usize;
            let page_size = params.page_size as usize;
            let page_items = items
                .into_iter()
                .skip(offset)
                .take(page_size)
                .collect();
            Ok(offset_page(page_items, total, params))
        })
    }

    fn load_package_group<'a>(
        &'a self,
        package_group_id: i64,
    ) -> AppMembershipReadFuture<'a, Option<AppMembershipPackageGroupItem>> {
        Box::pin(async move { Ok(builtin_package_group(package_group_id)) })
    }

    fn load_points_balance<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPointsBalanceResponse> {
        Box::pin(async { Ok(builtin_points_balance()) })
    }

    fn load_points_history<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
        query: AppMembershipPointsHistoryQuery,
    ) -> AppMembershipReadFuture<'a, SdkWorkPageData<AppMembershipPointsHistoryItem>> {
        Box::pin(async move {
            let page_size = query.limit() as usize;
            let items = builtin_points_history();
            let has_more = items.len() > page_size;
            let page_items: Vec<_> = items.into_iter().take(page_size).collect();
            let next_cursor = has_more
                .then(|| page_items.last().map(|item| item.id.clone()))
                .flatten();
            Ok(cursor_page(page_items, page_size, next_cursor, has_more))
        })
    }

    fn load_daily_reward_status<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardStatusResponse> {
        Box::pin(async { Ok(builtin_daily_reward_status()) })
    }

    fn claim_daily_reward<'a>(
        &'a self,
        _subject: AppMembershipSubject,
        _requested_at: String,
    ) -> AppMembershipReadFuture<'a, AppMembershipDailyRewardResponse> {
        Box::pin(async { Ok(builtin_daily_reward_claim()) })
    }

    fn load_privilege_usage<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
    ) -> AppMembershipReadFuture<'a, AppMembershipPrivilegeUsageResponse> {
        Box::pin(async { Ok(builtin_privilege_usage()) })
    }

    fn consume_speed_up<'a>(
        &'a self,
        _subject: AppMembershipSubject,
        _requested_at: String,
    ) -> AppMembershipReadFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }

    fn submit_purchase<'a>(
        &'a self,
        command: SubmitMembershipPurchaseCommand,
    ) -> AppMembershipCommandFuture<'a> {
        Box::pin(async move {
            let package = builtin_package(command.package_id)
                .ok_or_else(|| CommerceServiceError::not_found("package not found"))?;
            let plan_rank = match package.plan_name.as_deref() {
                Some("基础会员") => 1,
                Some("标准会员") => 2,
                Some("高级会员") => 3,
                _ => 0,
            };
            Ok(AppMembershipPurchaseOutcome {
                success: true,
                request_no: command.order_no,
                order_id: command.order_uuid,
                provider_code: "mock".to_string(),
                payment_method: "alipay".to_string(),
                payment_product: "web".to_string(),
                next_action: "cashier".to_string(),
                payment_id: command.payment_uuid,
                cashier_url: "https://example.com/cashier".to_string(),
                qr_code_payload: String::new(),
                qr_code_image_url: None,
                request_payment_payload: None,
                package_id: command.package_id,
                package_name: package.name,
                amount: package.price,
                duration_days: package.duration_days,
                target_plan_rank: plan_rank,
                target_plan_name: package.plan_name.unwrap_or_default(),
                status: "pending".to_string(),
            })
        })
    }
}

pub fn app_membership_router() -> Router {
    app_membership_router_with_state(
        Arc::new(EmptyAppMembershipStore),
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
        false,
    )
}

pub fn app_membership_router_with_builtin_catalog() -> Router {
    app_membership_router_with_state(
        Arc::new(CatalogAppMembershipStore),
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
        false,
    )
}

pub fn app_membership_router_with_sqlite_pool(pool: SqlitePool) -> Router {
    app_membership_router_with_store(
        Arc::new(SqliteCommerceMembershipStore::new(pool)),
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
    )
}

pub fn app_membership_router_with_postgres_pool(pool: PgPool) -> Router {
    app_membership_router_with_store(
        Arc::new(PostgresCommerceMembershipStore::new(pool)),
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
    )
}

pub fn app_membership_router_with_store(
    store: Arc<dyn AppMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
) -> Router {
    app_membership_router_with_state(store, entity_id_generator, true)
}

fn app_membership_router_with_state(
    store: Arc<dyn AppMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
    require_subject: bool,
) -> Router {
    Router::new()
        .route("/app/v3/api/memberships/current", get(fetch_info))
        .route("/app/v3/api/memberships/current/status", get(fetch_status))
        .route("/app/v3/api/memberships/plans", get(fetch_plans))
        .route("/app/v3/api/memberships/benefits", get(fetch_benefits))
        .route(
            "/app/v3/api/memberships/package_groups",
            get(fetch_package_groups),
        )
        .route(
            "/app/v3/api/memberships/package_groups/{packageGroupId}",
            get(fetch_package_group),
        )
        .route(
            "/app/v3/api/memberships/package_groups/{packageGroupId}/packages",
            get(fetch_package_group_packages),
        )
        .route("/app/v3/api/memberships/packages", get(fetch_packages))
        .route(
            "/app/v3/api/memberships/packages/{packageId}",
            get(fetch_package),
        )
        .route("/app/v3/api/memberships/purchases", post(purchase))
        .route("/app/v3/api/memberships/purchases/renew", post(renew))
        .route("/app/v3/api/memberships/purchases/upgrade", post(upgrade))
        .route(
            "/app/v3/api/memberships/points/balance",
            get(fetch_points_balance),
        )
        .route(
            "/app/v3/api/memberships/points/history",
            get(fetch_points_history),
        )
        .route(
            "/app/v3/api/memberships/points/daily_rewards",
            post(claim_daily_reward),
        )
        .route(
            "/app/v3/api/memberships/points/daily_rewards/status",
            get(fetch_daily_reward_status),
        )
        .route(
            "/app/v3/api/memberships/privileges/usage",
            get(fetch_privilege_usage),
        )
        .route(
            "/app/v3/api/memberships/privileges/speed_ups",
            post(create_speed_up),
        )
        .with_state(AppMembershipState {
            store,
            entity_id_generator,
            require_subject,
        })
}

async fn fetch_info(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_membership_subject(&state, &ctx)?;
            let data = state
                .store
                .load_info(subject)
                .await
                .map_err(|e| ApiProblem::from_service("membership info read model is unavailable", e))?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_status(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_membership_subject(&state, &ctx)?;
            let data = state
                .store
                .load_status(subject)
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership status read model is unavailable", e)
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_plans(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipCatalogQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let data = state
                .store
                .load_plans(list_query_from_params(
                    query.page,
                    query.page_size,
                    query.cursor,
                ))
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership plans read model is unavailable", e)
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_benefits(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipBenefitQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_membership_subject(&state, &ctx)?;
            let data = state
                .store
                .load_benefits(
                    subject,
                    query.plan_id,
                    list_query_from_params(query.page, query.page_size, query.cursor),
                )
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership benefits read model is unavailable", e)
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_package_groups(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipCatalogQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let data = state
                .store
                .load_package_groups(
                    query.plan_id,
                    query.recommended_only.unwrap_or(false),
                    list_query_from_params(query.page, query.page_size, query.cursor),
                )
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership package groups read model is unavailable",
                        e,
                    )
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_package_group(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Path(package_group_id): Path<i64>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            state
                .store
                .load_package_group(package_group_id)
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership package group read model is unavailable",
                        e,
                    )
                })?
                .ok_or_else(|| {
                    ApiProblem::not_found("membership package group was not found")
                })
        }
        .await,
    )
}

async fn fetch_package_group_packages(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Path(package_group_id): Path<i64>,
    Query(query): Query<MembershipPackagesQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let data = state
                .store
                .load_packages(
                    Some(package_group_id),
                    query.plan_id,
                    list_query_from_params(query.page, query.page_size, query.cursor),
                )
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership package read model is unavailable", e)
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_packages(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipPackagesQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let data = state
                .store
                .load_packages(
                    query.package_group_id,
                    query.plan_id,
                    list_query_from_params(query.page, query.page_size, query.cursor),
                )
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership package read model is unavailable", e)
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_package(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Path(package_id): Path<i64>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            state
                .store
                .load_package(package_id)
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership package read model is unavailable", e)
                })?
                .ok_or_else(|| ApiProblem::not_found("membership package was not found"))
        }
        .await,
    )
}

async fn fetch_points_balance(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_membership_subject(&state, &ctx)?;
            let data = state
                .store
                .load_points_balance(subject)
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership points balance read model is unavailable",
                        e,
                    )
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_points_history(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipPointsHistoryQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_membership_subject(&state, &ctx)?;
            let data = state
                .store
                .load_points_history(
                    subject,
                    AppMembershipPointsHistoryQuery {
                        page: query.page,
                        page_size: query.page_size,
                        cursor: normalize_optional_text(query.cursor),
                    },
                )
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership points history read model is unavailable",
                        e,
                    )
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_daily_reward_status(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_membership_subject(&state, &ctx)?;
            let data = state
                .store
                .load_daily_reward_status(subject)
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership daily reward status is unavailable",
                        e,
                    )
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn fetch_privilege_usage(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_membership_subject(&state, &ctx)?;
            let data = state
                .store
                .load_privilege_usage(subject)
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership privilege usage read model is unavailable",
                        e,
                    )
                })?;
            Ok(data)
        }
        .await,
    )
}

async fn purchase(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Json(request): Json<SubmitMembershipPurchaseRequest>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        submit_purchase(&ctx, &state, request, "purchase").await,
    )
}

async fn renew(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Json(request): Json<SubmitMembershipPurchaseRequest>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        submit_purchase(&ctx, &state, request, "renew").await,
    )
}

async fn upgrade(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
    Json(request): Json<SubmitMembershipPurchaseRequest>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        submit_purchase(&ctx, &state, request, "upgrade").await,
    )
}

async fn claim_daily_reward(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_required_membership_subject(&state, &ctx)?;
            state
                .store
                .claim_daily_reward(subject, current_timestamp_string())
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership daily reward command is unavailable", e)
                })
        }
        .await,
    )
}

async fn create_speed_up(
    ctx: WebRequestContext,
    State(state): State<AppMembershipState>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = resolve_required_membership_subject(&state, &ctx)?;
            state
                .store
                .consume_speed_up(subject, current_timestamp_string())
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership speed up command is unavailable", e)
                })?;
            Ok(())
        }
        .await,
    )
}

async fn submit_purchase(
    ctx: &WebRequestContext,
    state: &AppMembershipState,
    request: SubmitMembershipPurchaseRequest,
    action: &str,
) -> ApiResult<AppMembershipPurchaseOutcome> {
    let subject = resolve_required_membership_subject(state, ctx)?;
    let package_id = validate_purchase_request(request)?;
    let idempotency_key = ctx.request_id.0.clone();
    let command = build_submit_purchase_command(state, subject, package_id, action, idempotency_key)?;
    state
        .store
        .submit_purchase(command)
        .await
        .map_err(|e| {
            ApiProblem::from_service("membership purchase command store is unavailable", e)
        })
}

fn resolve_membership_subject(
    state: &AppMembershipState,
    ctx: &WebRequestContext,
) -> Result<Option<AppMembershipSubject>, ApiProblem> {
    match app_membership_subject_from_context(ctx) {
        Ok(subject) => Ok(Some(subject)),
        Err(error) if state.require_subject => Err(error),
        Err(_) => Ok(None),
    }
}

fn resolve_required_membership_subject(
    state: &AppMembershipState,
    ctx: &WebRequestContext,
) -> Result<AppMembershipSubject, ApiProblem> {
    match resolve_membership_subject(state, ctx)? {
        Some(subject) => Ok(subject),
        None => Err(ApiProblem::unauthorized(
            "trusted request subject is required for membership command",
        )),
    }
}

fn app_membership_subject_from_context(
    ctx: &WebRequestContext,
) -> Result<AppMembershipSubject, ApiProblem> {
    let subject = numeric_runtime_subject_from_context(ctx).map_err(ApiProblem::unauthorized)?;
    Ok(AppMembershipSubject {
        tenant_id: subject.tenant_id,
        organization_id: subject.organization_id,
        user_id: subject.user_id,
    })
}

fn validate_purchase_request(
    request: SubmitMembershipPurchaseRequest,
) -> Result<i64, ApiProblem> {
    let package_id = request.package_id;
    if package_id <= 0 {
        return Err(ApiProblem::bad_request(
            "membership package id must be greater than zero",
        ));
    }
    if let Some(coupon_id) = request.coupon_id.as_deref() {
        if !coupon_id.trim().is_empty() {
            return Err(ApiProblem::bad_request(
                "membership coupon purchases are not supported",
            ));
        }
    }
    Ok(package_id)
}

fn build_submit_purchase_command(
    state: &AppMembershipState,
    subject: AppMembershipSubject,
    package_id: i64,
    action: &str,
    idempotency_key: String,
) -> Result<SubmitMembershipPurchaseCommand, ApiProblem> {
    let order_uuid = state
        .entity_id_generator
        .generate_entity_uuid()
        .map_err(|e| ApiProblem::from_service("membership purchase id generation failed", e))?;
    let order_item_uuid = state
        .entity_id_generator
        .generate_entity_uuid()
        .map_err(|e| ApiProblem::from_service("membership purchase id generation failed", e))?;
    let payment_uuid = state
        .entity_id_generator
        .generate_entity_uuid()
        .map_err(|e| ApiProblem::from_service("membership purchase id generation failed", e))?;
    let attempt_uuid = state
        .entity_id_generator
        .generate_entity_uuid()
        .map_err(|e| ApiProblem::from_service("membership purchase id generation failed", e))?;
    let membership_uuid = state
        .entity_id_generator
        .generate_entity_uuid()
        .map_err(|e| ApiProblem::from_service("membership purchase id generation failed", e))?;
    let nonce_uuid = state
        .entity_id_generator
        .generate_entity_uuid()
        .map_err(|e| ApiProblem::from_service("membership purchase id generation failed", e))?;
    let now = current_unix_timestamp();
    let token = compact_token(&nonce_uuid);
    let order_no = format!("MEMBERSHIP{now}{}", take_prefix(&token, 16));
    let out_trade_no = format!("MEMBERSHIPTRADE{now}{}", take_prefix(&token, 32));
    let requested_at = format_unix_timestamp(now);
    let expire_at = format_unix_timestamp(now + PAYMENT_EXPIRE_SECONDS);

    Ok(SubmitMembershipPurchaseCommand {
        subject,
        package_id,
        order_uuid,
        order_item_uuid,
        payment_uuid,
        attempt_uuid,
        membership_uuid,
        order_no,
        out_trade_no,
        idempotency_key,
        requested_at,
        expire_at,
        action: action.to_owned(),
    })
}

fn current_unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

fn compact_token(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect()
}

fn take_prefix(value: &str, max_len: usize) -> String {
    value.chars().take(max_len).collect()
}
