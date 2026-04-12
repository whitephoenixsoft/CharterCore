# ENG-DOMAIN — Domain Object Schema
Canonical Engine Object Definitions

Status: REFACTORED (v20 – ENG-STRUCTURE / ENG-USABILITY Renaming & ON_HOLD Alignment)  
Applies to: Engine Core (V1/V2+)

Authority: Foundational authority for structural object schemas and structural field classification.

Subordinate references consumed from:

- ENG-CANON
- ENG-SPECVERIFY
- ENG-RECEIPT
- ENG-STRUCTURE
- ENG-USABILITY
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
- reference classes (structural and informational)
- candidate action structure
- canonical structural object completeness requirements

ENG-DOMAIN does not define:

- session lifecycle behavior
- acceptance mechanics
- structural graph meaning
- ON_HOLD / RETIRED usability semantics
- receipt verification policy
- canonical byte-level serialization rules
- runtime halt behavior
- atomic persistence boundaries
- dynamic candidate viability evaluation

Those are defined respectively in:

- ENG-SESSION
- ENG-DECISION
- ENG-STRUCTURE
- ENG-USABILITY
- ENG-INTEGRITY
- ENG-CANON
- ENG-PERSISTENCE
- ENG-API

ENG-DOMAIN defines what objects are.  
Other specifications define what those objects mean behaviorally.

---

# 2. General Principles

## ENG-DOMAIN-01 — Persisted Objects Are Structural Artifacts

Persisted domain objects are structural artifacts.

Once written, they are immutable except as replaced or evolved through lifecycle mechanisms defined elsewhere.

ENG-DOMAIN defines object shape, not mutation procedures.

No consuming specification may reinterpret a persisted object by silently adding omitted structure, deleting recognized structure, or reclassifying a structural field as informational.

---

## ENG-DOMAIN-02 — Canonical Serialization Is Consumed, Not Defined Here

Domain objects must be capable of canonical serialization.

ENG-CANON defines:

- field ordering
- canonical encoding
- hash input rules

ENG-DOMAIN defines:

- which fields exist
- which fields are required
- which fields are structural
- which fields are informational
- which contained collections require deterministic ordering preconditions

Canonical byte production is therefore downstream of domain schema truth.

---

## ENG-DOMAIN-03 — Structural vs Informational Data

A field is structural if it affects any of the following:

- legitimacy evaluation
- governance semantics
- structural graph meaning
- receipt identity
- canonical artifact integrity
- rule provenance
- structural validation
- runtime legitimacy compilation

A field is informational if it does not affect those concerns.

ENG-DOMAIN is the authority for that classification at the schema level.  
ENG-CANON consumes that classification when building canonical hash inputs.

A field may be informational even when it is preserved canonically for completeness or audit portability.  
Informational preservation does not give the field legitimacy semantics.

---

## ENG-DOMAIN-04 — Lifecycle State Is a Single Enum Field

Where lifecycle state exists, it must be represented as a single enum field.

ENG-DOMAIN defines the allowed enum values per object category.  
ENG-STRUCTURE and ENG-USABILITY define the behavioral meaning of those values.

Unknown lifecycle values are structurally invalid.

---

## ENG-DOMAIN-05 — Schema Version Required

Every domain object must include:

- schema_version

Schema version determines structural interpretation only.

Runtime acceptance of schema versions is enforced by ENG-INTEGRITY.

Schema version must be explicit.  
It must not be inferred from file format, transport envelope, API version, or storage layout.

---

## ENG-DOMAIN-06 — Engine-Owned Identity

All Engine-generated identifiers must be UUIDv7 unless explicitly stated otherwise.

Caller-provided structural identifiers are prohibited where the Engine owns identity generation.

ENG-DOMAIN defines identifier form.  
ENG-INTEGRITY enforces restore validity.  
UUID timestamp components must not carry legitimacy meaning.

UUID timestamp components must not determine:

