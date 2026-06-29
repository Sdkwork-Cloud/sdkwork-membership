use crate::{EntitlementGrantDraft, MembershipActivationDraft};
use sdkwork_contract_service::CommerceServiceError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MembershipRepositoryCommand {
    CreateMembershipPlan,
    UpsertMembershipPackageGroup,
    UpsertMembershipPackage,
    ActivateMembership,
    GrantEntitlement,
    RenewMembership,
    ExpireMembership,
    CreateMembershipPurchase,
    RenewMembershipPurchase,
    UpgradeMembershipPurchase,
}

pub struct MembershipPortRequirement;

pub trait MembershipRepositoryPort {
    fn activate_membership(
        &self,
        draft: &MembershipActivationDraft,
    ) -> Result<(), CommerceServiceError>;

    fn grant_entitlement(&self, draft: &EntitlementGrantDraft) -> Result<(), CommerceServiceError>;
}

pub const MEMBERSHIP_REPOSITORY_PORT: &str = "membership.repository";
pub const IDEMPOTENCY_REPOSITORY_PORT: &str = "idempotency.repository";

impl MembershipPortRequirement {
    pub fn standard_commands() -> Vec<MembershipRepositoryCommand> {
        vec![
            MembershipRepositoryCommand::CreateMembershipPlan,
            MembershipRepositoryCommand::UpsertMembershipPackageGroup,
            MembershipRepositoryCommand::UpsertMembershipPackage,
            MembershipRepositoryCommand::ActivateMembership,
            MembershipRepositoryCommand::GrantEntitlement,
            MembershipRepositoryCommand::RenewMembership,
            MembershipRepositoryCommand::ExpireMembership,
            MembershipRepositoryCommand::CreateMembershipPurchase,
            MembershipRepositoryCommand::RenewMembershipPurchase,
            MembershipRepositoryCommand::UpgradeMembershipPurchase,
        ]
    }
}
