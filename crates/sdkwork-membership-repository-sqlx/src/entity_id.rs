use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{AppMembershipEntityIdGenerator, AppMembershipResult};

#[derive(Debug, Default)]
pub struct TimestampMembershipEntityIdGenerator {
    sequence: AtomicU64,
}

impl AppMembershipEntityIdGenerator for TimestampMembershipEntityIdGenerator {
    fn generate_entity_uuid(&self) -> AppMembershipResult<String> {
        let now = current_unix_timestamp();
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        Ok(format!("membership-{now}-{sequence:016x}"))
    }
}

fn current_unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}
