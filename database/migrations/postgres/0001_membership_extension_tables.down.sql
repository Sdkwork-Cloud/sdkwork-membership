-- Membership extension tables rollback (PostgreSQL)

DROP TABLE IF EXISTS commerce_membership_change_log;
DROP TABLE IF EXISTS commerce_membership_privilege_usage;
DROP TABLE IF EXISTS commerce_membership_daily_reward;

-- Note: ALTER TABLE additions to existing commerce tables are not reversed
-- to avoid data loss. The added columns (uuid, version, deleted_at, tags,
-- auto_renew, growth_value) are harmless if left in place.
