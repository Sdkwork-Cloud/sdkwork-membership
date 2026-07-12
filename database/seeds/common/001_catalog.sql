-- Membership token plan catalog seed for tenant 100001
-- Full commercial catalog: 4 billing-cycle package groups x 3 purchasable tiers.
-- External ids align with app-api read models (101-103 annual, 201-203 monthly, 301-303 quarterly, 401-403 single).
-- Prices and benefits align with frontend subscription-catalog-content.ts fallback display.

INSERT OR IGNORE INTO commerce_product_spu (
  id, tenant_id, organization_id, spu_no, name, title, status, created_at, updated_at
) VALUES (
  'seed-product-membership', '100001', '0', 'membership-catalog', 'Membership Catalog', 'Membership Catalog', 'active', datetime('now'), datetime('now')
);

INSERT OR IGNORE INTO membership_plan (
  id, tenant_id, organization_id, plan_no, plan_code, name, rank, description, status, created_at, updated_at
) VALUES
  ('plan-free', '100001', '0', 'free', 'free', 'Free', 0, 'Experience core features with daily bonus points.', 'active', datetime('now'), datetime('now')),
  ('plan-basic', '100001', '0', 'basic', 'basic', '基础会员', 1, '适合个人创作者，解锁基础AI能力。', 'active', datetime('now'), datetime('now')),
  ('plan-standard', '100001', '0', 'standard', 'standard', '标准会员', 2, '适合专业创作者，解锁全部AI模型。', 'active', datetime('now'), datetime('now')),
  ('plan-premium', '100001', '0', 'premium', 'premium', '高级会员', 3, '适合团队和重度用户，享受专属通道。', 'active', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO membership_plan_version (
  id, tenant_id, organization_id, plan_id, version_no, title, description, lifecycle_status, effective_from, published_at, created_at, updated_at
) VALUES
  ('plan-free-v1', '100001', '0', 'plan-free', 'v1', 'Free', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now')),
  ('plan-basic-v1', '100001', '0', 'plan-basic', 'v1', '基础会员', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now')),
  ('plan-standard-v1', '100001', '0', 'plan-standard', 'v1', '标准会员', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now')),
  ('plan-premium-v1', '100001', '0', 'plan-premium', 'v1', '高级会员', NULL, 'published', datetime('now'), datetime('now'), datetime('now'), datetime('now'));

-- Benefit definitions: display benefits (points, queues, watermark, support)
INSERT OR IGNORE INTO benefit_definition (
  id, tenant_id, organization_id, benefit_code, name, benefit_type, value_unit, measurement_type, description, status, created_at, updated_at
) VALUES
  ('benefit-daily-points', '100001', '0', 'daily_points', 'Daily bonus points', 'points', 'count', 'counter', 'Claim daily login rewards.', 'active', datetime('now'), datetime('now')),
  ('benefit-standard-queue', '100001', '0', 'standard_queue', 'Standard generation lane', 'queue', 'count', 'counter', 'Standard priority queue.', 'active', datetime('now'), datetime('now')),
  ('benefit-fast-queue', '100001', '0', 'fast_queue', 'Fast generation lane', 'queue', 'count', 'counter', 'High priority fast queue.', 'active', datetime('now'), datetime('now')),
  ('benefit-vip-queue', '100001', '0', 'vip_queue', 'VIP priority lane', 'queue', 'count', 'counter', 'Dedicated VIP queue.', 'active', datetime('now'), datetime('now')),
  ('benefit-no-watermark', '100001', '0', 'no_watermark', 'Watermark-free export', 'feature', 'count', 'counter', 'Export without watermark.', 'active', datetime('now'), datetime('now')),
  ('benefit-vip-support', '100001', '0', 'vip_support', 'Dedicated support', 'service', 'count', 'counter', 'Priority support channel.', 'active', datetime('now'), datetime('now'));

-- Benefit definitions: privilege benefits (speed_up, priority_queue, ai_quota)
INSERT OR IGNORE INTO benefit_definition (
  id, tenant_id, organization_id, benefit_code, name, benefit_type, value_unit, measurement_type, description, status, created_at, updated_at
) VALUES
  ('seed-benefit-priority-speed-up', '100001', '0', 'priority_speed_up', 'Priority speed-up', 'quota', 'count', 'counter', 'Manual acceleration quota for generation jobs.', 'active', datetime('now'), datetime('now')),
  ('benefit-definition-priority_queue', '100001', '0', 'priority_queue', 'Priority queue', 'quota', 'count', 'counter', 'Priority queue capacity for active members.', 'active', datetime('now'), datetime('now')),
  ('seed-benefit-ai-quota', '100001', '0', 'ai_quota', 'AI quota', 'quota', 'count', 'counter', 'Exclusive model and AI quota allowance.', 'active', datetime('now'), datetime('now'));

-- Plan benefits: display benefits (daily_points, queues, watermark, support)
INSERT OR IGNORE INTO membership_plan_benefit (
  id, tenant_id, organization_id, plan_id, plan_version_id, benefit_id, benefit_code, grant_quantity, grant_period, usage_policy, sort_weight, status, created_at, updated_at
) VALUES
  ('plan-basic-v1-benefit-1', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-daily-points', 'daily_points', '725', 'membership_period', 'included', 1, 'active', datetime('now'), datetime('now')),
  ('plan-basic-v1-benefit-2', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-standard-queue', 'standard_queue', '1', 'membership_period', 'included', 2, 'active', datetime('now'), datetime('now')),
  ('plan-standard-v1-benefit-1', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-daily-points', 'daily_points', '2210', 'membership_period', 'included', 1, 'active', datetime('now'), datetime('now')),
  ('plan-standard-v1-benefit-2', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-fast-queue', 'fast_queue', '1', 'membership_period', 'included', 2, 'active', datetime('now'), datetime('now')),
  ('plan-standard-v1-benefit-3', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-no-watermark', 'no_watermark', '1', 'membership_period', 'included', 3, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-1', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-daily-points', 'daily_points', '6160', 'membership_period', 'included', 1, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-2', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-vip-queue', 'vip_queue', '1', 'membership_period', 'included', 2, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-3', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-no-watermark', 'no_watermark', '1', 'membership_period', 'included', 3, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-4', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-vip-support', 'vip_support', '1', 'membership_period', 'included', 4, 'active', datetime('now'), datetime('now'));

-- Plan benefits: privilege benefits (priority_queue, speed_up, ai_quota)
INSERT OR IGNORE INTO membership_plan_benefit (
  id, tenant_id, organization_id, plan_id, plan_version_id, benefit_id, benefit_code, grant_quantity, grant_period, usage_policy, sort_weight, status, created_at, updated_at
) VALUES
  ('plan-basic-v1-benefit-priority-queue', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-definition-priority_queue', 'priority_queue', '20', 'membership_period', 'included', 10, 'active', datetime('now'), datetime('now')),
  ('plan-standard-v1-benefit-speed-up', '100001', '0', 'plan-standard', 'plan-standard-v1', 'seed-benefit-priority-speed-up', 'priority_speed_up', '20', 'membership_period', 'included', 10, 'active', datetime('now'), datetime('now')),
  ('plan-standard-v1-benefit-priority-queue', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-definition-priority_queue', 'priority_queue', '100', 'membership_period', 'included', 11, 'active', datetime('now'), datetime('now')),
  ('plan-standard-v1-benefit-ai-quota', '100001', '0', 'plan-standard', 'plan-standard-v1', 'seed-benefit-ai-quota', 'ai_quota', '50', 'membership_period', 'included', 12, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-speed-up', '100001', '0', 'plan-premium', 'plan-premium-v1', 'seed-benefit-priority-speed-up', 'priority_speed_up', '80', 'membership_period', 'included', 10, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-priority-queue', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-definition-priority_queue', 'priority_queue', '300', 'membership_period', 'included', 11, 'active', datetime('now'), datetime('now')),
  ('plan-premium-v1-benefit-ai-quota', '100001', '0', 'plan-premium', 'plan-premium-v1', 'seed-benefit-ai-quota', 'ai_quota', '200', 'membership_period', 'included', 12, 'active', datetime('now'), datetime('now'));

-- Package groups: 4 billing cycles with discount labels matching frontend display
INSERT OR IGNORE INTO membership_package_group (
  id, tenant_id, organization_id, external_id, group_no, name, description, billing_cycle, duration_days, display_channel, sort_weight, status, created_at, updated_at
) VALUES
  ('package-group-annual', '100001', '0', 1, 'annual', '连续包年', '5.8折', 'year', 365, 'app', 1, 'active', datetime('now'), datetime('now')),
  ('package-group-monthly', '100001', '0', 2, 'monthly', '连续包月', '6折', 'month', 30, 'app', 2, 'active', datetime('now'), datetime('now')),
  ('package-group-quarterly', '100001', '0', 3, 'quarterly', '连续包季', '7折', 'quarter', 90, 'app', 3, 'active', datetime('now'), datetime('now')),
  ('package-group-single', '100001', '0', 4, 'single', '单月购买', NULL, 'month', 30, 'app', 4, 'active', datetime('now'), datetime('now'));

-- SKUs with spec_json tags matching frontend display (first tag = discount label)
INSERT OR IGNORE INTO commerce_product_sku (
  id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at
) VALUES
  ('sku-basic-annual', '100001', '0', 'seed-product-membership', 'basic-annual', 'Basic Annual', 'Basic Annual', '393', '659', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["首年6折","每日24积分"]}', datetime('now'), datetime('now')),
  ('sku-standard-annual', '100001', '0', 'seed-product-membership', 'standard-annual', 'Standard Annual', 'Standard Annual', '1099', '1899', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["首年5.8折","每日74积分"]}', datetime('now'), datetime('now')),
  ('sku-premium-annual', '100001', '0', 'seed-product-membership', 'premium-annual', 'Premium Annual', 'Premium Annual', '3099', '5199', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["首年6折","每日205积分"]}', datetime('now'), datetime('now')),
  ('sku-basic-monthly', '100001', '0', 'seed-product-membership', 'basic-monthly', 'Basic Monthly', 'Basic Monthly', '33', '55', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["6折","每日24积分"]}', datetime('now'), datetime('now')),
  ('sku-standard-monthly', '100001', '0', 'seed-product-membership', 'standard-monthly', 'Standard Monthly', 'Standard Monthly', '95', '158', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["6折","每日74积分"]}', datetime('now'), datetime('now')),
  ('sku-premium-monthly', '100001', '0', 'seed-product-membership', 'premium-monthly', 'Premium Monthly', 'Premium Monthly', '260', '433', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["6折","每日205积分"]}', datetime('now'), datetime('now')),
  ('sku-basic-quarterly', '100001', '0', 'seed-product-membership', 'basic-quarterly', 'Basic Quarterly', 'Basic Quarterly', '116', '165', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["7折","每日24积分"]}', datetime('now'), datetime('now')),
  ('sku-standard-quarterly', '100001', '0', 'seed-product-membership', 'standard-quarterly', 'Standard Quarterly', 'Standard Quarterly', '332', '474', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["7折","每日74积分"]}', datetime('now'), datetime('now')),
  ('sku-premium-quarterly', '100001', '0', 'seed-product-membership', 'premium-quarterly', 'Premium Quarterly', 'Premium Quarterly', '909', '1299', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["7折","每日205积分"]}', datetime('now'), datetime('now')),
  ('sku-basic-single', '100001', '0', 'seed-product-membership', 'basic-single', 'Basic Single', 'Basic Single', '55', NULL, 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["每日24积分"]}', datetime('now'), datetime('now')),
  ('sku-standard-single', '100001', '0', 'seed-product-membership', 'standard-single', 'Standard Single', 'Standard Single', '158', NULL, 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["每日74积分"]}', datetime('now'), datetime('now')),
  ('sku-premium-single', '100001', '0', 'seed-product-membership', 'premium-single', 'Premium Single', 'Premium Single', '433', NULL, 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["每日205积分"]}', datetime('now'), datetime('now'));

-- Packages: 12 sellable packages with prices aligned to frontend display
INSERT OR IGNORE INTO membership_package (
  id, tenant_id, organization_id, external_id, package_no, package_group_id, plan_id, plan_version_id, sku_id, name, description, price_amount, original_price_amount, currency_code, point_amount, duration_days, recurrence_cycle, sort_weight, recommended, status, created_at, updated_at
) VALUES
  ('package-basic-annual', '100001', '0', 101, 'basic-annual', 'package-group-annual', 'plan-basic', 'plan-basic-v1', 'sku-basic-annual', '基础会员·年卡', '连续包年，到期自动续费', '393', '659', 'CNY', 8700, 365, 'year', 1, 0, 'active', datetime('now'), datetime('now')),
  ('package-standard-annual', '100001', '0', 102, 'standard-annual', 'package-group-annual', 'plan-standard', 'plan-standard-v1', 'sku-standard-annual', '标准会员·年卡', '连续包年，到期自动续费', '1099', '1899', 'CNY', 26520, 365, 'year', 2, 1, 'active', datetime('now'), datetime('now')),
  ('package-premium-annual', '100001', '0', 103, 'premium-annual', 'package-group-annual', 'plan-premium', 'plan-premium-v1', 'sku-premium-annual', '高级会员·年卡', '连续包年，到期自动续费', '3099', '5199', 'CNY', 73920, 365, 'year', 3, 0, 'active', datetime('now'), datetime('now')),
  ('package-basic-monthly', '100001', '0', 201, 'basic-monthly', 'package-group-monthly', 'plan-basic', 'plan-basic-v1', 'sku-basic-monthly', '基础会员·月卡', '连续包月，到期自动续费', '33', '55', 'CNY', 725, 30, 'month', 1, 0, 'active', datetime('now'), datetime('now')),
  ('package-standard-monthly', '100001', '0', 202, 'standard-monthly', 'package-group-monthly', 'plan-standard', 'plan-standard-v1', 'sku-standard-monthly', '标准会员·月卡', '连续包月，到期自动续费', '95', '158', 'CNY', 2210, 30, 'month', 2, 1, 'active', datetime('now'), datetime('now')),
  ('package-premium-monthly', '100001', '0', 203, 'premium-monthly', 'package-group-monthly', 'plan-premium', 'plan-premium-v1', 'sku-premium-monthly', '高级会员·月卡', '连续包月，到期自动续费', '260', '433', 'CNY', 6160, 30, 'month', 3, 0, 'active', datetime('now'), datetime('now')),
  ('package-basic-quarterly', '100001', '0', 301, 'basic-quarterly', 'package-group-quarterly', 'plan-basic', 'plan-basic-v1', 'sku-basic-quarterly', '基础会员·季卡', '连续包季，到期自动续费', '116', '165', 'CNY', 2175, 90, 'quarter', 1, 0, 'active', datetime('now'), datetime('now')),
  ('package-standard-quarterly', '100001', '0', 302, 'standard-quarterly', 'package-group-quarterly', 'plan-standard', 'plan-standard-v1', 'sku-standard-quarterly', '标准会员·季卡', '连续包季，到期自动续费', '332', '474', 'CNY', 6630, 90, 'quarter', 2, 0, 'active', datetime('now'), datetime('now')),
  ('package-premium-quarterly', '100001', '0', 303, 'premium-quarterly', 'package-group-quarterly', 'plan-premium', 'plan-premium-v1', 'sku-premium-quarterly', '高级会员·季卡', '连续包季，到期自动续费', '909', '1299', 'CNY', 18480, 90, 'quarter', 3, 0, 'active', datetime('now'), datetime('now')),
  ('package-basic-single', '100001', '0', 401, 'basic-single', 'package-group-single', 'plan-basic', 'plan-basic-v1', 'sku-basic-single', '基础会员·单月', '单次购买，不自动续费', '55', NULL, 'CNY', 725, 30, 'once', 1, 0, 'active', datetime('now'), datetime('now')),
  ('package-standard-single', '100001', '0', 402, 'standard-single', 'package-group-single', 'plan-standard', 'plan-standard-v1', 'sku-standard-single', '标准会员·单月', '单次购买，不自动续费', '158', NULL, 'CNY', 2210, 30, 'once', 2, 0, 'active', datetime('now'), datetime('now')),
  ('package-premium-single', '100001', '0', 403, 'premium-single', 'package-group-single', 'plan-premium', 'plan-premium-v1', 'sku-premium-single', '高级会员·单月', '单次购买，不自动续费', '433', NULL, 'CNY', 6160, 30, 'once', 3, 0, 'active', datetime('now'), datetime('now'));
