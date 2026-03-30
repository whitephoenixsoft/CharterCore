# ENG-CANON — Canonical Serialization & Hashing Specification

Status: REFACTORED (v6 – Structure-Aligned, Boundary Tightening)  
Applies to: Engine Core (V1/V2+)  
Scope: Canonical serialization and hashing rules for structural artifacts

Authority: Foundational authority for canonical byte representation and hash input construction.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-RECEIPT
- ENG-SPECVERIFY
- ENG-INTEGRITY
- ENG-STRUCTURE
- ENG-USABILITY

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
- structural graph semantics
- usability semantics

Those are defined respectively in:

- ENG-DOMAIN
- ENG-RECEIPT
- ENG-SPECVERIFY
- ENG-INTEGRITY
- ENG-PERSISTENCE
- ENG-STRUCTURE
- ENG-USABILITY

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
- whether a field participates in structural graph meaning
- whether a field affects usability
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

ENG-CANON does not define semantic ordering rules for arrays.

Instead:

- authoritative specifications define required ordering
- ENG-CANON requires that canonical serialization preserves that order exactly

Examples include:

- participant_set ordering (ENG-SESSION / ENG-DOMAIN)
- candidate_set ordering (ENG-SESSION / ENG-DOMAIN)
- constraint_set ordering (ENG-SESSION / ENG-DOMAIN)
- vote_set ordering (ENG-SESSION / ENG-DOMAIN)
- round ordering (ENG-RECEIPT)
- informational reference lists (ENG-DOMAIN)

Implementations must not reorder arrays during canonicalization.

If ordering requirements are not satisfied prior to canonicalization:

- canonicalization must fail, or  
- the artifact must be rejected by the consuming layer  

---

# 6. Structural vs Informational Inclusion

## ENG-CANON-05 — Schema-Driven Inclusion Only

Canonical hashing includes only recognized structural fields.

ENG-CANON does not classify fields.  
Classification is defined by:

- ENG-DOMAIN
- ENG-RECEIPT
- ENG-SPECVERIFY

ENG-CANON requires:

- all recognized structural fields must be included
- informational fields are included only if the governing schema explicitly classifies them as structural
- unknown informational fields must be excluded
- unknown structural fields must cause canonicalization failure

Canonicalization must not reinterpret or reclassify fields.

---

## ENG-CANON-05A — Informational References Are Encoded but Non-Semantic

Informational references may be present as structural fields.

When present and schema-recognized:

- they must be serialized canonically
- they must be included in hash input

However, canonicalization must not:

- treat them as structural edges
- assign graph meaning
- affect structural ACTIVE derivation
- affect conflict detection
- introduce ordering or precedence semantics

Graph meaning belongs exclusively to ENG-STRUCTURE.  
Usability meaning belongs exclusively to ENG-USABILITY.

ENG-CANON encodes; it does not interpret.

---

# 7. Object Completeness

## ENG-CANON-06 — Canonical Input Must Be Structurally Complete

Canonicalization requires that all required recognized structural fields be present.

Rules:

- missing required structural fields cause failure
- explicit structural nulls must remain explicit
- fields must not be synthesized
- fields must not be inferred

Field requirements are defined externally.

ENG-CANON enforces completeness only.

---

# 8. String Encoding

## ENG-CANON-07 — Strings Must Preserve Exact Content

Strings must follow standard JSON encoding.

Requirements:

- UTF-8 encoding
- minimal required escaping only
- no unnecessary Unicode escaping
- exact content preservation

Canonicalization must never:

- normalize Unicode
- change case
- trim whitespace
- alter content

---

# 9. Number Encoding

## ENG-CANON-08 — Integer Canonical Form

Integers must:

- have no leading zeros
- be represented in minimal decimal form

---

## ENG-CANON-09 — Floating-Point Prohibition

Floating-point values are not permitted in structural canonical content.

If encountered:

- canonicalization must fail

---

# 10. Boolean Encoding

## ENG-CANON-10 — Standard JSON Booleans

Allowed values:

- true
- false

No alternatives permitted.

---

# 11. Null Encoding

## ENG-CANON-11 — Explicit Null Preservation

Null values must be encoded as:

- null

Missing vs null must not be conflated.

---

# 12. Timestamp Encoding

## ENG-CANON-12 — Timestamp Strings Are Non-Authoritative

Timestamps must:

- follow RFC 3339 / ISO-8601
- use UTC where required
- maintain fixed precision when present

Timestamps must not influence:

- ordering
- structural graph meaning
- legitimacy
- precedence

---

# 13. Canonical Hash Input

## ENG-CANON-13 — Canonical Hash Input Definition

Hash input is:

- UTF-8 bytes of canonical JSON of recognized structural content

Must include:

- all recognized structural fields
- nested structural content
- rule identity fields when required

Must exclude:

- unknown informational fields
- runtime-only metadata

---

# 14. Hash Algorithm Declaration

## ENG-CANON-14 — Explicit Algorithm Requirement

Artifacts must declare:

- hash_algorithm

Hash output:

- lowercase hexadecimal unless specified otherwise

---

# 15. Determinism Guarantee

## ENG-CANON-15 — Full Determinism

Identical structural input must produce identical:

- canonical JSON
- byte sequences
- hash outputs

---

# 16. Prohibited Behaviors

## ENG-CANON-16 — No Structural Mutation

Canonicalization must not:

- reorder arrays
- normalize data
- infer fields
- omit required fields
- include unknown structural fields
- ignore unknown structural fields

---

# 17. Rule Identity Integration

## ENG-CANON-17 — Provenance Inclusion Only

Where required fields exist:

- engine_version
- spec_set_hash

They must be included exactly.

Meaning belongs to ENG-SPECVERIFY.

---

# 18. Receipt Relationship

## ENG-CANON-18 — Receipts Consume Canonicalization

ENG-RECEIPT defines structure.

ENG-CANON defines encoding.

---

# 19. Integrity Relationship

## ENG-CANON-19 — Integrity Consumes Output

ENG-INTEGRITY may:

- validate hashes
- determine trust outcomes

ENG-CANON does not define consequences.

---

# 20. Engine Invariants

- deterministic encoding
- no structural inference
- no semantic interpretation
- strict inclusion rules
- UTF-8 JSON canonical output
- informational references remain non-semantic

---

# 21. Mental Model

ENG-CANON defines byte truth.

It transforms structure into bytes.

It does not interpret meaning.

It does not validate legitimacy.

It does not decide outcomes.