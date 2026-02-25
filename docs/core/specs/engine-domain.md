# ENG-DOMAIN — Domain Object Schema  
Canonical Engine Object Definitions  
Status: DRAFT (Pre-Freeze, Solo Mode Clarified)  
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

Behavioral rules are defined in:

- ENG-DECISION
- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION
- ENG-API

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

# 3. Enumerations

## 3.1 SessionState

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

No additional states are permitted.

---

## 3.2 SessionPhase

- PRE_STANCE
- VOTING
- TERMINAL

Phase is distinct from state.

TERMINAL phase applies only when state is ACCEPTED or CLOSED.

---

## 3.3 ResolutionState

- ACTIVE
- UNDER_REVIEW
- RETIRED
- SUPERSEDED

States are mutually exclusive.

UNDER_REVIEW and RETIRED do not alter supersession edges.

SUPERSEDED is terminal and graph-altering.

---

## 3.4 ScopeState

- ACTIVE
- UNDER_REVIEW
- SUPERSEDED

Scope does not support RETIRED.

---

## 3.5 Stance

- ACCEPT
- REJECT
- ABSTAIN

---

# 4. Session Schema

A Session object must contain:

- session_id (engine-generated UUIDv7)
- area_id (opaque external reference)
- authority_id (Resolution reference at creation)
- scope_id (Scope reference at creation)
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

## 4.1 Structural Notes

- participants reflect the current governance set for the session.
- participants may be modified only while phase = PRE_STANCE.
- When phase transitions to VOTING:
  - participant set becomes structurally frozen.
  - candidate set becomes structurally frozen.
- votes are cleared when state transitions to BLOCK_TEMPORARY.
- BLOCK_PERMANENT does not clear history; it freezes progression.
- ACCEPTED implies phase = TERMINAL.
- CLOSED implies phase = TERMINAL.
- BLOCK_PERMANENT must not imply TERMINAL phase until explicitly CLOSED.
- No boolean flags may represent blocking.

Participant mutability rules are phase-bound, not state-bound.

---

# 5. Candidate Schema

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

# 6. Vote Schema

A Vote must contain:

- vote_id (engine-generated UUIDv7)
- participant_id
- candidate_id
- stance (Stance enum)
- recorded_at (timestamp)
- schema_version (string)

Rules (structural constraints):

- At most one active vote per participant per candidate.
- Votes are cleared upon BLOCK_TEMPORARY.
- Votes become immutable once recorded.
- Votes must not be encoded as a mapping (avoid ordering ambiguity).
- Votes form part of the acceptance evaluation record.

Votes are domain objects and must remain deterministic across implementations.

---

## 6.1 Solo Mode Vote Semantics (Structural Clarification)

In Solo Governance Mode (V1):

- Exactly one participant exists in the session.
- Acceptance may occur without a prior explicit Vote object.

If acceptance is attempted and no Vote exists:

- The engine must create an implicit Vote object:
  - participant_id = sole participant
  - stance = ACCEPT
  - recorded_at = deterministic timestamp
  - vote_id = engine-generated UUIDv7
- That Vote becomes part of the session’s immutable vote list.

This preserves:

- Structural consistency
- Deterministic evaluation
- Snapshot completeness

There is no alternative acceptance path that bypasses Vote modeling.

---

# 7. Constraint Schema

A Constraint must contain:

- constraint_id (engine-generated UUIDv7)
- required_participants (set of participant IDs)
- created_at (timestamp)
- annotations (optional)
- schema_version (string)

Constraints:

- Immutable once session phase = VOTING.
- Do not encode permanence or block type.
- Are structurally neutral objects.

Behavior defined in ENG-DECISION.

---

# 8. Resolution Schema

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

## 8.1 Snapshot Requirements

participant_snapshot must:

- Reflect the complete participant set at the moment of acceptance.
- Be immutable.
- Be independent of any later session mutations.

candidate_snapshot must:

- Contain the full accepted candidate content.
- Be complete and self-contained.
- Not rely on external references for legitimacy.

In Solo Mode:

- participant_snapshot contains exactly one participant ID.
- If an implicit Vote was created, it must exist in the originating session record prior to snapshot.

---

# 9. Scope Schema

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

# 10. Supersession Encoding

Supersession is represented structurally by:

- superseded object containing superseded_by reference
- superseding Resolution referencing originating_session_id

No supersession type field permitted.

Supersession is:

- Explicit
- Permanent
- Graph-altering
- Terminal for superseded object

Behavior defined in ENG-SUPERSESSION.

Supersession may occur only as a consequence of Resolution creation via session acceptance.

---

# 11. Cross-Object References

Permitted references:

- Session → Authority (Resolution)
- Session → Scope
- Resolution → originating Session
- Resolution → superseded Resolution
- Scope → superseded Scope

Sessions may reference objects in other Areas for informational purposes only.

Legitimacy evaluation must consider only:

- Authority
- Scope
- Referenced Resolution (if applicable)

Inter-area references must not affect legitimacy.

---

# 12. Deterministic Encoding Requirements

All implementations must ensure:

- Deterministic field ordering
- Deterministic enum string values
- Stable timestamp format
- Stable set ordering (lexicographically sorted)
- Canonical JSON serialization

Object identity and hashing must be invariant across:

- Programming languages
- Storage backends
- Operating systems

UUID ordering must never determine legitimacy.

---

# 13. Schema Versioning

Every persisted object must include:

- schema_version (string)

Rules:

- Backward-incompatible changes require version increment.
- Migrations must be explicit and deterministic.
- Multiple schema versions may coexist.
- No implicit migration allowed.

---

# 14. Audit & Receipt Alignment

Domain objects are primary legitimacy artifacts.

Audit exists for accountability and traceability, not legitimacy creation.

LEGITIMACY receipts must reference:

- resolution_id
- session_id
- authority_snapshot_id
- scope_snapshot_id

REVIEW and EXPLORATION receipts:

- May reference objects
- Must not create legitimacy
- Must not mutate domain objects

Receipts are derived artifacts only.

---

# 15. Engine Invariants

- All domain object IDs are engine-generated UUIDv7.
- No object may encode conflicting states.
- Lifecycle must be represented by enum, not flags.
- BLOCK_PERMANENT is a valid session state and must be explicit.
- Participant and candidate mutability is restricted to phase = PRE_STANCE.
- Snapshots must be complete and immutable.
- Supersession must be structurally explicit.
- Deterministic encoding is mandatory across implementations.
- Legitimacy must never depend on timestamps, ordering, or external system behavior.
- Solo Mode acceptance must not bypass vote modeling; implicit votes are structural.

Violation of this schema breaks cross-system determinism and is considered a critical engine failure.