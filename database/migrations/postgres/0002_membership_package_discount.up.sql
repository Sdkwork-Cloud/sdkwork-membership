-- sdkwork:migration
-- id: 0002_membership_package_discount
-- engine: postgres
-- module: sdkwork-membership
-- purpose: Add the discount rate percentage column (1-100, 100 = no
--   discount) to membership_package. Existing rows default to 100 (no
--   discount); the baseline gains the same column with the same default and
--   CHECK constraint for fresh installs.
-- reversible: true
-- rollback: down-migration
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

ALTER TABLE membership_package ADD COLUMN IF NOT EXISTS discount BIGINT NOT NULL DEFAULT 100;

ALTER TABLE membership_package
    DROP CONSTRAINT IF EXISTS ck_membership_package_discount_range;

ALTER TABLE membership_package
    ADD CONSTRAINT ck_membership_package_discount_range
    CHECK (discount >= 1 AND discount <= 100);

COMMIT;
