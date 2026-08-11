-- sdkwork:migration
-- id: 0002_membership_package_discount
-- engine: postgres
-- module: sdkwork-membership
-- purpose: Revert the membership package discount column and its 1-100 range
--   constraint added by the 0002 up migration.
-- reversible: true
-- rollback: down-migration
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_discount_range;

ALTER TABLE membership_package DROP COLUMN IF EXISTS discount;

COMMIT;
