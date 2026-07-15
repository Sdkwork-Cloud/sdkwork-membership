-- ============================================================================
-- en-US locale seed: sets display text for membership catalog tables.
--
-- This file runs AFTER common/001_catalog.sql and explicitly sets English
-- display text. It uses UPDATE statements keyed by primary key so re-seeding
-- is idempotent.
--
-- Pricing model: ¥10 = 100 compute credits (1 CNY = 10 credits)
-- point_amount = price × 10
-- ============================================================================

-- ---------------------------------------------------------------------------
-- membership_plan: plan display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE membership_plan SET name = 'Free', description = 'Experience core features with daily bonus compute credits.', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-free';
UPDATE membership_plan SET name = 'Basic', description = 'For individual creators, unlock basic AI capabilities.', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic';
UPDATE membership_plan SET name = 'Standard', description = 'For professional creators, unlock all AI models.', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard';
UPDATE membership_plan SET name = 'Premium', description = 'For teams and power users, enjoy dedicated channels.', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium';
UPDATE membership_plan SET name = 'Super', description = 'Ultimate tier with VIP queue, maximum quotas, and exclusive effects.', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super';

-- ---------------------------------------------------------------------------
-- membership_plan_version: version display titles
-- ---------------------------------------------------------------------------
UPDATE membership_plan_version SET title = 'Free', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-free-v1';
UPDATE membership_plan_version SET title = 'Basic', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1';
UPDATE membership_plan_version SET title = 'Standard', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1';
UPDATE membership_plan_version SET title = 'Premium', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1';
UPDATE membership_plan_version SET title = 'Super', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1';

-- ---------------------------------------------------------------------------
-- benefit_definition: benefit display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE benefit_definition SET name = 'Platform free credits', description = 'Daily login free compute credits.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-platform-free-points';
UPDATE benefit_definition SET name = 'Purchased credits', description = 'Compute credits purchased via recharge.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-purchased-points';
UPDATE benefit_definition SET name = 'Subscription monthly credits', description = 'Monthly subscription bonus compute credits.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-daily-points';
UPDATE benefit_definition SET name = 'Seedance 2.0 VIP model', description = 'Access to Seedance 2.0 VIP model.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-seedance-vip-model';
UPDATE benefit_definition SET name = 'Seedance 1.5 Pro model', description = 'Seedance 1.5 Pro model 3% off credits.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-seedance-pro-model';
UPDATE benefit_definition SET name = 'Standard generation lane', description = 'Standard priority generation queue.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-standard-queue';
UPDATE benefit_definition SET name = 'Fast generation lane', description = 'High priority fast generation queue.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-fast-queue';
UPDATE benefit_definition SET name = 'VIP priority lane', description = 'Dedicated VIP queue.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-vip-queue';
UPDATE benefit_definition SET name = 'Video lip sync', description = 'Video lip-sync feature access.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-video-lip-sync';
UPDATE benefit_definition SET name = 'Video HD', description = 'Video HD export feature.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-video-hd';
UPDATE benefit_definition SET name = 'Video frame interpolation', description = 'Video frame interpolation feature.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-video-frame-interp';
UPDATE benefit_definition SET name = 'Image 4.0 limited free', description = 'Image 4.0 free tier resolution.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-image-4k-free';
UPDATE benefit_definition SET name = 'Smart upscale', description = 'Smart upscaling resolution.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-smart-upscale';
UPDATE benefit_definition SET name = 'Watermark-free export', description = 'Export without watermark.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-no-watermark';
UPDATE benefit_definition SET name = 'Generation acceleration', description = 'Accelerated generation speed.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-generation-acceleration';
UPDATE benefit_definition SET name = 'Worry-free refund', description = 'Worry-free refund and priority support.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-vip-support';

-- Privilege benefits (runtime quota)
UPDATE benefit_definition SET name = 'Priority speed-up', description = 'Manual acceleration quota for generation jobs.', updated_at = CURRENT_TIMESTAMP WHERE id = 'seed-benefit-priority-speed-up';
UPDATE benefit_definition SET name = 'Priority queue', description = 'Priority queue capacity for active members.', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-definition-priority_queue';
UPDATE benefit_definition SET name = 'AI quota', description = 'Exclusive model and AI quota allowance.', updated_at = CURRENT_TIMESTAMP WHERE id = 'seed-benefit-ai-quota';

-- ---------------------------------------------------------------------------
-- membership_plan_benefit: grant_quantity display text (text-type cells only)
-- Numeric cells (e.g. '330', '1', '20') remain unchanged.
-- ---------------------------------------------------------------------------
-- Free plan
UPDATE membership_plan_benefit SET grant_quantity = 'Daily login free credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-free-v1-benefit-platform-points';

-- Basic plan
UPDATE membership_plan_benefit SET grant_quantity = 'Daily login bonus credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '3% off credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = 'Standard generation lane', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1-benefit-standard-queue';

-- Standard plan
UPDATE membership_plan_benefit SET grant_quantity = 'Daily login bonus credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '3% off credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = 'Standard generation lane', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1-benefit-standard-queue';

-- Premium plan
UPDATE membership_plan_benefit SET grant_quantity = 'Daily login bonus credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '3% off credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = 'Fast generation lane', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1-benefit-fast-queue';

