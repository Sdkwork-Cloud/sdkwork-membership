-- ============================================================================
-- PostgreSQL Membership Seed Reset Script
--
-- Purpose:
--   Clears the membership module's seed history so the database lifecycle
--   framework will re-apply all seed scripts on next application boot.
--   This is the recommended approach when seed data was previously imported
--   with garbled content (e.g., UTF-8 double-encoding) and needs to be
--   overwritten with corrected locale-specific display text.
--
--   The seed scripts themselves use ON CONFLICT (id) DO UPDATE SET, so
--   re-applying them will overwrite existing rows with the correct data.
--
-- Usage:
--   psql -U <user> -d <database> -h <host> -p <port> -f scripts/reset-postgres-membership-seeds.sql
--
--   Or, when using the ClawRouter shared PostgreSQL instance:
--   psql -U sdkwork -d sdkwork -h localhost -p 5432 -f scripts/reset-postgres-membership-seeds.sql
--
-- After running this script, restart the application (or ClawRouter) so the
-- database lifecycle framework re-applies all seeds with the corrected content.
-- ============================================================================

-- ---------------------------------------------------------------------------
-- Step 1: Clear seed history for the membership module.
-- This forces the lifecycle framework to re-apply all seed scripts.
-- ---------------------------------------------------------------------------
DELETE FROM ops_seed_history WHERE module_id = 'membership';

-- ---------------------------------------------------------------------------
-- Step 2: Clear migration history for the membership module (optional).
-- Uncomment if you want the framework to also re-apply the DDL baseline.
-- This is NOT necessary if the table structure is already correct — only
-- the seed data (display text) needs to be refreshed.
-- ---------------------------------------------------------------------------
-- DELETE FROM ops_schema_migration_history WHERE module_id = 'membership';

-- ---------------------------------------------------------------------------
-- Step 3: Reset installation state so the lifecycle framework treats the
-- module as needing re-seeding.
-- ---------------------------------------------------------------------------
UPDATE ops_database_installation_state
SET seed_locale = NULL,
    seed_profile = NULL,
    status = 'bootstrapped'
WHERE module_id = 'membership';

-- ---------------------------------------------------------------------------
-- Step 4 (Optional): Truncate all membership data tables for a full reset.
-- Uncomment the block below if you want to completely clear all membership
-- data (catalog, subscriptions, entitlements, accounts, usage tracking).
-- The DDL baseline will NOT be re-applied (tables remain), but all rows
-- will be deleted. Seeds will re-insert the catalog and dev demo data.
-- ---------------------------------------------------------------------------
-- TRUNCATE TABLE commerce_membership_change_log CASCADE;
-- TRUNCATE TABLE commerce_membership_daily_reward CASCADE;
-- TRUNCATE TABLE commerce_membership_privilege_usage CASCADE;
-- TRUNCATE TABLE entitlement_ledger_entry CASCADE;
-- TRUNCATE TABLE entitlement_grant CASCADE;
-- TRUNCATE TABLE entitlement_account CASCADE;
-- TRUNCATE TABLE commerce_account_ledger CASCADE;
-- TRUNCATE TABLE commerce_account CASCADE;
-- TRUNCATE TABLE membership_period CASCADE;
-- TRUNCATE TABLE membership_subscription CASCADE;
-- TRUNCATE TABLE membership_package CASCADE;
-- TRUNCATE TABLE membership_package_group CASCADE;
-- TRUNCATE TABLE membership_plan_benefit CASCADE;
-- TRUNCATE TABLE benefit_definition CASCADE;
-- TRUNCATE TABLE membership_plan_version CASCADE;
-- TRUNCATE TABLE membership_plan CASCADE;
-- TRUNCATE TABLE commerce_product_sku CASCADE;
-- TRUNCATE TABLE commerce_product_spu CASCADE;

-- ---------------------------------------------------------------------------
-- Verification: Check remaining seed history for membership.
-- After running this script, the query below should return 0 rows.
-- ---------------------------------------------------------------------------
SELECT COUNT(*) AS remaining_seed_records
FROM ops_seed_history
WHERE module_id = 'membership';
