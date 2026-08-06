/** Membership feature access check result. */
export interface MembershipFeatureAccessCheckResult {
  allowed: boolean;
  active: boolean;
  currentLevel: string;
  requiredLevel: string;
  status: string;
  expiresAt?: string;
  reason?: string;
}
