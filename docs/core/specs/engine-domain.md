# ENG-DOMAIN — Domain Object Schema
Canonical Engine Object Definitions

Status: FROZEN (v15 – Resolution Provenance & Candidate Epoch Alignment)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

This document freezes the canonical domain object schemas for the Engine Core.

Goals:

- Ensure determinism across independent implementations
- Guarantee audit reconstructability
- Preserve canonical hash stability
- Prevent schema drift in multi-language embedding
- Separate structural encoding from behavioral rules
- Centralize identity generation within the engine
- Define exclusive governance slot invariants
- Distinguish structural references from informational cross-area references
- Formalize Area-local structural boundaries
- Integrate receipt integrity and degraded mode semantics
- Formalize session-scoped participation epochs
- Formalize round-scoped candidate epochs
- Define forward and backward compatibility guarantees
- Integrate round-segmented receipt verification
- Bind legitimacy artifacts to the rule system that produced them
- Define the complete canonical structural object model

Behavioral rules are defined in:

- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-API
- ENG-INTEGRITY
- ENG-ERROR
- ENG-RECEIPT
- ENG-AUD
- ENG-CANON
- ENG-SPECVERIFY

This document defines structure only.

---

# 2. General Principles

## 2.1 Immutability

All persisted domain objects are immutable once written.

Objects may only be replaced by new objects via engine-controlled lifecycle operations.

---

## 2.2 Canonical Serialization

All objects must be serialized according to ENG-CANON.

Requirements:

- Deterministic field ordering
- Deterministic enum string values
- Canonical UTF-8 JSON encoding
- No implicit defaults
- Explicit nullability

Canonical serialization is required for:

- Receipt hashing
- Deterministic restore validation

---

## 2.3 Informational vs Structural Data

Optional annotations and informational references may exist but must not affect structural legitimacy unless explicitly declared structural.

Structural fields affect:

- legitimacy evaluation
- governance semantics
- receipt identity
- rule provenance
- canonical hash integrity

Rule identity fields (`engine_version`, `spec_set_hash`) are structural.

---

## 2.4 Lifecycle State

Lifecycle state must be represented as a single enum field.

---

## 2.5 Schema Versioning

Every domain object must include:

schema_version

The schema version determines structural interpretation.

---

## 2.6 Identifier Generation

All engine-generated identifiers must be UUIDv7.

Rules:

- Engine exclusively generates IDs
- Caller-provided IDs prohibited
- UUID timestamp components must not influence legitimacy, precedence, or restore

---

## 2.7 Area Identity

area_id is externally provided and opaque.

The Engine:

- does not manage Area lifecycle
- does not interpret Area identity

area_id is used solely for scoping evaluation.

---

# 3. Schema Versioning & Compatibility Doctrine

## 3.1 Schema Version Field

Every domain object must include schema_version.

---

## 3.2 Major Version Compatibility

If an Engine encounters an object whose major schema version exceeds the supported major version:

- Rehydration must fail
- Error classification must be UNSUPPORTED_SCHEMA_VERSION
- Degraded mode is not permitted

Major version increments may introduce:

- New enums
- Structural fields
- Canonicalization rule changes
- Legitimacy semantics changes

---

## 3.3 Minor Version Compatibility

Minor versions must be strictly additive.

Permitted:

- Informational metadata
- Annotation fields
- Non-legitimacy metadata

Minor versions must not:

- Change structural meaning
- Add enum values affecting behavior
- Change canonical ordering
- Change supersession semantics
- Change legitimacy evaluation

Enum expansion requires a major version bump.

---

## 3.4 Unknown Enum Handling

Unknown enum values are structural violations.

If encountered:

- Rehydration must fail
- Error classification: UNSUPPORTED_SCHEMA_VERSION

Unknown enums must never be ignored.

---

## 3.5 Unknown Field Handling

Unknown fields are handled as follows:

