-- ============================================================================
-- zh-CN locale seed: overrides display text for membership catalog tables.
--
-- This file runs AFTER common/001_catalog.sql and overwrites English default
-- display fields with Simplified Chinese text. It uses UPDATE statements keyed
-- by primary key so re-seeding is idempotent.
--
-- Pricing model: 10元 = 100算力元 (1元 = 10算力元)
-- point_amount = price × 10
-- ============================================================================

-- ---------------------------------------------------------------------------
-- membership_plan: plan display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE membership_plan SET name = '免费版', description = '体验核心功能，每日赠送算力元。', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-free';
UPDATE membership_plan SET name = '基础版', description = '面向个人创作者，解锁基础AI能力。', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic';
UPDATE membership_plan SET name = '标准版', description = '面向专业创作者，解锁全部AI模型。', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard';
UPDATE membership_plan SET name = '巅峰版', description = '面向团队和重度用户，享受专属通道。', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium';
UPDATE membership_plan SET name = '超级版', description = '顶级VIP通道、最大配额与专属特效。', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super';

-- ---------------------------------------------------------------------------
-- membership_plan_version: version display titles
-- ---------------------------------------------------------------------------
UPDATE membership_plan_version SET title = '免费版', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-free-v1';
UPDATE membership_plan_version SET title = '基础版', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1';
UPDATE membership_plan_version SET title = '标准版', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1';
UPDATE membership_plan_version SET title = '巅峰版', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1';
UPDATE membership_plan_version SET title = '超级版', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1';

-- ---------------------------------------------------------------------------
-- benefit_definition: benefit display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE benefit_definition SET name = '平台免费算力元', description = '每日登录免费算力元。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-platform-free-points';
UPDATE benefit_definition SET name = '充值购买算力元', description = '充值购买的算力元。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-purchased-points';
UPDATE benefit_definition SET name = '订阅会员算力元', description = '每月订阅赠送算力元。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-daily-points';
UPDATE benefit_definition SET name = 'Seedance 2.0 VIP模型', description = 'Seedance 2.0 VIP模型使用权限。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-seedance-vip-model';
UPDATE benefit_definition SET name = 'Seedance 1.5 Pro模型', description = 'Seedance 1.5 Pro模型算力元折扣。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-seedance-pro-model';
UPDATE benefit_definition SET name = '标准生成通道', description = '标准优先级生成通道。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-standard-queue';
UPDATE benefit_definition SET name = '快速生成通道', description = '高优先级快速生成通道。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-fast-queue';
UPDATE benefit_definition SET name = 'VIP优先通道', description = '专属VIP生成通道。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-vip-queue';
UPDATE benefit_definition SET name = '视频对口型', description = '视频对口型功能权限。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-video-lip-sync';
UPDATE benefit_definition SET name = '视频高清', description = '视频高清导出功能。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-video-hd';
UPDATE benefit_definition SET name = '视频补帧', description = '视频补帧功能。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-video-frame-interp';
UPDATE benefit_definition SET name = '图片4.0限时免费', description = '图片4.0免费分辨率。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-image-4k-free';
UPDATE benefit_definition SET name = '智能超清', description = '智能超清分辨率。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-smart-upscale';
UPDATE benefit_definition SET name = '去除品牌水印', description = '导出无水印。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-no-watermark';
UPDATE benefit_definition SET name = '生成加速', description = '加速生成速度。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-generation-acceleration';
UPDATE benefit_definition SET name = '无忧退款', description = '无忧退款及优先客服支持。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-vip-support';

-- Privilege benefits (runtime quota)
UPDATE benefit_definition SET name = '加速加速', description = '生成任务手动加速配额。', updated_at = CURRENT_TIMESTAMP WHERE id = 'seed-benefit-priority-speed-up';
UPDATE benefit_definition SET name = '优先队列', description = '活跃会员优先队列容量。', updated_at = CURRENT_TIMESTAMP WHERE id = 'benefit-definition-priority_queue';
UPDATE benefit_definition SET name = 'AI配额', description = '专属模型和AI配额。', updated_at = CURRENT_TIMESTAMP WHERE id = 'seed-benefit-ai-quota';

-- ---------------------------------------------------------------------------
-- membership_plan_benefit: grant_quantity display text (text-type cells only)
-- Numeric cells (e.g. '330', '1', '20') remain unchanged.
-- ---------------------------------------------------------------------------
-- Free plan
UPDATE membership_plan_benefit SET grant_quantity = '每日登录免费算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-free-v1-benefit-platform-points';

-- Basic plan
UPDATE membership_plan_benefit SET grant_quantity = '每日登录赠送算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '9.7折算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = '标准生成通道', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-basic-v1-benefit-standard-queue';

-- Standard plan
UPDATE membership_plan_benefit SET grant_quantity = '每日登录赠送算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '9.7折算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = '标准生成通道', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-standard-v1-benefit-standard-queue';

-- Premium plan
UPDATE membership_plan_benefit SET grant_quantity = '每日登录赠送算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '9.7折算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = '快速生成通道', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-premium-v1-benefit-fast-queue';

-- Super plan
UPDATE membership_plan_benefit SET grant_quantity = '每日登录赠送算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1-benefit-platform-points';
UPDATE membership_plan_benefit SET grant_quantity = '9.7折算力元', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1-benefit-seedance-pro';
UPDATE membership_plan_benefit SET grant_quantity = 'VIP专属通道', updated_at = CURRENT_TIMESTAMP WHERE id = 'plan-super-v1-benefit-vip-queue';

