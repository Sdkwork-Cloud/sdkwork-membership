-- Dev demo data for authenticated frontend membership flows.
-- Scope: tenant 100001, organization 0, user 1.
-- Membership does not create commerce_order rows.
-- This file is loaded only in the "dev" and "standard" seed profiles.

INSERT INTO membership_subscription (
  id, tenant_id, organization_id, subscription_no, subject_type, subject_id,
  owner_user_id, plan_id, plan_version_id, package_id, current_period_id,
  source_order_id, status, starts_at, expires_at,
  grace_until, cancel_at_period_end, request_no, idempotency_key, created_at, updated_at
) VALUES (
  'seed-membership-subscription-user-1', '100001', '0', 'seed-membership-subscription-user-1', 'user', '1',
  '1', 'plan-standard', 'plan-standard-v1', 'package-standard-annual', 'seed-membership-period-user-1',
  NULL, 'active', '2026-06-28 00:00:00', '2027-06-29 00:00:00',
  NULL, 0, 'seed-membership-user-1', 'seed-membership-user-1', '2026-06-28 00:00:00', CURRENT_TIMESTAMP
) ON CONFLICT (id) DO NOTHING;

INSERT INTO membership_period (
  id, tenant_id, organization_id, period_no, subscription_id, subject_type,
  subject_id, plan_id, plan_version_id, starts_at, ends_at, status,
  source_order_id, request_no, idempotency_key,
  created_at, updated_at
) VALUES (
  'seed-membership-period-user-1', '100001', '0', 'seed-membership-period-user-1', 'seed-membership-subscription-user-1', 'user',
  '1', 'plan-standard', 'plan-standard-v1', '2026-06-28 00:00:00', '2027-06-29 00:00:00', 'active',
  NULL, 'seed-membership-period-user-1', 'seed-membership-period-user-1', '2026-06-28 00:00:00', CURRENT_TIMESTAMP
) ON CONFLICT (id) DO NOTHING;

INSERT INTO commerce_account (
  id, uuid, tenant_id, organization_id, owner_type, owner_id, asset_code, currency_code,
  account_purpose, available_amount, frozen_amount, pending_amount, status, version, created_at, updated_at
) VALUES (
  9101001, 'seed-commerce-account-user-1-points', 100001, 0, 'USER', 1, 'points', 'POINT',
  'GENERAL', 19000, 0, 0, 1, 0, '2026-06-23 00:00:00', CURRENT_TIMESTAMP
) ON CONFLICT (id) DO NOTHING;

INSERT INTO commerce_account_ledger (
  id, uuid, tenant_id, organization_id, account_id, journal_id, owner_type, owner_id,
  asset_code, currency_code, ledger_type, entry_type, direction, amount, balance_before,
  balance_after, business_type, business_no, request_no, idempotency_key, source_type,
  source_id, remark, metadata_json, trace_id, created_at
) VALUES
  (9101101, 'seed-commerce-account-ledger-user-1-membership-grant', 100001, 0, 9101001, NULL, 'USER', 1,
   'points', 'POINT', 'AVAILABLE', 'GRANT', 'CREDIT', 20000, 0,
   20000, 'membership_grant', 'seed-points-membership-grant-user-1', 'seed-points-membership-grant-user-1', 'seed-points-membership-grant-user-1', 'membership_subscription',
   9100001, 'Membership annual compute credits grant', '{}', 'seed-trace-membership-user-1', '2026-06-28 00:00:00'),
  (9101102, 'seed-commerce-account-ledger-user-1-ai-usage', 100001, 0, 9101001, NULL, 'USER', 1,
   'points', 'POINT', 'AVAILABLE', 'CONSUME', 'DEBIT', 1500, 20000,
   18500, 'ai_usage', 'seed-points-ai-usage-user-1', 'seed-points-ai-usage-user-1', 'seed-points-ai-usage-user-1', 'ai_generation',
   9100002, 'AI generation usage', '{}', 'seed-trace-membership-user-1', '2026-07-11 00:00:00'),
  (9101103, 'seed-commerce-account-ledger-user-1-daily-reward', 100001, 0, 9101001, NULL, 'USER', 1,
   'points', 'POINT', 'AVAILABLE', 'REWARD', 'CREDIT', 500, 18500,
   19000, 'daily_reward', 'seed-points-daily-reward-user-1', 'seed-points-daily-reward-user-1', 'seed-points-daily-reward-user-1', 'membership_daily_reward',
   9100003, 'Daily membership reward', '{}', 'seed-trace-membership-user-1', '2026-07-12 00:00:00') ON CONFLICT (id) DO NOTHING;

