//! Route manifest for the membership app-api surface.
//!
//! Every route entry declares `requestContext: WebRequestContext`
//! and `apiSurface: app-api` per `WEB_FRAMEWORK_SPEC.md` section 7.

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
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packageGroups",
        method: "GET",
        operation_id: "memberships.packageGroups.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packageGroups/:id",
        method: "GET",
        operation_id: "memberships.packageGroups.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packages",
        method: "GET",
        operation_id: "memberships.packages.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/plans",
        method: "GET",
        operation_id: "memberships.plans.list",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
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
        path: "/points/dailyRewards/status",
        method: "GET",
        operation_id: "memberships.points.dailyRewards.status.retrieve",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/points/dailyRewards",
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
        path: "/purchases",
        method: "POST",
        operation_id: "memberships.purchases.create",
        request_context: "WebRequestContext",
        api_surface: "app-api",
        auth_mode: "dual-token",
    },
];

/// Returns the route manifest as a JSON string for materialization and validation.
pub fn app_api_route_manifest_json() -> String {
    serde_json::to_string_pretty(APP_API_ROUTE_MANIFEST).unwrap_or_else(|_| "[]".to_owned())
}
