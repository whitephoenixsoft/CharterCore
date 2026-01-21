//! # Charter Core — Object Store & Hashing Specification
//! Status: LOCKED (v1)
//! Scope: Engine-internal
//! Change Policy: Any change requires a new hash version and explicit migration
//!
//! ### Rule OS-03 — Hash Version Is Explicit
//!
//! Every object hash MUST be computed under an explicit Charter hash version.
//!
//! Initial version: v1
//!
//! Future changes require:
//! - A new version identifier (v2, v3, …)
//! - Explicit rehash logic
//! - Parallel coexistence of versions
//!
//! Fail if:
//! - Hash version is inferred implicitly
//! - Hash version changes without migration
//!
//! ### Rule OS-04 — Multiple Hash Versions May Coexist
//!
//! The engine MUST:
//! - Accept objects with different hash versions
//! - Preserve original hashes during import
//! - Treat hash version as part of object identity
//!
//! The engine MUST NOT:
//! - Assume a single global hash version
//! - Auto-upgrade hashes implicitly
//!
//! ## Rule OS-05 — Hash Input Is Canonical and Deterministic (v1)
//!
//! For hash version v1, the digest input MUST be the following byte sequence:
//!    
//! "charter:<hash_version>\n
//! type:<object_type>\n
//! len:<byte_length>\n
//! <canonical_json_bytes>"
//!    
//! Where:
//! - `hash_version` = "v1"
//! - `object_type` ∈ { area, session, resolution, candidate, stance, audit, … }
//! - `byte_length` = length (in bytes) of `<canonical_json_bytes>`
//! - `<canonical_json_bytes>` is UTF-8 encoded
//!
//! ### Executable Specification
//!
//! ```rust
//! use serde::Serialize;
//! use charter_core::storage::hashing::HashInput;
//! use charter_core::storage::hashing::HashVersion;
//! use charter_core::storage::hashing::HashAlgorithm;
//! use charter_core::storage::CharterObjectKind;
//! use charter_core::storage::area::AreaRoot;
//!
//! let fields = HashInput {
//!     version: HashVersion::V1,
//!     algorithm: HashAlgorithm::Sha256,
//!     object_type: CharterObjectKind::Area(AreaRoot::default()),
//!     canonical_json: b"{\"value\":42}",
//! }.as_bytes();
//!
//! let expected = b"charter:v1\ntype:area\nlen:12\n{\"value\":42}";
//!
//! assert_eq!(fields, expected);
//! ```
//!
//! Any deviation in header order, newline placement, or length calculation
//! invalidates object identity.
//!
//! ## Rule OS-06 — Canonical JSON Rules (v1)
//!
//! Canonical JSON serialization MUST be deterministic and stable.
//!
//! ### Guarantees
//! 1. Canonical JSON serialization MUST obey:
//!    - UTF-8 encoding
//!    - Lexicographically sorted object keys
//!    - No insignificant whitespace
//!    - No pretty-print formatting
//!    - Arrays preserve order
//!    - Numbers serialized unambiguously
//!
//! 2. These rules MUST be implemented once and reused across the engine.
//!
//! ### Executable Specification
//! The following test MUST pass for any compliant implementation.
//!
//! ```rust
//! use serde::Serialize;
//! use charter_core::storage::hashing::get_canonical_json;
//!
//! #[derive(Serialize)]
//! struct ComplexState {
//!     beta: f64,
//!     alpha: String,
//!     data_points: Vec<u32>,
//!     metadata: Metadata,
//! }
//!
//! #[derive(Serialize)]
//! struct Metadata {
//!     version: u8,
//!     author: String,
//! }
//!
//! let state = ComplexState {
//!     beta: 3000.0,
//!     alpha: "engine_v1".to_string(),
//!     data_points: vec![10, 20, 30],
//!     metadata: Metadata {
//!         version: 1,
//!         author: "Admin".to_string(),
//!     },
//! };
//!
//! let result_bytes = get_canonical_json(&state)
//!     .expect("Failed to canonicalize JSON");
//!
//! let result_str = std::str::from_utf8(&result_bytes)
//!     .expect("Result must be valid UTF-8");
//!
//! let expected = r#"{"alpha":"engine_v1","beta":3000,"data_points":[10,20,30],"metadata":{"author":"Admin","version":1}}"#;
//!
//! assert_eq!(result_str, expected);
//! ```
//!
//! ### Failure Modes
//! - Different serializations produce different hashes
//! - Canonicalization logic diverges across components
//! - Any deviation invalidates object identity 
//!
//! ## OS-07 — Hash Algorithm (v1)
//!
//! Algorithm: sha256  
//! Output: lowercase hexadecimal string
//!
//! ### Guarantees
//!
//! The digest MUST be:
//! - Deterministic
//! - Lowercase hexadecimal
//! - Exactly 64 characters (sha256)
//!
//! ### Executable Specification
//!
//! ```rust
//! use serde::Serialize;
//! use charter_core::storage::hashing::HashInput;
//! use charter_core::storage::hashing::HashVersion;
//! use charter_core::storage::hashing::HashAlgorithm;
//! use charter_core::storage::CharterObjectKind;
//! use charter_core::storage::area::AreaRoot;
//! use charter_core::storage::hashing::hash_object;
//!
//! let version =  HashVersion::V1;
//! let algorithm = HashAlgorithm::Sha256;
//! let object_type = CharterObjectKind::Area(AreaRoot::default());
//! let canonical_json = b"{}";
//! 
//! let digest = hash_object(version, algorithm, object_type, canonical_json).ok();
//!
//! assert_eq!(
//!     digest.unwrap().0,
//!     "36a174b42c121bf434bd95dc3a229a773c4d6d1c3c71394e3b6ab0772dfa0c68"
//! );
//! ```
//!
//! # Rule OS-08 — Objects Are Stored with an Explicit Envelope
//! 
//! Objects MUST be persisted as self-describing envelopes:
//!
//! {
//!  "charter_hash_version": "v1",
//!  "hash_algorithm": "sha256",
//!  "object_type": "area",
//!  "object_hash": "<hex-digest>",
//!  "object": { ... canonical domain object ... }
//! }

