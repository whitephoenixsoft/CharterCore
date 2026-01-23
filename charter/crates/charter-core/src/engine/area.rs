use super::Engine;
use crate::model::ids::AreaId;
use crate::model::area::*;

pub struct CreateArea {
    pub label: AreaLabel,
    pub name: AreaName,
    pub annotation: Option<Annotation>,
}

/*
impl Engine {
    pub create_area(&self, area_id: AreaId) {

    }
}
*/
