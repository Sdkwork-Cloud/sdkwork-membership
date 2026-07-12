//! Backend-API HTTP adapters for the membership surface (`/backend/v3/api/memberships/*`).
//!
//! Handlers receive the canonical `WebRequestContext` injected by the
//! `sdkwork-web-framework` interceptor chain and emit responses through the
//! standard `SdkWorkApiResponse` / `application/problem+json` envelopes defined
//! in `API_SPEC.md` §15–§16. No handler hand-builds a wire envelope.

use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::routing::{get, patch, post, put};
use axum::Router;
use serde::Deserialize;
use sqlx::{PgPool, SqlitePool};

use crate::response::{
    finish_api_created, finish_api_json, finish_api_no_content, ApiProblem, ItemEnvelope,
};
use crate::subject::numeric_runtime_subject_from_context;
use sdkwork_membership_repository_sqlx::shared::{
    current_timestamp_string, normalize_optional_text,
};
use sdkwork_membership_repository_sqlx::{
    AdminMembershipPackageGroupMutation, AdminMembershipPackageMutation,
    AdminMembershipPlanMutation, AdminMembershipStore, AdminMembershipSubject,
    AppMembershipBenefitItem, AppMembershipEntityIdGenerator, AppMembershipStore,
    AppMembershipSubject, CreateAdminMembershipPackageCommand,
    CreateAdminMembershipPackageGroupCommand, CreateAdminMembershipPlanCommand,
    DeleteAdminMembershipPackageCommand, DeleteAdminMembershipPackageGroupCommand,
    DeleteAdminMembershipPlanCommand, FulfillMembershipPurchaseCommand,
    ListAdminMembershipEntitlementsQuery, ListAdminMembershipMembersQuery,
    ListAdminMembershipPackageGroupsQuery, ListAdminMembershipPackagesQuery,
    ListAdminMembershipPlansQuery, PostgresCommerceMembershipStore, SqliteCommerceMembershipStore,
    TimestampMembershipEntityIdGenerator, UpdateAdminMembershipMemberStatusCommand,
    UpdateAdminMembershipPackageCommand, UpdateAdminMembershipPackageGroupCommand,
    UpdateAdminMembershipPlanCommand,
};
use sdkwork_web_core::WebRequestContext;

const MAX_CODE_LEN: usize = 128;
const MAX_NAME_LEN: usize = 128;
const DEFAULT_OPERATOR_TYPE: i32 = 1;