- structural precedence
- replay ordering unless explicitly defined elsewhere for a different artifact
- restore precedence
- acceptance validity

---

## ENG-DOMAIN-07 — Area Identity Is Opaque

area_id is externally provided and opaque.

ENG-DOMAIN defines that it scopes structural objects.  
It does not define Area lifecycle or operational ownership.

Area identity has no semantic decomposition within the Engine.  
No rule may depend on parsing area_id structure.

---

## ENG-DOMAIN-08 — Annotation Is the Canonical User-Facing Explanatory Field

The canonical user-facing explanatory field name is:

- annotation

Alternate UX labels such as:

- notes
- rationale
- comments
- explanation
- background

must normalize to annotation in Engine-facing schemas and APIs.

Annotation is informational only.

Annotation must never affect:

- legitimacy
- acceptance eligibility
- governance semantics
- structural graph meaning
- ACTIVE derivation
- structural validation
- blocking semantics

Where present, annotation follows the lifecycle of its containing object and becomes immutable when that object becomes immutable.

---

# 3. Schema Versioning & Compatibility Doctrine

## ENG-DOMAIN-09 — Major / Minor Compatibility Rules

Schema compatibility rules:

- major version changes may alter structural meaning
- minor version changes must be additive and non-legitimacy-altering
- unknown structural enums or fields are incompatible
- missing required structural fields are incompatible

ENG-DOMAIN defines these compatibility rules.  
ENG-INTEGRITY enforces them during runtime rehydration and restore.

No implementation may silently coerce an incompatible object into compatibility.

---

## ENG-DOMAIN-10 — Unknown Field Handling

Unknown structural fields are not permitted.

Unknown informational fields may be ignored only if the consuming specification allows them to remain informational.

ENG-CANON must not include unknown structural fields in canonical hashing and must fail if they are encountered as structural content.

A field that is unknown to the implementation must not be guessed to be informational merely because it appears optional.

---

# 4. Single-Area Structural Model

## ENG-DOMAIN-11 — Structural Objects Are Area-Scoped

All structural objects in an active runtime graph must share the same area_id.

ENG-DOMAIN defines this as a schema boundary.

ENG-INTEGRITY enforces whether a runtime graph violating this boundary may proceed.

Cross-area references may exist only as informational metadata unless explicitly declared structural by schema.

Structural multi-Area graphs are not supported.

---

# 5. Governance Slot Model

## ENG-DOMAIN-12 — Authority and Scope Are Distinct Structural Slots

Each Area has two governance slot categories:

- Authority
- Scope

ENG-DOMAIN defines the object categories and slot participation.

ENG-DOMAIN does not define:

- ACTIVE derivation
- slot usability
- halt semantics when slot validity fails

Those belong to:

- ENG-STRUCTURE
- ENG-USABILITY
- ENG-INTEGRITY

Governance slot participation is structural even when slot usability is temporarily suspended by another specification.

---

# 6. Structural vs Informational References

## ENG-DOMAIN-13 — Structural References

Structural references are references whose existence or resolution affects legitimacy or runtime structural validity.

Permitted structural reference categories include:

- Session → Authority Resolution
- Session → Scope Resolution
- Resolution → originating Session
- Resolution → superseded Resolution
- Receipt → Session
- LEGITIMACY Receipt → Resolution
- Candidate action payload → supersession target Resolution(s)
- Candidate action payload → retirement target Resolution

Structural references must remain Area-local unless a schema explicitly states otherwise.

ENG-DOMAIN defines the reference classes.  
ENG-INTEGRITY enforces whether they resolve safely.

Structural references are not optional in effect merely because a field may be nullable in particular object classes.  
If present as structural content, they carry structural resolution requirements.

---

## ENG-DOMAIN-14 — Informational References

Informational references do not affect legitimacy, structural graph meaning, or structural ACTIVE derivation.

They exist for:

- lineage
- traceability
- external linkage
- contextual association
- future graph semantics

