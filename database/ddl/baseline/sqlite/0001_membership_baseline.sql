-- Membership extension tables migration (SQLite)
-- Adds membership-specific tables owned by sdkwork-membership capability.
-- Existing commerce domain tables are created by the commerce platform initial migration.

-- 1. Daily reward tracking table
CREATE TABLE IF NOT EXISTS commerce_membership_daily_reward (
    id              INTEGER      NOT NULL,
    uuid            TEXT         NOT NULL,
    tenant_id       INTEGER      NOT NULL,
    organization_id INTEGER      NOT NULL DEFAULT 0,
    user_id         INTEGER      NOT NULL,
    reward_date     TEXT         NOT NULL,
    reward_points   INTEGER      NOT NULL DEFAULT 0,
    consecutive_days INTEGER     NOT NULL DEFAULT 1,
    total_days      INTEGER      NOT NULL DEFAULT 1,
    status          TEXT         NOT NULL DEFAULT 'claimed',
    idempotency_key TEXT,
    created_at      TEXT         NOT NULL,
    PRIMARY KEY (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_commerce_membership_daily_reward_uuid
    ON commerce_membership_daily_reward (uuid);
CREATE UNIQUE INDEX IF NOT EXISTS uk_commerce_membership_daily_reward_user_date
    ON commerce_membership_daily_reward (tenant_id, user_id, reward_date);
CREATE INDEX IF NOT EXISTS idx_daily_reward_user_recent
    ON commerce_membership_daily_reward (tenant_id, user_id, reward_date DESC);

-- 2. Privilege usage tracking table
CREATE TABLE IF NOT EXISTS commerce_membership_privilege_usage (
    id              INTEGER      NOT NULL,
    uuid            TEXT         NOT NULL,
    tenant_id       INTEGER      NOT NULL,
    organization_id INTEGER      NOT NULL DEFAULT 0,
    user_id         INTEGER      NOT NULL,
    subscription_id INTEGER,
    benefit_code    TEXT         NOT NULL,
    period_start    TEXT         NOT NULL,
    period_end      TEXT         NOT NULL,
    used_count      INTEGER      NOT NULL DEFAULT 0,
    usage_limit     INTEGER      NOT NULL DEFAULT 0,
    last_used_at    TEXT,
    version         INTEGER      NOT NULL DEFAULT 0,
    created_at      TEXT         NOT NULL,
    updated_at      TEXT         NOT NULL,
    PRIMARY KEY (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_commerce_membership_privilege_usage_uuid
    ON commerce_membership_privilege_usage (uuid);
CREATE UNIQUE INDEX IF NOT EXISTS uk_commerce_membership_privilege_usage_unique
    ON commerce_membership_privilege_usage (tenant_id, user_id, benefit_code, period_start);
CREATE INDEX IF NOT EXISTS idx_priv_usage_user_period
    ON commerce_membership_privilege_usage (tenant_id, user_id, period_end);

-- 3. Membership change audit log (immutable)
CREATE TABLE IF NOT EXISTS commerce_membership_change_log (
    id              INTEGER      NOT NULL,
    uuid            TEXT         NOT NULL,
    tenant_id       INTEGER      NOT NULL,
    organization_id INTEGER      NOT NULL DEFAULT 0,
    subscription_id INTEGER      NOT NULL,
    user_id         INTEGER      NOT NULL,
    action          TEXT         NOT NULL,
    from_status     TEXT,
    to_status       TEXT,
    from_plan_id    TEXT,
    to_plan_id      TEXT,
    operator_id     INTEGER,
    operator_type   TEXT         DEFAULT 'system',
    reason          TEXT,
    metadata        TEXT,
    request_id      TEXT,
    created_at      TEXT         NOT NULL,
    PRIMARY KEY (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_commerce_membership_change_log_uuid
    ON commerce_membership_change_log (uuid);
CREATE INDEX IF NOT EXISTS idx_change_log_subscription
    ON commerce_membership_change_log (subscription_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_change_log_tenant_user
    ON commerce_membership_change_log (tenant_id, user_id, created_at DESC);


