//! Backend API path constants for the membership capability.
//!
//! All backend-api routes are mounted under `/backend/v3/api/membership`.

/// Base prefix for backend-api membership routes.
pub const BACKEND_API_PREFIX: &str = "/backend/v3/api/membership";

/// Plans management: `/backend/v3/api/membership/plans`
pub const PLANS: &str = "/plans";

/// Package groups management: `/backend/v3/api/membership/packageGroups`
pub const PACKAGE_GROUPS: &str = "/packageGroups";

/// Packages management: `/backend/v3/api/membership/packages`
pub const PACKAGES: &str = "/packages";

/// Members listing: `/backend/v3/api/membership/members`
pub const MEMBERS: &str = "/members";

/// Entitlements listing: `/backend/v3/api/membership/entitlements`
pub const ENTITLEMENTS: &str = "/entitlements";