INSERT INTO entitlement_account (
  id, tenant_id, organization_id, account_no, benefit_id, subject_type,
  subject_id, total_granted, total_used, balance, status, expires_at,
  version, created_at, updated_at
) VALUES
  ('seed-entitlement-account-user-1-speed-up', '100001', '0', 'seed-entitlement-account-user-1-speed-up', 'seed-benefit-priority-speed-up', 'user',
   '1', '20', '3', '17', 'active', '2027-06-29 00:00:00', 0, '2026-06-28 00:00:00', CURRENT_TIMESTAMP),
  ('seed-entitlement-account-user-1-priority-queue', '100001', '0', 'seed-entitlement-account-user-1-priority-queue', 'benefit-definition-priority_queue', 'user',
   '1', '100', '25', '75', 'active', '2027-06-29 00:00:00', 0, '2026-06-28 00:00:00', CURRENT_TIMESTAMP),
  ('seed-entitlement-account-user-1-ai-quota', '100001', '0', 'seed-entitlement-account-user-1-ai-quota', 'seed-benefit-ai-quota', 'user',
   '1', '50', '6', '44', 'active', '2027-06-29 00:00:00', 0, '2026-06-28 00:00:00', CURRENT_TIMESTAMP) ON CONFLICT (id) DO NOTHING;

INSERT INTO entitlement_grant (
  id, tenant_id, organization_id, grant_no, benefit_id, subject_type, subject_id,
  source_type, source_id, grant_policy, granted_quantity, status, starts_at,
  expires_at, request_no, idempotency_key, created_at, updated_at
) VALUES
  ('seed-entitlement-grant-user-1-speed-up', '100001', '0', 'seed-entitlement-grant-user-1-speed-up', 'seed-benefit-priority-speed-up', 'user', '1',
   'membership_subscription', 'seed-membership-subscription-user-1', 'membership_plan', '20', 'active', '2026-06-28 00:00:00',
   '2027-06-29 00:00:00', 'seed-entitlement-grant-user-1-speed-up', 'seed-entitlement-grant-user-1-speed-up', '2026-06-28 00:00:00', CURRENT_TIMESTAMP),
  ('seed-entitlement-grant-user-1-priority-queue', '100001', '0', 'seed-entitlement-grant-user-1-priority-queue', 'benefit-definition-priority_queue', 'user', '1',
   'membership_subscription', 'seed-membership-subscription-user-1', 'membership_plan', '100', 'active', '2026-06-28 00:00:00',
   '2027-06-29 00:00:00', 'seed-entitlement-grant-user-1-priority-queue', 'seed-entitlement-grant-user-1-priority-queue', '2026-06-28 00:00:00', CURRENT_TIMESTAMP),
  ('seed-entitlement-grant-user-1-ai-quota', '100001', '0', 'seed-entitlement-grant-user-1-ai-quota', 'seed-benefit-ai-quota', 'user', '1',
   'membership_subscription', 'seed-membership-subscription-user-1', 'membership_plan', '50', 'active', '2026-06-28 00:00:00',
   '2027-06-29 00:00:00', 'seed-entitlement-grant-user-1-ai-quota', 'seed-entitlement-grant-user-1-ai-quota', '2026-06-28 00:00:00', CURRENT_TIMESTAMP) ON CONFLICT (id) DO NOTHING;

