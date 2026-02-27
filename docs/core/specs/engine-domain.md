# ENG-DOMAIN — Domain Object Schema  
Canonical Engine Object Definitions  
Status: DRAFT (Pre-Freeze, Governance Slots Integrated)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

This document freezes the canonical domain object schemas for the engine.

Its goals are to:

- Ensure determinism across independent implementations.
- Guarantee audit reconstructability.
- Prevent schema drift in multi-language embedding.
- Preserve canonical hash stability.
- Separate structural encoding from behavioral rules.
- Centralize identity generation within the engine.
- Define exclusive governance slot invariants.

Behavioral rules are defined in:

- ENG-DECISION
- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION
- ENG-API
- ENG-INTEGRITY

This document defines structure only.

---

# 2. General Principles

2.1 All persisted domain objects are immutable once written.

2.2 All objects must use canonical JSON encoding with:

- Deterministic field ordering
- Deterministic enum values
- No implicit defaults
- No nullable ambiguity unless explicitly defined

2.3 Optional annotations are allowed but must not affect object identity or hashing.

2.4 Lifecycle state must be encoded using a single enum field — never boolean flags.

2.5 Schema changes require explicit versioning.

2.6 All engine-generated identifiers must be UUID version 7.

- IDs must be generated exclusively by the engine.
- External systems (including CLI) must not inject domain object IDs.
- UUID timestamp components must not influence legitimacy, precedence, or restore behavior.

2.7 area_id is externally provided and treated as opaque.

- The engine does not manage area lifecycle.
- The engine does not validate area ownership.
- area_id exists solely for scoping evaluation.

---

# 3. Governance Slot Model

## 3.1 Exclusive Governance Slots

Each Area contains two exclusive legitimacy slots:

- Authority slot (Resolution objects)
- Scope slot (Scope objects)

Structural invariants:

- At most one ACTIVE Authority per Area.
- At most one ACTIVE Scope per Area.
- Authority and Scope participate in standard supersession rules.
- Slot exclusivity is structural and must be validated at restore.

Multiple ACTIVE objects in either slot is a structural integrity violation.

---

## 3.2 Slot Presence Rules

Authority slot:

- Exactly one ACTIVE Authority is required before governance begins.
- Authority slot may be empty only in UNINITIALIZED state.

Scope slot:

- Scope may be absent only before first Scope definition.
- Once a Scope has been defined, the slot must never return to empty.
- An Area containing regular sessions must have exactly one ACTIVE Scope.

Absence in violation of these rules is a structural integrity failure.

---

## 3.3 Derived Governance State (Not Persisted)

Area governance state is derived, not stored.

Possible derived states:

UNINITIALIZED  
- No ACTIVE Authority exists.

AUTHORITY_DEFINED  
- Exactly one ACTIVE Authority exists.
- No ACTIVE Scope exists.

FULLY_GOVERNED  
- Exactly one ACTIVE Authority exists.
- Exactly one ACTIVE Scope exists.

These states:

- Must never be stored as fields.
- Must be computed during evaluation or restore.
- Are used for invariant validation only.

Restore must validate:

- Slot exclusivity.
- Required presence conditions based on existing sessions.

---

# 4. Enumerations

## 4.1 SessionState

- ACTIVE
- PAUSED
- BLOCK_TEMPORARY
- BLOCK_PERMANENT
- ACCEPTED
- CLOSED

Definitions (structural only):

- BLOCK_TEMPORARY — reversible interruption
- BLOCK_PERMANENT — irreversible governance conflict requiring explicit closure
- ACCEPTED — produced Resolution
- CLOSED — explicitly terminated by user

No additional states permitted.

---

## 4.2 SessionPhase

- PRE_STANCE
- VOTING
- TERMINAL

TERMINAL phase applies only when state is ACCEPTED or CLOSED.

---

## 4.3 ResolutionState

- ACTIVE
- UNDER_REVIEW
- RETIRED
- SUPERSEDED

States are mutually exclusive.

UNDER_REVIEW and RETIRED do not alter supersession edges.

SUPERSEDED is terminal and graph-altering.

---

## 4.4 ScopeState

- ACTIVE
- UNDER_REVIEW
- SUPERSEDED

Scope does not support RETIRED.

---

## 4.5 Stance

- ACCEPT
- REJECT
- ABSTAIN

---

# 5. Session Schema

A Session object must contain:

- session_id (engine-generated UUIDv7)
- area_id (opaque external reference)
- session_type (AUTHORITY | SCOPE | REGULAR)
- authority_id (Resolution reference at creation)
- scope_id (Scope reference at creation, nullable only if governance not yet fully initialized)
- phase (SessionPhase enum)
- state (SessionState enum)
- participants (set of participant IDs — current)
- candidates (list of Candidate objects)
- constraints (list of Constraint objects)
- votes (list of Vote objects)
- annotations (optional)
- created_at (timestamp)
- updated_at (timestamp)
- schema_version (string)

---

## 5.1 Structural Notes

- session_type is immutable.
- authority_id must reference the ACTIVE Authority at creation (unless session_type = AUTHORITY during bootstrap).
- scope_id must reference the ACTIVE Scope at creation for SCOPE and REGULAR sessions.
- participants reflect the current governance set for the session.
- participants may be modified only while phase = PRE_STANCE.
- When phase transitions to VOTING:
  - participant set becomes structurally frozen.
  - candidate set becomes structurally frozen.
