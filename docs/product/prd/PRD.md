# SDKWork 会员管理产品需求文档 (PRD)

Status: active
Owner: SDKWork maintainers
Application: membership
Updated: 2026-07-06
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. 产品定位

SDKWork Membership 是 SDKWork 商业能力体系中的 **会员管理能力 (T1 commerce capability)**，为平台提供会员等级、权益、积分、订阅、签到等全生命周期管理。它以 PaaS 层能力形式嵌入 SDKWork 平台，向宿主应用（如 sdkwork-mall、sdkwork-appbase 等 commerce 组合应用）输出标准化的会员域 API、SDK 与可嵌入的 PC 界面。

本仓库是自包含的 T1 商业能力构建块，包含域逻辑、SQLx 持久化、HTTP 路由适配器、API 服务进程、IAM 中间件与可嵌入的 PC React 应用，遵循 `SdkWorkApiResponse` 信封与 `SdkWorkPageData` 分页契约。

## 2. 目标用户

- **平台运营者 (Operators)**：通过 backend-api 配置会员等级体系、套餐定价、权益规则与签到奖励，按租户隔离管理会员商品目录。
- **终端用户 (Buyers)**：通过 PC 会员页面对比等级、完成订阅购买、领取签到奖励、查看权益使用情况与积分余额。
- **平台集成者 (Integrators)**：通过 composed SDK（`@sdkwork/membership-app-sdk`）与 PC 包（`@sdkwork/membership-pc-shell`）将会员能力嵌入 mall/commerce 宿主应用。

## 3. 核心场景

1. **等级体系配置**：运营者在后台定义会员等级（Free/Basic/Standard/Premium）、排序与成长值门槛，发布等级版本。
2. **套餐购买**：终端用户在 PC 页面对比套餐（月/季/年），选择套餐后进入结算，通过 pending-payment 挂接支付能力。
3. **续费与升级**：已会员用户发起续费或升级到更高等级，系统校验当前等级与剩余天数，计算差价。
4. **权益领取**：会员激活后自动获得权益额度（加速、优先队列、专属模型等），用户在权益面板查看可用额度。
5. **积分签到**：终端用户每日签到领取积分奖励，连续签到触发阶梯奖励，月历展示签到记录。
6. **权益使用跟踪**：用户使用权益时（如调用 AI 模型加速），系统扣减额度并记录使用明细，按周期重置。

## 4. 功能模块

### 4.1 会员等级 (plan)
定义会员等级体系，包含等级编码、名称、排序 (rank)、描述与状态。每个等级对应一个 published 版本 (plan_version)，版本承载权益绑定关系。等级按 rank 升序排列，rank=0 为 Free 免费层。

### 4.2 权益 (benefit)
权益定义 (benefit_definition) 描述可授予会员的能力，包含权益编码、名称、类型 (points/queue/feature/service)、计量方式与描述。权益通过 plan_benefit 关联到等级版本，声明授予数量 (grant_quantity) 与使用策略 (usage_policy)。权益使用通过 privilege_usage 表跟踪周期内用量。

### 4.3 套餐 (package)
订阅套餐是可购买的最小单元，关联到等级版本与 SKU。包含价格、原价、币种、时长 (duration_days)、赠送积分 (point_amount)、是否推荐、标签与排序。套餐通过结算流程激活会员并发放权益。

### 4.4 套餐组 (packageGroup)
按计费周期（月/季/年/单次）对套餐分组，前端通过套餐组切换器在不同周期间切换。套餐组声明 billing_cycle、duration_days、展示渠道 (display_channel) 与排序权重。PC 订阅目录页 `/subscription/catalog` 经 `subscription-catalog-service` 消费 `memberships.packageGroups.list`；套餐展示标签来自 `commerce_product_sku.spec_json.tags`。

### 4.5 购买/续费/升级 (purchase)

**Commerce 能力边界（标准依赖方向）：**

