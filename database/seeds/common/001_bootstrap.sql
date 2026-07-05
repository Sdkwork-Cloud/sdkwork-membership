-- Membership catalog bootstrap seed for tenant 100001
-- Aligns with sdkwork-membership builtin catalog for commercial readiness demos.

INSERT OR IGNORE INTO commerce_product_spu (
  id, tenant_id, organization_id, spu_no, name, title, status, created_at, updated_at
) VALUES (
  'seed-product-membership', '100001', '0', 'membership-catalog', 'Membership Catalog', 'Membership Catalog', 'active', datetime('now'), datetime('now')
);

INSERT OR IGNORE INTO membership_plan (
  id, tenant_id, organization_id, plan_no, plan_code, name, rank, description, status, created_at, updated_at
) VALUES
  ('plan-free', '100001', '0', 'free', 'free', 'Free', 0, 'Experience core features with daily bonus points.', 'active', datetime('now'), datetime('now')),
  ('plan-basic', '100001', '0', 'basic', 'basic', 'Basic', 1, 'Unlock foundational AI capabilities.', 'active', datetime('now'), datetime('now')),
  ('plan-standard', '100001', '0', 'standard', 'standard', 'Standard', 2, 'Unlock the full AI model catalog.', 'active', datetime('now'), datetime('now')),
  ('plan-premium', '100001', '0', 'premium', 'premium', 'Premium', 3, 'Priority lanes and dedicated support.', 'active', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO membership_plan_version (
  id, tenant_id, organization_id, plan_id, version_no, title, description, lifecycle_status, effective_from, published_at, created_at, updated_at
) VALUES
  ('plan-free-v1', '100001', '0', 'plan-free', 'v1', 'Free', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now')),
  ('plan-basic-v1', '100001', '0', 'plan-basic', 'v1', 'Basic', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now')),
  ('plan-standard-v1', '100001', '0', 'plan-standard', 'v1', 'Standard', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now')),
  ('plan-premium-v1', '100001', '0', 'plan-premium', 'v1', 'Premium', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now'));

INSERT OR IGNORE INTO benefit_definition (
  id, tenant_id, organization_id, benefit_code, name, benefit_type, value_unit, measurement_type, description, status, created_at, updated_at
) VALUES
  ('benefit-daily-points', '100001', '0', 'daily_points', 'Daily bonus points', 'points', 'count', 'counter', 'Claim daily login rewards.', 'active', datetime('now'), datetime('now')),
  ('benefit-standard-queue', '100001', '0', 'standard_queue', 'Standard generation lane', 'queue', 'count', 'counter', 'Standard priority queue.', 'active', datetime('now'), datetime('now')),
  ('benefit-fast-queue', '100001', '0', 'fast_queue', 'Fast generation lane', 'queue', 'count', 'counter', 'High priority fast queue.', 'active', datetime('now'), datetime('now')),
  ('benefit-vip-queue', '100001', '0', 'vip_queue', 'VIP priority lane', 'queue', 'count', 'counter', 'Dedicated VIP queue.', 'active', datetime('now'), datetime('now')),
  ('benefit-no-watermark', '100001', '0', 'no_watermark', 'Watermark-free export', 'feature', 'count', 'counter', 'Export without watermark.', 'active', datetime('now'), datetime('now')),
  ('benefit-vip-support', '100001', '0', 'vip_support', 'Dedicated support', 'service', 'count', 'counter', 'Priority support channel.', 'active', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO membership_plan_benefit (
  id, tenant_id, organization_id, plan_id, plan_version_id, benefit_id, benefit_code, grant_quantity, grant_period, usage_policy, sort_weight, status, created_at, updated_at
) VALUES
  ('plan-basic-v1-benefit-1', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-daily-points', 'daily_points', '725', 'membership_period', 'included', 1, 'active', datetime('now'), datetime('now')),
  ('plan-standard-v1-benefit-1', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-daily-points', 'daily_points', '2210', 'membership_period', 'included', 1, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-1', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-daily-points', 'daily_points', '6160', 'membership_period', 'included', 1, 'active', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO membership_package_group (
  id, tenant_id, organization_id, external_id, group_no, name, description, billing_cycle, duration_days, display_channel, sort_weight, status, created_at, updated_at
) VALUES
  ('package-group-annual', '100001', '0', 1, 'annual', 'Annual plans', 'Save 25% with annual billing.', 'year', 365, 'app', 1, 'active', datetime('now'), datetime('now')),
  ('package-group-monthly', '100001', '0', 2, 'monthly', 'Monthly plans', 'First-month promotion available.', 'month', 30, 'app', 2, 'active', datetime('now'), datetime('now')),
  ('package-group-quarterly', '100001', '0', 3, 'quarterly', 'Quarterly plans', 'Save 15% with quarterly billing.', 'quarter', 90, 'app', 3, 'active', datetime('now'), datetime('now')),
  ('package-group-single', '100001', '0', 4, 'single', 'One-time purchase', 'Single purchase without auto-renewal.', 'month', 30, 'app', 4, 'active', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO commerce_product_sku (
  id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at
) VALUES
  ('sku-basic-monthly', '100001', '0', 'seed-product-membership', 'basic-monthly', 'Basic Monthly', 'Basic Monthly', '68', '73', 'CNY', 'membership_activation', 'untracked', 'active', '{}', datetime('now'), datetime('now')),
  ('sku-standard-monthly', '100001', '0', 'seed-product-membership', 'standard-monthly', 'Standard Monthly', 'Standard Monthly', '198', '221', 'CNY', 'membership_activation', 'untracked', 'active', '{}', datetime('now'), datetime('now')),
  ('sku-premium-monthly', '100001', '0', 'seed-product-membership', 'premium-monthly', 'Premium Monthly', 'Premium Monthly', '568', '616', 'CNY', 'membership_activation', 'untracked', 'active', '{}', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO membership_package (
  id, tenant_id, organization_id, external_id, package_no, package_group_id, plan_id, plan_version_id, sku_id, name, description, price_amount, original_price_amount, currency_code, point_amount, duration_days, recurrence_cycle, sort_weight, recommended, status, created_at, updated_at
) VALUES
  ('package-basic-monthly', '100001', '0', 201, 'basic-monthly', 'package-group-monthly', 'plan-basic', 'plan-basic-v1', 'sku-basic-monthly', 'Basic Monthly', 'Monthly auto-renew plan.', '68', '73', 'CNY', 725, 30, 'month', 1, 0, 'active', datetime('now'), datetime('now')),
  ('package-standard-monthly', '100001', '0', 202, 'standard-monthly', 'package-group-monthly', 'plan-standard', 'plan-standard-v1', 'sku-standard-monthly', 'Standard Monthly', 'Recommended monthly plan.', '198', '221', 'CNY', 2210, 30, 'month', 2, 1, 'active', datetime('now'), datetime('now')),
  ('package-premium-monthly', '100001', '0', 203, 'premium-monthly', 'package-group-monthly', 'plan-premium', 'plan-premium-v1', 'sku-premium-monthly', 'Premium Monthly', 'Premium monthly plan.', '568', '616', 'CNY', 6160, 30, 'month', 3, 0, 'active', datetime('now'), datetime('now'));
