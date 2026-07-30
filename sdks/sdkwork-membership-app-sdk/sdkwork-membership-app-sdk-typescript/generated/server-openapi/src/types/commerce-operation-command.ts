/** Membership operation command. purchase/renew/upgrade require packageId, orderId, and requestNo from order memberships.orders.create. */
export interface CommerceOperationCommand {
  action: 'purchase' | 'renew' | 'upgrade' | 'claim_daily_reward' | 'consume_speed_up';
  packageId?: string;
  /** Commerce order UUID from order memberships.orders.create. */
  orderId?: string;
  /** Human-readable order number from order memberships.orders.create. */
  requestNo?: string;
  couponId?: string;
  idempotencyKey?: string;
}