-- Super plan
UPDATE membership_plan_benefit SET grant_quantity = 'Daily login bonus credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '3% off credits', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = 'VIP priority lane', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1-benefit-vip-queue';

-- ---------------------------------------------------------------------------
-- membership_package_group: group display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE membership_package_group SET name = 'Annual subscription', description = '3% off', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-annual';
UPDATE membership_package_group SET name = 'Monthly subscription', description = '1% off', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-monthly';
UPDATE membership_package_group SET name = 'Quarterly subscription', description = '2% off', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-quarterly';
UPDATE membership_package_group SET name = 'Single month purchase', description = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-single';

-- ---------------------------------------------------------------------------
-- commerce_product_sku: SKU display names and spec_json tags
-- Tags: ¥10=100 credits, daily = point_amount/30 (monthly sub rate)
-- Basic: 540/30=18, Standard: 1560/30=52, Premium: 4290/30=143, Super: 10720/30≈357
-- ---------------------------------------------------------------------------
UPDATE commerce_product_sku SET name = 'Basic Annual', title = 'Basic Annual', spec_json = '{"tags":["3% off","18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-annual';
UPDATE commerce_product_sku SET name = 'Standard Annual', title = 'Standard Annual', spec_json = '{"tags":["3% off","52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-annual';
UPDATE commerce_product_sku SET name = 'Premium Annual', title = 'Premium Annual', spec_json = '{"tags":["3% off","143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-annual';
UPDATE commerce_product_sku SET name = 'Basic Monthly', title = 'Basic Monthly', spec_json = '{"tags":["1% off","18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-monthly';
UPDATE commerce_product_sku SET name = 'Standard Monthly', title = 'Standard Monthly', spec_json = '{"tags":["1% off","52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-monthly';
UPDATE commerce_product_sku SET name = 'Premium Monthly', title = 'Premium Monthly', spec_json = '{"tags":["1% off","143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-monthly';
UPDATE commerce_product_sku SET name = 'Basic Quarterly', title = 'Basic Quarterly', spec_json = '{"tags":["2% off","18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-quarterly';
UPDATE commerce_product_sku SET name = 'Standard Quarterly', title = 'Standard Quarterly', spec_json = '{"tags":["2% off","52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-quarterly';
UPDATE commerce_product_sku SET name = 'Premium Quarterly', title = 'Premium Quarterly', spec_json = '{"tags":["2% off","143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-quarterly';
UPDATE commerce_product_sku SET name = 'Basic Single', title = 'Basic Single', spec_json = '{"tags":["18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-single';
UPDATE commerce_product_sku SET name = 'Standard Single', title = 'Standard Single', spec_json = '{"tags":["52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-single';
UPDATE commerce_product_sku SET name = 'Premium Single', title = 'Premium Single', spec_json = '{"tags":["143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-single';
UPDATE commerce_product_sku SET name = 'Super Annual', title = 'Super Annual', spec_json = '{"tags":["3% off","357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-annual';
UPDATE commerce_product_sku SET name = 'Super Monthly', title = 'Super Monthly', spec_json = '{"tags":["1% off","357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-monthly';
UPDATE commerce_product_sku SET name = 'Super Quarterly', title = 'Super Quarterly', spec_json = '{"tags":["2% off","357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-quarterly';
UPDATE commerce_product_sku SET name = 'Super Single', title = 'Super Single', spec_json = '{"tags":["357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-single';

-- ---------------------------------------------------------------------------
-- membership_package: package display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE membership_package SET name = 'Basic - Annual', description = 'Annual subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-annual';
UPDATE membership_package SET name = 'Standard - Annual', description = 'Annual subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-annual';
UPDATE membership_package SET name = 'Premium - Annual', description = 'Annual subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-annual';
UPDATE membership_package SET name = 'Basic - Monthly', description = 'Monthly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-monthly';
UPDATE membership_package SET name = 'Standard - Monthly', description = 'Monthly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-monthly';
UPDATE membership_package SET name = 'Premium - Monthly', description = 'Monthly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-monthly';
UPDATE membership_package SET name = 'Basic - Quarterly', description = 'Quarterly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-quarterly';
UPDATE membership_package SET name = 'Standard - Quarterly', description = 'Quarterly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-quarterly';
UPDATE membership_package SET name = 'Premium - Quarterly', description = 'Quarterly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-quarterly';
UPDATE membership_package SET name = 'Basic - Single Month', description = 'One-time purchase, no auto-renew', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-single';
UPDATE membership_package SET name = 'Standard - Single Month', description = 'One-time purchase, no auto-renew', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-single';
UPDATE membership_package SET name = 'Premium - Single Month', description = 'One-time purchase, no auto-renew', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-single';
UPDATE membership_package SET name = 'Super - Annual', description = 'Annual subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-annual';
UPDATE membership_package SET name = 'Super - Monthly', description = 'Monthly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-monthly';
UPDATE membership_package SET name = 'Super - Quarterly', description = 'Quarterly subscription, auto-renew on expiry', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-quarterly';
UPDATE membership_package SET name = 'Super - Single Month', description = 'One-time purchase, no auto-renew', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-single';
