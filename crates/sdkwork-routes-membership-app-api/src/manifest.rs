//! Route manifest for the membership app-api surface.
//!
//! Every route entry declares `requestContext: WebRequestContext`
//! and `apiSurface: app-api` per `WEB_FRAMEWORK_SPEC.md` section 7.

use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteManifestEntry {
    pub path: &'static str,
    pub method: &'static str,
    pub operation_id: &'static str,
    pub request_context: &'static str,
    pub api_surface: &'static str,
    pub auth_mode: &'static str,
}

/// The canonical route manifest for the membership app-api surface.
pub const APP_API_ROUTE_MANIFEST: &[RouteManifestEntry] = &[
    RouteManifestEntry {
        path: "/current",
        method: "GET",
        operation_id: "memberships.current.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/current/status",
        method: "GET",
        operation_id: "memberships.current.status.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/benefits",
        method: "GET",
        operation_id: "memberships.benefits.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "public",
    },
    RouteManifestEntry {
        path: "/package_groups",
        method: "GET",
        operation_id: "memberships.packageGroups.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "public",
    },
    RouteManifestEntry {
        path: "/package_groups/:packageGroupId",
        method: "GET",
        operation_id: "memberships.packageGroups.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "public",
    },
    RouteManifestEntry {
        path: "/package_groups/:packageGroupId/packages",
        method: "GET",
        operation_id: "memberships.packageGroups.packages.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "public",
    },
    RouteManifestEntry {
        path: "/packages",
        method: "GET",
        operation_id: "memberships.packages.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "public",
    },
    RouteManifestEntry {
        path: "/packages/:packageId",
        method: "GET",
        operation_id: "memberships.packages.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "public",
    },
    RouteManifestEntry {
        path: "/plans",
        method: "GET",
        operation_id: "memberships.plans.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "public",
    },
    RouteManifestEntry {
        path: "/points/balance",
        method: "GET",
        operation_id: "memberships.points.balance.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/points/history",
        method: "GET",
        operation_id: "memberships.points.history.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/points/daily_rewards/status",
        method: "GET",
        operation_id: "memberships.points.dailyRewards.status.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/points/daily_rewards",
        method: "POST",
        operation_id: "memberships.points.dailyRewards.create",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/privileges/usage",
        method: "GET",
        operation_id: "memberships.privileges.usage.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/privileges/speed_ups",
        method: "POST",
        operation_id: "memberships.privileges.speedUps.create",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/purchases",
        method: "POST",
        operation_id: "memberships.purchases.create",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/purchases/renew",
        method: "POST",
        operation_id: "memberships.purchases.renew",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/purchases/upgrade",
        method: "POST",
        operation_id: "memberships.purchases.upgrade",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
];

/// Framework-level `HttpRouteManifest` wired into `WebFrameworkLayer`.
///
/// Catalog read routes (plans, benefits, packages, package_groups) are declared
/// `RouteAuth::Public` so anonymous visitors can browse the token-plan page
/// without an `Access-Token`.  If a token is present, the framework still
/// resolves the principal for tenant-scoped catalog reads.
///
/// All mutation and user-scoped routes remain `RouteAuth::DualToken`.
pub const APP_API_HTTP_ROUTE_MANIFEST: HttpRouteManifest = HttpRouteManifest::new(&[
    // ── Public catalog routes (anonymous-accessible) ──────────────────
    HttpRoute::public(
        HttpMethod::Get,
        "/app/v3/api/memberships/plans",
        "Membership",
        "memberships.plans.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/app/v3/api/memberships/benefits",
        "Membership",
        "memberships.benefits.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/app/v3/api/memberships/packages",
        "Membership",
        "memberships.packages.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/app/v3/api/memberships/packages/{packageId}",
        "Membership",
        "memberships.packages.retrieve",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/app/v3/api/memberships/package_groups",
        "Membership",
        "memberships.packageGroups.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/app/v3/api/memberships/package_groups/{packageGroupId}",
        "Membership",
        "memberships.packageGroups.retrieve",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/app/v3/api/memberships/package_groups/{packageGroupId}/packages",
        "Membership",
        "memberships.packageGroups.packages.list",
    ),
    // ── Protected routes (require dual-token authentication) ───────────
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/memberships/current",
        "Membership",
        "memberships.current.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/memberships/current/status",
        "Membership",
        "memberships.current.status.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/memberships/points/balance",
        "Membership",
        "memberships.points.balance.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/memberships/points/history",
        "Membership",
        "memberships.points.history.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/memberships/points/daily_rewards/status",
        "Membership",
        "memberships.points.dailyRewards.status.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/memberships/points/daily_rewards",
        "Membership",
        "memberships.points.dailyRewards.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/memberships/privileges/usage",
        "Membership",
        "memberships.privileges.usage.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/memberships/privileges/speed_ups",
        "Membership",
        "memberships.privileges.speedUps.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/memberships/purchases",
        "Membership",
        "memberships.purchases.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/memberships/purchases/renew",
        "Membership",
        "memberships.purchases.renew",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/memberships/purchases/upgrade",
        "Membership",
        "memberships.purchases.upgrade",
    ),
]);

/// Returns the route manifest as a JSON string for materialization and validation.
pub fn app_api_route_manifest_json() -> String {
    serde_json::to_string_pretty(APP_API_ROUTE_MANIFEST).unwrap_or_else(|_| "[]".to_owned())
}
