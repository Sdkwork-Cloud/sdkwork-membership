use sdkwork_contract_service::{
    CommerceMoney, CommerceServiceError, CommerceSurfaceProfile,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MembershipBillingCycle {
    Day,
    Week,
    Month,
    Year,
}

impl MembershipBillingCycle {
    pub const fn as_storage_str(&self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPlanDraft {
    pub plan_id: String,
    pub name: String,
    pub tenant_id: String,
    pub validity_days: u32,
    pub visible_surfaces: Vec<CommerceSurfaceProfile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPackageGroupDraft {
    pub tenant_id: String,
    pub organization_id: String,
    pub external_id: i64,
    pub package_group_no: String,
    pub name: String,
    pub description: Option<String>,
    pub billing_cycle: MembershipBillingCycle,
    pub duration_days: u32,
    pub sort_weight: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPackageGroupDraftInput {
    pub tenant_id: String,
    pub organization_id: String,
    pub external_id: i64,
    pub package_group_no: String,
    pub name: String,
    pub description: Option<String>,
    pub billing_cycle: MembershipBillingCycle,
    pub duration_days: u32,
    pub sort_weight: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPackageDraft {
    pub tenant_id: String,
    pub organization_id: String,
    pub external_id: i64,
    pub package_no: String,
    pub package_group_id: String,
    pub plan_id: String,
    pub sku_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub price_amount: String,
    pub original_price_amount: Option<String>,
    pub currency_code: String,
    pub point_amount: i64,
    pub duration_days: u32,
    pub sort_weight: i64,
    pub recommended: bool,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MembershipStatus {
    PendingActivation,
    Active,
    GracePeriod,
    Expired,
    Cancelled,
}

impl MembershipStatus {
    pub const fn as_storage_str(&self) -> &'static str {
        match self {
            Self::PendingActivation => "pending_activation",
            Self::Active => "active",
            Self::GracePeriod => "grace_period",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipTransition {
    from: MembershipStatus,
    to: MembershipStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipActivationDraft {
    pub idempotency_key: String,
    pub plan_id: String,
    pub order_id: String,
    pub owner_user_id: String,
    pub payment_id: String,
    pub tenant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntitlementGrantDraft {
    pub entitlement_code: String,
    pub membership_id: String,
    pub quantity: u64,
    pub tenant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPurchaseDraft {
    pub tenant_id: String,
    pub organization_id: String,
    pub owner_user_id: String,
    pub package_id: i64,
    pub payment_method: Option<String>,
    pub coupon_id: Option<String>,
}

impl MembershipPlanDraft {
    pub fn new(
        tenant_id: &str,
        plan_id: &str,
        name: &str,
        validity_days: u32,
        visible_surfaces: Vec<CommerceSurfaceProfile>,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", tenant_id)?;
        crate::validation::require_non_empty("plan_id", plan_id)?;
        crate::validation::require_non_empty("name", name)?;
        if validity_days == 0 {
            return Err(CommerceServiceError::validation(
                "validity_days must be greater than zero",
            ));
        }
        if visible_surfaces.is_empty() {
            return Err(CommerceServiceError::validation(
                "visible_surfaces requires at least one surface",
            ));
        }

        Ok(Self {
            plan_id: plan_id.to_string(),
            name: name.to_string(),
            tenant_id: tenant_id.to_string(),
            validity_days,
            visible_surfaces,
        })
    }
}

impl MembershipPackageGroupDraft {
    pub fn from_input(
        input: MembershipPackageGroupDraftInput,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", &input.tenant_id)?;
        crate::validation::require_non_empty("organization_id", &input.organization_id)?;
        crate::validation::require_non_empty("package_group_no", &input.package_group_no)?;
        crate::validation::require_non_empty("name", &input.name)?;
        if input.external_id <= 0 {
            return Err(CommerceServiceError::validation(
                "external_id must be greater than zero",
            ));
        }
        if input.duration_days == 0 {
            return Err(CommerceServiceError::validation(
                "duration_days must be greater than zero",
            ));
        }

        Ok(Self {
            tenant_id: input.tenant_id,
            organization_id: input.organization_id,
            external_id: input.external_id,
            package_group_no: input.package_group_no,
            name: input.name,
        description: crate::validation::normalize_optional_text(input.description.as_deref()),
        billing_cycle: input.billing_cycle,
            duration_days: input.duration_days,
            sort_weight: input.sort_weight,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        external_id: i64,
        package_group_no: &str,
        name: &str,
        description: Option<&str>,
        billing_cycle: MembershipBillingCycle,
        duration_days: u32,
        sort_weight: i64,
    ) -> Result<Self, CommerceServiceError> {
        Self::from_input(MembershipPackageGroupDraftInput {
            tenant_id: tenant_id.to_string(),
            organization_id: organization_id.to_string(),
            external_id,
            package_group_no: package_group_no.to_string(),
            name: name.to_string(),
            description: crate::validation::normalize_optional_text(description),
            billing_cycle,
            duration_days,
            sort_weight,
        })
    }
}

impl MembershipPackageDraft {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        external_id: i64,
        package_no: &str,
        package_group_id: &str,
        plan_id: &str,
        sku_id: Option<&str>,
        name: &str,
        description: Option<&str>,
        price_amount: &str,
        original_price_amount: Option<&str>,
        currency_code: &str,
        point_amount: i64,
        duration_days: u32,
        sort_weight: i64,
        recommended: bool,
        tags: Vec<String>,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", tenant_id)?;
        crate::validation::require_non_empty("organization_id", organization_id)?;
        crate::validation::require_non_empty("package_no", package_no)?;
        crate::validation::require_non_empty("package_group_id", package_group_id)?;
        crate::validation::require_non_empty("plan_id", plan_id)?;
        crate::validation::require_non_empty("name", name)?;
        crate::validation::require_non_empty("currency_code", currency_code)?;
        if external_id <= 0 {
            return Err(CommerceServiceError::validation(
                "external_id must be greater than zero",
            ));
        }
        if point_amount < 0 {
            return Err(CommerceServiceError::validation(
                "point_amount must be greater than or equal to zero",
            ));
        }
        if duration_days == 0 {
            return Err(CommerceServiceError::validation(
                "duration_days must be greater than zero",
            ));
        }
        validate_money_amount("price_amount", price_amount)?;
        if let Some(original_price_amount) = original_price_amount {
            validate_money_amount("original_price_amount", original_price_amount)?;
        }

        Ok(Self {
            tenant_id: tenant_id.to_string(),
            organization_id: organization_id.to_string(),
            external_id,
            package_no: package_no.to_string(),
            package_group_id: package_group_id.to_string(),
            plan_id: plan_id.to_string(),
            sku_id: crate::validation::normalize_optional_text(sku_id),
            name: name.to_string(),
            description: crate::validation::normalize_optional_text(description),
            price_amount: price_amount.to_string(),
            original_price_amount: crate::validation::normalize_optional_text(original_price_amount),
            currency_code: currency_code.to_string(),
            point_amount,
            duration_days,
            sort_weight,
            recommended,
            tags: tags
                .into_iter()
                .map(|tag| tag.trim().to_string())
                .filter(|tag| !tag.is_empty())
                .collect(),
        })
    }
}

impl MembershipTransition {
    pub fn new(from: MembershipStatus, to: MembershipStatus) -> Self {
        Self { from, to }
    }

    pub fn validate(&self) -> Result<(), CommerceServiceError> {
        match (&self.from, &self.to) {
            (MembershipStatus::PendingActivation, MembershipStatus::Active)
            | (MembershipStatus::Active, MembershipStatus::GracePeriod)
            | (MembershipStatus::GracePeriod, MembershipStatus::Active)
            | (MembershipStatus::Active, MembershipStatus::Expired)
            | (MembershipStatus::GracePeriod, MembershipStatus::Expired)
            | (MembershipStatus::PendingActivation, MembershipStatus::Cancelled)
            | (MembershipStatus::Active, MembershipStatus::Cancelled) => Ok(()),
            _ => Err(CommerceServiceError::invalid_state(
                "invalid membership status transition",
            )),
        }
    }
}

impl MembershipActivationDraft {
    pub fn new(
        tenant_id: &str,
        order_id: &str,
        payment_id: &str,
        owner_user_id: &str,
        plan_id: &str,
        idempotency_key: &str,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", tenant_id)?;
        crate::validation::require_non_empty("order_id", order_id)?;
        crate::validation::require_non_empty("payment_id", payment_id)?;
        crate::validation::require_non_empty("owner_user_id", owner_user_id)?;
        crate::validation::require_non_empty("plan_id", plan_id)?;
        crate::validation::require_non_empty("idempotency_key", idempotency_key)?;

        Ok(Self {
            idempotency_key: idempotency_key.to_string(),
            plan_id: plan_id.to_string(),
            order_id: order_id.to_string(),
            owner_user_id: owner_user_id.to_string(),
            payment_id: payment_id.to_string(),
            tenant_id: tenant_id.to_string(),
        })
    }
}

impl MembershipPurchaseDraft {
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        owner_user_id: &str,
        package_id: i64,
        payment_method: Option<&str>,
        coupon_id: Option<&str>,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", tenant_id)?;
        crate::validation::require_non_empty("organization_id", organization_id)?;
        crate::validation::require_non_empty("owner_user_id", owner_user_id)?;
        if package_id <= 0 {
            return Err(CommerceServiceError::validation(
                "package_id must be greater than zero",
            ));
        }

        Ok(Self {
            tenant_id: tenant_id.to_string(),
            organization_id: organization_id.to_string(),
            owner_user_id: owner_user_id.to_string(),
            package_id,
            payment_method: normalize_optional_payment_method(payment_method)?,
            coupon_id: crate::validation::normalize_optional_text(coupon_id),
        })
    }
}

fn validate_money_amount(field_name: &str, value: &str) -> Result<(), CommerceServiceError> {
    CommerceMoney::new(value)
        .map(|_| ())
        .map_err(|message| CommerceServiceError::validation(format!("{field_name}: {message}")))
}

fn normalize_optional_payment_method(
    value: Option<&str>,
) -> Result<Option<String>, CommerceServiceError> {
    let Some(method) = crate::validation::normalize_optional_text(value) else {
        return Ok(None);
    };
    let method = method.to_ascii_lowercase();
    match method.as_str() {
        "wechat_pay" | "alipay" | "paypal" | "card" | "apple_pay" | "google_pay"
        | "wallet_balance" => Ok(Some(method)),
        _ => Err(CommerceServiceError::validation(
            "payment_method must be a canonical payment method key",
        )),
    }
}

impl EntitlementGrantDraft {
    pub fn new(
        tenant_id: &str,
        membership_id: &str,
        entitlement_code: &str,
        quantity: u64,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", tenant_id)?;
        crate::validation::require_non_empty("membership_id", membership_id)?;
        crate::validation::require_non_empty("entitlement_code", entitlement_code)?;
        if quantity == 0 {
            return Err(CommerceServiceError::validation(
                "quantity must be greater than zero",
            ));
        }

        Ok(Self {
            entitlement_code: entitlement_code.to_string(),
            membership_id: membership_id.to_string(),
            quantity,
            tenant_id: tenant_id.to_string(),
        })
    }
}
