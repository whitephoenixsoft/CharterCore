use serde::Serialize;
use crate::model::ids::AreaId;
use crate::model::area::Area;
use crate::time::Timestamp;

#[derive(Debug, Clone, Serialize)]
pub struct AreaRoot {
    pub area_id: AreaId,
    pub created_at: Timestamp,
}

impl AreaRoot {
    pub fn default() -> Self {
        Self {
            area_id: AreaId("".into()),
            created_at: "".into(),
        }
    }
}

impl From<Area> for AreaRoot {
    fn from(a: Area) -> Self {
        AreaRoot { 
            area_id: a.area_id,
            created_at: a.created_at,
        }
    }
}
