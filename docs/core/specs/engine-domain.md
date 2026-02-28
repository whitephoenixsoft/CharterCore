# ENG-DOMAIN — Domain Object Schema  
Canonical Engine Object Definitions  
Status: DRAFT (Pre-Freeze, Cross-Area References & Single-Area Runtime Integrated)  
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
- Distinguish structural references from informational cross-area references.
- Formalize Area-local structural boundaries.

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

2.3 Optional annotations and informational references are allowed but must not affect object identity or hashing unless explicitly declared.

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
- area_id does not imply multi-area hosting within a single engine instance.

---

# 3. Single-Area Structural Model

The Engine operates on exactly one Area at a time.

Structural implications:

- A domain graph loaded into the Engine must contain objects belonging to exactly one area_id.
- Objects from multiple Areas must never coexist within a single in-memory domain graph.
- Structural validation assumes a single Area context.
- Governance slot evaluation is Area-local.
- Supersession graph reconstruction is Area-local.

Cross-area references do not constitute multi-Area hosting.

They are informational metadata only.

Multi-Area orchestration is the responsibility of the host (e.g., CLI), not the Engine Core.

---

# 4. Governance Slot Model

## 4.1 Exclusive Governance Slots

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

# 5. Structural vs Informational References

## 5.1 Structural References

Structural references are references that:

- Affect legitimacy
- Affect ACTIVE derivation
- Affect supersession graph
- Must resolve during restore

Structural references must always reference objects within the same area_id.

Permitted structural references:

- Session → Authority (Resolution)
- Session → Scope
- Resolution → originating Session
- Resolution → superseded Resolution
- Scope → superseded Scope

Missing structural references constitute integrity failure.

Cross-area structural references are prohibited.

---

## 5.2 Informational Cross-Area References

Cross-area references are informational only.

They may reference:

- External Areas
- External Resolutions in other Areas

They must not:

- Affect legitimacy
- Affect supersession
- Affect ACTIVE derivation
- Trigger restore failure if unresolved
- Be treated as ORPHAN or MISSING_REFERENCE
- Be interpreted as structural references

They are opaque metadata.

---

# 6. CrossAreaReference Schema

A CrossAreaReference must contain:

- external_area_id (opaque identifier, UUIDv7 format recommended)
- external_area_label (string, human-readable snapshot)
- external_resolution_id (nullable, opaque identifier)
- external_resolution_label (nullable string, human-readable snapshot)
- created_at (timestamp)
- schema_version (string)

Rules:

- external identifiers are not dereferenced by the engine.
- Labels are snapshots and must not be validated or canonicalized.
- Engine must not attempt to update labels.
- Absence of referenced Area or Resolution must not alter engine behavior.
- CrossAreaReference objects are immutable.
- CrossAreaReference must not be interpreted as belonging to the active Area.

CrossAreaReference objects are informational only and excluded from structural integrity validation.

---

# 7. Session Schema

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
- cross_area_references (optional list of CrossAreaReference)
- annotations (optional)
- created_at (timestamp)
- updated_at (timestamp)
- schema_version (string)

Constraints:

- All structural references must share the same area_id as the Session.
- Cross-area references must not alter evaluation.

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
- cross_area_references (optional list of CrossAreaReference)
- created_at (timestamp)
- annotations (optional)
- schema_version (string)

Constraints:

- superseded_by must reference a Resolution within the same area_id.
- Cross-area references are informational only.

---

# 9. Supersession Encoding

Supersession represented structurally by:

- superseded object containing superseded_by reference
- superseding Resolution referencing originating_session_id

Supersession is strictly Area-local.

Cross-area references must never participate in supersession.

Supersession is:

- Explicit
- Permanent
- Graph-altering
- Terminal for superseded object

Supersession may occur only through Resolution creation via session acceptance.

---

# 10. Deterministic Encoding Requirements

All implementations must ensure:

- Deterministic field ordering
- Deterministic enum string values
- Stable timestamp format
- Stable set ordering (lexicographically sorted)
- Canonical JSON serialization

Cross-area references must be serialized deterministically but excluded from legitimacy computation.

UUID ordering must never determine legitimacy.

Object identity and hashing must be invariant across:

- Programming languages
- Storage backends
- Operating systems

---

# 11. Schema Versioning

Every persisted object must include:

- schema_version (string)

Rules:

- Backward-incompatible changes require version increment.
- Migrations must be explicit and deterministic.
- Multiple schema versions may coexist.
- No implicit migration allowed.

---

# 12. Engine Invariants

- Engine operates on exactly one Area at a time.
- Domain graph loaded into Engine must contain a single area_id.
- Governance slots are exclusive per Area.
- Governance state is derived, never stored.
- Structural references must resolve and must be Area-local.
- Cross-area references must not be validated for existence.
- All domain object IDs are engine-generated UUIDv7.
- Lifecycle represented by enum, never flags.
- BLOCK_PERMANENT explicit.
- Participant and candidate mutability restricted to PRE_STANCE.
- Snapshots complete and immutable.
- Supersession structurally explicit and Area-local.
- Deterministic encoding mandatory.
- Legitimacy must never depend on timestamps or ordering.
- Solo Mode must not bypass Vote modeling.

Violation of this schema breaks cross-system determinism and constitutes critical engine failure.