# ENG-DOMAIN — Domain Object Schema
Canonical Engine Object Definitions

Status: REFACTORED (v16 – Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)

Authority: Foundational authority for structural object schemas and structural field classification.

Subordinate references consumed from:

- ENG-CANON
- ENG-SPECVERIFY
- ENG-RECEIPT
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-INTEGRITY
- ENG-SESSION
- ENG-DECISION

---

# 1. Purpose

ENG-DOMAIN defines the canonical structural object schemas of the Engine Core.

It is the authoritative specification for:

- persisted domain object types
- structural fields and object identity
- schema versioning rules
- structural vs informational field classification
- Area-local structural boundaries
- governance slot object categories
- reference classes
- canonical structural object completeness requirements

ENG-DOMAIN does not define:

- session lifecycle behavior
- acceptance mechanics
- supersession graph meaning
- UNDER_REVIEW / RETIRED usability semantics
- receipt verification policy
- canonical byte-level serialization rules
- runtime halt behavior
- atomic persistence boundaries

Those are defined respectively in:

- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-INTEGRITY
- ENG-CANON
- ENG-PERSISTENCE

ENG-DOMAIN defines what objects are.  
Other specifications define what those objects mean behaviorally.

---

# 2. General Principles

## ENG-DOMAIN-01 — Persisted Objects Are Structural Artifacts

Persisted domain objects are structural artifacts.

Once written, they are immutable except as replaced or evolved through lifecycle mechanisms defined elsewhere.

ENG-DOMAIN defines object shape, not mutation procedures.

---

## ENG-DOMAIN-02 — Canonical Serialization Is Consumed, Not Defined Here

Domain objects must be capable of canonical serialization.

ENG-CANON defines:

- field ordering
- canonical encoding
- hash input rules

ENG-DOMAIN defines which fields exist and whether they are structural or informational.

---

## ENG-DOMAIN-03 — Structural vs Informational Data

A field is structural if it affects any of the following:

- legitimacy evaluation
- governance semantics
- supersession
- receipt identity
- canonical artifact integrity
- rule provenance
- structural validation
- runtime legitimacy compilation

A field is informational if it does not affect those concerns.

ENG-DOMAIN is the authority for that classification at the schema level.  
ENG-CANON consumes that classification when building canonical hash inputs.

---

## ENG-DOMAIN-04 — Lifecycle State Is a Single Enum Field

Where lifecycle state exists, it must be represented as a single enum field.

ENG-DOMAIN defines the allowed enum values per object category.  
ENG-SUPERSESSION and ENG-REVIEW-RETIRED define the behavioral meaning of those values.

---

## ENG-DOMAIN-05 — Schema Version Required

Every domain object must include:

- schema_version

Schema version determines structural interpretation only.

Runtime acceptance of schema versions is enforced by ENG-INTEGRITY.

---

## ENG-DOMAIN-06 — Engine-Owned Identity

All Engine-generated identifiers must be UUIDv7 unless explicitly stated otherwise.

Caller-provided structural identifiers are prohibited where the Engine owns identity generation.

ENG-DOMAIN defines identifier form.  
ENG-INTEGRITY enforces restore validity.  
UUID timestamp components must not carry legitimacy meaning.

---

## ENG-DOMAIN-07 — Area Identity Is Opaque

area_id is externally provided and opaque.

ENG-DOMAIN defines that it scopes structural objects.  
It does not define Area lifecycle or operational ownership.

---

# 3. Schema Versioning & Compatibility Doctrine

## ENG-DOMAIN-08 — Major / Minor Compatibility Rules

Schema compatibility rules:

- major version changes may alter structural meaning
- minor version changes must be additive and non-legitimacy-altering
- unknown structural enums or fields are incompatible
- missing required structural fields are incompatible

ENG-DOMAIN defines these compatibility rules.  
ENG-INTEGRITY enforces them during runtime rehydration and restore.

---

## ENG-DOMAIN-09 — Unknown Field Handling

Unknown structural fields are not permitted.

Unknown informational fields may be ignored only if the consuming specification allows them to remain informational.

ENG-CANON must not include unknown structural fields in canonical hashing and must fail if they are encountered as structural content.

---

# 4. Single-Area Structural Model

## ENG-DOMAIN-10 — Structural Objects Are Area-Scoped

All structural objects in an active runtime graph must share the same area_id.

ENG-DOMAIN defines this as a schema boundary.

ENG-INTEGRITY enforces whether a runtime graph violating this boundary may proceed.

Cross-area references may exist only as informational metadata unless explicitly declared structural by schema.

---

# 5. Governance Slot Model

## ENG-DOMAIN-11 — Authority and Scope Are Distinct Structural Slots

