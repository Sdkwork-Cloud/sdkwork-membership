#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivateMembershipCommand {
    pub idempotency_key: String,
    pub plan_id: String,
    pub order_id: String,
    pub owner_user_id: String,
    pub tenant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrantEntitlementCommand {
    pub entitlement_code: String,
    pub membership_id: String,
    pub quantity: u64,
    pub tenant_id: String,
}
