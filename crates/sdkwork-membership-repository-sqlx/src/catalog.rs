use crate::{
    AppMembershipBenefitItem, AppMembershipDailyRewardResponse,
    AppMembershipDailyRewardStatusResponse, AppMembershipInfoResponse,
    AppMembershipPackageGroupItem, AppMembershipPackageItem, AppMembershipPlanItem,
    AppMembershipPointsBalanceResponse, AppMembershipPointsHistoryItem,
    AppMembershipPrivilegeUsageResponse, AppMembershipStatusResponse,
};

pub fn builtin_info() -> AppMembershipInfoResponse {
    AppMembershipInfoResponse {
        plan_rank: 0,
        plan_name: "Free".to_string(),
        membership_status: "inactive".to_string(),
        started_at: None,
        expires_at: None,
        remaining_days: None,
        total_days: None,
        total_spent: Some("0".to_string()),
        points: Some(50),
        growth_value: Some(0),
        upgrade_growth_value: Some(725),
        benefits: builtin_benefits(None),
    }
}

pub fn builtin_status() -> AppMembershipStatusResponse {
    AppMembershipStatusResponse {
        active: false,
        plan_rank: 0,
        expires_at: None,
        point_balance: Some(50),
    }
}

pub fn builtin_plans() -> Vec<AppMembershipPlanItem> {
    vec![
        AppMembershipPlanItem {
            id: 0,
            name: "Free".to_string(),
            rank: 0,
            required_points: Some(0),
            description: Some("体验基础功能，每日赠送积分".to_string()),
            icon: None,
            badge: Some("免费".to_string()),
        },
        AppMembershipPlanItem {
            id: 1,
            name: "基础会员".to_string(),
            rank: 1,
            required_points: Some(725),
            description: Some("适合个人创作者，解锁基础AI能力".to_string()),
            icon: None,
            badge: Some("热门".to_string()),
        },
        AppMembershipPlanItem {
            id: 2,
            name: "标准会员".to_string(),
            rank: 2,
            required_points: Some(2210),
            description: Some("适合专业创作者，解锁全部AI模型".to_string()),
            icon: None,
            badge: Some("推荐".to_string()),
        },
        AppMembershipPlanItem {
            id: 3,
            name: "高级会员".to_string(),
            rank: 3,
            required_points: Some(6160),
            description: Some("适合团队和重度用户，享受专属通道".to_string()),
            icon: None,
            badge: Some("旗舰".to_string()),
        },
    ]
}

pub fn builtin_benefits(_plan_id: Option<i64>) -> Vec<AppMembershipBenefitItem> {
    vec![
        AppMembershipBenefitItem {
            id: 1,
            name: "每日赠送积分".to_string(),
            benefit_key: Some("daily_points".to_string()),
            r#type: Some("points".to_string()),
            description: Some("每日登录可领取积分".to_string()),
            icon: None,
            claimed: false,
            usage_limit: Some(50),
            used_count: Some(0),
        },
        AppMembershipBenefitItem {
            id: 2,
            name: "标准生成通道".to_string(),
            benefit_key: Some("standard_queue".to_string()),
            r#type: Some("queue".to_string()),
            description: Some("标准优先级生成队列".to_string()),
            icon: None,
            claimed: true,
            usage_limit: None,
            used_count: None,
        },
        AppMembershipBenefitItem {
            id: 3,
            name: "快速生成通道".to_string(),
            benefit_key: Some("fast_queue".to_string()),
            r#type: Some("queue".to_string()),
            description: Some("高优先级快速生成队列".to_string()),
            icon: None,
            claimed: false,
            usage_limit: None,
            used_count: None,
        },
        AppMembershipBenefitItem {
            id: 4,
            name: "VIP专属通道".to_string(),
            benefit_key: Some("vip_queue".to_string()),
            r#type: Some("queue".to_string()),
            description: Some("最高优先级专属队列，无需等待".to_string()),
            icon: None,
            claimed: false,
            usage_limit: None,
            used_count: None,
        },
        AppMembershipBenefitItem {
            id: 5,
            name: "无水印导出".to_string(),
            benefit_key: Some("no_watermark".to_string()),
            r#type: Some("feature".to_string()),
            description: Some("导出作品无平台水印".to_string()),
            icon: None,
            claimed: false,
            usage_limit: None,
            used_count: None,
        },
        AppMembershipBenefitItem {
            id: 6,
            name: "专属客服".to_string(),
            benefit_key: Some("vip_support".to_string()),
            r#type: Some("service".to_string()),
            description: Some("专属客服通道优先响应".to_string()),
            icon: None,
            claimed: false,
            usage_limit: None,
            used_count: None,
        },
    ]
}

