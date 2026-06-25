use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use sdkwork_commerce_contract_service::CommerceServiceError;
use sdkwork_iam_context_service::IamAppContext;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, SqlitePool};

use crate::request_identity::with_request_identity;
use crate::{
    AppMembershipBenefitItem, AppMembershipCommandFuture, AppMembershipDailyRewardResponse,
    AppMembershipDailyRewardStatusResponse, AppMembershipEntityIdGenerator,
    AppMembershipInfoResponse, AppMembershipPackageGroupItem, AppMembershipPackageItem,
    AppMembershipPlanItem, AppMembershipPointsBalanceResponse, AppMembershipPointsHistoryItem,
    AppMembershipPointsHistoryQuery, AppMembershipPrivilegeUsageResponse, AppMembershipReadFuture,
    AppMembershipResult, AppMembershipStatusResponse, AppMembershipStore, AppMembershipSubject,
    PostgresCommerceMembershipStore, SqliteCommerceMembershipStore,
    SubmitMembershipPurchaseCommand,
};

const PAYMENT_EXPIRE_SECONDS: i64 = 1_800;

use crate::subject::numeric_runtime_subject_from_extension;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AppMembershipApiResult<T: Serialize> {
    code: String,
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> AppMembershipApiResult<T> {
    fn success(data: T) -> Self {
        Self {
            code: "2000".to_owned(),
            msg: "SUCCESS".to_owned(),
            data: Some(data),
        }
    }
}

impl AppMembershipApiResult<()> {
    fn error(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
            data: None,
        }
    }
}

struct AppMembershipState {
    store: Arc<dyn AppMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
    require_subject: bool,
}

impl Clone for AppMembershipState {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            entity_id_generator: Arc::clone(&self.entity_id_generator),
            require_subject: self.require_subject,
        }
    }
}

#[derive(Debug, Deserialize)]
struct MembershipCatalogQuery {
    plan_id: Option<i64>,
    recommended_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct MembershipBenefitQuery {
    plan_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct MembershipPackagesQuery {
    package_group_id: Option<i64>,
    plan_id: Option<i64>,
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

    fn load_plans<'a>(&'a self) -> AppMembershipReadFuture<'a, Vec<AppMembershipPlanItem>> {
        Box::pin(async { Ok(Vec::new()) })
    }

    fn load_benefits<'a>(
        &'a self,
        _subject: Option<AppMembershipSubject>,
        _plan_id: Option<i64>,
    ) -> AppMembershipReadFuture<'a, Vec<AppMembershipBenefitItem>> {
        Box::pin(async { Ok(Vec::new()) })
    }

    fn load_packages<'a>(
        &'a self,
        _package_group_id: Option<i64>,
        _plan_id: Option<i64>,
    ) -> AppMembershipReadFuture<'a, Vec<AppMembershipPackageItem>> {
        Box::pin(async { Ok(Vec::new()) })
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
    ) -> AppMembershipReadFuture<'a, Vec<AppMembershipPackageGroupItem>> {
        Box::pin(async { Ok(Vec::new()) })
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
        _query: AppMembershipPointsHistoryQuery,
    ) -> AppMembershipReadFuture<'a, Vec<AppMembershipPointsHistoryItem>> {
        Box::pin(async { Ok(Vec::new()) })
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

pub fn app_membership_router() -> Router {
    app_membership_router_with_state(
        Arc::new(EmptyAppMembershipStore),
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
    with_request_identity(
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
            }),
    )
}

async fn fetch_info(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state.store.load_info(subject).await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership info read model is unavailable", error)
        }
    }
}

async fn fetch_status(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state.store.load_status(subject).await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership status read model is unavailable", error)
        }
    }
}

async fn fetch_plans(State(state): State<AppMembershipState>) -> Response {
    match state.store.load_plans().await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership plans read model is unavailable", error)
        }
    }
}

async fn fetch_benefits(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(query): Query<MembershipBenefitQuery>,
) -> Response {
    let subject = match resolve_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state.store.load_benefits(subject, query.plan_id).await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership benefits read model is unavailable", error)
        }
    }
}

async fn fetch_package_groups(
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipCatalogQuery>,
) -> Response {
    match state
        .store
        .load_package_groups(query.plan_id, query.recommended_only.unwrap_or(false))
        .await
    {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership package groups read model is unavailable", error)
        }
    }
}

async fn fetch_package_group(
    State(state): State<AppMembershipState>,
    Path(package_group_id): Path<i64>,
) -> Response {
    match state.store.load_package_group(package_group_id).await {
        Ok(Some(data)) => Json(AppMembershipApiResult::success(data)).into_response(),
        Ok(None) => conflict_response("membership package group was not found"),
        Err(error) => {
            membership_system_response("membership package group read model is unavailable", error)
        }
    }
}

async fn fetch_package_group_packages(
    State(state): State<AppMembershipState>,
    Path(package_group_id): Path<i64>,
    Query(query): Query<MembershipPackagesQuery>,
) -> Response {
    match state
        .store
        .load_packages(Some(package_group_id), query.plan_id)
        .await
    {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership package read model is unavailable", error)
        }
    }
}

async fn fetch_packages(
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipPackagesQuery>,
) -> Response {
    match state
        .store
        .load_packages(query.package_group_id, query.plan_id)
        .await
    {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership package read model is unavailable", error)
        }
    }
}

async fn fetch_package(
    State(state): State<AppMembershipState>,
    Path(package_id): Path<i64>,
) -> Response {
    match state.store.load_package(package_id).await {
        Ok(Some(data)) => Json(AppMembershipApiResult::success(data)).into_response(),
        Ok(None) => conflict_response("membership package was not found"),
        Err(error) => {
            membership_system_response("membership package read model is unavailable", error)
        }
    }
}

