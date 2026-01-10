use crate::types::{Label, DisplayName, Annotation};
use crate::model::ids::AreaId;

pub struct AreaMetadata {
    label: Label,
    display_name: Option<DisplayName>,
    annotation: Option<Annotation>,
}

trait MetadataStore {
    fn put_area_metadata(&mut self, area_id: AreaId, meta: AreaMetadata);
    fn get_area_metadata(&self, area_id: &AreaId) -> Option<&AreaMetadata>;
    fn delete_area_metadata(&mut self, area_id: &AreaId);
}
