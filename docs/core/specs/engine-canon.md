# ENG-CANON — Canonical Serialization & Hashing Specification
Status: FROZEN (v1 – Deterministic Engine Serialization)
Applies to: Engine Core (V1/V2+)
Scope: Canonical serialization rules used for deterministic hashing of structural artifacts

Authority: Subordinate to ENG-DOMAIN, ENG-RECEIPT, ENG-INTEGRITY


---

# 1. Purpose

This specification defines the canonical serialization format used by the Engine when computing structural content hashes.

Goals:

- Guarantee deterministic hashing across independent implementations
- Prevent cross-language serialization drift
- Ensure receipts and other structural artifacts produce identical hashes everywhere
- Eliminate ambiguity in encoding, ordering, or formatting

Canonical serialization is used when computing:

- Receipt `content_hash`
- Any future canonical structural hash defined by the Engine

Serialization rules in this specification are mandatory.

Deviation constitutes an Engine defect.


---

# 2. Canonical Encoding Format

Canonical serialization must use:

- **UTF-8 encoded JSON**
- **No BOM (Byte Order Mark)**
- **No whitespace outside of string values**

Allowed whitespace characters inside string values:

- Space (U+0020)
- Tab (U+0009)
- Newline (U+000A)

Whitespace must never be inserted for formatting purposes.

Example canonical JSON:

{"a":1,"b":"text","c":["x","y"]}


---

# 3. Deterministic Object Field Ordering

JSON object fields must be serialized in **strict lexicographic order by field name**.

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

Implementations must **not reorder arrays during serialization**.

The ordering must already be correct before serialization.


---

# 5. String Encoding

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

# 6. Number Encoding

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

Floating-point numbers are **not permitted in canonical structural artifacts**.

Structural Engine objects must use:

- integers
- strings

Floating-point values must never appear in canonical serialized structures.

If encountered, the engine must fail serialization.


---

# 7. Boolean Encoding

Booleans must use standard JSON values:

true  
false

No capitalization variation allowed.


---

# 8. Null Encoding

Null values must be represented using:

null

Null fields must be explicitly present if the schema defines them.

Missing fields are not equivalent to null.


---

# 9. Timestamp Representation

Timestamps must be serialized as strings using:

RFC 3339 / ISO-8601 format with timezone.

Example:

"2026-03-05T14:22:11Z"

Rules:

- UTC required
- Z suffix required
- Milliseconds optional but must not vary within identical objects

Timestamps are informational unless explicitly declared structural.


---

# 10. Canonical Hash Input

The canonical hash input must be:

UTF-8 bytes of the canonical serialized JSON object.

Hash input must include:

- all recognized structural fields
- all canonical arrays
- nested structures

Hash input must exclude:

- unknown informational fields
- runtime metadata
- external host metadata


---

# 11. Hash Algorithms

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

# 12. Determinism Guarantee

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

# 13. Prohibited Behaviors

The Engine must never:

- reorder object fields arbitrarily
- normalize Unicode
- insert formatting whitespace
- emit floating-point numbers
- omit required null fields
- infer missing fields
- include unknown informational fields in canonical hashing


---

# 14. Engine Invariants

Canonical serialization guarantees:

- Cross-language deterministic hashing
- Stable receipt verification
- Federation-safe legitimacy artifacts
- Deterministic DAG reconstruction validation

Violation of canonical serialization rules constitutes a critical Engine defect.


---

# Mental Model

Canonical serialization defines the exact byte representation of structural artifacts.

The same object must always produce the same bytes.

The same bytes must always produce the same hash.

Canonical serialization is the foundation of deterministic legitimacy verification.