use crate::error::EngineError;
use crate::spec::SpecificationManifest;

pub struct Engine;

impl Engine {
    pub fn rehydrate(
        input: RehydrateInput,
    ) -> Result<Self, EngineError> {
        Ok(Self {})
    }

    pub fn specification_manifest() -> &'static SpecificationManifest {
        crate::spec::embedded_manifest()
    }
}

pub struct RehydrateInput;
