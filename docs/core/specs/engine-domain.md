# ENG-DOMAIN — Domain Object Schema  
Canonical Engine Object Definitions  
Status: DRAFT (Pre-Freeze)  
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

Behavioral rules are defined in:

- ENG-DECISION
- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION

This document defines structure only.

---

# 2. General Principles

2.1 All persisted domain objects are immutable once written.

2.2 All objects must use canonical JSON encoding with:

- Deterministic field ordering
- Deterministic enum values
- No implicit defaults
- No nullable ambiguity unless explicitly defined

2.3 Optional annotations are allowed but must not affect object identity.

2.4 Lifecycle state must be encoded using a single enum field — never boolean flags.

2.5 Schema changes require explicit versioning.

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

- session_id (engine-assigned unique ID)
- area_id (reference)
- authority_id (Resolution reference at creation)
- scope_id (Scope reference at creation)
- phase (SessionPhase enum)
- state (SessionState enum)
- participants (set of participant IDs — current)
- candidates (list of candidate objects)
- constraints (list of constraint objects)
- votes (list of Vote objects)
- annotations (optional)
- created_at (timestamp)
- updated_at (timestamp)
- schema_version (string)

Structural Notes:

- participants reflect current governance set.
- votes are cleared when state transitions to BLOCK_TEMPORARY.
- BLOCK_PERMANENT does not clear history; it freezes progression.
- ACCEPTED implies phase = TERMINAL.
- CLOSED implies phase = TERMINAL.
- BLOCK_PERMANENT must not imply TERMINAL phase until explicitly CLOSED.
- No boolean flags may represent blocking.

---

# 5. Candidate Schema

A Candidate must contain:

- candidate_id (unique within session)
- content (opaque payload)
- created_by (participant ID)
- created_at (timestamp)
- annotations (optional)
- schema_version (string)

Candidates do not exist independently outside sessions.

---

# 6. Vote Schema

A Vote must contain:

- participant_id
- candidate_id
- stance (Stance enum)
- recorded_at (timestamp)
- schema_version (string)

Rules (structural constraints):

- At most one active vote per participant per candidate.
- Votes are cleared upon BLOCK_TEMPORARY.
- Votes become immutable once session state = ACCEPTED.
- Votes must not be encoded as a mapping (avoid ordering ambiguity).

---

# 7. Constraint Schema

A Constraint must contain:

- constraint_id
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

- resolution_id (engine-assigned unique ID)
- area_id (reference)
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

Structural Rules:

- state must be exactly one ResolutionState.
- superseded_by must be null unless state = SUPERSEDED.
- UNDER_REVIEW and RETIRED must not modify supersession edges.
- Snapshots must be complete and immutable.

---

# 9. Scope Schema

A Scope must contain:

- scope_id (engine-assigned unique ID)
- area_id (reference)
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

- superseded object containing `superseded_by` reference
- superseding Resolution referencing `originating_session_id`

No supersession type field permitted.

Supersession is:

- Explicit
- Permanent
- Graph-altering
- Terminal for superseded object

Behavior defined in ENG-SUPERSESSION.

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

All domain objects must be reconstructible from audit logs.

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

- No object may encode conflicting states.
- Lifecycle must be represented by enum, not flags.
- BLOCK_PERMANENT is a valid session state and must be explicit.
- Snapshots must be complete and immutable.
- Supersession must be structurally explicit.
- Deterministic encoding is mandatory across implementations.

Violation of this schema breaks cross-system determinism and is considered a critical engine failure.