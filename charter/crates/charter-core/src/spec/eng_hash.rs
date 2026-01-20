//! # Charter Core — Object Store & Hashing Specification
//! Status: LOCKED (v1)
//! Scope: Engine-internal
//! Change Policy: Any change requires a new hash version and explicit migration
//!
//! ## Rule OS-05 — Hash Input Is Canonical and Deterministic (v1)
//!
//! For hash version v1, the digest input MUST be the following byte sequence:
//!    
//! charter:<hash_version>\n
//! type:<object_type>\n
//! len:<byte_length>\n
//! <canonical_json_bytes>
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
//! use charter_core::types::CharterObjectKind;
//!
//! let fields = HashInput {
//!     version: HashVersion::V1,
//!     algorithm: HashAlgorithm::Sha256,
//!     object_type: CharterObjectKind::Area,
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
//! ### Executable Specification
//!
//! ```rust
//! use serde::Serialize;
//! use charter_core::storage::hashing::HashInput;
//! use charter_core::storage::hashing::HashVersion;
//! use charter_core::storage::hashing::HashAlgorithm;
//! use charter_core::types::CharterObjectKind;
//! use charter_core::storage::hashing::compute_hash;
//!
//! let input = HashInput {
//!     version: HashVersion::V1,
//!     algorithm: HashAlgorithm::Sha256,
//!     object_type: CharterObjectKind::Area,
//!     canonical_json: b"{}",
//! };
//! 
//! let digest = compute_hash(&input).ok();
//!
//! assert_eq!(
//!     digest.unwrap().0,
//!     "550b249927be192ec93800d097619da6e8fb7c0d1e4357dc97178bce6f5fd1db"
//! );
//! ```
//!
//! ### Guarantees
//!
//! The digest MUST be:
//! - Deterministic
//! - Lowercase hexadecimal
//! - Exactly 64 characters (sha256)
