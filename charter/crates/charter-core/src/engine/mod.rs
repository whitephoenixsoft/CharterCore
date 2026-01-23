pub mod area;

use crate::storage::object_store::ObjectStore;
use crate::storage::ref_store::RefStore;

pub struct Engine {
    pub objects: Box<dyn ObjectStore>,
    pub refs: Box<dyn RefStore>,
}

// Note: Need this for later
// fn ensure_resolution_id_unique(id: &ResolutionId) -> Result<(), Error>
