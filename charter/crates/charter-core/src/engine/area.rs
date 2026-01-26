use super::Engine;
use crate::model::ids::AreaId;
use crate::model::area::*;
use crate::model::CharterModelKind;

#[derive(Clone, Debug, Display)]
pub struct CreateArea {
    pub label: AreaLabel,
    pub name: AreaName,
    pub annotation: Option<Annotation>,
}

impl Engine {
    pub fn create_area(&self, area_info: CreateArea) {
        let new_area = CharterModelKind::Area(AreaRuntime::new(
            label: area_info.label.clone(),
            name: area_info.name.clone(),
            annotation: area_info.annotation.clone(),
        ));

        let 

    }
}
