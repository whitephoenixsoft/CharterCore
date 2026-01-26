use serde::{Serialize, Deserialize};
use crate::model::ids::AreaId;
use crate::model::area::AreaRuntime;
use crate::time::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

