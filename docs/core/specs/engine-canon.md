# ENG-CANON — Canonical Serialization & Hashing Specification

Status: REFACTORED (v3 – Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)  
Scope: Canonical serialization and hashing rules for structural artifacts

Authority: Foundational authority for canonical byte representation and hash input construction.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-RECEIPT
- ENG-SPECVERIFY
- ENG-INTEGRITY

---

# 1. Purpose

ENG-CANON defines the canonical serialization and canonical hash input rules used by the Engine.

It is the authoritative specification for:

- canonical encoding format
- canonical object field ordering
- canonical array ordering consumption
- structural-vs-informational field inclusion in hash input
- primitive encoding rules
- canonical hash input construction
- deterministic hashing preconditions

ENG-CANON does not define:

- object schemas
- which artifacts must exist
- receipt field structure
- rule identity meaning
- runtime halt policy
- persistence boundaries

Those are defined respectively in:

- ENG-DOMAIN
- ENG-RECEIPT
- ENG-SPECVERIFY
- ENG-INTEGRITY
- ENG-PERSISTENCE

ENG-CANON defines only how recognized structural content becomes canonical bytes suitable for deterministic hashing.

Deviation from this specification constitutes an Engine defect.

---

# 2. Canonicalization Authority Boundary

## ENG-CANON-01 — Canonicalization Is an Encoding Authority

ENG-CANON answers:

- how structural artifacts are serialized into canonical bytes
- how object fields are ordered
- how primitive values are encoded
- what recognized structural content is included in canonical hashing

ENG-CANON does not answer:

- whether an artifact is structurally valid
- whether an artifact is legitimate
- whether a runtime must halt
- which fields are structurally required by a given schema
- which rule set a spec_set_hash represents

Those truths must be supplied by other specifications before canonicalization is attempted.

---

# 3. Canonical Encoding Format

## ENG-CANON-02 — UTF-8 JSON Without Formatting Whitespace

Canonical serialization must use:

- UTF-8 encoded JSON
- no BOM
- no whitespace outside string values

Whitespace may appear only inside string values as literal data.

Canonicalization must never insert formatting whitespace for readability.

The same structural object must always serialize to the same UTF-8 byte sequence.

---

# 4. Object Field Ordering

## ENG-CANON-03 — Lexicographic Field Ordering

JSON object fields must be serialized in strict lexicographic order by field name.

Ordering rules:

- Unicode code point ordering
- exact string comparison
- case-sensitive comparison

Field order must never depend on:

- insertion order
- language runtime map ordering
- storage order
- database column order

ENG-CANON is the sole authority for canonical object field ordering.

---

# 5. Array Ordering

## ENG-CANON-04 — Arrays Preserve External Deterministic Order

ENG-CANON does not define the semantic ordering rules for every array.

Instead:

- the authoritative structural specification defines the required order
- ENG-CANON requires that canonical serialization preserve that order exactly

Examples of ordering authority defined elsewhere include:

- participant_set ordering
- candidate_set ordering
- constraint_set ordering
- vote_set ordering
- round ordering

Implementations must not reorder arrays during canonicalization.

If an array is not already in the deterministic order required by its governing specification, canonicalization must fail or the artifact must be treated as invalid by the consuming layer.

---

# 6. Structural vs Informational Inclusion

## ENG-CANON-05 — Only Recognized Structural Fields Participate in Canonical Hashing

Canonical hashing includes only recognized structural fields.

ENG-CANON does not independently classify fields as structural or informational.  
That classification is supplied by the authoritative schema or artifact specification, including:

- ENG-DOMAIN
- ENG-RECEIPT
- ENG-SPECVERIFY

ENG-CANON requires:

- recognized structural fields included
- unknown informational fields excluded
- unknown structural fields cause canonicalization failure

Canonicalization must not silently reinterpret unrecognized structural content.

---

# 7. Object Completeness

## ENG-CANON-06 — Canonical Input Must Be Structurally Complete

Canonicalization requires that all required recognized structural fields be present.

Rules:

- missing required structural fields cause failure
- explicit structural nulls must remain explicit
- fields must not be synthesized during canonicalization
- fields must not be inferred during canonicalization

Whether a field is required is not decided by ENG-CANON.  
That is decided by the governing schema or artifact specification.

ENG-CANON only requires that the canonicalized object be structurally complete according to those definitions.

---

# 8. String Encoding

## ENG-CANON-07 — Strings Must Preserve Exact Content

Strings must be encoded using standard JSON string rules.

Requirements:

- UTF-8 encoding required
- escape sequences used only as required by JSON syntax
- Unicode characters must not be unnecessarily escaped
- string contents must remain semantically unchanged

Canonicalization must never:

- normalize Unicode
- change case
- trim whitespace
- alter string content
- reinterpret escaped content as normalized content

The same logical string must always produce the same canonical JSON string encoding under the same input representation.

---

# 9. Number Encoding

## ENG-CANON-08 — Integer Canonical Form

Integers must be encoded without leading zeros.

Valid examples:

- 1
- 10
- 42

Invalid examples:

- 01
- 0005

---

## ENG-CANON-09 — Floating-Point Numbers Prohibited in Structural Canonical Content

Floating-point values are not permitted in canonical structural artifacts.

If encountered in structural content, canonicalization must fail.

Whether a field is structural is determined externally.  
ENG-CANON enforces that structural canonical byte generation may not proceed with floating-point values.

---

