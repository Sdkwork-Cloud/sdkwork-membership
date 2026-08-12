-- sdkwork:migration
-- id: 0003_membership_category
-- engine: postgres
-- module: sdkwork-membership
-- purpose: Revert the membership category dimension and the catalog enum
--   CHECK constraints added by the 0003 up migration.
-- reversible: true
-- rollback: down-migration
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

-- Category-aware indexes
DROP INDEX IF EXISTS idx_membership_plan_tenant_category;
DROP INDEX IF EXISTS idx_membership_package_group_tenant_category;
DROP INDEX IF EXISTS idx_membership_package_tenant_category;
DROP INDEX IF EXISTS idx_membership_subscription_tenant_category;
DROP INDEX IF EXISTS idx_membership_period_tenant_category;

-- Enum vocabulary CHECK constraints
ALTER TABLE membership_entitlement_grant
    DROP CONSTRAINT IF EXISTS ck_membership_entitlement_grant_status;
ALTER TABLE membership_entitlement_account
    DROP CONSTRAINT IF EXISTS ck_membership_entitlement_account_status;
ALTER TABLE membership_period
    DROP CONSTRAINT IF EXISTS ck_membership_period_status;
ALTER TABLE membership_subscription
    DROP CONSTRAINT IF EXISTS ck_membership_subscription_status;
ALTER TABLE membership_benefit_definition
    DROP CONSTRAINT IF EXISTS ck_membership_benefit_definition_type;
ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_display_channel;
ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_recurrence_cycle;
ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_billing_cycle;
ALTER TABLE membership_plan_version
    DROP CONSTRAINT IF EXISTS ck_membership_plan_version_lifecycle;
ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_status;
ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_status;
ALTER TABLE membership_plan
    DROP CONSTRAINT IF EXISTS ck_membership_plan_status;

-- Category dimension
ALTER TABLE membership_period
    DROP CONSTRAINT IF EXISTS ck_membership_period_category;
ALTER TABLE membership_period DROP COLUMN IF EXISTS category;

ALTER TABLE membership_subscription
    DROP CONSTRAINT IF EXISTS ck_membership_subscription_category;
ALTER TABLE membership_subscription DROP COLUMN IF EXISTS category;

ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_category;
ALTER TABLE membership_package DROP COLUMN IF EXISTS category;

ALTER TABLE membership_package_group
    DROP CONSTRAINT IF EXISTS ck_membership_package_group_category;
ALTER TABLE membership_package_group DROP COLUMN IF EXISTS category;

ALTER TABLE membership_plan
    DROP CONSTRAINT IF EXISTS ck_membership_plan_category;
ALTER TABLE membership_plan DROP COLUMN IF EXISTS category;
ALTER TABLE membership_plan DROP COLUMN IF EXISTS sort_weight;

COMMIT;
