//! App API path constants for the membership capability.
//!
//! All app-api routes are mounted under `/app/v3/api/membership`.

/// Base prefix for app-api membership routes.
pub const APP_API_PREFIX: &str = "/app/v3/api/membership";

/// Current membership info: `GET /app/v3/api/membership/current`
pub const CURRENT: &str = "/current";

/// Membership status: `GET /app/v3/api/membership/current/status`
pub const CURRENT_STATUS: &str = "/current/status";

/// Benefit list: `GET /app/v3/api/membership/benefits`
pub const BENEFITS: &str = "/benefits";

/// Package group list: `GET /app/v3/api/membership/packageGroups`
pub const PACKAGE_GROUPS: &str = "/packageGroups";

/// Package group detail: `GET /app/v3/api/membership/packageGroups/:id`
pub const PACKAGE_GROUP_DETAIL: &str = "/packageGroups/:id";

/// Package list: `GET /app/v3/api/membership/packages`
pub const PACKAGES: &str = "/packages";

/// Plan list: `GET /app/v3/api/membership/plans`
pub const PLANS: &str = "/plans";

/// Points balance: `GET /app/v3/api/membership/points/balance`
pub const POINTS_BALANCE: &str = "/points/balance";

/// Points history: `GET /app/v3/api/membership/points/history`
pub const POINTS_HISTORY: &str = "/points/history";

/// Daily reward status: `GET /app/v3/api/membership/points/dailyRewards/status`
pub const DAILY_REWARD_STATUS: &str = "/points/dailyRewards/status";

/// Claim daily reward: `POST /app/v3/api/membership/points/dailyRewards`
pub const DAILY_REWARD_CLAIM: &str = "/points/dailyRewards";

/// Privilege usage: `GET /app/v3/api/membership/privileges/usage`
pub const PRIVILEGE_USAGE: &str = "/privileges/usage";

/// Submit purchase: `POST /app/v3/api/membership/purchases`
pub const PURCHASES: &str = "/purchases";
