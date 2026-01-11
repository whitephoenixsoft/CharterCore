use super::core::ObjectHash;
use crate::time::Timestamp;
use crate::model::ids::AreaId;

pub enum AuditEvent {
    AreaCreated {
        area_id: AreaId,
        timestamp: Timestamp,
        area_root_hash: ObjectHash,
    },
}

trait AuditStore {
    fn record(&mut self, event: AuditEvent);
    fn all_events(&self,) -> Vec<AuditEvent>;
}