async fn fetch_points_balance(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state.store.load_points_balance(subject).await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership points balance read model is unavailable", error)
        }
    }
}

async fn fetch_points_history(
    State(state): State<AppMembershipState>,
    Query(query): Query<MembershipPointsHistoryQuery>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
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
    {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership points history read model is unavailable", error)
        }
    }
}

async fn fetch_daily_reward_status(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state.store.load_daily_reward_status(subject).await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => {
            membership_system_response("membership daily reward status is unavailable", error)
        }
    }
}

async fn fetch_privilege_usage(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state.store.load_privilege_usage(subject).await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) => membership_system_response(
            "membership privilege usage read model is unavailable",
            error,
        ),
    }
}

async fn purchase(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(request): Json<SubmitMembershipPurchaseRequest>,
) -> Response {
    submit_purchase(state, runtime_context, request, "purchase").await
}

async fn renew(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(request): Json<SubmitMembershipPurchaseRequest>,
) -> Response {
    submit_purchase(state, runtime_context, request, "renew").await
}

async fn upgrade(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(request): Json<SubmitMembershipPurchaseRequest>,
) -> Response {
    submit_purchase(state, runtime_context, request, "upgrade").await
}

async fn submit_purchase(
    state: AppMembershipState,
    runtime_context: Option<Extension<IamAppContext>>,
    request: SubmitMembershipPurchaseRequest,
    action: &str,
) -> Response {
    let subject = match resolve_required_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let package_id = match validate_purchase_request(request) {
        Ok(value) => value,
        Err(message) => return bad_request_response(message),
    };
    let command = match build_submit_purchase_command(state.clone(), subject, package_id, action) {
        Ok(command) => command,
        Err(error) => {
            return membership_system_response("membership purchase command build failed", error)
        }
    };
    match state.store.submit_purchase(command).await {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => {
            membership_system_response("membership purchase command store is unavailable", error)
        }
    }
}

async fn claim_daily_reward(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_required_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .store
        .claim_daily_reward(subject, current_timestamp_string())
        .await
    {
        Ok(data) => Json(AppMembershipApiResult::success(data)).into_response(),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => {
            membership_system_response("membership daily reward command is unavailable", error)
        }
    }
}

async fn create_speed_up(
    State(state): State<AppMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match resolve_required_membership_subject(&state, runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .store
        .consume_speed_up(subject, current_timestamp_string())
        .await
    {
        Ok(()) => Json(AppMembershipApiResult::success(())).into_response(),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => {
            membership_system_response("membership speed up command is unavailable", error)
        }
    }
}

#[allow(clippy::result_large_err)]
fn resolve_membership_subject(
    state: &AppMembershipState,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Result<Option<AppMembershipSubject>, Response> {
    match app_membership_subject_from_extension(runtime_context) {
        Ok(subject) => Ok(Some(subject)),
        Err(error) if state.require_subject => Err(unauthorized_response(error)),
        Err(_) => Ok(None),
    }
}

#[allow(clippy::result_large_err)]
fn resolve_required_membership_subject(
    state: &AppMembershipState,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Result<AppMembershipSubject, Response> {
    match resolve_membership_subject(state, runtime_context)? {
        Some(subject) => Ok(subject),
        None => Err(unauthorized_response(
            "trusted request subject is required for membership command".to_owned(),
        )),
    }
}

fn app_membership_subject_from_extension(
    runtime_context: Option<Extension<IamAppContext>>,
) -> Result<AppMembershipSubject, String> {
    let subject = numeric_runtime_subject_from_extension(runtime_context)?;
    Ok(AppMembershipSubject {
        tenant_id: subject.tenant_id,
        organization_id: subject.organization_id,
        user_id: subject.user_id,
    })
}

fn validate_purchase_request(request: SubmitMembershipPurchaseRequest) -> Result<i64, String> {
    let package_id = request.package_id;
    if package_id <= 0 {
        return Err("membership package id must be greater than zero".to_owned());
    }
    let _ = request.coupon_id;
    Ok(package_id)
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn build_submit_purchase_command(
    state: AppMembershipState,
    subject: AppMembershipSubject,
    package_id: i64,
    action: &str,
) -> AppMembershipResult<SubmitMembershipPurchaseCommand> {
    let order_uuid = state.entity_id_generator.generate_entity_uuid()?;
    let order_item_uuid = state.entity_id_generator.generate_entity_uuid()?;
    let payment_uuid = state.entity_id_generator.generate_entity_uuid()?;
    let attempt_uuid = state.entity_id_generator.generate_entity_uuid()?;
    let membership_uuid = state.entity_id_generator.generate_entity_uuid()?;
    let nonce_uuid = state.entity_id_generator.generate_entity_uuid()?;
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
        requested_at,
        expire_at,
        action: action.to_owned(),
    })
}

fn bad_request_response(message: String) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(AppMembershipApiResult::error("4001", message)),
    )
        .into_response()
}

fn unauthorized_response(message: String) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(AppMembershipApiResult::error("4010", message)),
    )
        .into_response()
}

fn conflict_response(message: impl Into<String>) -> Response {
    (
        StatusCode::CONFLICT,
        Json(AppMembershipApiResult::error("4090", message)),
    )
        .into_response()
}

fn membership_system_response(context: &str, error: CommerceServiceError) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(AppMembershipApiResult::error(
            "5000",
            format!("{context}: {}", error.message()),
        )),
    )
        .into_response()
}

fn current_timestamp_string() -> String {
    format_unix_timestamp(current_unix_timestamp())
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
