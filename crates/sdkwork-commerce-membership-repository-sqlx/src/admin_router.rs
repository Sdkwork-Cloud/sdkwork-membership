use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Bytes;
use axum::extract::{Extension, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, patch, put};
use axum::{Json, Router};
use sdkwork_commerce_contract_service::CommerceServiceError;
use sdkwork_iam_context_service::IamAppContext;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, SqlitePool};

use crate::request_identity::{resolve_request_id, with_backend_request_identity};
use crate::subject::numeric_runtime_subject_from_extension;
use crate::{
    AdminMembershipPackageGroupMutation, AdminMembershipPackageMutation,
    AdminMembershipPlanMutation, AdminMembershipStore, AdminMembershipSubject,
    AppMembershipBenefitItem, AppMembershipEntityIdGenerator, AppMembershipResult,
    CreateAdminMembershipPackageCommand, CreateAdminMembershipPackageGroupCommand,
    CreateAdminMembershipPlanCommand, DeleteAdminMembershipPackageCommand,
    DeleteAdminMembershipPackageGroupCommand, DeleteAdminMembershipPlanCommand,
    ListAdminMembershipEntitlementsQuery, ListAdminMembershipMembersQuery,
    ListAdminMembershipPackageGroupsQuery, ListAdminMembershipPackagesQuery,
    ListAdminMembershipPlansQuery, PostgresCommerceMembershipStore, SqliteCommerceMembershipStore,
    TimestampMembershipEntityIdGenerator, UpdateAdminMembershipMemberStatusCommand,
    UpdateAdminMembershipPackageCommand, UpdateAdminMembershipPackageGroupCommand,
    UpdateAdminMembershipPlanCommand,
};

