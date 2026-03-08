# ENG-CANON — Canonical Serialization & Hashing Specification
Status: FROZEN (v2 – Deterministic Engine Serialization & Rule Identity Alignment)
Applies to: Engine Core (V1/V2+)
Scope: Canonical serialization rules used for deterministic hashing of structural artifacts

Authority: Subordinate to ENG-DOMAIN, ENG-RECEIPT, ENG-INTEGRITY, ENG-SPECVERIFY


---

# 1. Purpose

This specification defines the canonical serialization format used by the Engine when computing structural content hashes.

Goals:

- Guarantee deterministic hashing across independent implementations
- Prevent cross-language serialization drift
- Ensure receipts and other structural artifacts produce identical hashes everywhere
- Eliminate ambiguity in encoding, ordering, or formatting
- Bind structural artifacts to the rule identity that produced them

Canonical serialization is used when computing:

- Receipt `content_hash`
- Any future canonical structural hash defined by the Engine

Serialization rules in this specification are mandatory.

Deviation constitutes an Engine defect.


---

# 2. Canonical Encoding Format

Canonical serialization must use:

- UTF-8 encoded JSON
- No BOM (Byte Order Mark)
- No whitespace outside of string values

Allowed whitespace characters inside string values:

- Space (U+0020)
- Tab (U+0009)
- Newline (U+000A)

Whitespace must never be inserted for formatting purposes.

Example canonical JSON:

{"a":1,"b":"text","c":["x","y"]}


---

# 3. Deterministic Object Field Ordering

JSON object fields must be serialized in strict lexicographic order by field name.

Ordering uses:

- Unicode code point ordering
- Exact string comparison
- Case-sensitive

Example:

Correct ordering:

{"area_id":"A","engine_version":"1","session_id":"S"}

Incorrect:

{"session_id":"S","area_id":"A","engine_version":"1"}

Field ordering must never depend on:

- insertion order
- programming language map ordering
- database column order


---

# 4. Deterministic Array Ordering

Arrays must appear exactly in the deterministic order defined by the structural specification.

Examples:

Participant sets:

ordered lexicographically by `participant_id`

Candidate sets:

ordered lexicographically by `candidate_id`

Vote sets:

ordered lexicographically by `vote_id`

Rounds:

ordered by ascending `round_index`

Implementations must not reorder arrays during serialization.

The ordering must already be correct before serialization.


---

# 5. Structural vs Informational Fields

Canonical hashing includes only structural fields.

Structural fields affect:

- legitimacy evaluation
- governance semantics
- receipt identity
- canonical artifact integrity
- rule identity provenance

Examples of structural fields include:

- session identifiers
- participant identifiers
- candidate identifiers
- governance references
- vote structures
- rule identity fields (`engine_version`, `spec_set_hash`)

Informational fields must not influence canonical hashing.

Examples include:

- external labels
- annotations
- host metadata
- presentation-only data

Unknown informational fields must be excluded from canonical hashing.

Unknown structural fields must cause canonicalization failure.


---

# 6. Object Completeness Rule

Canonical objects must contain every structural field defined by their schema.

Rules:

- Missing structural fields must cause serialization failure
- Structural null values must be explicitly represented
- Fields must never be inferred or synthesized during serialization

Canonical serialization must represent the complete structural state.


---

# 7. String Encoding

Strings must be encoded using standard JSON string rules.

Rules:

- UTF-8 encoding required
- Escape sequences allowed only when required by JSON
- Unicode characters must not be unnecessarily escaped

Example:

Preferred:

"José"

Allowed but discouraged:

"\u004a\u006f\u0073\u00e9"

Implementations must never:

- normalize Unicode
- change case
- trim whitespace
- alter string contents


---

# 8. Number Encoding

Numbers must follow strict canonical rules.

### Integers

Must be encoded without leading zeros.

Valid:

1  
10  
42  

Invalid:

01  
0005  

---

### Floating-Point Numbers

Floating-point numbers are not permitted in canonical structural artifacts.

Structural Engine objects must use:

- integers
- strings

Floating-point values must never appear in canonical serialized structures.

If encountered, the engine must fail serialization.


---

# 9. Boolean Encoding

Booleans must use standard JSON values:

true  
false

No capitalization variation allowed.


---

# 10. Null Encoding

Null values must be represented using:

null

Null fields must be explicitly present if the schema defines them.

Missing fields are not equivalent to null.


---

# 11. Timestamp Representation

Timestamps must be serialized as strings using:

RFC 3339 / ISO-8601 format with timezone.

Example:

"2026-03-05T14:22:11Z"

Rules:

- UTC required
- Z suffix required
- Milliseconds optional
- If milliseconds are present they must remain consistent for identical objects

Timestamps are informational unless explicitly declared structural.

Timestamps must never influence canonical ordering or legitimacy evaluation.


---

# 12. Canonical Hash Input

The canonical hash input must be:

UTF-8 bytes of the canonical serialized JSON object.

Hash input must include:

- all recognized structural fields
- rule identity fields (`engine_version`, `spec_set_hash`)
- canonical arrays
- nested structures

Hash input must exclude:

- unknown informational fields
- runtime metadata
- external host metadata


---

# 13. Hash Algorithms

The algorithm used to compute a canonical hash must be explicitly declared in the object containing the hash.

Example field:

hash_algorithm

Example value:

SHA-256

The hash output must be encoded using lowercase hexadecimal.

Example:

"7e9a5e1a3b1c6a7c0e2a1c9a6f7e5c1a3d6f9a8b4c1d0e2f3a4b5c6d7e8f9a0"

Algorithm migration does not change the identity of the artifact.


---

# 14. Determinism Guarantee

Given identical structural input objects:

Two independent engines must produce:

- identical canonical JSON
- identical byte streams
- identical content_hash values

Any divergence indicates:

- serialization defect
- canonicalization violation
- implementation error


---

# 15. Prohibited Behaviors

The Engine must never:

- reorder object fields arbitrarily
- normalize Unicode
- insert formatting whitespace
- emit floating-point numbers
- omit required null fields
- infer missing fields
- include unknown informational fields in canonical hashing


---

# 16. Engine Invariants

Canonical serialization guarantees:

- Cross-language deterministic hashing
- Stable receipt verification
- Federation-safe legitimacy artifacts
- Deterministic DAG reconstruction validation
- Stable rule identity binding through `spec_set_hash`

Violation of canonical serialization rules constitutes a critical Engine defect.


---

# 17. Mental Model

Canonical serialization defines the exact byte representation of structural artifacts.

The same object must always produce the same bytes.

The same bytes must always produce the same hash.

Rule identity fields ensure artifacts are bound to the specification set that produced them.

Canonical serialization is the foundation of deterministic legitimacy verification.