pub fn builtin_package_groups() -> Vec<AppMembershipPackageGroupItem> {
    vec![
        AppMembershipPackageGroupItem {
            id: 1,
            name: "连续包年".to_string(),
            description: Some("5.8折".to_string()),
            sort_weight: 1,
            packages: vec![
                AppMembershipPackageItem {
                    id: 101,
                    name: "基础会员·年卡".to_string(),
                    description: Some("连续包年，到期自动续费".to_string()),
                    price: "393".to_string(),
                    original_price: Some("659".to_string()),
                    point_amount: 8700,
                    duration_days: 365,
                    plan_name: Some("基础会员".to_string()),
                    sort_weight: 1,
                    recommended: false,
                    tags: vec!["首年6折".to_string(), "每日24积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 102,
                    name: "标准会员·年卡".to_string(),
                    description: Some("连续包年，到期自动续费".to_string()),
                    price: "1099".to_string(),
                    original_price: Some("1899".to_string()),
                    point_amount: 26520,
                    duration_days: 365,
                    plan_name: Some("标准会员".to_string()),
                    sort_weight: 2,
                    recommended: true,
                    tags: vec!["首年5.8折".to_string(), "每日74积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 103,
                    name: "高级会员·年卡".to_string(),
                    description: Some("连续包年，到期自动续费".to_string()),
                    price: "3099".to_string(),
                    original_price: Some("5199".to_string()),
                    point_amount: 73920,
                    duration_days: 365,
                    plan_name: Some("高级会员".to_string()),
                    sort_weight: 3,
                    recommended: false,
                    tags: vec!["首年6折".to_string(), "每日205积分".to_string()],
                },
            ],
        },
        AppMembershipPackageGroupItem {
            id: 2,
            name: "连续包月".to_string(),
            description: Some("6折".to_string()),
            sort_weight: 2,
            packages: vec![
                AppMembershipPackageItem {
                    id: 201,
                    name: "基础会员·月卡".to_string(),
                    description: Some("连续包月，到期自动续费".to_string()),
                    price: "33".to_string(),
                    original_price: Some("55".to_string()),
                    point_amount: 725,
                    duration_days: 30,
                    plan_name: Some("基础会员".to_string()),
                    sort_weight: 1,
                    recommended: false,
                    tags: vec!["6折".to_string(), "每日24积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 202,
                    name: "标准会员·月卡".to_string(),
                    description: Some("连续包月，到期自动续费".to_string()),
                    price: "95".to_string(),
                    original_price: Some("158".to_string()),
                    point_amount: 2210,
                    duration_days: 30,
                    plan_name: Some("标准会员".to_string()),
                    sort_weight: 2,
                    recommended: true,
                    tags: vec!["6折".to_string(), "每日74积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 203,
                    name: "高级会员·月卡".to_string(),
                    description: Some("连续包月，到期自动续费".to_string()),
                    price: "260".to_string(),
                    original_price: Some("433".to_string()),
                    point_amount: 6160,
                    duration_days: 30,
                    plan_name: Some("高级会员".to_string()),
                    sort_weight: 3,
                    recommended: false,
                    tags: vec!["6折".to_string(), "每日205积分".to_string()],
                },
            ],
        },
        AppMembershipPackageGroupItem {
            id: 3,
            name: "连续包季".to_string(),
            description: Some("7折".to_string()),
            sort_weight: 3,
            packages: vec![
                AppMembershipPackageItem {
                    id: 301,
                    name: "基础会员·季卡".to_string(),
                    description: Some("连续包季，到期自动续费".to_string()),
                    price: "116".to_string(),
                    original_price: Some("165".to_string()),
                    point_amount: 2175,
                    duration_days: 90,
                    plan_name: Some("基础会员".to_string()),
                    sort_weight: 1,
                    recommended: false,
                    tags: vec!["7折".to_string(), "每日24积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 302,
                    name: "标准会员·季卡".to_string(),
                    description: Some("连续包季，到期自动续费".to_string()),
                    price: "332".to_string(),
                    original_price: Some("474".to_string()),
                    point_amount: 6630,
                    duration_days: 90,
                    plan_name: Some("标准会员".to_string()),
                    sort_weight: 2,
                    recommended: false,
                    tags: vec!["7折".to_string(), "每日74积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 303,
                    name: "高级会员·季卡".to_string(),
                    description: Some("连续包季，到期自动续费".to_string()),
                    price: "909".to_string(),
                    original_price: Some("1299".to_string()),
                    point_amount: 18480,
                    duration_days: 90,
                    plan_name: Some("高级会员".to_string()),
                    sort_weight: 3,
                    recommended: false,
                    tags: vec!["7折".to_string(), "每日205积分".to_string()],
                },
            ],
        },
        AppMembershipPackageGroupItem {
            id: 4,
            name: "单月购买".to_string(),
            description: None,
            sort_weight: 4,
            packages: vec![
                AppMembershipPackageItem {
                    id: 401,
                    name: "基础会员·单月".to_string(),
                    description: Some("单次购买，不自动续费".to_string()),
                    price: "55".to_string(),
                    original_price: None,
                    point_amount: 725,
                    duration_days: 30,
                    plan_name: Some("基础会员".to_string()),
                    sort_weight: 1,
                    recommended: false,
                    tags: vec!["每日24积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 402,
                    name: "标准会员·单月".to_string(),
                    description: Some("单次购买，不自动续费".to_string()),
                    price: "158".to_string(),
                    original_price: None,
                    point_amount: 2210,
                    duration_days: 30,
                    plan_name: Some("标准会员".to_string()),
                    sort_weight: 2,
                    recommended: false,
                    tags: vec!["每日74积分".to_string()],
                },
                AppMembershipPackageItem {
                    id: 403,
                    name: "高级会员·单月".to_string(),
                    description: Some("单次购买，不自动续费".to_string()),
                    price: "433".to_string(),
                    original_price: None,
                    point_amount: 6160,
                    duration_days: 30,
                    plan_name: Some("高级会员".to_string()),
                    sort_weight: 3,
                    recommended: false,
                    tags: vec!["每日205积分".to_string()],
                },
            ],
        },
    ]
}

pub fn builtin_packages(
    package_group_id: Option<i64>,
    plan_id: Option<i64>,
) -> Vec<AppMembershipPackageItem> {
    let groups = builtin_package_groups();
    let mut packages: Vec<AppMembershipPackageItem> = Vec::new();

    for group in groups {
        if let Some(gid) = package_group_id {
            if group.id != gid {
                continue;
            }
        }
        for package in group.packages {
            if let Some(pid) = plan_id {
                let plan_rank = match package.plan_name.as_deref() {
                    Some("基础会员") => 1,
                    Some("标准会员") => 2,
                    Some("高级会员") => 3,
                    _ => 0,
                };
                if plan_rank != pid {
                    continue;
                }
            }
            packages.push(package);
        }
    }
    packages
}

pub fn builtin_package(package_id: i64) -> Option<AppMembershipPackageItem> {
    builtin_packages(None, None)
        .into_iter()
        .find(|p| p.id == package_id)
}

pub fn builtin_package_group(package_group_id: i64) -> Option<AppMembershipPackageGroupItem> {
    builtin_package_groups()
        .into_iter()
        .find(|g| g.id == package_group_id)
}

pub fn builtin_points_balance() -> AppMembershipPointsBalanceResponse {
    AppMembershipPointsBalanceResponse {
        points: 50,
        available_points: 50,
        frozen_points: 0,
    }
}

pub fn builtin_points_history() -> Vec<AppMembershipPointsHistoryItem> {
    vec![AppMembershipPointsHistoryItem {
        id: "points-1".to_string(),
        change_type: "daily_reward".to_string(),
        change_amount: 50,
        before_balance: Some(0),
        after_balance: 50,
        source_type: "system".to_string(),
        remark: Some("每日登录奖励".to_string()),
        created_at: Some("2025-01-01T08:00:00Z".to_string()),
    }]
}

pub fn builtin_daily_reward_status() -> AppMembershipDailyRewardStatusResponse {
    AppMembershipDailyRewardStatusResponse {
        can_claim: true,
        claimed_today: false,
        consecutive_days: 1,
        total_days: 1,
    }
}

pub fn builtin_daily_reward_claim() -> AppMembershipDailyRewardResponse {
    AppMembershipDailyRewardResponse {
        reward_points: 50,
        claimed_at: Some("2025-01-01T08:00:00Z".to_string()),
        consecutive_days: 1,
    }
}

pub fn builtin_privilege_usage() -> AppMembershipPrivilegeUsageResponse {
    AppMembershipPrivilegeUsageResponse {
        speed_up_used: 0,
        speed_up_limit: 3,
        priority_queue_used: 0,
        priority_queue_limit: 100,
        exclusive_model_used: 0,
        exclusive_model_limit: 0,
    }
}
