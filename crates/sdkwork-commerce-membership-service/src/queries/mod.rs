use sdkwork_commerce_contract_service::CommerceServiceError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentMembershipQuery {
    pub owner_user_id: String,
    pub tenant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPlanListQuery {
    pub tenant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntitlementListQuery {
    pub membership_id: Option<String>,
    pub owner_user_id: String,
    pub tenant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPackageGroupListQuery {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub billing_cycle: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipPackageListQuery {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub package_group_id: Option<String>,
    pub plan_id: Option<String>,
}

impl MembershipPackageGroupListQuery {
    pub fn new(
        tenant_id: &str,
        organization_id: Option<&str>,
        billing_cycle: Option<&str>,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", tenant_id)?;

        Ok(Self {
            tenant_id: tenant_id.to_string(),
            organization_id: normalize_optional_text(organization_id),
            billing_cycle: normalize_optional_text(billing_cycle),
        })
    }
}

impl MembershipPackageListQuery {
    pub fn new(
        tenant_id: &str,
        organization_id: Option<&str>,
        package_group_id: Option<&str>,
        plan_id: Option<&str>,
    ) -> Result<Self, CommerceServiceError> {
        crate::validation::require_non_empty("tenant_id", tenant_id)?;

        Ok(Self {
            tenant_id: tenant_id.to_string(),
            organization_id: normalize_optional_text(organization_id),
            package_group_id: normalize_optional_text(package_group_id),
            plan_id: normalize_optional_text(plan_id),
        })
    }
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}
