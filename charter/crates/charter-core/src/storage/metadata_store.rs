use crate::model::ids::{
    AreaId
};

pub struct AreaMetadata {
    label: String,
    display_name: Option<String>,
    annotation: Option<String>,
}

trait MetadataStore {
    fn put_area_metadata(&mut self, area_id: AreaId, meta: AreaMetadata);
    fn get_area_metadata(&self, area_id: &AreaId) -> Option<&AreaMetadata>;
    fn delete_area_metadata(&mut self, area_id: &AreaId);
}