They must never:

- affect legitimacy evaluation
- affect ACTIVE derivation
- be interpreted as supersession
- be used for acceptance decisions
- create ordering semantics
- create graph precedence

Informational references may still be preserved canonically if the containing artifact treats them as part of structural completeness for export or audit portability.  
That does not make them graph edges.

---

## ENG-DOMAIN-15 — Cross-Area Informational References

Cross-area references may point to external Areas or Resolutions.

They:

- do not affect legitimacy
- do not affect ACTIVE derivation
- do not affect legitimacy structural graph meaning
- do not affect governance slot occupancy
- must not be traversed by legitimacy logic

They are opaque metadata only.

Cross-area informational references may be retained on Sessions, Candidates, Resolutions, or Receipts for context, but their external targets are not part of the local structural graph.

Cross-area references are unilateral declarations by the local Area and are not required to be reciprocated or validated by external Areas.

---

## ENG-DOMAIN-16 — Intra-Area Informational Resolution References

A Session or Resolution may reference zero or more other Resolutions within the same Area through explicitly informational reference fields.

These references:

- must be Area-local (same area_id)
- must resolve to existing Resolution objects within the Area
- must not contain duplicates
- must be deterministically ordered (lexicographically by resolution_id)

They are informational and must not:

- affect legitimacy
- affect ACTIVE derivation
- affect supersession
- affect acceptance
- introduce ordering semantics
- be interpreted as graph edges in the current specification set

These references exist to support:

- lineage tracking
- contextual relationships
- future DAG semantics

Future specification versions may assign meaning to these references, but no such meaning exists in the current specification set.

Where present on Session objects, these references represent current-round pending informational references that may be frozen into an accepted Resolution according to ENG-SESSION and ENG-DECISION.  
Where present on Resolution objects, they represent the informational references carried by the accepted artifact itself.

---

# 7. CrossAreaReference Schema

## ENG-DOMAIN-17 — CrossAreaReference Object Shape

CrossAreaReference contains:

- external_area_id (required)
- external_area_label (required)
- external_resolution_id (nullable)
- external_resolution_label (nullable)
- relationship (required)

Where relationship is an enum defined by ENG-DOMAIN and may be extended in future schema versions. Currently the following is defined:

- derived_from
- affects
- affected_by
- related

The Engine must not derive, infer, or validate any semantic meaning from the relationship field.

This object is informational only. The Engine must not require the existence, accessibility, or validity of external references for evaluation or acceptance.

The Engine must not reinterpret this object as:

- a structural edge
- a structural dependency
- an acceptance prerequisite
- a local governance reference

---

# 8. Participant Schema

## ENG-DOMAIN-18 — Participant Object

Participant fields:

- participant_id
- session_id
- area_id
- round_index
- display_name
- annotation (optional)
- created_at
- schema_version

Participant is a structural epoch object.

ENG-DOMAIN defines that:

- participant_id must be unique within a session
- participant belongs to exactly one round
- display_name is structurally recorded
- participant identity is session-scoped and epoch-based
- annotation is informational only

Behavioral meaning of:

- removal
- resume
- vote eligibility
- freeze timing

belongs to:

- ENG-SESSION
- ENG-DECISION

ENG-INTEGRITY validates epoch safety across restore and receipt alignment.

---

# 9. Candidate Schema

## ENG-DOMAIN-19 — Candidate Object

Candidate fields:

- candidate_id
- session_id
- area_id
- round_index
- candidate_action_type
- candidate_payload
- reversibility_intent
- annotation (optional)
- created_at
- schema_version

Candidate is a structural round-scoped proposal object.

reversibility_intent is informational only and can have only the following values:

- REVERSIBLE
- CONDITIONALLY_REVERSIBLE
- IRREVERSIBLE

ENG-DOMAIN defines that:

