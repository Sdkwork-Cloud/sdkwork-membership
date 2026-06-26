//! Route manifest for the membership backend-api surface.
//!
//! Every route entry declares `requestContext: WebRequestContext`
//! and `apiSurface: backend-api` per `WEB_FRAMEWORK_SPEC.md` section 7.

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

/// The canonical route manifest for the membership backend-api surface.
pub const BACKEND_API_ROUTE_MANIFEST: &[RouteManifestEntry] = &[
    RouteManifestEntry {
        path: "/plans",
        method: "GET",
        operation_id: "memberships.plans.list",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/plans",
        method: "POST",
        operation_id: "memberships.plans.create",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/plans/:id",
        method: "PUT",
        operation_id: "memberships.plans.update",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/plans/:id",
        method: "DELETE",
        operation_id: "memberships.plans.delete",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packageGroups",
        method: "GET",
        operation_id: "memberships.packageGroups.list",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packageGroups",
        method: "POST",
        operation_id: "memberships.packageGroups.create",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packageGroups/:id",
        method: "PUT",
        operation_id: "memberships.packageGroups.update",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packageGroups/:id",
        method: "DELETE",
        operation_id: "memberships.packageGroups.delete",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packages",
        method: "GET",
        operation_id: "memberships.packages.list",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packages",
        method: "POST",
        operation_id: "memberships.packages.create",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packages/:id",
        method: "PUT",
        operation_id: "memberships.packages.update",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/packages/:id",
        method: "DELETE",
        operation_id: "memberships.packages.delete",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/members",
        method: "GET",
        operation_id: "memberships.members.list",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/members/:id/status",
        method: "PATCH",
        operation_id: "memberships.members.status.update",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
    RouteManifestEntry {
        path: "/entitlements",
        method: "GET",
        operation_id: "memberships.entitlements.list",
        request_context: "WebRequestContext",
        api_surface: "backend-api",
        auth_mode: "dual-token",
    },
];

/// Returns the route manifest as a JSON string for materialization and validation.
pub fn backend_api_route_manifest_json() -> String {
    serde_json::to_string_pretty(BACKEND_API_ROUTE_MANIFEST).unwrap_or_else(|_| "[]".to_owned())
}
