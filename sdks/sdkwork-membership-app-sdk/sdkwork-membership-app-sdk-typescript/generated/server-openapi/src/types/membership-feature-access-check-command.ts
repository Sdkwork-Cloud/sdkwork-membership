export interface MembershipFeatureAccessCheckCommand {
  /** Registered feature code (ai_chat, image_generation, priority_speed_up, priority_queue, exclusive_model). Required when requiredLevel is omitted. */
  featureCode?: string;
  /** Explicit required member level; overrides the feature registry when provided. */
  requiredLevel?: string;
}