#[derive(Clone)]
struct AdminMembershipState {
    store: Arc<dyn AdminMembershipStore + Send + Sync>,
    app_store: Arc<dyn AppMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipStatusQuery {
    status: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipPackagesQuery {
    package_group_id: Option<String>,
    plan_id: Option<String>,
    status: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipMembershipsQuery {
    user_id: Option<String>,
    plan_id: Option<String>,
    status: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipEntitlementsQuery {
    plan_id: Option<String>,
    membership_id: Option<String>,
    status: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipPlanMutationRequest {
    code: Option<String>,
    name: Option<String>,
    rank: Option<i64>,
    benefits: Option<Vec<AdminMembershipBenefitMutationRequest>>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipBenefitMutationRequest {
    id: Option<i64>,
    name: Option<String>,
    benefit_key: Option<String>,
    r#type: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    claimed: Option<bool>,
    usage_limit: Option<i64>,
    used_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipPackageMutationRequest {
    code: Option<String>,
    package_group_id: Option<String>,
    plan_id: Option<String>,
    name: Option<String>,
    price_amount: Option<String>,
    currency_code: Option<String>,
    duration_days: Option<i64>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipPackageGroupMutationRequest {
    code: Option<String>,
    name: Option<String>,
    description: Option<String>,
    billing_cycle: Option<String>,
    duration_days: Option<i64>,
    sort_weight: Option<i64>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipMembershipStatusRequest {
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FulfillMembershipPurchaseRequest {
    tenant_id: i64,
    organization_id: i64,
    owner_user_id: i64,
    order_id: String,
    request_no: String,
    idempotency_key: String,
}

pub fn admin_membership_router_with_sqlite_pool(pool: SqlitePool) -> Router {
    let store = Arc::new(SqliteCommerceMembershipStore::new(pool));
    admin_membership_router_with_store(
        store.clone(),
        store,
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
    )
}

pub fn admin_membership_router_with_postgres_pool(pool: PgPool) -> Router {
    let store = Arc::new(PostgresCommerceMembershipStore::new(pool));
    admin_membership_router_with_store(
        store.clone(),
        store,
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
    )
}

pub fn admin_membership_router_with_store(
    store: Arc<dyn AdminMembershipStore + Send + Sync>,
    app_store: Arc<dyn AppMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
) -> Router {
    Router::new()
        .route(
            "/backend/v3/api/memberships/plans",
            get(list_plans).post(create_plan),
        )
        .route(
            "/backend/v3/api/memberships/plans/{planId}",
            put(update_plan_by_id).delete(delete_plan),
        )
        .route(
            "/backend/v3/api/memberships/packages",
            get(list_packages).post(create_package),
        )
        .route(
            "/backend/v3/api/memberships/package_groups",
            get(list_package_groups).post(create_package_group),
        )
        .route(
            "/backend/v3/api/memberships/package_groups/{packageGroupId}",
            put(update_package_group).delete(delete_package_group),
        )
        .route(
            "/backend/v3/api/memberships/packages/{packageId}",
            put(update_package).delete(delete_package),
        )
        .route("/backend/v3/api/memberships/members", get(list_memberships))
        .route(
            "/backend/v3/api/memberships/members/{membershipId}/status",
            patch(update_membership_status),
        )
        .route(
            "/backend/v3/api/memberships/entitlements",
            get(list_entitlements),
        )
        .route(
            "/backend/v3/api/memberships/purchases/fulfillments",
            post(fulfill_membership_purchase),
        )
        .with_state(AdminMembershipState {
            store,
            app_store,
            entity_id_generator,
        })
}

async fn list_plans(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipStatusQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let items = state
                .store
                .list_admin_membership_plans(ListAdminMembershipPlansQuery {
                    subject,
                    status: normalize_optional_status_filter(params.status)?,
                    page: params.page,
                    page_size: params.page_size,
                    cursor: normalize_optional_text(params.cursor),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership plan read model is unavailable", e)
                })?;
            Ok(items)
        }
        .await,
    )
}

async fn create_plan(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    finish_api_created(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let request =
                parse_body::<AdminMembershipPlanMutationRequest>(&body, "membership plan")?;
            let input = normalize_plan_mutation(request)?;
            let request_id = request_id(&headers)?;
            let plan_id = state
                .entity_id_generator
                .generate_entity_uuid()
                .map_err(|e| ApiProblem::from_service("membership plan id generation failed", e))?;
            let item = state
                .store
                .create_admin_membership_plan(CreateAdminMembershipPlanCommand {
                    subject,
                    plan_id,
                    input,
                    request_id,
                    requested_at: current_timestamp_string(),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership plan command store is unavailable", e)
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

async fn update_plan_by_id(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    Path(plan_id): Path<String>,
    body: Bytes,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let plan_id = normalize_path_id(&plan_id, "membership plan id")?;
            let request =
                parse_body::<AdminMembershipPlanMutationRequest>(&body, "membership plan")?;
            let input = normalize_plan_mutation(request)?;
            let request_id = request_id(&headers)?;
            let command = UpdateAdminMembershipPlanCommand {
                subject,
                plan_id,
                input,
                request_id,
                requested_at: current_timestamp_string(),
            };
            let item = state
                .store
                .update_admin_membership_plan(command)
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership plan command store is unavailable", e)
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

async fn delete_plan(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    Path(plan_id): Path<String>,
) -> axum::response::Response {
    finish_api_no_content(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let plan_id = normalize_path_id(&plan_id, "membership plan id")?;
            let request_id = request_id(&headers)?;
            let command = DeleteAdminMembershipPlanCommand {
                subject,
                plan_id: plan_id.clone(),
                request_id,
                requested_at: current_timestamp_string(),
            };
            match state
                .store
                .delete_admin_membership_plan(command)
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership plan command store is unavailable", e)
                })? {
                true => Ok(()),
                false => Err(ApiProblem::not_found("membership plan was not found")),
            }
        }
        .await,
    )
}

async fn list_packages(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipPackagesQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let items = state
                .store
                .list_admin_membership_packages(ListAdminMembershipPackagesQuery {
                    subject,
                    package_group_id: normalize_optional_text(params.package_group_id),
                    plan_id: normalize_optional_text(params.plan_id),
                    status: normalize_optional_status_filter(params.status)?,
                    page: params.page,
                    page_size: params.page_size,
                    cursor: normalize_optional_text(params.cursor),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership package read model is unavailable", e)
                })?;
            Ok(items)
        }
        .await,
    )
}

async fn create_package(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    finish_api_created(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let request =
                parse_body::<AdminMembershipPackageMutationRequest>(&body, "membership package")?;
            let input = normalize_package_mutation(request)?;
            let request_id = request_id(&headers)?;
            let package_id = state
                .entity_id_generator
                .generate_entity_uuid()
                .map_err(|e| {
                    ApiProblem::from_service("membership package id generation failed", e)
                })?;
            let item = state
                .store
                .create_admin_membership_package(CreateAdminMembershipPackageCommand {
                    subject,
                    package_id,
                    input,
                    request_id,
                    requested_at: current_timestamp_string(),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership package command store is unavailable", e)
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

async fn update_package(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    Path(package_id): Path<String>,
    body: Bytes,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let package_id = normalize_path_id(&package_id, "membership package id")?;
            let request =
                parse_body::<AdminMembershipPackageMutationRequest>(&body, "membership package")?;
            let input = normalize_package_mutation(request)?;
            let request_id = request_id(&headers)?;
            let item = state
                .store
                .update_admin_membership_package(UpdateAdminMembershipPackageCommand {
                    subject,
                    package_id,
                    input,
                    request_id,
                    requested_at: current_timestamp_string(),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership package command store is unavailable", e)
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

async fn delete_package(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    Path(package_id): Path<String>,
) -> axum::response::Response {
    finish_api_no_content(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let package_id = normalize_path_id(&package_id, "membership package id")?;
            let request_id = request_id(&headers)?;
            let command = DeleteAdminMembershipPackageCommand {
                subject,
                package_id: package_id.clone(),
                request_id,
                requested_at: current_timestamp_string(),
            };
            match state
                .store
                .delete_admin_membership_package(command)
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership package command store is unavailable", e)
                })? {
                true => Ok(()),
                false => Err(ApiProblem::not_found("membership package was not found")),
            }
        }
        .await,
    )
}

async fn list_package_groups(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipStatusQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let items = state
                .store
                .list_admin_membership_package_groups(ListAdminMembershipPackageGroupsQuery {
                    subject,
                    status: normalize_optional_status_filter(params.status)?,
                    page: params.page,
                    page_size: params.page_size,
                    cursor: normalize_optional_text(params.cursor),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership package group read model is unavailable",
                        e,
                    )
                })?;
            Ok(items)
        }
        .await,
    )
}

async fn create_package_group(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    finish_api_created(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let request = parse_body::<AdminMembershipPackageGroupMutationRequest>(
                &body,
                "membership package group",
            )?;
            let input = normalize_package_group_mutation(request)?;
            let request_id = request_id(&headers)?;
            let package_group_id =
                state
                    .entity_id_generator
                    .generate_entity_uuid()
                    .map_err(|e| {
                        ApiProblem::from_service("membership package group id generation failed", e)
                    })?;
            let item = state
                .store
                .create_admin_membership_package_group(CreateAdminMembershipPackageGroupCommand {
                    subject,
                    package_group_id,
                    input,
                    request_id,
                    requested_at: current_timestamp_string(),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership package group command store is unavailable",
                        e,
                    )
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

async fn update_package_group(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    Path(package_group_id): Path<String>,
    body: Bytes,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let package_group_id =
                normalize_path_id(&package_group_id, "membership package group id")?;
            let request = parse_body::<AdminMembershipPackageGroupMutationRequest>(
                &body,
                "membership package group",
            )?;
            let input = normalize_package_group_mutation(request)?;
            let request_id = request_id(&headers)?;
            let item = state
                .store
                .update_admin_membership_package_group(UpdateAdminMembershipPackageGroupCommand {
                    subject,
                    package_group_id,
                    input,
                    request_id,
                    requested_at: current_timestamp_string(),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership package group command store is unavailable",
                        e,
                    )
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

async fn delete_package_group(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    Path(package_group_id): Path<String>,
) -> axum::response::Response {
    finish_api_no_content(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let package_group_id =
                normalize_path_id(&package_group_id, "membership package group id")?;
            let request_id = request_id(&headers)?;
            let command = DeleteAdminMembershipPackageGroupCommand {
                subject,
                package_group_id: package_group_id.clone(),
                request_id,
                requested_at: current_timestamp_string(),
            };
            match state
                .store
                .delete_admin_membership_package_group(command)
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership package group command store is unavailable",
                        e,
                    )
                })? {
                true => Ok(()),
                false => Err(ApiProblem::not_found(
                    "membership package group was not found",
                )),
            }
        }
        .await,
    )
}

async fn list_memberships(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipMembershipsQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let items = state
                .store
                .list_admin_membership_members(ListAdminMembershipMembersQuery {
                    subject,
                    user_id: normalize_optional_text(params.user_id),
                    plan_id: normalize_optional_text(params.plan_id),
                    status: normalize_optional_membership_status_filter(params.status)?,
                    page: params.page,
                    page_size: params.page_size,
                    cursor: normalize_optional_text(params.cursor),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership membership read model is unavailable", e)
                })?;
            Ok(items)
        }
        .await,
    )
}

async fn update_membership_status(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    Path(membership_id): Path<String>,
    body: Bytes,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let membership_id = normalize_path_id(&membership_id, "membership membership id")?;
            let request = parse_body::<AdminMembershipMembershipStatusRequest>(
                &body,
                "membership membership",
            )?;
            let status = normalize_membership_status(request.status.as_deref())?;
            let request_id = request_id(&headers)?;
            let item = state
                .store
                .update_admin_membership_member_status(UpdateAdminMembershipMemberStatusCommand {
                    subject,
                    membership_id,
                    status,
                    request_id,
                    requested_at: current_timestamp_string(),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service(
                        "membership membership command store is unavailable",
                        e,
                    )
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

async fn list_entitlements(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipEntitlementsQuery>,
) -> axum::response::Response {
    finish_api_json(
        &ctx,
        async {
            let subject = admin_membership_subject_from_context(&ctx)?;
            let items = state
                .store
                .list_admin_membership_entitlements(ListAdminMembershipEntitlementsQuery {
                    subject,
                    plan_id: normalize_optional_text(params.plan_id),
                    membership_id: normalize_optional_text(params.membership_id),
                    status: normalize_optional_entitlement_status_filter(params.status)?,
                    page: params.page,
                    page_size: params.page_size,
                    cursor: normalize_optional_text(params.cursor),
                })
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership entitlement read model is unavailable", e)
                })?;
            Ok(items)
        }
        .await,
    )
}

async fn fulfill_membership_purchase(
    ctx: WebRequestContext,
    State(state): State<AdminMembershipState>,
    headers: HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    finish_api_created(
        &ctx,
        async {
            validate_fulfillment_service_auth(&headers)?;
            let request = parse_body::<FulfillMembershipPurchaseRequest>(
                &body,
                "membership purchase fulfillment",
            )?;
            let command = normalize_fulfillment_command(request)?;
            let item = state
                .app_store
                .fulfill_purchase(command)
                .await
                .map_err(|e| {
                    ApiProblem::from_service("membership fulfillment store is unavailable", e)
                })?;
            Ok(ItemEnvelope { item })
        }
        .await,
    )
}

fn validate_fulfillment_service_auth(headers: &HeaderMap) -> Result<(), ApiProblem> {
    let allow_insecure = std::env::var("MEMBERSHIP_FULFILL_ALLOW_INSECURE").as_deref() == Ok("1");
    let expected = std::env::var("SDKWORK_ACCESS_TOKEN")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    if expected.is_none() {
        if allow_insecure {
            return Ok(());
        }
        return Err(ApiProblem::unauthorized(
            "membership fulfillment service auth token is required",
        ));
    }

    let expected = expected.expect("membership fulfillment token checked above");
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let token = auth_header
        .strip_prefix("Bearer ")
        .or_else(|| auth_header.strip_prefix("bearer "))
        .unwrap_or_default()
        .trim();
    if token != expected {
        return Err(ApiProblem::unauthorized(
            "membership fulfillment service auth token is invalid",
        ));
    }
    Ok(())
}

fn normalize_fulfillment_command(
    request: FulfillMembershipPurchaseRequest,
) -> Result<FulfillMembershipPurchaseCommand, ApiProblem> {
    let order_id = request.order_id.trim().to_owned();
    if order_id.is_empty() {
        return Err(ApiProblem::bad_request("membership order id is required"));
    }
    let request_no = request.request_no.trim().to_owned();
    if request_no.is_empty() {
        return Err(ApiProblem::bad_request("membership request no is required"));
    }
    let idempotency_key = request.idempotency_key.trim().to_owned();
    if idempotency_key.is_empty() {
        return Err(ApiProblem::bad_request(
            "membership fulfillment idempotency key is required",
        ));
    }
    if request.tenant_id <= 0 || request.organization_id <= 0 || request.owner_user_id <= 0 {
        return Err(ApiProblem::bad_request(
            "membership fulfillment subject identifiers must be greater than zero",
        ));
    }

    Ok(FulfillMembershipPurchaseCommand {
        subject: AppMembershipSubject {
            tenant_id: request.tenant_id,
            organization_id: request.organization_id,
            user_id: request.owner_user_id,
        },
        order_id,
        request_no,
        idempotency_key,
    })
}

fn admin_membership_subject_from_context(
    ctx: &WebRequestContext,
) -> Result<AdminMembershipSubject, ApiProblem> {
    let subject = numeric_runtime_subject_from_context(ctx).map_err(ApiProblem::unauthorized)?;
    Ok(AdminMembershipSubject {
        tenant_id: subject.tenant_id,
        organization_id: subject.organization_id,
        operator_id: subject.user_id,
        operator_type: DEFAULT_OPERATOR_TYPE,
    })
}

fn parse_body<T>(body: &[u8], entity_name: &str) -> Result<T, ApiProblem>
where
    T: for<'de> Deserialize<'de>,
{
    if body.iter().all(u8::is_ascii_whitespace) {
        return Err(ApiProblem::bad_request(format!(
            "{entity_name} request body is required"
        )));
    }
    serde_json::from_slice(body).map_err(|error| {
        ApiProblem::bad_request(format!("invalid {entity_name} request body: {error}"))
    })
}

fn normalize_plan_mutation(
    request: AdminMembershipPlanMutationRequest,
) -> Result<AdminMembershipPlanMutation, ApiProblem> {
    let code = normalize_code(request.code.as_deref(), "membership plan code")?;
    let benefits = normalize_plan_benefits(request.benefits)?;
    Ok(AdminMembershipPlanMutation {
        name: normalize_required_text(
            request.name.as_deref(),
            "membership plan name",
            MAX_NAME_LEN,
        )?,
        rank: request.rank.unwrap_or_else(|| rank_from_plan_code(&code)),
        benefits,
        status: normalize_status(request.status.as_deref())?,
        code,
    })
}

fn normalize_plan_benefits(
    request: Option<Vec<AdminMembershipBenefitMutationRequest>>,
) -> Result<Option<Vec<AppMembershipBenefitItem>>, ApiProblem> {
    let Some(items) = request else {
        return Ok(None);
    };

    let mut benefits = Vec::with_capacity(items.len());
    for (index, item) in items.into_iter().enumerate() {
        let id = item.id.unwrap_or((index + 1) as i64);
        if id < 0 {
            return Err(ApiProblem::bad_request(
                "membership benefit id must be a non-negative integer",
            ));
        }
        let usage_limit = item.usage_limit;
        if matches!(usage_limit, Some(value) if value < 0) {
            return Err(ApiProblem::bad_request(
                "membership benefit usageLimit must be a non-negative integer",
            ));
        }
        let used_count = item.used_count;
        if matches!(used_count, Some(value) if value < 0) {
            return Err(ApiProblem::bad_request(
                "membership benefit usedCount must be a non-negative integer",
            ));
        }
        benefits.push(AppMembershipBenefitItem {
            id,
            name: normalize_required_text(
                item.name.as_deref(),
                "membership benefit name",
                MAX_NAME_LEN,
            )?,
            benefit_key: normalize_optional_bounded_text(
                item.benefit_key.as_deref(),
                "membership benefit benefitKey",
                MAX_CODE_LEN,
            )?,
            r#type: normalize_optional_bounded_text(
                item.r#type.as_deref(),
                "membership benefit type",
                64,
            )?,
            description: normalize_optional_bounded_text(
                item.description.as_deref(),
                "membership benefit description",
                512,
            )?,
            icon: normalize_optional_bounded_text(
                item.icon.as_deref(),
                "membership benefit icon",
                512,
            )?,
            claimed: item.claimed.unwrap_or(false),
            usage_limit,
            display_value: None,
            used_count,
        });
    }

    Ok(Some(benefits))
}

fn normalize_package_mutation(
    request: AdminMembershipPackageMutationRequest,
) -> Result<AdminMembershipPackageMutation, ApiProblem> {
    let duration_days = request.duration_days.unwrap_or(0);
    if duration_days <= 0 {
        return Err(ApiProblem::bad_request(
            "membership package durationDays must be a positive integer",
        ));
    }
    let currency_code = request
        .currency_code
        .as_deref()
        .unwrap_or("CNY")
        .trim()
        .to_ascii_uppercase();
    if currency_code.is_empty()
        || currency_code.chars().count() > 16
        || !currency_code
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(ApiProblem::bad_request(
            "membership package currencyCode is invalid",
        ));
    }
    Ok(AdminMembershipPackageMutation {
        code: normalize_code(request.code.as_deref(), "membership package code")?,
        package_group_id: normalize_required_text(
            request.package_group_id.as_deref(),
            "membership package packageGroupId",
            128,
        )?,
        plan_id: normalize_required_text(
            request.plan_id.as_deref(),
            "membership package planId",
            128,
        )?,
        name: normalize_required_text(
            request.name.as_deref(),
            "membership package name",
            MAX_NAME_LEN,
        )?,
        price_amount: normalize_money(
            request.price_amount.as_deref(),
            "membership package priceAmount",
        )?,
        currency_code,
        duration_days,
        status: normalize_status(request.status.as_deref())?,
    })
}

fn normalize_package_group_mutation(
    request: AdminMembershipPackageGroupMutationRequest,
) -> Result<AdminMembershipPackageGroupMutation, ApiProblem> {
    let duration_days = request.duration_days.unwrap_or(0);
    if duration_days <= 0 {
        return Err(ApiProblem::bad_request(
            "membership package group durationDays must be a positive integer",
        ));
    }
    let sort_weight = request.sort_weight.unwrap_or(0);
    if sort_weight < 0 {
        return Err(ApiProblem::bad_request(
            "membership package group sortWeight must be a non-negative integer",
        ));
    }
    Ok(AdminMembershipPackageGroupMutation {
        code: normalize_code(request.code.as_deref(), "membership package group code")?,
        name: normalize_required_text(
            request.name.as_deref(),
            "membership package group name",
            MAX_NAME_LEN,
        )?,
        description: normalize_optional_bounded_text(
            request.description.as_deref(),
            "membership package group description",
            512,
        )?,
        billing_cycle: normalize_required_text(
            request.billing_cycle.as_deref(),
            "membership package group billingCycle",
            64,
        )?,
        duration_days,
        sort_weight,
        status: normalize_status(request.status.as_deref())?,
    })
}

fn normalize_code(value: Option<&str>, field_name: &str) -> Result<String, ApiProblem> {
    let value = normalize_required_text(value, field_name, MAX_CODE_LEN)?;
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(ApiProblem::bad_request(format!(
            "{field_name} may only contain letters, numbers, -, and _"
        )));
    }
    Ok(value)
}

fn normalize_required_text(
    value: Option<&str>,
    field_name: &str,
    max_len: usize,
) -> Result<String, ApiProblem> {
    let value = value.map(str::trim).unwrap_or("");
    if value.is_empty() {
        return Err(ApiProblem::bad_request(format!("{field_name} is required")));
    }
    if value.chars().count() > max_len {
        return Err(ApiProblem::bad_request(format!(
            "{field_name} must be at most {max_len} characters"
        )));
    }
    Ok(value.to_owned())
}

fn normalize_optional_bounded_text(
    value: Option<&str>,
    field_name: &str,
    max_len: usize,
) -> Result<Option<String>, ApiProblem> {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    if value.chars().count() > max_len {
        return Err(ApiProblem::bad_request(format!(
            "{field_name} must be at most {max_len} characters"
        )));
    }
    Ok(Some(value.to_owned()))
}

fn normalize_money(value: Option<&str>, field_name: &str) -> Result<String, ApiProblem> {
    let value = normalize_required_text(value, field_name, 64)?;
    if value.starts_with('-') || value.starts_with('+') {
        return Err(ApiProblem::bad_request(format!(
            "{field_name} must be a non-negative money amount"
        )));
    }
    let parts = value.split('.').collect::<Vec<_>>();
    if parts.len() > 2 || parts[0].is_empty() || !parts[0].chars().all(|ch| ch.is_ascii_digit()) {
        return Err(ApiProblem::bad_request(format!(
            "{field_name} must be a valid money amount"
        )));
    }
    if let Some(fraction) = parts.get(1) {
        if fraction.len() > 2 || !fraction.chars().all(|ch| ch.is_ascii_digit()) {
            return Err(ApiProblem::bad_request(format!(
                "{field_name} must have at most 2 decimal places"
            )));
        }
    }
    let cents = parts
        .first()
        .and_then(|value| value.parse::<i64>().ok())
        .and_then(|whole| whole.checked_mul(100))
        .and_then(|whole_cents| {
            let fraction = parts.get(1).copied().unwrap_or("");
            let mut fraction = fraction.to_owned();
            while fraction.len() < 2 {
                fraction.push('0');
            }
            whole_cents.checked_add(fraction.parse::<i64>().unwrap_or(0))
        })
        .ok_or_else(|| ApiProblem::bad_request(format!("{field_name} is too large")))?;
    Ok(format!("{}.{:02}", cents / 100, cents.rem_euclid(100)))
}

fn normalize_status(value: Option<&str>) -> Result<String, ApiProblem> {
    match value
        .unwrap_or("active")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "active" | "enabled" | "normal" => Ok("active".to_owned()),
        "inactive" => Ok("inactive".to_owned()),
        "disabled" => Ok("disabled".to_owned()),
        _ => Err(ApiProblem::bad_request(
            "membership status must be active, inactive, or disabled",
        )),
    }
}

fn normalize_membership_status(value: Option<&str>) -> Result<String, ApiProblem> {
    let normalized = value.unwrap_or("").trim().to_ascii_lowercase();
    match normalized.as_str() {
        "active" | "inactive" | "expired" | "suspended" | "cancelled" => Ok(normalized),
        _ => Err(ApiProblem::bad_request(
            "membership status must be active, inactive, expired, suspended, or cancelled",
        )),
    }
}

fn normalize_optional_status_filter(value: Option<String>) -> Result<Option<String>, ApiProblem> {
    match value {
        None => Ok(None),
        Some(raw) if raw.trim().is_empty() => Ok(None),
        Some(raw) => normalize_status(Some(&raw)).map(Some),
    }
}

fn normalize_optional_membership_status_filter(
    value: Option<String>,
) -> Result<Option<String>, ApiProblem> {
    match value {
        None => Ok(None),
        Some(raw) if raw.trim().is_empty() => Ok(None),
        Some(raw) => normalize_membership_status(Some(&raw)).map(Some),
    }
}

fn normalize_optional_entitlement_status_filter(
    value: Option<String>,
) -> Result<Option<String>, ApiProblem> {
    match value {
        None => Ok(None),
        Some(raw) if raw.trim().is_empty() => Ok(None),
        Some(raw) => match raw.trim().to_ascii_lowercase().as_str() {
            "active" | "inactive" | "disabled" | "exhausted" => {
                Ok(Some(raw.trim().to_ascii_lowercase()))
            }
            _ => Err(ApiProblem::bad_request(
                "entitlement status must be active, inactive, disabled, or exhausted",
            )),
        },
    }
}

fn normalize_path_id(value: &str, field_name: &str) -> Result<String, ApiProblem> {
    normalize_required_text(Some(value), field_name, 128)
}

fn rank_from_plan_code(code: &str) -> i64 {
    match code.trim().to_ascii_lowercase().as_str() {
        "pro" => 1,
        "max" => 2,
        "vip" => 3,
        _ => 0,
    }
}

fn request_id(headers: &HeaderMap) -> Result<String, ApiProblem> {
    sdkwork_web_core::resolve_request_id(headers)
        .map_err(|e| ApiProblem::bad_request(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_utils_rust::validation::is_uuid;
    use sdkwork_web_core::REQUEST_ID_HEADER;

    #[tokio::test]
    async fn request_id_generates_canonical_uuid_when_upstream_header_is_absent() {
        let headers = HeaderMap::new();

        let request_id = request_id(&headers).expect("request id");

        assert!(is_uuid(&request_id), "{request_id}");
    }

    #[tokio::test]
    async fn request_id_overwrites_malformed_upstream_header() {
        let mut headers = HeaderMap::new();
        headers.insert(REQUEST_ID_HEADER, "browser-generated-id".parse().unwrap());

        let request_id = request_id(&headers).expect("request id");

        assert!(is_uuid(&request_id), "{request_id}");
        assert_ne!("browser-generated-id", request_id);
    }
}