Each Area has two governance slot categories:

- Authority
- Scope

ENG-DOMAIN defines the object categories and slot participation.

ENG-DOMAIN does not define:

- ACTIVE derivation
- slot usability
- halt semantics when slot validity fails

Those belong to:

- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-INTEGRITY

---

# 6. Structural vs Informational References

## ENG-DOMAIN-12 — Structural References

Structural references are references whose existence or resolution affects legitimacy or runtime structural validity.

Permitted structural reference categories include:

- Session → Authority Resolution
- Session → Scope Resolution
- Resolution → originating Session
- Resolution → superseded Resolution
- Receipt → Session
- LEGITIMACY Receipt → Resolution

Structural references must remain Area-local unless a schema explicitly states otherwise.

ENG-DOMAIN defines the reference classes.  
ENG-INTEGRITY enforces whether they resolve safely.

---

## ENG-DOMAIN-13 — Informational Cross-Area References

Informational cross-area references may point to external Areas or Resolutions.

They:

- do not affect legitimacy
- do not affect ACTIVE derivation
- do not affect supersession structure
- must not be traversed by legitimacy logic

ENG-DOMAIN defines them as opaque informational metadata only.

---

# 7. CrossAreaReference Schema

## ENG-DOMAIN-14 — CrossAreaReference Object Shape

CrossAreaReference contains:

- external_area_id
- external_area_label
- external_resolution_id (nullable)
- external_resolution_label (nullable)
- created_at
- schema_version

This object is informational only.

The Engine must never interpret it as a structural edge unless a future schema version explicitly changes that classification.

---

# 8. Participant Schema

## ENG-DOMAIN-15 — Participant Object

Participant fields:

- participant_id
- session_id
- area_id
- round_index
- display_name
- created_at
- schema_version

Participant is a structural epoch object.

ENG-DOMAIN defines that:

- participant_id must be unique within a session
- participant belongs to exactly one round
- display_name is structurally recorded
- participant identity is session-scoped and epoch-based

Behavioral meaning of removal, resume, and vote eligibility belongs to ENG-SESSION and ENG-DECISION.

---

# 9. Candidate Schema

## ENG-DOMAIN-16 — Candidate Object

Candidate fields:

- candidate_id
- session_id
- area_id
- round_index
- candidate_content
- created_at
- schema_version

ENG-DOMAIN defines Candidate as a round-scoped proposal epoch.

ENG-DOMAIN defines that:

- candidate_id must be unique within its round
- candidate belongs to exactly one round

Behavioral mutability boundaries belong to ENG-SESSION.

---

# 10. Constraint Schema

## ENG-DOMAIN-17 — Constraint Object

Constraint fields:

- constraint_id
- session_id
- area_id
- round_index
- constraint_type
- constraint_parameters
- created_at
- schema_version

ENG-DOMAIN defines Constraint as a round-scoped structural object.

Constraint evaluation semantics are defined in ENG-DECISION.

---

# 11. Vote Schema

## ENG-DOMAIN-18 — Vote Object

Vote fields:

- vote_id
- session_id
- area_id
- round_index
- participant_id
- candidate_id
- stance
- created_at
- schema_version

ENG-DOMAIN defines Vote as a round-bound structural stance object.

ENG-DOMAIN defines that:

- vote_id must be unique
- vote references participant and candidate from the same round
- stance is represented structurally as an enum

Behavioral vote mutability and acceptance meaning belong to ENG-DECISION and ENG-SESSION.

---

# 12. Session Schema

## ENG-DOMAIN-19 — Session Object

Session fields:

- session_id
- area_id
- session_type
- authority_id
- scope_id
- phase
- state
- round_index
- participants
- candidates
- constraints
- votes
- terminal_receipt_id
- cross_area_references (optional)
- annotations (optional)
- created_at
- updated_at
- schema_version

ENG-DOMAIN defines Session as the structural container for one decision process.

ENG-DOMAIN defines only the structural presence of these fields.

Behavioral meaning of:

- session_type
- phase
- state
- round_index mutation
- acceptance
- blocking
- closure

belongs to:

- ENG-SESSION
- ENG-DECISION

Historical round storage rules belong to ENG-RECEIPT.

---

# 13. Resolution Schema

## ENG-DOMAIN-20 — Resolution Object

Resolution fields:

- resolution_id
- area_id
- originating_session_id
- authority_snapshot_id
- scope_snapshot_id
- accepted_candidate_id
- engine_version
- spec_set_hash
- state
- superseded_by
- cross_area_references (optional)
- annotations (optional)
- created_at
- schema_version

ENG-DOMAIN defines Resolution as the persisted governance artifact created by successful session acceptance.