Unknown structural fields → UNSUPPORTED_SCHEMA_VERSION  
Unknown informational fields → ignored

A field is structural if it affects:

- Legitimacy
- Supersession
- Governance slot evaluation
- ACTIVE derivation
- Canonical receipt validation
- Rule provenance

Canonical hashing must include only recognized structural fields.

---

## 3.6 Backward Compatibility

Engines must accept earlier minor versions within the same major version.

Missing required fields must cause restore failure.

No implicit defaults are permitted.

---

# 4. Single-Area Structural Model

The Engine operates on exactly one Area at runtime.

Requirements:

- All structural objects share the same area_id
- Mixed-area graphs are invalid
- Supersession is Area-local
- Governance slot evaluation is Area-local

Cross-area references are informational only.

---

# 5. Governance Slot Model

Each Area contains two exclusive governance slots:

Authority Slot  
Scope Slot

Rules:

- At most one ACTIVE Authority Resolution
- At most one ACTIVE Scope Resolution
- Slot exclusivity validated at restore

Violations cause StructuralIntegrityFailure.

---

# 6. Structural vs Informational References

## 6.1 Structural References

Structural references affect legitimacy and must resolve during restore.

Permitted:

Session → Authority Resolution  
Session → Scope Resolution  
Resolution → originating Session  
Resolution → superseded Resolution  
Receipt → Session  
Receipt → Resolution (LEGITIMACY receipts only)

Structural references must remain Area-local.

---

## 6.2 Informational Cross-Area References

Cross-area references:

- May reference external Areas or Resolutions
- Must not affect legitimacy
- Must not affect ACTIVE derivation
- Must not trigger restore failure

They are opaque metadata.

---

# 7. CrossAreaReference Schema

Fields:

external_area_id  
external_area_label  
external_resolution_id (nullable)  
external_resolution_label (nullable)  
created_at  
schema_version

Rules:

- Engine never dereferences them
- Labels are immutable snapshots
- They do not affect legitimacy

---

# 8. Participant Schema

Participants represent participation epochs within a specific round.

Fields:

participant_id  
session_id  
area_id  
round_index  
display_name  
created_at  
schema_version

Rules:

- participant_id unique within the session
- participant_id never reused
- Participant belongs to exactly one round
- Removal terminates the epoch
- Resume generates new participant epochs

display_name:

- Legitimacy-bearing
- Must be unique among active participants within a round
- Frozen in receipt round snapshots

Participant identity is session-scoped and round-epoch-based.

---

# 9. Candidate Schema

Candidates represent proposals introduced during a specific round.

Fields:

candidate_id  
session_id  
area_id  
round_index  
candidate_content  
created_at  
schema_version

Rules:

- candidate belongs to exactly one round
- candidate_id unique within the round
- candidate_id never reused within that round
- candidate content immutable once VOTING begins

Candidate identity is round-scoped proposal epochs.

---

# 10. Constraint Schema

Constraints represent mechanical acceptance conditions.

Fields:

constraint_id  
session_id  
area_id  
round_index  
constraint_type  
constraint_parameters  
created_at  
schema_version

Rules:

- Constraints belong to exactly one round
- Constraints immutable after VOTING begins
- Constraint evaluation defined in ENG-DECISION

---

# 11. Vote Schema

Votes represent participant stances on candidates.

Fields:

vote_id  
session_id  
area_id  
round_index  
participant_id  
candidate_id  
stance (ACCEPT | REJECT | ABSTAIN)  
created_at  
schema_version

Rules:

- vote_id unique
- Vote references participant and candidate from same round
- One vote per participant per candidate
- Votes immutable once recorded
- Votes never cross round boundaries

---

# 12. Session Schema

Fields:

session_id  
area_id  
session_type (AUTHORITY | SCOPE | REGULAR)

authority_id  
scope_id (nullable during bootstrap)

phase (SessionPhase)  
state (SessionState)