| 能力 | 职责 | 依赖方向 |
| --- | --- | --- |
| **Payment** (`sdkwork-payment`) | 支付执行：intent、attempt、渠道、退款；经 port 持久化 webhook 事件 | 基础模块，**不得**依赖 order / membership |
| **Order** (`sdkwork-order`) | 统一订单中心：**创建 `commerce_order`**、收银、`orders.pay`、**PSP webhook 接入**、支付结算；结算后调用 membership 履约 port | order → payment；order → membership（履约 port，对标 order → account） |
| **Membership**（本仓库） | 会员域：套餐目录、订阅/权益/积分；**预留 `pending_activation` 订阅 + 支付成功后履约激活** | 客户端消费 `@sdkwork/order-app-sdk` / `@sdkwork/payment-app-sdk`；**不得**在生产路径写入 `commerce_order` |

规范约束：`specs/commerce-order-membership-boundary.spec.json`、`specs/COMMERCE_ORDER_BOUNDARY_SPEC.md`。

PSP 生产 notify URL 配置在 **订单网关**：

`POST /app/v3/api/orders/payments/webhooks/{providerCode}`

（legacy `POST /app/v3/api/payments/webhooks/*` 返回 410 Gone。权威说明见 `../sdkwork-order/docs/architecture/commerce/COMMERCE_CHECKOUT_ARCHITECTURE.md`。）

**标准购买写路径（生产 / composed）：**

1. **Order** 创建 `subject=membership` 的 `commerce_order`（checkout 或 membership-subject order create，与 `recharges.orders.create` 同模式）。
2. **Membership** 创建/关联 `pending_activation` 订阅（按 `order_id` 幂等）。
3. 客户端调用 **`orders.pay`**，打开 `paymentParams.cashierUrl`（`@sdkwork/payment-app-sdk`）完成收银。
4. PSP 回调 **订单域** webhook；订单经 payment port 验签、结算。
5. **Order 结算 saga** 调用 **membership 履约 port**，将订阅从 `pending_activation` 激活为 `active` 并发放权益。

动作语义：

- **purchase（新购）**：创建新的 `membership_subscription`（`pending_activation`）、首个 `membership_period` 与权益发放记录（激活前为 pending）。
- **renew（续费）**：复用现有订阅 ID，延长 `membership_period`。
- **upgrade（升级）**：切换更高 `plan_id`/`plan_version_id`，写入新周期并发放升级后权益。

支付确认前订阅状态均为 `pending_activation`；**订单支付结算完成后**由 membership 履约激活为 `active`。幂等键作用域为 `tenant_id + organization_id + owner_user_id + idempotency_key`。响应 `orderId` 为订单 UUID，`requestNo` 为可读订单号。

### 4.6 积分 (points)
积分账户记录用户当前余额与历史流水。积分来源包括签到奖励、套餐赠送与运营发放；消耗场景包括权益兑换与抵扣。积分变更通过 entitlement_ledger_entry 留存审计流水。

### 4.7 签到 (dailyReward)
每日签到 (commerce_membership_daily_reward) 领取积分奖励，按连续签到天数触发阶梯奖励（如 7 天双倍）。签到记录按用户-日期唯一约束防止重复领取，月历视图展示当月签到与补签机会。

### 4.8 权益使用 (privilegeUsage)
权益用量以 `entitlement_account.total_used` 为权威读模型（加速扣减实时反映）；`commerce_membership_privilege_usage` 作为周期聚合补充表。`GET /privileges/usage` 合并权益计划额度与账户已用量。达到额度上限时拒绝新请求（如 `POST /privileges/speed_ups` 返回冲突）。用量按订阅周期重置（支付激活后生效）。

## 5. 对标分析

| 对标产品 | 对标维度 | SDKWork Membership 差异 |
| --- | --- | --- |
| Stripe Membership / Billing | 订阅管理、计费周期、续费失败重试 | SDKWork Membership 是 PaaS 层域能力，不持有支付通道与 PSP webhook；支付执行经 `sdkwork-payment`，回调与结算经 `sdkwork-order`；本能力专注会员履约与权益发放。 |
| Patreon (创作者分层) | 等级分层、权益绑定、会员专享内容 | SDKWork Membership 面向平台级会员体系（AI 加速、模型队列等），权益以额度/标志位计量，不绑定内容分发；支持多租户隔离。 |
| 阿里 88VIP (生态联动) | 跨能力权益联动、成长值、生态折扣 | SDKWork Membership 通过 SDK 组合（sdkwork-promotion 优惠券、sdkwork-payment 支付）实现联动，但不绑定具体生态场景；成长值 (growth_value) 驱动等级跃迁。 |

