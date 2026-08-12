-- sdkwork:migration
-- id: 0003_membership_category
-- engine: postgres
-- module: sdkwork-membership
-- purpose: Introduce the membership category classification dimension
--   (token plans, community/circle plans, future extensions) on plan,
--   package group, package, subscription, and period, and enforce the
--   catalog enum vocabulary with CHECK constraints (status, billing and
--   recurrence cycles, display channel, benefit type). Existing rows are
--   backfilled to category 'token' (all pre-standard catalog data is
--   token-plan data); subscription and period categories are snapshotted
--   from their plan at write time by the application layer.
-- reversible: true
-- rollback: down-migration
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

-- ============================================================================
-- 1. Category dimension (extensible enum: token | community | future values)
--    Existing rows default to 'token'; new writes are constrained to the
--    declared vocabulary and must be extended via migration.
-- ============================================================================

ALTER TABLE membership_plan
    ADD COLUMN IF NOT EXISTS category TEXT NOT NULL DEFAULT 'token';

ALTER TABLE membership_plan
    ADD COLUMN IF NOT EXISTS sort_weight INTEGER NOT NULL DEFAULT 0;

ALTER TABLE membership_plan
    DROP CONSTRAINT IF EXISTS ck_membership_plan_category;
ALTER TABLE membership_plan
    ADD CONSTRAINT ck_membership_plan_category
    CHECK (category IN ('token', 'community'));

ALTER TABLE membership_package_group
    ADD COLUMN IF NOT EXISTS category TEXT NOT NULL DEFAULT 'token';

ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_category;
ALTER TABLE membership_package_group
    ADD CONSTRAINT ck_membership_package_group_category
    CHECK (category IN ('token', 'community'));

ALTER TABLE membership_package
    ADD COLUMN IF NOT EXISTS category TEXT NOT NULL DEFAULT 'token';

ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_category;
ALTER TABLE membership_package
    ADD CONSTRAINT ck_membership_package_category
    CHECK (category IN ('token', 'community'));

ALTER TABLE membership_subscription
    ADD COLUMN IF NOT EXISTS category TEXT NOT NULL DEFAULT 'token';

ALTER TABLE membership_subscription
    DROP CONSTRAINT IF EXISTS ck_membership_subscription_category;
ALTER TABLE membership_subscription
    ADD CONSTRAINT ck_membership_subscription_category
    CHECK (category IN ('token', 'community'));

ALTER TABLE membership_period
    ADD COLUMN IF NOT EXISTS category TEXT NOT NULL DEFAULT 'token';

ALTER TABLE membership_period
    DROP CONSTRAINT IF EXISTS ck_membership_period_category;
ALTER TABLE membership_period
    ADD CONSTRAINT ck_membership_period_category
    CHECK (category IN ('token', 'community'));

-- ============================================================================
-- 2. Catalog lifecycle status vocabulary (active | inactive | disabled)
--    'disabled' is the soft-delete marker written by admin delete commands.
-- ============================================================================

ALTER TABLE membership_plan
    DROP CONSTRAINT IF EXISTS ck_membership_plan_status;
ALTER TABLE membership_plan
    ADD CONSTRAINT ck_membership_plan_status
    CHECK (status IN ('active', 'inactive', 'disabled'));

ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_status;
ALTER TABLE membership_package_group
    ADD CONSTRAINT ck_membership_package_group_status
    CHECK (status IN ('active', 'inactive', 'disabled'));

ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_status;
ALTER TABLE membership_package
    ADD CONSTRAINT ck_membership_package_status
    CHECK (status IN ('active', 'inactive', 'disabled'));

ALTER TABLE membership_plan_version
    DROP CONSTRAINT IF EXISTS ck_membership_plan_version_lifecycle;
ALTER TABLE membership_plan_version
    ADD CONSTRAINT ck_membership_plan_version_lifecycle
    CHECK (lifecycle_status IN ('draft', 'published', 'archived'));

-- ============================================================================
-- 3. Billing / recurrence cycle vocabulary (once | day | week | month | quarter | year)
-- ============================================================================

ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_billing_cycle;
ALTER TABLE membership_package_group
    ADD CONSTRAINT ck_membership_package_group_billing_cycle
    CHECK (billing_cycle IN ('once', 'day', 'week', 'month', 'quarter', 'year'));

ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_recurrence_cycle;
ALTER TABLE membership_package
    ADD CONSTRAINT ck_membership_package_recurrence_cycle
    CHECK (recurrence_cycle IN ('once', 'day', 'week', 'month', 'quarter', 'year'));

-- ============================================================================
-- 4. Display channel and benefit type vocabulary
-- ============================================================================

ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_display_channel;
ALTER TABLE membership_package_group
    ADD CONSTRAINT ck_membership_package_group_display_channel
    CHECK (display_channel IS NULL OR display_channel IN ('app', 'pc', 'all'));

ALTER TABLE membership_benefit_definition
    DROP CONSTRAINT IF EXISTS ck_membership_benefit_definition_type;
ALTER TABLE membership_benefit_definition
    ADD CONSTRAINT ck_membership_benefit_definition_type
    CHECK (benefit_type IN ('points', 'feature', 'queue', 'quota', 'service'));

-- ============================================================================
-- 5. Subscription / period status vocabulary (aligned with the domain state
--    machine: pending, pending_activation, active, grace_period, expired,
--    cancelled)
-- ============================================================================

ALTER TABLE membership_subscription
    DROP CONSTRAINT IF EXISTS ck_membership_subscription_status;
ALTER TABLE membership_subscription
    ADD CONSTRAINT ck_membership_subscription_status
    CHECK (status IN ('pending', 'pending_activation', 'active', 'grace_period', 'expired', 'cancelled'));

ALTER TABLE membership_period
    DROP CONSTRAINT IF EXISTS ck_membership_period_status;
ALTER TABLE membership_period
    ADD CONSTRAINT ck_membership_period_status
    CHECK (status IN ('pending', 'pending_activation', 'active', 'expired', 'cancelled'));

ALTER TABLE membership_entitlement_account
    DROP CONSTRAINT IF EXISTS ck_membership_entitlement_account_status;
ALTER TABLE membership_entitlement_account
    ADD CONSTRAINT ck_membership_entitlement_account_status
    CHECK (status IN ('pending', 'active', 'expired', 'cancelled'));

ALTER TABLE membership_entitlement_grant
    DROP CONSTRAINT IF EXISTS ck_membership_entitlement_grant_status;
ALTER TABLE membership_entitlement_grant
    ADD CONSTRAINT ck_membership_entitlement_grant_status
    CHECK (status IN ('pending', 'active', 'expired', 'cancelled'));

-- ============================================================================
-- 6. Category-aware catalog and entitlement indexes
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_membership_plan_tenant_category
    ON membership_plan (tenant_id, organization_id, category, status, sort_weight);

CREATE INDEX IF NOT EXISTS idx_membership_package_group_tenant_category
    ON membership_package_group (tenant_id, organization_id, category, status, sort_weight);

CREATE INDEX IF NOT EXISTS idx_membership_package_tenant_category
    ON membership_package (tenant_id, organization_id, category, status, sort_weight);

CREATE INDEX IF NOT EXISTS idx_membership_subscription_tenant_category
    ON membership_subscription (tenant_id, organization_id, category, status, expires_at DESC);

CREATE INDEX IF NOT EXISTS idx_membership_period_tenant_category
    ON membership_period (tenant_id, organization_id, category, status, ends_at DESC);

COMMIT;
