use serde::Serialize;
use crate::model::ids::AreaId;
use crate::model::area::AreaRuntime;
use crate::time::Timestamp;

#[derive(Debug, Clone, Serialize)]
pub struct AreaObject {
    pub area_id: AreaId,
    pub created_at: Timestamp,
}

impl AreaObject {
    pub fn default() -> Self {
        Self {
            area_id: AreaId::new(),
            created_at: Timestamp::now(),
        }
    }
}

impl From<AreaRuntime> for AreaObject {
    fn from(a: AreaRuntime) -> Self {
        AreaObject { 
            area_id: a.area_id,
            created_at: a.created_at,
        }
    }
}

#[enum_dispatch]
trait GetBytes {
    fn get_bytes(&self) -> Result<Vec<u8>, serde_json::Error>;
}

impl GetBytes for AreaObject {
    fn get_bytes(&self) -> Result<Vec<u8>, serdes_json::Error> {
        get_canonical_json(&self)
    }
}