- candidate_id must be unique within its round
- candidate belongs to exactly one round
- proposal_text is non-interpreted content
- candidate_action_type is structural
- candidate_payload is structural and must match candidate_action_type
- annotation is informational only
- candidate does not contain a persisted status field
- candidate viability or blocked status is evaluated dynamically outside ENG-DOMAIN

Behavioral meaning of:

- candidate mutability during PRE_STANCE
- candidate freeze at first stance
- candidate eligibility for acceptance
- candidate-level blocking or advisories

belongs to:

- ENG-SESSION
- ENG-DECISION
- ENG-API

---

## ENG-DOMAIN-19A — Candidate Action Payload Rules

Candidate action payload is action-type-specific structural content.

### For ADOPT_RESOLUTION

candidate_payload must contain:

- resolution_content

Rules:

- resolution_content is structural
- no supersession targets may be encoded for this action type
- retirement target must not be encoded for this action type

### For SUPERSEDE_RESOLUTIONS

candidate_payload must contain:

- resolution_content
- supersedes_resolution_ids

Rules:

- supersedes_resolution_ids must be a non-empty ordered list of Resolution IDs
- all target Resolution IDs must resolve locally within the same Area
- all target Resolution IDs must be unique
- supersedes_resolution_ids must be lexicographically ordered
- supersession targets are structural action targets, not informational references

### For RETIRE_RESOLUTION

candidate_payload must contain:

- target_resolution_id

Rules:

- target_resolution_id must resolve locally within the same Area
- target_resolution_id is a structural action target, not an informational reference
- no resolution_content may be required for this action type
- no supersession target list may be encoded for this action type

ENG-DOMAIN defines shape and classification only.  
Execution meaning belongs to ENG-DECISION, ENG-STRUCTURE, and ENG-USABILITY.

---

# 10. Constraint Schema

## ENG-DOMAIN-20 — Constraint Object

Constraint fields:

- constraint_id
- session_id
- area_id
- round_index
- constraint_type
- constraint_parameters
- annotation (optional)
- created_at
- schema_version

Constraint is a structural round-scoped object.

ENG-DOMAIN defines that:

- constraint_id must be unique within the applicable session context
- constraint belongs to exactly one round
- constraint structure must be preserved exactly as recorded
- annotation is informational only

Constraint meaning and evaluation are defined in ENG-DECISION.  
Constraint mutability boundaries are defined in ENG-SESSION.

---

# 11. Vote Schema

## ENG-DOMAIN-21 — Vote Object

Vote fields:

- vote_id
- session_id
- area_id
- round_index
- participant_id
- candidate_id
- stance
- annotation (optional)
- created_at
- schema_version

Vote is a structural round-bound stance object.

ENG-DOMAIN defines that:

- vote_id must be unique
- vote references participant and candidate from the same round
- stance is represented structurally as an enum
- vote is bound to exactly one session and one round
- annotation is informational only

Behavioral vote mutability and vote counting semantics belong to:

- ENG-SESSION
- ENG-DECISION

---

# 12. Session Schema

## ENG-DOMAIN-22 — Session Object

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
- internal_resolution_references (optional)
- cross_area_references (optional)
- terminal_receipt_id
- annotation (optional)
- created_at
- updated_at
- schema_version

ENG-DOMAIN defines Session as the structural container for one decision process.

ENG-DOMAIN defines only the structural presence and classification of these fields.

Session object rules include:

- authority_id and scope_id are structural governance snapshot references
- phase is a structural enum
- state is a structural enum
- round_index is structural and must be explicit
- participants, candidates, constraints, and votes represent the active runtime round only
- internal_resolution_references, if present, represent the current round’s informational same-Area Resolution reference set
- cross_area_references, if present, represent the current round’s informational cross-Area reference set
- terminal_receipt_id is required when the session is terminal according to governing lifecycle specifications
- annotation is informational only

ENG-DOMAIN does not define:

- whether session creation is allowed
- when phase changes
- when state changes
- when round_index increments
- when references freeze
- whether a session may accept

