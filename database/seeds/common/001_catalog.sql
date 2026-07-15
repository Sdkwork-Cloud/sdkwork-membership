-- Membership token plan catalog seed for tenant 100001
-- Full commercial catalog: 4 billing-cycle package groups x 3 purchasable tiers.
-- External ids align with app-api read models (101-103 annual, 201-203 monthly, 301-303 quarterly, 401-403 single).
-- Prices and benefits align with frontend subscription-catalog-content.ts fallback display.
--
-- This file contains locale-neutral structural data with English display text.
-- Locale-specific display text (zh-CN, en-US, etc.) is applied by locale seed
-- files in seeds/locales/<locale>/001_catalog_locale.sql which run AFTER this
-- common script and overwrite display fields via UPDATE statements.
--
-- ON CONFLICT (id) DO UPDATE SET is used (not DO NOTHING) so that re-seeding
-- after seed file corrections will overwrite existing rows, including
-- previously garbled data from older seed versions.

INSERT INTO commerce_product_spu (
  id, tenant_id, organization_id, spu_no, name, title, status, created_at, updated_at
) VALUES (
  'seed-product-membership', '100001', '0', 'membership-catalog', 'Membership Catalog', 'Membership Catalog', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  title = EXCLUDED.title,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;

INSERT INTO membership_plan (
  id, tenant_id, organization_id, plan_no, plan_code, name, rank, description, status, created_at, updated_at
) VALUES
  ('plan-free', '100001', '0', 'free', 'free', 'Free', 0, 'Experience core features with daily bonus compute credits.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic', '100001', '0', 'basic', 'basic', 'Basic', 1, 'For individual creators, unlock basic AI capabilities.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard', '100001', '0', 'standard', 'standard', 'Standard', 2, 'For professional creators, unlock all AI models.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium', '100001', '0', 'premium', 'premium', 'Premium', 3, 'For teams and power users, enjoy dedicated channels.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super', '100001', '0', 'super', 'super', 'Super', 4, 'Ultimate tier with VIP queue, maximum quotas, and exclusive effects.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  rank = EXCLUDED.rank,
  description = EXCLUDED.description,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;

INSERT INTO membership_plan_version (
  id, tenant_id, organization_id, plan_id, version_no, title, description, lifecycle_status, effective_from, published_at, created_at, updated_at
) VALUES
  ('plan-free-v1', '100001', '0', 'plan-free', 'v1', 'Free', NULL, 'published', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1', '100001', '0', 'plan-basic', 'v1', 'Basic', NULL, 'published', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1', '100001', '0', 'plan-standard', 'v1', 'Standard', NULL, 'published', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1', '100001', '0', 'plan-premium', 'v1', 'Premium', NULL, 'published', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1', '100001', '0', 'plan-super', 'v1', 'Super', NULL, 'published', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  title = EXCLUDED.title,
  description = EXCLUDED.description,
  lifecycle_status = EXCLUDED.lifecycle_status,
  updated_at = EXCLUDED.updated_at;

-- =============================================================================
-- Benefit definitions: comparison-table display benefits
-- grant_quantity stores text for non-numeric cells (display_value) and numbers
-- for numeric/boolean cells (usage_limit).
-- Benefit names/descriptions are English here; locale files override display text.
-- =============================================================================
INSERT INTO benefit_definition (
  id, tenant_id, organization_id, benefit_code, name, benefit_type, value_unit, measurement_type, description, status, created_at, updated_at
) VALUES
  ('benefit-platform-free-points', '100001', '0', 'platform_free_points', 'Platform free compute credits', 'points', 'text', 'text', 'Daily login free compute credits.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-purchased-points', '100001', '0', 'purchased_points', 'Purchased compute credits', 'points', 'count', 'counter', 'Compute credits purchased via recharge.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-daily-points', '100001', '0', 'daily_points', 'Subscription monthly compute credits', 'points', 'count', 'counter', 'Monthly subscription bonus compute credits.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-seedance-vip-model', '100001', '0', 'seedance_vip_model', 'Seedance 2.0 VIP model', 'feature', 'count', 'counter', 'Access to Seedance 2.0 VIP model.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-seedance-pro-model', '100001', '0', 'seedance_pro_model', 'Seedance 1.5 Pro model', 'feature', 'text', 'text', 'Seedance 1.5 Pro model 3% off credits.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-standard-queue', '100001', '0', 'standard_queue', 'Standard generation lane', 'queue', 'text', 'text', 'Standard priority generation queue.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-fast-queue', '100001', '0', 'fast_queue', 'Fast generation lane', 'queue', 'text', 'text', 'High priority fast generation queue.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-vip-queue', '100001', '0', 'vip_queue', 'VIP priority lane', 'queue', 'text', 'text', 'Dedicated VIP queue.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-video-lip-sync', '100001', '0', 'video_lip_sync', 'Video lip sync', 'feature', 'count', 'counter', 'Video lip-sync feature access.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-video-hd', '100001', '0', 'video_hd', 'Video HD', 'feature', 'count', 'counter', 'Video HD export feature.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-video-frame-interp', '100001', '0', 'video_frame_interp', 'Video frame interpolation', 'feature', 'count', 'counter', 'Video frame interpolation feature.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-image-4k-free', '100001', '0', 'image_4k_free', 'Image 4.0 limited free', 'feature', 'text', 'text', 'Image 4.0 free tier resolution.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-smart-upscale', '100001', '0', 'smart_upscale', 'Smart upscale', 'feature', 'text', 'text', 'Smart upscaling resolution.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-no-watermark', '100001', '0', 'no_watermark', 'Watermark-free export', 'feature', 'count', 'counter', 'Export without watermark.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-generation-acceleration', '100001', '0', 'generation_acceleration', 'Generation acceleration', 'feature', 'count', 'counter', 'Accelerated generation speed.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-vip-support', '100001', '0', 'vip_support', 'Worry-free refund', 'service', 'count', 'counter', 'Worry-free refund and priority support.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  benefit_type = EXCLUDED.benefit_type,
  value_unit = EXCLUDED.value_unit,
  measurement_type = EXCLUDED.measurement_type,
  description = EXCLUDED.description,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;

-- Benefit definitions: privilege benefits (runtime quota, not shown in comparison table)
INSERT INTO benefit_definition (
  id, tenant_id, organization_id, benefit_code, name, benefit_type, value_unit, measurement_type, description, status, created_at, updated_at
) VALUES
  ('seed-benefit-priority-speed-up', '100001', '0', 'priority_speed_up', 'Priority speed-up', 'quota', 'count', 'counter', 'Manual acceleration quota for generation jobs.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('benefit-definition-priority_queue', '100001', '0', 'priority_queue', 'Priority queue', 'quota', 'count', 'counter', 'Priority queue capacity for active members.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('seed-benefit-ai-quota', '100001', '0', 'ai_quota', 'AI quota', 'quota', 'count', 'counter', 'Exclusive model and AI quota allowance.', 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  benefit_type = EXCLUDED.benefit_type,
  value_unit = EXCLUDED.value_unit,
  measurement_type = EXCLUDED.measurement_type,
  description = EXCLUDED.description,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;

-- =============================================================================
-- Plan benefits: grant_quantity display text is locale-neutral here (English).
-- Locale seed files override grant_quantity display text via UPDATE.
-- =============================================================================
INSERT INTO membership_plan_benefit (
  id, tenant_id, organization_id, plan_id, plan_version_id, benefit_id, benefit_code, grant_quantity, grant_period, usage_policy, sort_weight, status, created_at, updated_at
) VALUES
  -- Free plan (rank 0)
  ('plan-free-v1-benefit-platform-points', '100001', '0', 'plan-free', 'plan-free-v1', 'benefit-platform-free-points', 'platform_free_points', 'Daily login free credits', 'membership_period', 'included', 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-free-v1-benefit-purchased-points', '100001', '0', 'plan-free', 'plan-free-v1', 'benefit-purchased-points', 'purchased_points', '1', 'membership_period', 'included', 2, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  -- Basic plan (rank 1)
  ('plan-basic-v1-benefit-platform-points', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-platform-free-points', 'platform_free_points', 'Daily login bonus credits', 'membership_period', 'included', 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-purchased-points', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-purchased-points', 'purchased_points', '1', 'membership_period', 'included', 2, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-daily-points', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-daily-points', 'daily_points', '540', 'membership_period', 'included', 3, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-seedance-vip', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-seedance-vip-model', 'seedance_vip_model', '1', 'membership_period', 'included', 4, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-seedance-pro', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-seedance-pro-model', 'seedance_pro_model', '3% off credits', 'membership_period', 'included', 5, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-standard-queue', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-standard-queue', 'standard_queue', 'Standard generation lane', 'membership_period', 'included', 6, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-video-lip-sync', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-video-lip-sync', 'video_lip_sync', '1', 'membership_period', 'included', 7, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-video-hd', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-video-hd', 'video_hd', '1', 'membership_period', 'included', 8, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-video-frame-interp', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-video-frame-interp', 'video_frame_interp', '1', 'membership_period', 'included', 9, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-image-4k', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-image-4k-free', 'image_4k_free', '2K', 'membership_period', 'included', 10, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-smart-upscale', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-smart-upscale', 'smart_upscale', '4K', 'membership_period', 'included', 11, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-no-watermark', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-no-watermark', 'no_watermark', '1', 'membership_period', 'included', 12, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-gen-acceleration', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-generation-acceleration', 'generation_acceleration', '1', 'membership_period', 'included', 13, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-basic-v1-benefit-vip-support', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-vip-support', 'vip_support', '1', 'membership_period', 'included', 14, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  -- Standard plan (rank 2)
  ('plan-standard-v1-benefit-platform-points', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-platform-free-points', 'platform_free_points', 'Daily login bonus credits', 'membership_period', 'included', 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-purchased-points', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-purchased-points', 'purchased_points', '1', 'membership_period', 'included', 2, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-daily-points', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-daily-points', 'daily_points', '1560', 'membership_period', 'included', 3, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-seedance-vip', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-seedance-vip-model', 'seedance_vip_model', '1', 'membership_period', 'included', 4, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-seedance-pro', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-seedance-pro-model', 'seedance_pro_model', '3% off credits', 'membership_period', 'included', 5, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-standard-queue', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-standard-queue', 'standard_queue', 'Standard generation lane', 'membership_period', 'included', 6, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-video-lip-sync', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-video-lip-sync', 'video_lip_sync', '1', 'membership_period', 'included', 7, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-video-hd', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-video-hd', 'video_hd', '1', 'membership_period', 'included', 8, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-video-frame-interp', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-video-frame-interp', 'video_frame_interp', '1', 'membership_period', 'included', 9, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-image-4k', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-image-4k-free', 'image_4k_free', '2K', 'membership_period', 'included', 10, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-smart-upscale', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-smart-upscale', 'smart_upscale', '4K', 'membership_period', 'included', 11, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-no-watermark', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-no-watermark', 'no_watermark', '1', 'membership_period', 'included', 12, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-gen-acceleration', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-generation-acceleration', 'generation_acceleration', '1', 'membership_period', 'included', 13, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-vip-support', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-vip-support', 'vip_support', '1', 'membership_period', 'included', 14, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  -- Premium plan (rank 3)
  ('plan-premium-v1-benefit-platform-points', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-platform-free-points', 'platform_free_points', 'Daily login bonus credits', 'membership_period', 'included', 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-purchased-points', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-purchased-points', 'purchased_points', '1', 'membership_period', 'included', 2, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-daily-points', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-daily-points', 'daily_points', '4290', 'membership_period', 'included', 3, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-seedance-vip', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-seedance-vip-model', 'seedance_vip_model', '1', 'membership_period', 'included', 4, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-seedance-pro', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-seedance-pro-model', 'seedance_pro_model', '3% off credits', 'membership_period', 'included', 5, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-fast-queue', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-fast-queue', 'fast_queue', 'Fast generation lane', 'membership_period', 'included', 6, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-video-lip-sync', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-video-lip-sync', 'video_lip_sync', '1', 'membership_period', 'included', 7, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-video-hd', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-video-hd', 'video_hd', '1', 'membership_period', 'included', 8, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-video-frame-interp', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-video-frame-interp', 'video_frame_interp', '1', 'membership_period', 'included', 9, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-image-4k', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-image-4k-free', 'image_4k_free', '2K/4K', 'membership_period', 'included', 10, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-smart-upscale', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-smart-upscale', 'smart_upscale', '4K/8K', 'membership_period', 'included', 11, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-no-watermark', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-no-watermark', 'no_watermark', '1', 'membership_period', 'included', 12, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-gen-acceleration', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-generation-acceleration', 'generation_acceleration', '1', 'membership_period', 'included', 13, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-vip-support', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-vip-support', 'vip_support', '1', 'membership_period', 'included', 14, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  -- Privilege benefits (runtime quota)
  ('plan-basic-v1-benefit-priority-queue', '100001', '0', 'plan-basic', 'plan-basic-v1', 'benefit-definition-priority_queue', 'priority_queue', '20', 'membership_period', 'included', 20, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-speed-up', '100001', '0', 'plan-standard', 'plan-standard-v1', 'seed-benefit-priority-speed-up', 'priority_speed_up', '20', 'membership_period', 'included', 20, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-priority-queue', '100001', '0', 'plan-standard', 'plan-standard-v1', 'benefit-definition-priority_queue', 'priority_queue', '100', 'membership_period', 'included', 21, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-standard-v1-benefit-ai-quota', '100001', '0', 'plan-standard', 'plan-standard-v1', 'seed-benefit-ai-quota', 'ai_quota', '50', 'membership_period', 'included', 22, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-speed-up', '100001', '0', 'plan-premium', 'plan-premium-v1', 'seed-benefit-priority-speed-up', 'priority_speed_up', '80', 'membership_period', 'included', 20, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-priority-queue', '100001', '0', 'plan-premium', 'plan-premium-v1', 'benefit-definition-priority_queue', 'priority_queue', '300', 'membership_period', 'included', 21, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-premium-v1-benefit-ai-quota', '100001', '0', 'plan-premium', 'plan-premium-v1', 'seed-benefit-ai-quota', 'ai_quota', '200', 'membership_period', 'included', 22, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  -- Super plan (rank 4)
  ('plan-super-v1-benefit-platform-points', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-platform-free-points', 'platform_free_points', 'Daily login bonus credits', 'membership_period', 'included', 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-purchased-points', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-purchased-points', 'purchased_points', '1', 'membership_period', 'included', 2, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-daily-points', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-daily-points', 'daily_points', '10720', 'membership_period', 'included', 3, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-seedance-vip', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-seedance-vip-model', 'seedance_vip_model', '1', 'membership_period', 'included', 4, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-seedance-pro', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-seedance-pro-model', 'seedance_pro_model', '3% off credits', 'membership_period', 'included', 5, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-vip-queue', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-vip-queue', 'vip_queue', 'VIP priority lane', 'membership_period', 'included', 6, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-video-lip-sync', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-video-lip-sync', 'video_lip_sync', '1', 'membership_period', 'included', 7, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-video-hd', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-video-hd', 'video_hd', '1', 'membership_period', 'included', 8, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-video-frame-interp', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-video-frame-interp', 'video_frame_interp', '1', 'membership_period', 'included', 9, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-image-4k', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-image-4k-free', 'image_4k_free', '2K/4K', 'membership_period', 'included', 10, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-smart-upscale', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-smart-upscale', 'smart_upscale', '4K/8K', 'membership_period', 'included', 11, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-no-watermark', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-no-watermark', 'no_watermark', '1', 'membership_period', 'included', 12, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-gen-acceleration', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-generation-acceleration', 'generation_acceleration', '1', 'membership_period', 'included', 13, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-vip-support', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-vip-support', 'vip_support', '1', 'membership_period', 'included', 14, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-speed-up', '100001', '0', 'plan-super', 'plan-super-v1', 'seed-benefit-priority-speed-up', 'priority_speed_up', '200', 'membership_period', 'included', 20, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-priority-queue', '100001', '0', 'plan-super', 'plan-super-v1', 'benefit-definition-priority_queue', 'priority_queue', '500', 'membership_period', 'included', 21, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('plan-super-v1-benefit-ai-quota', '100001', '0', 'plan-super', 'plan-super-v1', 'seed-benefit-ai-quota', 'ai_quota', '500', 'membership_period', 'included', 22, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  grant_quantity = EXCLUDED.grant_quantity,
  grant_period = EXCLUDED.grant_period,
  usage_policy = EXCLUDED.usage_policy,
  sort_weight = EXCLUDED.sort_weight,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;

-- Package groups: 4 billing cycles (English defaults, locale files override)
INSERT INTO membership_package_group (
  id, tenant_id, organization_id, external_id, group_no, name, description, billing_cycle, duration_days, display_channel, sort_weight, status, created_at, updated_at
) VALUES
  ('package-group-annual', '100001', '0', 1, 'annual', 'Annual subscription', '3% off', 'year', 365, 'app', 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-group-monthly', '100001', '0', 2, 'monthly', 'Monthly subscription', '1% off', 'month', 30, 'app', 2, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-group-quarterly', '100001', '0', 3, 'quarterly', 'Quarterly subscription', '2% off', 'quarter', 90, 'app', 3, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-group-single', '100001', '0', 4, 'single', 'Single month purchase', NULL, 'month', 30, 'app', 4, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  description = EXCLUDED.description,
  billing_cycle = EXCLUDED.billing_cycle,
  duration_days = EXCLUDED.duration_days,
  sort_weight = EXCLUDED.sort_weight,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;

-- SKUs with spec_json tags (English defaults, locale files override tags)
INSERT INTO commerce_product_sku (
  id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount, original_price_amount, currency_code, fulfillment_type, inventory_tracking, status, spec_json, created_at, updated_at
) VALUES
  ('sku-basic-annual', '100001', '0', 'seed-product-membership', 'basic-annual', 'Basic Annual', 'Basic Annual', '640', '660', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["3% off","18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-annual', '100001', '0', 'seed-product-membership', 'standard-annual', 'Standard Annual', 'Standard Annual', '1839', '1896', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["3% off","52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-annual', '100001', '0', 'seed-product-membership', 'premium-annual', 'Premium Annual', 'Premium Annual', '5040', '5196', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["3% off","143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-annual', '100001', '0', 'seed-product-membership', 'super-annual', 'Super Annual', 'Super Annual', '12606', '12996', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["3% off","357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-basic-monthly', '100001', '0', 'seed-product-membership', 'basic-monthly', 'Basic Monthly', 'Basic Monthly', '54', '55', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["1% off","18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-monthly', '100001', '0', 'seed-product-membership', 'standard-monthly', 'Standard Monthly', 'Standard Monthly', '156', '158', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["1% off","52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-monthly', '100001', '0', 'seed-product-membership', 'premium-monthly', 'Premium Monthly', 'Premium Monthly', '429', '433', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["1% off","143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-monthly', '100001', '0', 'seed-product-membership', 'super-monthly', 'Super Monthly', 'Super Monthly', '1072', '1083', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["1% off","357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-basic-quarterly', '100001', '0', 'seed-product-membership', 'basic-quarterly', 'Basic Quarterly', 'Basic Quarterly', '162', '165', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["2% off","18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-quarterly', '100001', '0', 'seed-product-membership', 'standard-quarterly', 'Standard Quarterly', 'Standard Quarterly', '465', '474', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["2% off","52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-quarterly', '100001', '0', 'seed-product-membership', 'premium-quarterly', 'Premium Quarterly', 'Premium Quarterly', '1273', '1299', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["2% off","143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-quarterly', '100001', '0', 'seed-product-membership', 'super-quarterly', 'Super Quarterly', 'Super Quarterly', '3184', '3249', 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["2% off","357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-basic-single', '100001', '0', 'seed-product-membership', 'basic-single', 'Basic Single', 'Basic Single', '55', NULL, 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-single', '100001', '0', 'seed-product-membership', 'standard-single', 'Standard Single', 'Standard Single', '158', NULL, 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-single', '100001', '0', 'seed-product-membership', 'premium-single', 'Premium Single', 'Premium Single', '433', NULL, 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-single', '100001', '0', 'seed-product-membership', 'super-single', 'Super Single', 'Super Single', '1083', NULL, 'CNY', 'membership_activation', 'untracked', 'active', '{"tags":["357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  title = EXCLUDED.title,
  price_amount = EXCLUDED.price_amount,
  original_price_amount = EXCLUDED.original_price_amount,
  currency_code = EXCLUDED.currency_code,
  spec_json = EXCLUDED.spec_json,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;

-- Packages: 12 sellable packages (English defaults, locale files override)
INSERT INTO membership_package (
  id, tenant_id, organization_id, external_id, package_no, package_group_id, plan_id, plan_version_id, sku_id, name, description, price_amount, original_price_amount, currency_code, point_amount, duration_days, recurrence_cycle, sort_weight, recommended, status, created_at, updated_at
) VALUES
  ('package-basic-annual', '100001', '0', 101, 'basic-annual', 'package-group-annual', 'plan-basic', 'plan-basic-v1', 'sku-basic-annual', 'Basic - Annual', 'Annual subscription, auto-renew on expiry', '640', '660', 'CNY', 6400, 365, 'year', 1, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-standard-annual', '100001', '0', 102, 'standard-annual', 'package-group-annual', 'plan-standard', 'plan-standard-v1', 'sku-standard-annual', 'Standard - Annual', 'Annual subscription, auto-renew on expiry', '1839', '1896', 'CNY', 18390, 365, 'year', 2, 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-premium-annual', '100001', '0', 103, 'premium-annual', 'package-group-annual', 'plan-premium', 'plan-premium-v1', 'sku-premium-annual', 'Premium - Annual', 'Annual subscription, auto-renew on expiry', '5040', '5196', 'CNY', 50400, 365, 'year', 3, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-super-annual', '100001', '0', 104, 'super-annual', 'package-group-annual', 'plan-super', 'plan-super-v1', 'sku-super-annual', 'Super - Annual', 'Annual subscription, auto-renew on expiry', '12606', '12996', 'CNY', 126060, 365, 'year', 4, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-basic-monthly', '100001', '0', 201, 'basic-monthly', 'package-group-monthly', 'plan-basic', 'plan-basic-v1', 'sku-basic-monthly', 'Basic - Monthly', 'Monthly subscription, auto-renew on expiry', '54', '55', 'CNY', 540, 30, 'month', 1, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-standard-monthly', '100001', '0', 202, 'standard-monthly', 'package-group-monthly', 'plan-standard', 'plan-standard-v1', 'sku-standard-monthly', 'Standard - Monthly', 'Monthly subscription, auto-renew on expiry', '156', '158', 'CNY', 1560, 30, 'month', 2, 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-premium-monthly', '100001', '0', 203, 'premium-monthly', 'package-group-monthly', 'plan-premium', 'plan-premium-v1', 'sku-premium-monthly', 'Premium - Monthly', 'Monthly subscription, auto-renew on expiry', '429', '433', 'CNY', 4290, 30, 'month', 3, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-super-monthly', '100001', '0', 204, 'super-monthly', 'package-group-monthly', 'plan-super', 'plan-super-v1', 'sku-super-monthly', 'Super - Monthly', 'Monthly subscription, auto-renew on expiry', '1072', '1083', 'CNY', 10720, 30, 'month', 4, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-basic-quarterly', '100001', '0', 301, 'basic-quarterly', 'package-group-quarterly', 'plan-basic', 'plan-basic-v1', 'sku-basic-quarterly', 'Basic - Quarterly', 'Quarterly subscription, auto-renew on expiry', '162', '165', 'CNY', 1620, 90, 'quarter', 1, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-standard-quarterly', '100001', '0', 302, 'standard-quarterly', 'package-group-quarterly', 'plan-standard', 'plan-standard-v1', 'sku-standard-quarterly', 'Standard - Quarterly', 'Quarterly subscription, auto-renew on expiry', '465', '474', 'CNY', 4650, 90, 'quarter', 2, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-premium-quarterly', '100001', '0', 303, 'premium-quarterly', 'package-group-quarterly', 'plan-premium', 'plan-premium-v1', 'sku-premium-quarterly', 'Premium - Quarterly', 'Quarterly subscription, auto-renew on expiry', '1273', '1299', 'CNY', 12730, 90, 'quarter', 3, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-super-quarterly', '100001', '0', 304, 'super-quarterly', 'package-group-quarterly', 'plan-super', 'plan-super-v1', 'sku-super-quarterly', 'Super - Quarterly', 'Quarterly subscription, auto-renew on expiry', '3184', '3249', 'CNY', 31840, 90, 'quarter', 4, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-basic-single', '100001', '0', 401, 'basic-single', 'package-group-single', 'plan-basic', 'plan-basic-v1', 'sku-basic-single', 'Basic - Single Month', 'One-time purchase, no auto-renew', '55', NULL, 'CNY', 550, 30, 'once', 1, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-standard-single', '100001', '0', 402, 'standard-single', 'package-group-single', 'plan-standard', 'plan-standard-v1', 'sku-standard-single', 'Standard - Single Month', 'One-time purchase, no auto-renew', '158', NULL, 'CNY', 1580, 30, 'once', 2, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-premium-single', '100001', '0', 403, 'premium-single', 'package-group-single', 'plan-premium', 'plan-premium-v1', 'sku-premium-single', 'Premium - Single Month', 'One-time purchase, no auto-renew', '433', NULL, 'CNY', 4330, 30, 'once', 3, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('package-super-single', '100001', '0', 404, 'super-single', 'package-group-single', 'plan-super', 'plan-super-v1', 'sku-super-single', 'Super - Single Month', 'One-time purchase, no auto-renew', '1083', NULL, 'CNY', 10830, 30, 'once', 4, 0, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  description = EXCLUDED.description,
  price_amount = EXCLUDED.price_amount,
  original_price_amount = EXCLUDED.original_price_amount,
  currency_code = EXCLUDED.currency_code,
  point_amount = EXCLUDED.point_amount,
  duration_days = EXCLUDED.duration_days,
  recurrence_cycle = EXCLUDED.recurrence_cycle,
  sort_weight = EXCLUDED.sort_weight,
  recommended = EXCLUDED.recommended,
  status = EXCLUDED.status,
  updated_at = EXCLUDED.updated_at;