- votes are cleared when state transitions to BLOCK_TEMPORARY.
- BLOCK_PERMANENT freezes progression but does not imply TERMINAL.
- ACCEPTED implies phase = TERMINAL.
- CLOSED implies phase = TERMINAL.
- No boolean flags may represent blocking.

---

# 6. Candidate Schema

A Candidate must contain:

- candidate_id (engine-generated UUIDv7, unique within session)
- content (opaque payload)
- created_by (participant ID)
- created_at (timestamp)
- annotations (optional)
- schema_version (string)

Candidates do not exist independently outside sessions.

Candidate objects become part of the immutable Resolution snapshot if accepted.

---

# 7. Vote Schema

A Vote must contain:

- vote_id (engine-generated UUIDv7)
- participant_id
- candidate_id
- stance (Stance enum)
- recorded_at (timestamp)
- schema_version (string)

Structural constraints:

- At most one vote per participant per candidate.
- Votes are cleared upon BLOCK_TEMPORARY.
- Votes immutable once recorded.
- Votes must not be encoded as a mapping (avoid ordering ambiguity).

Votes form part of the acceptance evaluation record.

---

## 7.1 Solo Mode Structural Clarification

In Solo Governance Mode:

- Exactly one participant exists.

If acceptance attempted without a Vote:

- Engine must create implicit Vote object:
  - participant_id = sole participant
  - stance = ACCEPT
  - recorded_at = deterministic timestamp
  - vote_id = engine-generated UUIDv7

There is no acceptance path that bypasses Vote modeling.

---

# 8. Constraint Schema

A Constraint must contain:

- constraint_id (engine-generated UUIDv7)
- required_participants (set of participant IDs)
- created_at (timestamp)
- annotations (optional)
- schema_version (string)

Constraints:

- Immutable once phase = VOTING.
- Do not encode permanence or block type.

Behavior defined in ENG-DECISION.

---

# 9. Resolution Schema

A Resolution must contain:

- resolution_id (engine-generated UUIDv7)
- area_id (opaque external reference)
- originating_session_id (reference)
- authority_snapshot_id (reference)
- scope_snapshot_id (reference)
- participant_snapshot (set of participant IDs at acceptance)
- candidate_snapshot (complete accepted candidate content)
- state (ResolutionState enum)
- superseded_by (resolution_id, nullable)
- created_at (timestamp)
- annotations (optional)
- schema_version (string)

---

## 9.1 Snapshot Requirements

participant_snapshot must:

- Reflect complete participant set at acceptance.
- Be immutable.
- Be independent of later session mutations.

candidate_snapshot must:

- Contain full accepted candidate content.
- Be self-contained.
- Not rely on external references for legitimacy.

In Solo Mode:

- participant_snapshot contains exactly one participant ID.
- Implicit Vote must exist in session prior to snapshot.

---

# 10. Scope Schema

A Scope must contain:

- scope_id (engine-generated UUIDv7)
- area_id (opaque external reference)
- state (ScopeState enum)
- superseded_by (scope_id, nullable)
- created_at (timestamp)
- annotations (optional)
- schema_version (string)

Rules:

- Scope does not support RETIRED.
- superseded_by must be null unless state = SUPERSEDED.

---

# 11. Supersession Encoding

Supersession represented structurally by:

- superseded object containing superseded_by reference
- superseding Resolution referencing originating_session_id

No supersession type field permitted.

Supersession is:

- Explicit
- Permanent
- Graph-altering
- Terminal for superseded object

Supersession may occur only through Resolution creation via session acceptance.

Behavior defined in ENG-SUPERSESSION.

---

# 12. Cross-Object References

Permitted references:

- Session → Authority (Resolution)
- Session → Scope
- Resolution → originating Session
- Resolution → superseded Resolution
- Scope → superseded Scope

Inter-area references must not affect legitimacy.

---

# 13. Deterministic Encoding Requirements

All implementations must ensure:

- Deterministic field ordering
- Deterministic enum string values
- Stable timestamp format
- Stable set ordering (lexicographically sorted)
- Canonical JSON serialization

UUID ordering must never determine legitimacy.

Object identity and hashing must be invariant across:

- Programming languages
- Storage backends
- Operating systems

---

# 14. Schema Versioning

Every persisted object must include:

- schema_version (string)

Rules:

- Backward-incompatible changes require version increment.
- Migrations must be explicit and deterministic.
- Multiple schema versions may coexist.
- No implicit migration allowed.

---

# 15. Audit & Receipt Alignment

Domain objects are primary legitimacy artifacts.

LEGITIMACY receipts must reference:

- resolution_id
- session_id
- authority_snapshot_id
- scope_snapshot_id

Receipts:

- Must not create legitimacy.
- Must not mutate domain objects.
- Are derived artifacts only.

---

# 16. Engine Invariants

- Governance slots are exclusive per Area.
- Governance state is derived, never stored.
- All domain object IDs are engine-generated UUIDv7.
- Lifecycle represented by enum, never flags.
- BLOCK_PERMANENT explicit.
- Participant and candidate mutability restricted to PRE_STANCE.
- Snapshots complete and immutable.
- Supersession structurally explicit.
- Deterministic encoding mandatory.
- Legitimacy must never depend on timestamps or ordering.
- Solo Mode must not bypass Vote modeling.

Violation of this schema breaks cross-system determinism and constitutes critical engine failure.