# 10. Boolean Encoding

## ENG-CANON-10 — Standard JSON Booleans

Boolean values must use standard JSON literals:

- true
- false

No alternate capitalization or non-JSON representation is allowed.

---

# 11. Null Encoding

## ENG-CANON-11 — Explicit Null Preservation

Null values must be encoded as:

- null

If a governing specification requires explicit null presence, canonicalization must preserve that field explicitly.

Missing and null are not equivalent in canonical structural content.

---

# 12. Timestamp Encoding

## ENG-CANON-12 — Timestamp Strings Preserve Structural Policy but Do Not Create Ordering

Timestamps must be serialized as strings using RFC 3339 / ISO-8601 format with timezone when the governing artifact specification requires timestamp fields.

Canonical timestamp requirements:

- UTC required where the governing spec requires UTC
- Z suffix required where UTC normalization is required
- millisecond precision must be stable for identical objects when present

ENG-CANON does not decide whether a timestamp is structural or informational.  
That decision belongs to the governing schema or artifact specification.

Timestamps must never influence canonical field ordering or legitimacy evaluation merely by existing.

---

# 13. Canonical Hash Input

## ENG-CANON-13 — Canonical Hash Input Is UTF-8 Canonical JSON Bytes

Canonical hash input must be:

- the UTF-8 bytes of the canonical serialized JSON representation of the recognized structural content

Hash input must include:

- all recognized structural fields
- nested structural content
- rule identity fields where the governing artifact requires them

Hash input must exclude:

- unknown informational fields
- runtime-only metadata not classified as structural
- host-specific presentation metadata not classified as structural

ENG-CANON defines byte construction only.  
It does not define which artifact types require hashing.

---

# 14. Hash Algorithm Declaration

## ENG-CANON-14 — Declared Hash Algorithm Required

The artifact containing a canonical hash must declare the algorithm used.

Typical examples are defined elsewhere, such as in receipts.

ENG-CANON requires:

- algorithm explicitly declared by the artifact schema
- hash output encoded as lowercase hexadecimal unless the governing artifact specification states otherwise

ENG-CANON does not define artifact identity semantics for algorithm migration.  
That belongs to the artifact specification and rule provenance specifications.

---

# 15. Determinism Guarantee

## ENG-CANON-15 — Identical Structural Input Produces Identical Canonical Bytes

Given identical recognized structural input content, compliant implementations must produce:

- identical canonical JSON
- identical UTF-8 byte streams
- identical canonical hash input
- identical hash outputs when the same declared hash algorithm is applied

Any divergence indicates:

- serialization defect
- canonicalization defect
- violation of upstream structural ordering requirements
- implementation error

---

# 16. Prohibited Behaviors

## ENG-CANON-16 — Canonicalization Must Not Invent or Normalize Structure

The Engine must never, during canonicalization:

- reorder object fields arbitrarily
- reorder arrays contrary to authoritative structural ordering
- normalize Unicode
- insert formatting whitespace
- emit floating-point structural content
- omit required explicit nulls
- infer missing structural fields
- synthesize missing fields
- include unknown informational fields in canonical hash input
- silently ignore unknown structural fields

---

# 17. Relationship to Rule Identity

## ENG-CANON-17 — Rule Identity Fields Participate Only When Required by Governing Artifacts

ENG-CANON does not define rule provenance semantics.

It consumes them from governing artifact specifications.

Where a governing artifact requires fields such as:

- engine_version
- spec_set_hash

ENG-CANON requires that those recognized structural fields participate in canonical serialization and canonical hash input exactly as defined by the artifact specification.

Meaning and verification of those fields belong to ENG-SPECVERIFY.

---

# 18. Relationship to Receipts

## ENG-CANON-18 — Receipts Consume Canonicalization, Not Vice Versa

ENG-RECEIPT defines the receipt artifact structure.

ENG-CANON defines how that structure becomes canonical bytes for hashing.

ENG-CANON does not redefine:

- receipt fields
- round snapshot schemas
- receipt emission conditions
- receipt immutability

It only defines canonical representation of the recognized structural receipt content.

---

# 19. Relationship to Runtime Integrity

## ENG-CANON-19 — Integrity Consumes Canonicalization Results

ENG-INTEGRITY may use canonicalization results to determine:

- hash mismatch
- structural trust failure
- degraded or halt outcomes

ENG-CANON does not decide runtime consequences.

It defines only whether canonical serialization and canonical hash input construction are valid and deterministic.

---

# 20. Engine Invariants

- canonical object field ordering is deterministic
- canonicalization preserves authoritative array ordering
- only recognized structural content participates in canonical hashing
- unknown structural fields cause canonicalization failure
- explicit structural nulls are preserved
- floating-point structural content is prohibited
- canonical output is UTF-8 JSON without formatting whitespace
- identical structural input yields identical canonical bytes
- rule identity fields participate only where required by governing artifact specifications
- dependent specifications must consume ENG-CANON rather than redefine byte-level canonicalization

---

# 21. Mental Model

ENG-CANON defines byte truth.

It answers:

- how structural content becomes canonical JSON
- how canonical bytes are constructed
- how deterministic hash input is formed

It does not answer:

- which artifacts exist
- whether an artifact is valid
- whether a runtime must halt
- what a spec_set_hash means
- when a receipt is emitted

Those belong elsewhere.

ENG-CANON is the serialization layer.  
Other specifications must consume it rather than duplicate it.