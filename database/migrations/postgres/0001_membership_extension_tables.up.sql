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

-- 4. Add missing standard columns to existing membership tables
-- These ALTER TABLE statements are idempotent via DO blocks.

-- Add uuid column to membership_plan
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_plan' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE membership_plan ADD COLUMN uuid VARCHAR(64);
        CREATE UNIQUE INDEX IF NOT EXISTS uk_membership_plan_uuid ON membership_plan (uuid);
    END IF;
END $$;

-- Add version column to membership_plan
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_plan' AND column_name = 'version'
    ) THEN
        ALTER TABLE membership_plan ADD COLUMN version BIGINT NOT NULL DEFAULT 0;
    END IF;
END $$;

-- Add deleted_at column to membership_plan
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_plan' AND column_name = 'deleted_at'
    ) THEN
        ALTER TABLE membership_plan ADD COLUMN deleted_at TIMESTAMPTZ;
    END IF;
END $$;

-- Add uuid, version, deleted_at to membership_package_group
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_package_group' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE membership_package_group ADD COLUMN uuid VARCHAR(64);
        ALTER TABLE membership_package_group ADD COLUMN version BIGINT NOT NULL DEFAULT 0;
        ALTER TABLE membership_package_group ADD COLUMN deleted_at TIMESTAMPTZ;
        CREATE UNIQUE INDEX IF NOT EXISTS uk_membership_package_group_uuid ON membership_package_group (uuid);
    END IF;
END $$;

-- Add uuid, version, deleted_at to membership_package
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_package' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE membership_package ADD COLUMN uuid VARCHAR(64);
        ALTER TABLE membership_package ADD COLUMN version BIGINT NOT NULL DEFAULT 0;
        ALTER TABLE membership_package ADD COLUMN deleted_at TIMESTAMPTZ;
        CREATE UNIQUE INDEX IF NOT EXISTS uk_membership_package_uuid ON membership_package (uuid);
    END IF;
END $$;

-- Add tags column to membership_package if missing
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_package' AND column_name = 'tags'
    ) THEN
        ALTER TABLE membership_package ADD COLUMN tags JSONB NOT NULL DEFAULT '[]'::jsonb;
    END IF;
END $$;

-- Add uuid, version to membership_subscription
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_subscription' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE membership_subscription ADD COLUMN uuid VARCHAR(64);
        ALTER TABLE membership_subscription ADD COLUMN version BIGINT NOT NULL DEFAULT 0;
        ALTER TABLE membership_subscription ADD COLUMN auto_renew BOOLEAN NOT NULL DEFAULT FALSE;
        ALTER TABLE membership_subscription ADD COLUMN growth_value BIGINT NOT NULL DEFAULT 0;
        CREATE UNIQUE INDEX IF NOT EXISTS uk_membership_subscription_uuid ON membership_subscription (uuid);
    END IF;
END $$;

-- Add uuid to membership_period
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_period' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE membership_period ADD COLUMN uuid VARCHAR(64);
        CREATE UNIQUE INDEX IF NOT EXISTS uk_membership_period_uuid ON membership_period (uuid);
    END IF;
END $$;

-- Add uuid to membership_plan_version
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'membership_plan_version' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE membership_plan_version ADD COLUMN uuid VARCHAR(64);
        ALTER TABLE membership_plan_version ADD COLUMN version BIGINT NOT NULL DEFAULT 0;
        CREATE UNIQUE INDEX IF NOT EXISTS uk_membership_plan_version_uuid ON membership_plan_version (uuid);
    END IF;
END $$;

-- Add uuid to benefit_definition
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'benefit_definition' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE benefit_definition ADD COLUMN uuid VARCHAR(64);
        ALTER TABLE benefit_definition ADD COLUMN version BIGINT NOT NULL DEFAULT 0;
        CREATE UNIQUE INDEX IF NOT EXISTS uk_benefit_definition_uuid ON benefit_definition (uuid);
    END IF;
END $$;

-- Add uuid to entitlement_account
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'entitlement_account' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE entitlement_account ADD COLUMN uuid VARCHAR(64);
        ALTER TABLE entitlement_account ADD COLUMN version BIGINT NOT NULL DEFAULT 0;
        CREATE UNIQUE INDEX IF NOT EXISTS uk_entitlement_account_uuid ON entitlement_account (uuid);
    END IF;
END $$;

-- Add uuid to entitlement_grant
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'entitlement_grant' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE entitlement_grant ADD COLUMN uuid VARCHAR(64);
        CREATE UNIQUE INDEX IF NOT EXISTS uk_entitlement_grant_uuid ON entitlement_grant (uuid);
    END IF;
END $$;

-- Add uuid to entitlement_ledger_entry
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = current_schema() AND table_name = 'entitlement_ledger_entry' AND column_name = 'uuid'
    ) THEN
        ALTER TABLE entitlement_ledger_entry ADD COLUMN uuid VARCHAR(64);
        CREATE UNIQUE INDEX IF NOT EXISTS uk_entitlement_ledger_uuid ON entitlement_ledger_entry (uuid);
    END IF;
END $$;

-- 5. Add performance indexes for high-frequency queries
CREATE INDEX IF NOT EXISTS idx_membership_plan_tenant_status_rank
    ON membership_plan (tenant_id, organization_id, status, rank);

CREATE INDEX IF NOT EXISTS idx_membership_package_group_tenant_status_sort
    ON membership_package_group (tenant_id, status, sort_weight);

CREATE INDEX IF NOT EXISTS idx_membership_package_group_status_sort
    ON membership_package (package_group_id, status, sort_weight);

CREATE INDEX IF NOT EXISTS idx_membership_package_tenant_status
    ON membership_package (tenant_id, organization_id, status, sort_weight);

CREATE INDEX IF NOT EXISTS idx_membership_subscription_owner_status
    ON membership_subscription (tenant_id, owner_user_id, status, expires_at DESC);

CREATE INDEX IF NOT EXISTS idx_membership_subscription_expire
    ON membership_subscription (status, expires_at)
    WHERE status IN ('active', 'grace_period');

CREATE INDEX IF NOT EXISTS idx_membership_plan_benefit_version_status
    ON membership_plan_benefit (plan_version_id, status, sort_weight);

CREATE INDEX IF NOT EXISTS idx_entitlement_account_subject_status
    ON entitlement_account (tenant_id, subject_id, status);

CREATE INDEX IF NOT EXISTS idx_entitlement_grant_source
    ON entitlement_grant (source_type, source_id, status);

CREATE INDEX IF NOT EXISTS idx_entitlement_grant_expire
    ON entitlement_grant (status, expires_at)
    WHERE status = 'active';

CREATE INDEX IF NOT EXISTS idx_entitlement_ledger_account
    ON entitlement_ledger_entry (account_id, occurred_at DESC);