round_index

participants  
candidates  
constraints  
votes

terminal_receipt_id (nullable)

cross_area_references (optional)  
annotations (optional)

created_at  
updated_at  
schema_version

Rules:

ACTIVE session → terminal_receipt_id must be null

ACCEPTED session → terminal_receipt_id references LEGITIMACY receipt

CLOSED session → terminal_receipt_id references EXPLORATION receipt

round_index represents the current active round.

Collections contain objects belonging to the active round only.

Historical rounds are preserved exclusively inside receipts.

---

# 13. Resolution Schema

Resolutions are minimal legitimacy artifacts.

Fields:

resolution_id  
area_id  
originating_session_id  
authority_snapshot_id  
scope_snapshot_id  
accepted_candidate_id  
engine_version  
spec_set_hash  
state (ACTIVE | SUPERSEDED)  
superseded_by (nullable)  
cross_area_references (optional)  
created_at  
annotations (optional)  
schema_version

Rules:

- Created only through session acceptance
- Represents the accepted candidate
- Supersession must remain Area-local

engine_version and spec_set_hash record the rule context under which the resolution was created.

Participant and vote history is not stored in the Resolution.

That history exists only in the receipt.

---

# 14. Receipt Schema

Receipts are first-class domain objects.

Fields:

receipt_id  
session_id  
resolution_id (nullable)  
receipt_type (LEGITIMACY | EXPLORATION)  
area_id  
engine_version  
spec_set_hash  
authority_snapshot_id  
scope_snapshot_id  
problem_statement (optional)  
rounds  
final_round_index  
session_state_at_close  
acceptance_result  
annotations (optional)  
created_at  
hash_algorithm  
content_hash  
schema_version

Rules:

LEGITIMACY receipts:

- resolution_id required
- resolution.originating_session_id must equal receipt.session_id

EXPLORATION receipts:

- resolution_id must be null

Round sequence must be contiguous.

Receipt content_hash must match canonical serialization defined in ENG-CANON.

engine_version and spec_set_hash together define the rule context under which the session was evaluated.

spec_set_hash must match the specification manifest embedded in the Engine binary that produced the receipt.

spec_set_hash is a structural field and participates in canonical serialization used to compute content_hash.

Changing spec_set_hash must produce a different content_hash.

This ensures that receipt integrity binds the legitimacy artifact to the exact rule system that evaluated it.

---

# 15. Round Snapshot Schema

Each round snapshot contains:

round_index  
round_state (COMPLETED | FINAL_ACCEPTED | ABANDONED)  
participant_set  
candidate_set  
constraint_set  
vote_set

Rules:

- Sets represent full structural state of that round
- No diffs permitted
- Ordering deterministic
- Sets ordered lexicographically by UUID

---

# 16. Supersession Encoding

Supersession represented via:

superseded_by

Rules:

- Immutable
- Area-local
- Applied only through session acceptance
- Never cross-area

---

# 17. Deterministic Encoding Requirements

All canonical serialization must follow ENG-CANON.

Requirements:

- Deterministic field ordering
- Deterministic enum values
- Lexicographic ordering of sets
- Canonical UTF-8 JSON encoding

Canonical hashing includes only recognized structural fields.

Unknown informational fields excluded.

---

# 18. Engine Invariants

- Exactly one Area active at runtime
- Domain graph contains one area_id
- Governance slots exclusive
- Structural references resolve locally
- Cross-area references informational only
- IDs engine-generated UUIDv7
- participant_id never reused
- candidate_id never reused within a round
- Vote and constraint objects bound to a single round
- terminal_receipt_id required for terminal sessions
- LEGITIMACY receipts bind to Resolution
- spec_set_hash must match the spec manifest of the Engine that emitted the receipt
- Round indices contiguous
- Deterministic encoding mandatory
- Legitimacy independent of timestamps or ordering

Violation of this schema constitutes critical Engine failure.
