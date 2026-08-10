-- sdkwork:migration
-- id: 0001_organization_id_not_null
-- engine: postgres
-- module: sdkwork-membership
-- purpose: Enforce organization_id NOT NULL DEFAULT on all tables in the
--   consolidated baseline. NULL rows (pre-standard data anomalies) are
--   backfilled with the platform sentinel before NOT NULL is set, and
--   NOT NULL columns without an explicit default receive the sentinel
--   default, keeping existing deployments consistent with fresh baseline
--   installs.
-- reversible: false
-- rollback: forward-fix (sentinel backfill is the canonical fix; NULL
--   organization rows are data anomalies)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

UPDATE membership_plan SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_plan ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_plan ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_plan_version SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_plan_version ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_plan_version ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_benefit_definition SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_benefit_definition ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_benefit_definition ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_plan_benefit SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_plan_benefit ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_plan_benefit ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_package_group SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_package_group ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_package_group ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_package SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_package ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_package ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_subscription SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_subscription ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_subscription ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_period SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_period ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_period ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_entitlement_account SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_entitlement_account ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_entitlement_account ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_entitlement_grant SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_entitlement_grant ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_entitlement_grant ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_entitlement_ledger_entry SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE membership_entitlement_ledger_entry ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE membership_entitlement_ledger_entry ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_points_account SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE membership_points_account ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE membership_points_account ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_points_ledger SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE membership_points_ledger ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE membership_points_ledger ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_daily_reward SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE membership_daily_reward ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE membership_daily_reward ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_privilege_usage SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE membership_privilege_usage ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE membership_privilege_usage ALTER COLUMN organization_id SET NOT NULL;

UPDATE membership_change_log SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE membership_change_log ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE membership_change_log ALTER COLUMN organization_id SET NOT NULL;

COMMIT;