Allowed Resolution lifecycle enum values:

- ACTIVE
- SUPERSEDED
- UNDER_REVIEW
- RETIRED

ENG-DOMAIN defines the structural existence of those states only.

Their meanings are defined in:

- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED

Rule provenance fields:

- engine_version
- spec_set_hash

are structural.

ENG-SPECVERIFY defines their provenance meaning.

---

# 14. Receipt Schema

## ENG-DOMAIN-21 — Receipt Object

Receipt fields:

- receipt_id
- session_id
- resolution_id (nullable)
- receipt_type
- area_id
- engine_version
- spec_set_hash
- authority_snapshot_id
- scope_snapshot_id
- problem_statement (optional)
- rounds
- final_round_index
- session_state_at_close
- acceptance_result
- annotations (optional)
- created_at
- hash_algorithm
- content_hash
- schema_version

ENG-DOMAIN defines the structural field set of Receipt.

Receipt artifact meaning, immutability, and terminal semantics are defined in ENG-RECEIPT.

Canonical hashing mechanics are defined in ENG-CANON.

Rule identity meaning is defined in ENG-SPECVERIFY.

---

# 15. Round Snapshot Schema

## ENG-DOMAIN-22 — Round Snapshot Object

Round snapshot fields:

- round_index
- round_state
- participant_set
- candidate_set
- constraint_set
- vote_set

ENG-DOMAIN defines the structural shape of the round snapshot.

Behavioral meaning of round creation belongs to ENG-SESSION.  
Receipt preservation of rounds belongs to ENG-RECEIPT.

---

# 16. Supersession Encoding

## ENG-DOMAIN-23 — superseded_by Field

Supersession is structurally encoded through:

- superseded_by

ENG-DOMAIN defines only the structural field.

Graph meaning, ACTIVE derivation, and acyclicity belong to ENG-SUPERSESSION.

---

# 17. Deterministic Structural Encoding Preconditions

## ENG-DOMAIN-24 — Domain Objects Must Be Canonicalizable

All structural domain objects must be representable under ENG-CANON.

ENG-DOMAIN requires:

- deterministic field availability
- deterministic enum values
- explicit structural nullability where required
- stable structural ordering prerequisites for contained collections

ENG-CANON defines actual byte-level canonicalization.

---

# 18. Structural Lifecycle Enumerations

## ENG-DOMAIN-25 — Enum Definitions Are Structural

The following enums are structural and schema-governed:

Resolution state:

- ACTIVE
- SUPERSEDED
- UNDER_REVIEW
- RETIRED

Session phase:

- PRE_STANCE
- VOTING
- TERMINAL

Session state:

- ACTIVE
- PAUSED
- BLOCK_TEMPORARY
- BLOCK_PERMANENT
- ACCEPTED
- CLOSED

Receipt type:

- LEGITIMACY
- EXPLORATION

Acceptance result:

- SUCCESS
- ABANDONED

Round state:

- COMPLETED
- FINAL_ACCEPTED
- ABANDONED

Stance:

- ACCEPT
- REJECT
- ABSTAIN

ENG-DOMAIN defines their structural existence only.  
Behavioral meanings belong to the relevant behavioral specifications.

---

# 19. Incremental Compilation Structural Boundary

## ENG-DOMAIN-26 — Domain Supports Historical Structural Comparison

ENG-DOMAIN does not define compilation algorithms.

It does define the structural artifacts that make deterministic historical comparison possible, including:

- session snapshots
- resolution identity
- supersession encoding
- round snapshots
- receipt provenance
- canonicalizable structural fields

Historical ordering and replay semantics belong to ENG-COMPILATION.

---

# 20. Engine Invariants

- all structural objects are schema-versioned
- structural object identity is UUIDv7 where Engine-owned
- structural references are classified explicitly
- cross-area informational references are not structural
- governance object categories are structurally distinct
- Resolution lifecycle values are structural enum values
- Session state and phase values are structural enum values
- Receipt and round snapshot structures are first-class structural artifacts
- rule provenance fields are structural
- domain objects must be canonicalizable under ENG-CANON
- dependent specifications must consume ENG-DOMAIN rather than redefine object schemas

---

# 21. Mental Model

ENG-DOMAIN defines object truth.

It answers:

- what objects exist
- what fields they contain
- which fields are structural
- how structural references are classified
- which enums are legal structurally

It does not answer:

- how sessions behave
- when acceptance succeeds
- how supersession changes ACTIVE derivation
- how UNDER_REVIEW or RETIRED affect usability
- how receipts are canonically hashed
- when the Engine halts

Those belong elsewhere.

ENG-DOMAIN is the schema layer.  
Other specifications must consume it rather than duplicate object definitions.