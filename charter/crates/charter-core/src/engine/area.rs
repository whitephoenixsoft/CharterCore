use super::Engine;
use crate::model::ids::AreaId;
use crate::model::area::*;
use crate::model::CharterModelKind;
use crate::storage::envelope::ObjectEnvelope;
use crate::storage::CharterObjectKind;
use derive_more::Display;

#[derive(Clone, Debug, Display)]
#[display("CreateArea({},{})", label, name)]
pub struct CreateArea {
    pub label: AreaLabel,
    pub name: AreaName,
    pub annotation: Option<Annotation>,
}

impl Engine {
    pub fn create_area(&mut self, area_info: &CreateArea) -> Result<(), String> {
        //todo: build and assert engine valid state here
        //todo: query for label duplication
        //todo: query for name duplication
        //todo find a way of making this all atomic
        let new_area = CharterModelKind::Area(AreaRuntime::new(
            area_info.label.clone(),
            area_info.name.clone(),
            area_info.annotation.clone(),
        ));

        //save the area id & timestamp
        let new_area_object: CharterObjectKind  = new_area.into();
        let envelope = ObjectEnvelope::new(new_area_object).expect("failed to create envelope");

        //Integrity check
        assert_eq!(envelope.verify().ok(), Some(true));

        let json = serde_json::to_vec(&envelope).expect("failed to create json");
        self.objects.put(envelope.object_hash, &json).expect("failed to save area");

        //todo:save the area label, name, and annotation to metadata store
        Ok(())
    }
}