核心差异：SDKWork Membership 是 **PaaS 层能力**，以标准 API + 可嵌入 PC 包形式输出，宿主应用按需组合；不替代支付、促销、内容分发等垂直能力。

## 6. 商业化 KPI

- **会员转化率**：注册用户中付费会员占比，Phase 1 目标 ≥ 5%。
- **续费率**：到期会员中续费/升级占比，Phase 2 目标 ≥ 60%。
- **ARPU**：付费会员平均客单价，按月度统计。
- **DAU/MAU**：会员页面日活/月活，反映会员权益使用粘性。
- **签到参与率**：会员中日签到占比，衡量积分体系活跃度。

## 7. 合规要求

- **数据保留**：会员订阅、积分流水、权益使用记录按租户策略保留，默认 24 个月；删除请求按 PRIVACY_SPEC.md 执行软删除 (deleted_at)。
- **隐私保护**：用户标识 (user_id) 与租户 (tenant_id) 隔离；不向客户端泄露其他租户数据；subject 范围投影遵循 SUBJECT_ID_SPEC.md。
- **自动续费合规**：续费需用户显式授权，取消入口可在会员页面一键触达；续费前推送提醒。
- **税务与发票**：套餐价格含税标注，发票信息由 commerce 订单域承载，本仓库不持有开票逻辑。

## 8. Phase 规划

### Phase 1（基础会员管理 — 已完成）
- 会员等级、权益、套餐、套餐组的 CRUD 与后台管理。
- PC 会员页面（等级展示、套餐对比、结算挂接）通过 `@sdkwork/membership-app-sdk` Service 层对接 app-api。
- 订阅目录页（`SubscriptionCatalogPage`）经 `subscription-catalog-service` 读取 `packageGroups`/`plans`/`benefits` 并委托 `subscription-service` 完成 purchase/renew/upgrade。
- 种子数据初始化 4 个计费分组（年/月/季/单次）× 3 档可售套餐（external_id 101–403）。
- 积分余额查询、签到基础流程。
- 标准化 API 信封、分页、租户隔离。

### Phase 2（订单主导下单 + 结算履约 + 运营后台 — 进行中）
- **Order 主导** `subject=membership` 订单创建（`memberships.orders.create`）；membership 仅预留订阅与履约，不写 `commerce_order`。
- Order 结算 saga 注入 **membership 履约 port**（order → membership，对标 `points_recharge` → account）。
- PC 结算链路：`@sdkwork/order-app-sdk` 创建订单 → `@sdkwork/membership-app-sdk` 预留 → `orders.pay` 收银。
- 运营后台（backend-api）全量管理与数据看板。
- 续费失败重试、自动续费授权管理。
- 权益使用跟踪与额度扣减闭环。

### Phase 3（营销 + 分析）
- 优惠券组合（sdkwork-promotion）、限时折扣、邀请奖励。
- 会员行为分析（转化漏斗、留存曲线、权益热度）。
- 跨能力权益联动（如会员专享模型、生态折扣）。
- A/B 实验框架接入。

## 9. 关联契约

- 组件契约：`specs/component.spec.json`
- 机器契约：`apis/`、`sdks/`、本地 `specs/`
- 文档正典：`docs/product/prd/PRD.md`、`docs/architecture/tech/TECH_ARCHITECTURE.md`

## 10. 待决问题

- 补充 purchase/renew/upgrade 与 order 结算联动的 HTTP 集成测试（sqlite 测试库）。
- 接入 IAM PC 登录引导，替换当前 env-token 开发引导，支撑生产鉴权。
- 多租户访客目录：宿主应用需注入租户上下文，避免长期依赖种子租户 `100001` 浏览目录。
- 权益对比表扩展字段：后续可在 `benefit_definition` 增加分类 metadata，替代前端 Service 层 benefit_key 映射。
