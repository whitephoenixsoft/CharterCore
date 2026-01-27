use super::Engine;
use crate::model::ids::AreaId;
use crate::model::area::*;
use crate::model::CharterModelKind;
use crate::storage::envelope::ObjectEnvelope;
use derive_more::Display;

#[derive(Clone, Debug, Display)]
#[display("CreateArea({},{})", label, name)]
pub struct CreateArea {
    pub label: AreaLabel,
    pub name: AreaName,
    pub annotation: Option<Annotation>,
}

impl Engine {
    pub fn create_area(&self, area_info: &CreateArea) {
        let new_area = CharterModelKind::Area(AreaRuntime::new(
            area_info.label.clone(),
            area_info.name.clone(),
            area_info.annotation.clone(),
        ));

        let new_area_object = new_area.into();
        let envelope = ObjectEnvelope::new(new_area_object);
    }
}