INSERT INTO entitlement_ledger_entry (
  id, tenant_id, organization_id, ledger_no, account_id, grant_id, benefit_id,
  subject_type, subject_id, direction, amount, balance_after, business_type,
  source_type, source_id, request_no, idempotency_key, occurred_at, created_at
) VALUES
  ('seed-entitlement-ledger-user-1-speed-up-credit', '100001', '0', 'seed-entitlement-ledger-user-1-speed-up-credit', 'seed-entitlement-account-user-1-speed-up', 'seed-entitlement-grant-user-1-speed-up', 'seed-benefit-priority-speed-up',
   'user', '1', 'credit', '20', '20', 'membership_grant', 'membership_subscription', 'seed-membership-subscription-user-1', 'seed-entitlement-ledger-user-1-speed-up-credit', 'seed-entitlement-ledger-user-1-speed-up-credit', '2026-06-28 00:00:00', '2026-06-28 00:00:00'),
  ('seed-entitlement-ledger-user-1-priority-queue-credit', '100001', '0', 'seed-entitlement-ledger-user-1-priority-queue-credit', 'seed-entitlement-account-user-1-priority-queue', 'seed-entitlement-grant-user-1-priority-queue', 'benefit-definition-priority_queue',
   'user', '1', 'credit', '100', '100', 'membership_grant', 'membership_subscription', 'seed-membership-subscription-user-1', 'seed-entitlement-ledger-user-1-priority-queue-credit', 'seed-entitlement-ledger-user-1-priority-queue-credit', '2026-06-28 00:00:00', '2026-06-28 00:00:00'),
  ('seed-entitlement-ledger-user-1-ai-quota-credit', '100001', '0', 'seed-entitlement-ledger-user-1-ai-quota-credit', 'seed-entitlement-account-user-1-ai-quota', 'seed-entitlement-grant-user-1-ai-quota', 'seed-benefit-ai-quota',
   'user', '1', 'credit', '50', '50', 'membership_grant', 'membership_subscription', 'seed-membership-subscription-user-1', 'seed-entitlement-ledger-user-1-ai-quota-credit', 'seed-entitlement-ledger-user-1-ai-quota-credit', '2026-06-28 00:00:00', '2026-06-28 00:00:00') ON CONFLICT (id) DO NOTHING;

INSERT INTO commerce_membership_privilege_usage (
  id, uuid, tenant_id, organization_id, user_id, subscription_id, benefit_code,
  period_start, period_end, used_count, usage_limit, last_used_at, version, created_at, updated_at
) VALUES
  (9102001, 'seed-membership-privilege-user-1-speed-up', 100001, 0, 1, NULL, 'priority_speed_up',
   '2026-06-28 00:00:00', '2027-06-29 00:00:00', 3, 20, '2026-07-12 00:00:00', 0, '2026-06-28 00:00:00', CURRENT_TIMESTAMP),
  (9102002, 'seed-membership-privilege-user-1-priority-queue', 100001, 0, 1, NULL, 'priority_queue',
   '2026-06-28 00:00:00', '2027-06-29 00:00:00', 25, 100, '2026-07-12 00:00:00', 0, '2026-06-28 00:00:00', CURRENT_TIMESTAMP),
  (9102003, 'seed-membership-privilege-user-1-ai-quota', 100001, 0, 1, NULL, 'ai_quota',
   '2026-06-28 00:00:00', '2027-06-29 00:00:00', 6, 50, '2026-07-12 00:00:00', 0, '2026-06-28 00:00:00', CURRENT_TIMESTAMP) ON CONFLICT (id) DO NOTHING;

INSERT INTO commerce_membership_daily_reward (
  id, uuid, tenant_id, organization_id, user_id, reward_date, reward_points,
  consecutive_days, total_days, status, idempotency_key, created_at, version, updated_at
) VALUES (
  9103001, 'seed-membership-daily-reward-user-1-yesterday', 100001, 0, 1, '2026-07-12', 500,
  3, 12, 'claimed', 'seed-membership-daily-reward-user-1-yesterday', '2026-07-12 00:00:00', 0, '2026-07-12 00:00:00'
) ON CONFLICT (id) DO NOTHING;
