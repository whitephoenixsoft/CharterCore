//! Rule OS-06 — Canonical JSON Rules (v1)
//!
//! # Description
//! Canonical JSON serialization MUST be deterministic and stable.
//!
//! ## Guarantees
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
//! ## Executable Specification
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
//! ## Failure Modes
//! - Different serializations produce different hashes
//! - Canonicalization logic diverges across components
//! - Any deviation invalidates object identity 