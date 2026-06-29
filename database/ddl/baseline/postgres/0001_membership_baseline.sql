-- Membership extension tables migration (PostgreSQL)
-- Adds membership-specific tables owned by sdkwork-membership capability.
-- Existing commerce domain tables (membership_plan, membership_package, etc.)
-- are created by the commerce platform initial migration.

-- 1. Daily reward tracking table
CREATE TABLE IF NOT EXISTS commerce_membership_daily_reward (
    id              BIGINT       NOT NULL,
    uuid            VARCHAR(64)  NOT NULL,
    tenant_id       BIGINT       NOT NULL,
    organization_id BIGINT       NOT NULL DEFAULT 0,
    user_id         BIGINT       NOT NULL,
    reward_date     DATE         NOT NULL,
    reward_points   BIGINT       NOT NULL DEFAULT 0,
    consecutive_days INTEGER     NOT NULL DEFAULT 1,
    total_days      INTEGER      NOT NULL DEFAULT 1,
    status          VARCHAR(32)  NOT NULL DEFAULT 'claimed',
    idempotency_key VARCHAR(64),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_commerce_membership_daily_reward PRIMARY KEY (id),
    CONSTRAINT uk_commerce_membership_daily_reward_uuid UNIQUE (uuid),
    CONSTRAINT uk_commerce_membership_daily_reward_user_date UNIQUE (tenant_id, user_id, reward_date),
    CONSTRAINT chk_daily_reward_points CHECK (reward_points >= 0),
    CONSTRAINT chk_daily_reward_consecutive CHECK (consecutive_days >= 1),
    CONSTRAINT chk_daily_reward_status CHECK (status IN ('claimed', 'revoked'))
);

CREATE INDEX IF NOT EXISTS idx_daily_reward_user_recent
    ON commerce_membership_daily_reward (tenant_id, user_id, reward_date DESC);

-- 2. Privilege usage tracking table
CREATE TABLE IF NOT EXISTS commerce_membership_privilege_usage (
    id              BIGINT       NOT NULL,
    uuid            VARCHAR(64)  NOT NULL,
    tenant_id       BIGINT       NOT NULL,
    organization_id BIGINT       NOT NULL DEFAULT 0,
    user_id         BIGINT       NOT NULL,
    subscription_id BIGINT,
    benefit_code    VARCHAR(64)  NOT NULL,
    period_start    TIMESTAMPTZ  NOT NULL,
    period_end      TIMESTAMPTZ  NOT NULL,
    used_count      BIGINT       NOT NULL DEFAULT 0,
    usage_limit     BIGINT       NOT NULL DEFAULT 0,
    last_used_at    TIMESTAMPTZ,
    version         BIGINT       NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_commerce_membership_privilege_usage PRIMARY KEY (id),
    CONSTRAINT uk_commerce_membership_privilege_usage_uuid UNIQUE (uuid),
    CONSTRAINT uk_commerce_membership_privilege_usage_unique UNIQUE (tenant_id, user_id, benefit_code, period_start),
    CONSTRAINT chk_priv_usage_used CHECK (used_count >= 0),
    CONSTRAINT chk_priv_usage_limit CHECK (usage_limit >= 0)
);

CREATE INDEX IF NOT EXISTS idx_priv_usage_user_period
    ON commerce_membership_privilege_usage (tenant_id, user_id, period_end);

-- 3. Membership change audit log (immutable)
CREATE TABLE IF NOT EXISTS commerce_membership_change_log (
    id              BIGINT       NOT NULL,
    uuid            VARCHAR(64)  NOT NULL,
    tenant_id       BIGINT       NOT NULL,
    organization_id BIGINT       NOT NULL DEFAULT 0,
    subscription_id BIGINT       NOT NULL,
    user_id         BIGINT       NOT NULL,
    action          VARCHAR(64)  NOT NULL,
    from_status     VARCHAR(32),
    to_status       VARCHAR(32),
    from_plan_id    VARCHAR(128),
    to_plan_id      VARCHAR(128),
    operator_id     BIGINT,
    operator_type   VARCHAR(32)  DEFAULT 'system',
    reason          VARCHAR(256),
    metadata        JSONB,
    request_id      VARCHAR(64),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_commerce_membership_change_log PRIMARY KEY (id),
    CONSTRAINT uk_commerce_membership_change_log_uuid UNIQUE (uuid),
    CONSTRAINT chk_change_log_action CHECK (action IN (
        'activate', 'renew', 'upgrade', 'downgrade',
        'expire', 'cancel', 'grace', 'restore'
    ))
);

CREATE INDEX IF NOT EXISTS idx_change_log_subscription
    ON commerce_membership_change_log (subscription_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_change_log_tenant_user
    ON commerce_membership_change_log (tenant_id, user_id, created_at DESC);