-- ---------------------------------------------------------------------------
-- membership_package_group: group display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE membership_package_group SET name = '连续包年', description = '9.7折', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-annual';
UPDATE membership_package_group SET name = '连续包月', description = '9.9折', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-monthly';
UPDATE membership_package_group SET name = '连续包季', description = '9.8折', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-quarterly';
UPDATE membership_package_group SET name = '单月购买', description = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = 'package-group-single';

-- ---------------------------------------------------------------------------
-- commerce_product_sku: SKU display names and spec_json tags
-- Tags: 10元=100算力元, daily = point_amount/30 (monthly sub rate)
-- Basic: 540/30=18, Standard: 1560/30=52, Premium: 4290/30=143, Super: 10720/30≈357
-- ---------------------------------------------------------------------------
UPDATE commerce_product_sku SET name = '基础版-连续包年', title = '基础版-连续包年', spec_json = '{"tags":["首年9.7折","18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-annual';
UPDATE commerce_product_sku SET name = '标准版-连续包年', title = '标准版-连续包年', spec_json = '{"tags":["首年9.7折","52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-annual';
UPDATE commerce_product_sku SET name = '巅峰版-连续包年', title = '巅峰版-连续包年', spec_json = '{"tags":["首年9.7折","143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-annual';
UPDATE commerce_product_sku SET name = '基础版-连续包月', title = '基础版-连续包月', spec_json = '{"tags":["9.9折","18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-monthly';
UPDATE commerce_product_sku SET name = '标准版-连续包月', title = '标准版-连续包月', spec_json = '{"tags":["9.9折","52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-monthly';
UPDATE commerce_product_sku SET name = '巅峰版-连续包月', title = '巅峰版-连续包月', spec_json = '{"tags":["9.9折","143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-monthly';
UPDATE commerce_product_sku SET name = '基础版-连续包季', title = '基础版-连续包季', spec_json = '{"tags":["9.8折","18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-quarterly';
UPDATE commerce_product_sku SET name = '标准版-连续包季', title = '标准版-连续包季', spec_json = '{"tags":["9.8折","52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-quarterly';
UPDATE commerce_product_sku SET name = '巅峰版-连续包季', title = '巅峰版-连续包季', spec_json = '{"tags":["9.8折","143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-quarterly';
UPDATE commerce_product_sku SET name = '基础版-单月购买', title = '基础版-单月购买', spec_json = '{"tags":["18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-single';
UPDATE commerce_product_sku SET name = '标准版-单月购买', title = '标准版-单月购买', spec_json = '{"tags":["52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-single';
UPDATE commerce_product_sku SET name = '巅峰版-单月购买', title = '巅峰版-单月购买', spec_json = '{"tags":["143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-single';
UPDATE commerce_product_sku SET name = '超级版-连续包年', title = '超级版-连续包年', spec_json = '{"tags":["首年9.7折","357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-annual';
UPDATE commerce_product_sku SET name = '超级版-连续包月', title = '超级版-连续包月', spec_json = '{"tags":["9.9折","357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-monthly';
UPDATE commerce_product_sku SET name = '超级版-连续包季', title = '超级版-连续包季', spec_json = '{"tags":["9.8折","357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-quarterly';
UPDATE commerce_product_sku SET name = '超级版-单月购买', title = '超级版-单月购买', spec_json = '{"tags":["357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-single';

-- ---------------------------------------------------------------------------
-- membership_package: package display names and descriptions
-- ---------------------------------------------------------------------------
UPDATE membership_package SET name = '基础版 - 连续包年', description = '连续包年订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-annual';
UPDATE membership_package SET name = '标准版 - 连续包年', description = '连续包年订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-annual';
UPDATE membership_package SET name = '巅峰版 - 连续包年', description = '连续包年订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-annual';
UPDATE membership_package SET name = '基础版 - 连续包月', description = '连续包月订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-monthly';
UPDATE membership_package SET name = '标准版 - 连续包月', description = '连续包月订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-monthly';
UPDATE membership_package SET name = '巅峰版 - 连续包月', description = '连续包月订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-monthly';
UPDATE membership_package SET name = '基础版 - 连续包季', description = '连续包季订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-quarterly';
UPDATE membership_package SET name = '标准版 - 连续包季', description = '连续包季订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-quarterly';
UPDATE membership_package SET name = '巅峰版 - 连续包季', description = '连续包季订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-quarterly';
UPDATE membership_package SET name = '基础版 - 单月购买', description = '一次性购买，不自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-basic-single';
UPDATE membership_package SET name = '标准版 - 单月购买', description = '一次性购买，不自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-standard-single';
UPDATE membership_package SET name = '巅峰版 - 单月购买', description = '一次性购买，不自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-premium-single';
UPDATE membership_package SET name = '超级版 - 连续包年', description = '连续包年订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-annual';
UPDATE membership_package SET name = '超级版 - 连续包月', description = '连续包月订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-monthly';
UPDATE membership_package SET name = '超级版 - 连续包季', description = '连续包季订阅，到期自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-quarterly';
UPDATE membership_package SET name = '超级版 - 单月购买', description = '一次性购买，不自动续费', updated_at = CURRENT_TIMESTAMP WHERE id = 'package-super-single';