const MAX_CODE_LEN: usize = 128;
const MAX_NAME_LEN: usize = 128;
const DEFAULT_OPERATOR_TYPE: i32 = 1;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipApiResult<T: Serialize> {
    code: String,
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> AdminMembershipApiResult<T> {
    fn success(data: T) -> Self {
        Self {
            code: "2000".to_owned(),
            msg: "SUCCESS".to_owned(),
            data: Some(data),
        }
    }
}

impl AdminMembershipApiResult<()> {
    fn error(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipListResponse<T: Serialize> {
    items: Vec<T>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipItemResponse<T: Serialize> {
    item: T,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipPackageDeleteResponse {
    deleted: bool,
    package_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipPlanDeleteResponse {
    deleted: bool,
    plan_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdminMembershipPackageGroupDeleteResponse {
    deleted: bool,
    package_group_id: String,
}

#[derive(Clone)]
struct AdminMembershipState {
    store: Arc<dyn AdminMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipStatusQuery {
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipPackagesQuery {
    #[serde(rename = "package_group_id")]
    package_group_id: Option<String>,
    #[serde(rename = "plan_id")]
    plan_id: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipMembershipsQuery {
    #[serde(rename = "user_id")]
    user_id: Option<String>,
    #[serde(rename = "plan_id")]
    plan_id: Option<String>,
    status: Option<String>,
    page: Option<i64>,
    #[serde(rename = "page_size")]
    page_size: Option<i64>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AdminMembershipEntitlementsQuery {
    #[serde(rename = "plan_id")]
    plan_id: Option<String>,
    #[serde(rename = "membership_id")]
    membership_id: Option<String>,
    status: Option<String>,
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

pub fn admin_membership_router_with_sqlite_pool(pool: SqlitePool) -> Router {
    admin_membership_router_with_store(
        Arc::new(SqliteCommerceMembershipStore::new(pool)),
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
    )
}

pub fn admin_membership_router_with_postgres_pool(pool: PgPool) -> Router {
    admin_membership_router_with_store(
        Arc::new(PostgresCommerceMembershipStore::new(pool)),
        Arc::new(TimestampMembershipEntityIdGenerator::default()),
    )
}

pub fn admin_membership_router_with_store(
    store: Arc<dyn AdminMembershipStore + Send + Sync>,
    entity_id_generator: Arc<dyn AppMembershipEntityIdGenerator + Send + Sync>,
) -> Router {
    with_backend_request_identity(
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
            .with_state(AdminMembershipState {
                store,
                entity_id_generator,
            }),
    )
}

async fn list_plans(
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipStatusQuery>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .store
        .list_admin_membership_plans(ListAdminMembershipPlansQuery {
            subject,
            status: params.status.and_then(normalize_optional_status),
        })
        .await
    {
        Ok(items) => list_response(items),
        Err(error) => system_response("membership plan read model is unavailable", error),
    }
}

async fn create_plan(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let request = match parse_body::<AdminMembershipPlanMutationRequest>(&body, "membership plan") {
        Ok(request) => request,
        Err(message) => return bad_request(message),
    };
    let input = match normalize_plan_mutation(request) {
        Ok(input) => input,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    let plan_id = match state.entity_id_generator.generate_entity_uuid() {
        Ok(value) => value,
        Err(error) => return system_response("membership plan id generation failed", error),
    };
    match state
        .store
        .create_admin_membership_plan(CreateAdminMembershipPlanCommand {
            subject,
            plan_id,
            input,
            request_id,
            requested_at: current_timestamp_string(),
        })
        .await
    {
        Ok(item) => item_response(item),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => system_response("membership plan command store is unavailable", error),
    }
}

async fn update_plan_by_id(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    Path(plan_id): Path<String>,
    body: Bytes,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let plan_id = match normalize_path_id(&plan_id, "membership plan id") {
        Ok(value) => value,
        Err(message) => return bad_request(message),
    };
    let request = match parse_body::<AdminMembershipPlanMutationRequest>(&body, "membership plan") {
        Ok(request) => request,
        Err(message) => return bad_request(message),
    };
    let input = match normalize_plan_mutation(request) {
        Ok(input) => input,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    let command = UpdateAdminMembershipPlanCommand {
        subject,
        plan_id,
        input,
        request_id,
        requested_at: current_timestamp_string(),
    };
    match state.store.update_admin_membership_plan(command).await {
        Ok(item) => item_response(item),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => system_response("membership plan command store is unavailable", error),
    }
}

async fn delete_plan(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    Path(plan_id): Path<String>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let plan_id = match normalize_path_id(&plan_id, "membership plan id") {
        Ok(value) => value,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    let command = DeleteAdminMembershipPlanCommand {
        subject,
        plan_id: plan_id.clone(),
        request_id,
        requested_at: current_timestamp_string(),
    };
    match state.store.delete_admin_membership_plan(command).await {
        Ok(true) => Json(AdminMembershipApiResult::success(
            AdminMembershipPlanDeleteResponse {
                deleted: true,
                plan_id,
            },
        ))
        .into_response(),
        Ok(false) => not_found_response("membership plan was not found"),
        Err(error) => system_response("membership plan command store is unavailable", error),
    }
}

async fn list_packages(
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipPackagesQuery>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .store
        .list_admin_membership_packages(ListAdminMembershipPackagesQuery {
            subject,
            package_group_id: normalize_optional_text(params.package_group_id),
            plan_id: normalize_optional_text(params.plan_id),
            status: params.status.and_then(normalize_optional_status),
        })
        .await
    {
        Ok(items) => list_response(items),
        Err(error) => system_response("membership package read model is unavailable", error),
    }
}

async fn create_package(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let request =
        match parse_body::<AdminMembershipPackageMutationRequest>(&body, "membership package") {
            Ok(request) => request,
            Err(message) => return bad_request(message),
        };
    let input = match normalize_package_mutation(request) {
        Ok(input) => input,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    let package_id = match state.entity_id_generator.generate_entity_uuid() {
        Ok(value) => value,
        Err(error) => return system_response("membership package id generation failed", error),
    };
    match state
        .store
        .create_admin_membership_package(CreateAdminMembershipPackageCommand {
            subject,
            package_id,
            input,
            request_id,
            requested_at: current_timestamp_string(),
        })
        .await
    {
        Ok(item) => item_response(item),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => system_response("membership package command store is unavailable", error),
    }
}

async fn update_package(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    Path(package_id): Path<String>,
    body: Bytes,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let package_id = match normalize_path_id(&package_id, "membership package id") {
        Ok(value) => value,
        Err(message) => return bad_request(message),
    };
    let request =
        match parse_body::<AdminMembershipPackageMutationRequest>(&body, "membership package") {
            Ok(request) => request,
            Err(message) => return bad_request(message),
        };
    let input = match normalize_package_mutation(request) {
        Ok(input) => input,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    match state
        .store
        .update_admin_membership_package(UpdateAdminMembershipPackageCommand {
            subject,
            package_id,
            input,
            request_id,
            requested_at: current_timestamp_string(),
        })
        .await
    {
        Ok(item) => item_response(item),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => system_response("membership package command store is unavailable", error),
    }
}

async fn delete_package(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    Path(package_id): Path<String>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let package_id = match normalize_path_id(&package_id, "membership package id") {
        Ok(value) => value,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    let command = DeleteAdminMembershipPackageCommand {
        subject,
        package_id: package_id.clone(),
        request_id,
        requested_at: current_timestamp_string(),
    };
    match state.store.delete_admin_membership_package(command).await {
        Ok(true) => Json(AdminMembershipApiResult::success(
            AdminMembershipPackageDeleteResponse {
                deleted: true,
                package_id,
            },
        ))
        .into_response(),
        Ok(false) => not_found_response("membership package was not found"),
        Err(error) => system_response("membership package command store is unavailable", error),
    }
}

async fn list_package_groups(
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipStatusQuery>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .store
        .list_admin_membership_package_groups(ListAdminMembershipPackageGroupsQuery {
            subject,
            status: params.status.and_then(normalize_optional_status),
        })
        .await
    {
        Ok(items) => list_response(items),
        Err(error) => system_response("membership package group read model is unavailable", error),
    }
}

async fn create_package_group(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let request = match parse_body::<AdminMembershipPackageGroupMutationRequest>(
        &body,
        "membership package group",
    ) {
        Ok(request) => request,
        Err(message) => return bad_request(message),
    };
    let input = match normalize_package_group_mutation(request) {
        Ok(input) => input,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    let package_group_id = match state.entity_id_generator.generate_entity_uuid() {
        Ok(value) => value,
        Err(error) => {
            return system_response("membership package group id generation failed", error)
        }
    };
    match state
        .store
        .create_admin_membership_package_group(CreateAdminMembershipPackageGroupCommand {
            subject,
            package_group_id,
            input,
            request_id,
            requested_at: current_timestamp_string(),
        })
        .await
    {
        Ok(item) => item_response(item),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => system_response(
            "membership package group command store is unavailable",
            error,
        ),
    }
}

async fn update_package_group(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    Path(package_group_id): Path<String>,
    body: Bytes,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let package_group_id = match normalize_path_id(&package_group_id, "membership package group id")
    {
        Ok(value) => value,
        Err(message) => return bad_request(message),
    };
    let request = match parse_body::<AdminMembershipPackageGroupMutationRequest>(
        &body,
        "membership package group",
    ) {
        Ok(request) => request,
        Err(message) => return bad_request(message),
    };
    let input = match normalize_package_group_mutation(request) {
        Ok(input) => input,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    match state
        .store
        .update_admin_membership_package_group(UpdateAdminMembershipPackageGroupCommand {
            subject,
            package_group_id,
            input,
            request_id,
            requested_at: current_timestamp_string(),
        })
        .await
    {
        Ok(item) => item_response(item),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => system_response(
            "membership package group command store is unavailable",
            error,
        ),
    }
}

async fn delete_package_group(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    Path(package_group_id): Path<String>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let package_group_id = match normalize_path_id(&package_group_id, "membership package group id")
    {
        Ok(value) => value,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
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
    {
        Ok(true) => Json(AdminMembershipApiResult::success(
            AdminMembershipPackageGroupDeleteResponse {
                deleted: true,
                package_group_id,
            },
        ))
        .into_response(),
        Ok(false) => not_found_response("membership package group was not found"),
        Err(error) => system_response(
            "membership package group command store is unavailable",
            error,
        ),
    }
}

async fn list_memberships(
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipMembershipsQuery>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let _ = (params.page, params.page_size, params.cursor);
    match state
        .store
        .list_admin_membership_members(ListAdminMembershipMembersQuery {
            subject,
            user_id: normalize_optional_text(params.user_id),
            plan_id: normalize_optional_text(params.plan_id),
            status: params.status.and_then(normalize_optional_membership_status),
        })
        .await
    {
        Ok(items) => list_response(items),
        Err(error) => system_response("membership membership read model is unavailable", error),
    }
}

async fn update_membership_status(
    State(state): State<AdminMembershipState>,
    runtime_context: Option<Extension<IamAppContext>>,
    headers: HeaderMap,
    Path(membership_id): Path<String>,
    body: Bytes,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let membership_id = match normalize_path_id(&membership_id, "membership membership id") {
        Ok(value) => value,
        Err(message) => return bad_request(message),
    };
    let request = match parse_body::<AdminMembershipMembershipStatusRequest>(
        &body,
        "membership membership",
    ) {
        Ok(request) => request,
        Err(message) => return bad_request(message),
    };
    let status = match normalize_membership_status(request.status.as_deref()) {
        Ok(status) => status,
        Err(message) => return bad_request(message),
    };
    let request_id = match request_id(&headers, &state) {
        Ok(request_id) => request_id,
        Err(error) => return command_error_response(error),
    };
    match state
        .store
        .update_admin_membership_member_status(UpdateAdminMembershipMemberStatusCommand {
            subject,
            membership_id,
            status,
            request_id,
            requested_at: current_timestamp_string(),
        })
        .await
    {
        Ok(item) => item_response(item),
        Err(error) if error.code() == "conflict" => conflict_response(error.message()),
        Err(error) => system_response("membership membership command store is unavailable", error),
    }
}

async fn list_entitlements(
    State(state): State<AdminMembershipState>,
    Query(params): Query<AdminMembershipEntitlementsQuery>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match admin_membership_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .store
        .list_admin_membership_entitlements(ListAdminMembershipEntitlementsQuery {
            subject,
            plan_id: normalize_optional_text(params.plan_id),
            membership_id: normalize_optional_text(params.membership_id),
            status: params
                .status
                .and_then(normalize_optional_entitlement_status),
        })
        .await
    {
        Ok(items) => list_response(items),
        Err(error) => system_response("membership entitlement read model is unavailable", error),
    }
}

#[allow(clippy::result_large_err)]
fn admin_membership_subject_from_extension(
    runtime_context: Option<Extension<IamAppContext>>,
) -> Result<AdminMembershipSubject, Response> {
    let subject =
        numeric_runtime_subject_from_extension(runtime_context).map_err(unauthorized_response)?;
    Ok(AdminMembershipSubject {
        tenant_id: subject.tenant_id,
        organization_id: subject.organization_id,
        operator_id: subject.user_id,
        operator_type: DEFAULT_OPERATOR_TYPE,
    })
}

fn parse_body<T>(body: &[u8], entity_name: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    if body.iter().all(u8::is_ascii_whitespace) {
        return Err(format!("{entity_name} request body is required"));
    }
    serde_json::from_slice(body)
        .map_err(|error| format!("invalid {entity_name} request body: {error}"))
}

fn normalize_plan_mutation(
    request: AdminMembershipPlanMutationRequest,
) -> Result<AdminMembershipPlanMutation, String> {
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
) -> Result<Option<Vec<AppMembershipBenefitItem>>, String> {
    let Some(items) = request else {
        return Ok(None);
    };

    let mut benefits = Vec::with_capacity(items.len());
    for (index, item) in items.into_iter().enumerate() {
        let id = item.id.unwrap_or((index + 1) as i64);
        if id < 0 {
            return Err("membership benefit id must be a non-negative integer".to_owned());
        }
        let usage_limit = item.usage_limit;
        if matches!(usage_limit, Some(value) if value < 0) {
            return Err("membership benefit usageLimit must be a non-negative integer".to_owned());
        }
        let used_count = item.used_count;
        if matches!(used_count, Some(value) if value < 0) {
            return Err("membership benefit usedCount must be a non-negative integer".to_owned());
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
            used_count,
        });
    }

    Ok(Some(benefits))
}

fn normalize_package_mutation(
    request: AdminMembershipPackageMutationRequest,
) -> Result<AdminMembershipPackageMutation, String> {
    let duration_days = request.duration_days.unwrap_or(0);
    if duration_days <= 0 {
        return Err("membership package durationDays must be a positive integer".to_owned());
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
        return Err("membership package currencyCode is invalid".to_owned());
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
) -> Result<AdminMembershipPackageGroupMutation, String> {
    let duration_days = request.duration_days.unwrap_or(0);
    if duration_days <= 0 {
        return Err("membership package group durationDays must be a positive integer".to_owned());
    }
    let sort_weight = request.sort_weight.unwrap_or(0);
    if sort_weight < 0 {
        return Err(
            "membership package group sortWeight must be a non-negative integer".to_owned(),
        );
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

fn normalize_code(value: Option<&str>, field_name: &str) -> Result<String, String> {
    let value = normalize_required_text(value, field_name, MAX_CODE_LEN)?;
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(format!(
            "{field_name} may only contain letters, numbers, -, and _"
        ));
    }
    Ok(value)
}

fn normalize_required_text(
    value: Option<&str>,
    field_name: &str,
    max_len: usize,
) -> Result<String, String> {
    let value = value.map(str::trim).unwrap_or("");
    if value.is_empty() {
        return Err(format!("{field_name} is required"));
    }
    if value.chars().count() > max_len {
        return Err(format!("{field_name} must be at most {max_len} characters"));
    }
    Ok(value.to_owned())
}

fn normalize_optional_bounded_text(
    value: Option<&str>,
    field_name: &str,
    max_len: usize,
) -> Result<Option<String>, String> {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    if value.chars().count() > max_len {
        return Err(format!("{field_name} must be at most {max_len} characters"));
    }
    Ok(Some(value.to_owned()))
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn normalize_money(value: Option<&str>, field_name: &str) -> Result<String, String> {
    let value = normalize_required_text(value, field_name, 64)?;
    if value.starts_with('-') || value.starts_with('+') {
        return Err(format!("{field_name} must be a non-negative money amount"));
    }
    let parts = value.split('.').collect::<Vec<_>>();
    if parts.len() > 2 || parts[0].is_empty() || !parts[0].chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("{field_name} must be a valid money amount"));
    }
    if let Some(fraction) = parts.get(1) {
        if fraction.len() > 2 || !fraction.chars().all(|ch| ch.is_ascii_digit()) {
            return Err(format!("{field_name} must have at most 2 decimal places"));
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
        .ok_or_else(|| format!("{field_name} is too large"))?;
    Ok(format!("{}.{:02}", cents / 100, cents.rem_euclid(100)))
}

fn normalize_status(value: Option<&str>) -> Result<String, String> {
    match value
        .unwrap_or("active")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "active" | "enabled" | "normal" => Ok("active".to_owned()),
        "inactive" => Ok("inactive".to_owned()),
        "disabled" => Ok("disabled".to_owned()),
        _ => Err("membership status must be active, inactive, or disabled".to_owned()),
    }
}

fn normalize_membership_status(value: Option<&str>) -> Result<String, String> {
    match value.unwrap_or("").trim().to_ascii_lowercase().as_str() {
        "active" | "inactive" | "expired" | "suspended" | "cancelled" => {
            Ok(value.unwrap().trim().to_ascii_lowercase())
        }
        _ => Err(
            "membership status must be active, inactive, expired, suspended, or cancelled"
                .to_owned(),
        ),
    }
}

fn normalize_optional_status(value: String) -> Option<String> {
    normalize_status(Some(&value)).ok()
}

fn normalize_optional_membership_status(value: String) -> Option<String> {
    normalize_membership_status(Some(&value)).ok()
}

fn normalize_optional_entitlement_status(value: String) -> Option<String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "active" | "inactive" | "disabled" | "exhausted" => Some(value.trim().to_ascii_lowercase()),
        _ => None,
    }
}

fn normalize_path_id(value: &str, field_name: &str) -> Result<String, String> {
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

fn request_id(headers: &HeaderMap, _state: &AdminMembershipState) -> AppMembershipResult<String> {
    resolve_request_id(headers).map_err(CommerceServiceError::validation)
}

fn list_response<T>(items: Vec<T>) -> Response
where
    T: Serialize,
{
    Json(AdminMembershipApiResult::success(
        AdminMembershipListResponse { items },
    ))
    .into_response()
}

fn item_response<T>(item: T) -> Response
where
    T: Serialize,
{
    Json(AdminMembershipApiResult::success(
        AdminMembershipItemResponse { item },
    ))
    .into_response()
}

fn bad_request(message: impl Into<String>) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(AdminMembershipApiResult::error("4001", message.into())),
    )
        .into_response()
}

fn unauthorized_response(message: impl Into<String>) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(AdminMembershipApiResult::error("4010", message.into())),
    )
        .into_response()
}

fn not_found_response(message: impl Into<String>) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(AdminMembershipApiResult::error("4040", message.into())),
    )
        .into_response()
}

fn conflict_response(message: impl Into<String>) -> Response {
    (
        StatusCode::CONFLICT,
        Json(AdminMembershipApiResult::error("4090", message.into())),
    )
        .into_response()
}

fn command_error_response(error: CommerceServiceError) -> Response {
    bad_request(error.message())
}

fn system_response(context: &str, error: CommerceServiceError) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(AdminMembershipApiResult::error(
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

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_web_core::REQUEST_ID_HEADER;

    #[tokio::test]
    async fn request_id_generates_canonical_uuid_when_upstream_header_is_absent() {
        let headers = HeaderMap::new();
        let state = test_state().await;

        let request_id = request_id(&headers, &state).expect("request id");

        assert!(is_canonical_uuid(&request_id), "{request_id}");
    }

    #[tokio::test]
    async fn request_id_overwrites_malformed_upstream_header() {
        let mut headers = HeaderMap::new();
        headers.insert(REQUEST_ID_HEADER, "browser-generated-id".parse().unwrap());
        let state = test_state().await;

        let request_id = request_id(&headers, &state).expect("request id");

        assert!(is_canonical_uuid(&request_id), "{request_id}");
        assert_ne!("browser-generated-id", request_id);
    }

    async fn test_state() -> AdminMembershipState {
        let pool = sdkwork_commerce_storage_repository_sqlx::commerce_sqlite_memory_pool().await;
        AdminMembershipState {
            store: Arc::new(SqliteCommerceMembershipStore::new(pool)),
            entity_id_generator: Arc::new(TimestampMembershipEntityIdGenerator::default()),
        }
    }

    fn is_canonical_uuid(value: &str) -> bool {
        let bytes = value.as_bytes();
        if bytes.len() != 36 {
            return false;
        }
        for (index, byte) in bytes.iter().enumerate() {
            if matches!(index, 8 | 13 | 18 | 23) {
                if *byte != b'-' {
                    return false;
                }
                continue;
            }
            if !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase() {
                return false;
            }
        }
        true
    }
}
