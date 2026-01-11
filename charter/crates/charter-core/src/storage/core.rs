use crate::model::ids::AreaId;
use crate::time::Timestamp;
use serde::Serialize;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ObjectHash(pub String);

#[derive(Debug, Serialize)]
pub struct AreaRoot {
    pub area_id: AreaId,
    pub created_at: Timestamp,
}