Those belong to:

- ENG-SESSION
- ENG-DECISION
- ENG-INTEGRITY

Historical round storage rules belong to ENG-RECEIPT.

---

# 13. Resolution Schema

## ENG-DOMAIN-23 — Resolution Object

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
- internal_resolution_references (optional)
- cross_area_references (optional)
- resolution_content
- annotation (optional)
- created_at
- schema_version
- **reversibility_intent** — enum("REVERSIBLE", "CONDITIONALLY_REVERSIBLE", "IRREVERSIBLE")

resolution_content is the substantive content of the resolution. It is considered structural but non-interpreted. It is free form text that contains the accepted result from the session candidate.

Reversibility intent is a non-structural, informational field expressing the author’s intent regarding how the Resolution is expected to be treated over time. This field is advisory only and must not influence structural validation, decision evaluation, acceptance eligibility, supersession capability, or any form of legitimacy determination. All Resolutions remain mechanically supersedable regardless of this value.

ENG-DOMAIN defines Resolution as the persisted governance artifact created by successful session acceptance.

Allowed Resolution lifecycle enum values:

- ACTIVE
- SUPERSEDED
- ON_HOLD
- RETIRED

Resolution rules include:

- originating_session_id is structural and must resolve locally
- authority_snapshot_id and scope_snapshot_id are structural governance snapshot references
- accepted_candidate_id is structural and records which candidate was accepted
- superseded_by is structural when present
- engine_version is structural rule provenance
- spec_set_hash is structural rule provenance
- internal_resolution_references, if present, are informational same-Area Resolution references
- cross_area_references, if present, are informational cross-Area references
- annotation is informational only

internal_resolution_references:

- list of resolution_id values
- must reference existing Resolution objects in the same Area
- must be unique
- must be lexicographically ordered
- are informational only

cross_area_references:

- contain CrossAreaReference objects
- are informational only
- must never be interpreted as structural edges

State semantics are defined behaviorally in:

- ENG-STRUCTURE
- ENG-USABILITY

Rule provenance meaning is defined in ENG-SPECVERIFY.

---

# 14. Receipt Schema

## ENG-DOMAIN-24 — Receipt Object

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
- resolution_content
- annotation (optional)
- created_at
- hash_algorithm
- content_hash
- schema_version

ENG-DOMAIN defines the structural field set of Receipt.

Receipt rules include:

- receipt_id is structural identity
- session_id is a structural local reference
- resolution_id is required for LEGITIMACY receipts and null for EXPLORATION receipts
- engine_version and spec_set_hash are structural provenance fields
- rounds is a structural ordered snapshot sequence
- final_round_index is structural
- session_state_at_close is structural
- resolution_content is structural
- hash_algorithm and content_hash are structural integrity fields
- annotation is informational only

Receipt artifact meaning, immutability, and terminal semantics are defined in ENG-RECEIPT.  
Canonical hashing mechanics are defined in ENG-CANON.  
Rule identity meaning is defined in ENG-SPECVERIFY.

---

# 15. Round Snapshot Schema

## ENG-DOMAIN-25 — Round Snapshot Object

Round snapshot fields:

- round_index
- round_state
- participant_set
- candidate_set
- constraint_set
- vote_set
- internal_resolution_references
- cross_area_references

ENG-DOMAIN defines the structural shape of the round snapshot.

Round snapshot rules include:

- round_index is explicit and structural
- round_state is a structural enum
- participant_set is a full structural snapshot, not a diff
- candidate_set is a full structural snapshot, not a diff
- constraint_set is a full structural snapshot, not a diff
- vote_set is a full structural snapshot, not a diff

Behavioral meaning of round creation belongs to ENG-SESSION.  
Receipt preservation of rounds belongs to ENG-RECEIPT.

If future specifications require explicit snapshotting of informational reference sets, they must extend this schema explicitly rather than relying on inference.

---

# 16. Structural Edge Encoding

