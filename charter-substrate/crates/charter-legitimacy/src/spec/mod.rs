pub struct SpecificationEntry {
    pub spec_name: &'static str,
    pub spec_hash: &'static str,
}

pub struct SpecificationManifest {
    pub engine_version: &'static str,
    pub hash_algorithm: &'static str,
    pub spec_set_hash: &'static str,
    pub entries: &'static [SpecificationEntry],
}

pub fn embedded_manifest() -> &'static SpecificationManifest {
        &MANIFEST
}

static MANIFEST: SpecificationManifest = SpecificationManifest
{
    engine_version: "0.1.0",
    hash_algorithm: "sha256",
    spec_set_hash: "UNSET",
    entries: &[],
}; 