## ENG-DOMAIN-26 — superseded_by Field

The current structural Resolution-to-Resolution graph is encoded through:

- superseded_by

ENG-DOMAIN defines only the field and its structural role.

Graph meaning, ACTIVE derivation, conflict handling, and acyclicity belong to ENG-STRUCTURE.

Rules:

- superseded_by is structural when present
- superseded_by must reference a local Resolution
- superseded_by must never be inferred from informational references
- superseded_by must not be overloaded to encode lineage or context

---

# 17. Deterministic Structural Encoding Preconditions

## ENG-DOMAIN-27 — Domain Objects Must Be Canonicalizable

All structural domain objects must be representable under ENG-CANON.

ENG-DOMAIN requires:

- deterministic field availability
- deterministic enum values
- explicit structural nullability where required
- deterministic ordering prerequisites for contained collections
- lexicographic ordering for declared ordered reference lists
- no ambiguous structural omission

ENG-CANON defines actual byte-level canonicalization.

---

# 18. Structural Lifecycle and Action Enumerations

## ENG-DOMAIN-28 — Enum Definitions Are Structural

The following enums are structural and schema-governed.

### Resolution state

- ACTIVE
- SUPERSEDED
- ON_HOLD
- RETIRED

### Session phase

- PRE_STANCE
- VOTING
- TERMINAL

### Session state

- ACTIVE
- PAUSED
- BLOCK_TEMPORARY
- BLOCK_PERMANENT
- ACCEPTED
- CLOSED

### Session type

- AUTHORITY
- SCOPE
- REGULAR

### Candidate action type

- ADOPT_RESOLUTION
- SUPERSEDE_RESOLUTIONS
- RETIRE_RESOLUTION

### Receipt type

- LEGITIMACY
- EXPLORATION

### Acceptance result

- SUCCESS
- ABANDONED

### Round state

- COMPLETED
- FINAL_ACCEPTED
- ABANDONED

### Stance

- ACCEPT
- REJECT
- ABSTAIN

ENG-DOMAIN defines the structural existence of these enums only.  
Behavioral meanings belong to the relevant behavioral specifications.

Unknown enum values are structurally invalid.

---

# 19. Incremental Compilation Structural Boundary

## ENG-DOMAIN-29 — Domain Supports Historical Structural Comparison

ENG-DOMAIN does not define compilation algorithms.

It does define the structural artifacts that make deterministic historical comparison possible, including:

- session identity
- candidate identity and action structure
- resolution identity
- structural edge encoding
- round snapshots
- receipt provenance
- canonicalizable structural fields
- explicit lifecycle state recording
- explicit governance snapshot references

Historical ordering and replay semantics belong to ENG-COMPILATION.

Informational references may be preserved in historical artifacts, but they do not alter structural replay meaning in the current specification set unless another authority explicitly states otherwise.

---

# 20. Engine Invariants

- all structural objects are schema-versioned
- structural object identity is UUIDv7 where Engine-owned
- structural references are classified explicitly
- candidate action targets are structural and not informational
- cross-area informational references are not structural
- intra-Area informational Resolution references are not structural
- informational references do not affect legitimacy
- informational references do not affect ACTIVE derivation
- annotation is the canonical user-facing explanatory field
- annotation is informational only
- governance object categories are structurally distinct
- Resolution lifecycle values are structural enum values
- Session state and phase values are structural enum values
- Session type values are structural enum values
- Candidate action type values are structural enum values
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
- which fields are informational
- how structural references are classified
- how informational references are classified
- how candidate action payloads are shaped
- which enums are structurally legal

It does not answer:

- how sessions behave
- when acceptance succeeds
- how structural graph meaning changes ACTIVE derivation
- how ON_HOLD or RETIRED affect usability
- how candidate viability is evaluated
- how receipts are canonically hashed
- when the Engine halts

ENG-DOMAIN is the schema layer.  
Other specifications must consume it rather than duplicate object definitions.