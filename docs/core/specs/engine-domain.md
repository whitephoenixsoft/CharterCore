# ENG-DOMAIN — Domain Object Schema (Rewritten v10)  
Canonical Engine Object Definitions  
Status: FROZEN (v10 – Forward Compatibility & Schema Evolution Integrated)  
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
- Formalize session-scoped Participant epoch identity  
- Define forward and backward compatibility guarantees  

Behavioral rules are defined in:

- ENG-SESSION  
- ENG-SUPERSESSION  
- ENG-API  
- ENG-INTEGRITY  
- ENG-ERROR  
- ENG-RECEIPT  
- ENG-AUD  

This document defines structure only.

---

# 2. General Principles

2.1 All persisted domain objects are immutable once written.

2.2 All objects must use canonical JSON encoding:

- Deterministic field ordering  
- Deterministic enum string values  
- No implicit defaults  
- Nullability explicit  

2.3 Optional annotations and informational references may exist but must not affect structural legitimacy unless explicitly declared structural.

2.4 Lifecycle state must be represented as a single enum field.

2.5 Schema changes require explicit versioning.

2.6 All engine-generated identifiers must be UUIDv7:

- Engine exclusively generates IDs  
- Caller-provided IDs prohibited  
- UUID timestamp components must not influence legitimacy, precedence, or restore  

2.7 `area_id` is externally provided and opaque:

- Used solely for scoping evaluation  
- Does not imply multi-Area hosting  
- Engine does not manage Area lifecycle or ownership  

---

# 3. Schema Versioning & Compatibility Doctrine

## 3.1 Schema Version Field

Every domain object must include:

- `schema_version` (string, semantic version format recommended)

Schema version applies to structural interpretation.

---

## 3.2 Major Version Compatibility

If an Engine encounters a domain object whose **major schema version** exceeds the Engine’s supported major version:

- Rehydration must fail deterministically.
- Error classification must be `UNSUPPORTED_SCHEMA_VERSION`.
- Degraded mode is not permitted.

Major version increments may:

- Add new enum values
- Change structural meaning
- Add or remove structural fields
- Change canonicalization rules

Major version mismatches are fatal to restore.

---

## 3.3 Minor Version Compatibility (Forward-Compatible Additive Rule)

Minor version increments must be strictly additive.

Permitted in minor versions:

- Addition of informational fields  
- Addition of optional non-structural metadata  
- Addition of new annotation types  

Minor versions must not:

- Introduce new enum values that affect structural behavior  
- Change canonical ordering rules  
- Change structural reference semantics  
- Change legitimacy computation  
- Change supersession interpretation  

Enum expansion requires a **major version bump**.

---

## 3.4 Unknown Enum Handling

If the Engine encounters an unknown enum value in any structural field:

- Rehydration must fail deterministically.
- Error classification must be `UNSUPPORTED_SCHEMA_VERSION`.

Unknown enum values must never:

- Be ignored  
- Be coerced to defaults  
- Be treated as informational  

Enums are structural.

---

## 3.5 Unknown Field Handling

Unknown fields are handled as follows:

- Unknown structural fields → `UNSUPPORTED_SCHEMA_VERSION`
- Unknown informational fields → ignored

A field is structural if:

- It affects legitimacy  
- It affects ACTIVE derivation  
- It affects supersession  
- It affects governance slot evaluation  
- It affects canonical receipt snapshot  

Unknown informational fields:

- Must not participate in canonical hashing  
- Must not affect legitimacy  
- Must not affect restore validation  
- Must be safely ignored  

Canonical serialization and hashing must include **only recognized structural fields**.

---

## 3.6 Backward Compatibility Guarantee

An Engine must:

- Successfully rehydrate objects from earlier minor versions of the same major version.
- Interpret missing fields as invalid unless explicitly defined as optional in that version.

No implicit defaults are permitted.

Backward compatibility must never require inference.

---

# 4. Single-Area Structural Model

The Engine operates on exactly one Area at a time.

Implications:

- Domain graph must contain objects from exactly one `area_id`  
- Mixed-area graphs are invalid and trigger StructuralIntegrityFailure  
- Governance slot evaluation is Area-local  
- Supersession reconstruction is Area-local  

Cross-area references are informational only and do not constitute multi-Area hosting.

Multi-Area orchestration is the host’s responsibility.

---

# 5. Governance Slot Model

## 5.1 Exclusive Governance Slots

Each Area contains two exclusive legitimacy slots:

- Authority slot (Resolution objects)  
- Scope slot (Scope objects)  

Invariants:

- At most one ACTIVE Authority per Area  
- At most one ACTIVE Scope per Area  
- Slot exclusivity is structural and validated at restore  
- Authority and Scope participate in standard supersession rules  

Violations:

- Multiple ACTIVE objects → StructuralIntegrityFailure  
- Empty required slot after initialization → StructuralIntegrityFailure  

---

# 6. Structural vs Informational References

## 6.1 Structural References

Structural references affect legitimacy:

- Must resolve during restore  
- Must reference objects within the same `area_id`  

Permitted:

- Session → Authority (Resolution)  
- Session → Scope  
- Resolution → originating Session  
- Resolution → superseded Resolution  
- Scope → superseded Scope  

Missing references trigger structural failure.

Cross-area structural references are prohibited.

## 6.2 Informational Cross-Area References

Cross-area references:

- May reference external Areas or Resolutions  
- Must not affect legitimacy, ACTIVE derivation, or supersession  
- Must not trigger restore failure  
- Excluded from orphan detection  

They are immutable and treated as opaque metadata.

---

# 7. CrossAreaReference Schema

A CrossAreaReference contains:

- external_area_id  
- external_area_label  
- external_resolution_id (nullable)  
- external_resolution_label (nullable)  
- created_at  
- schema_version  

Rules:

- Engine does not dereference or update external identifiers  
- Absence of external references does not affect evaluation  
- Labels are historical snapshots and are legitimacy-bearing within the referencing object  
- Unknown fields in CrossAreaReference follow minor-version additive rules  

---

# 8. Participant Schema

A Participant represents a session-scoped participation epoch.

A Participant contains:

- participant_id (UUIDv7)  
- session_id  
- area_id  
- display_name  
- created_at  
- schema_version  

Structural Rules:

- participant_id is unique within a Session  
- participant_id must never be reused within the same Session  
- participant_id represents a continuous participation epoch  
- Removal terminates the epoch  
- Re-adding generates a new participant_id  

Display Name Rules:

- display_name is legitimacy-bearing  
- display_name must be unique among active Participants  
- display_name participates in canonical hashing  
- Frozen in Resolution snapshots upon acceptance  

Participant identity is strictly session-scoped.

Host-level grouping is outside Engine boundary.

---

# 9. Session Schema

A Session object contains:

- session_id  
- area_id  
- session_type (AUTHORITY | SCOPE | REGULAR)  
- authority_id  
- scope_id (nullable before initialization)  
- phase (SessionPhase enum)  
- state (SessionState enum)  
- participants  
- candidates  
- constraints  
- votes  
- receipts  
- cross_area_references (optional)  
- annotations (optional)  
- created_at  
- updated_at  
- schema_version  

Constraints:

- Structural references must share the Session’s area_id  
- Active Participants must have unique display_name values  
- Votes must reference active participant_id values  
- Receipts must exist for CLOSED/ACCEPTED sessions  
- Unknown enum values → `UNSUPPORTED_SCHEMA_VERSION`  
- Unknown structural fields → `UNSUPPORTED_SCHEMA_VERSION`  

---

# 10. Resolution Schema

A Resolution object contains:

- resolution_id  
- area_id  
- originating_session_id  
- authority_snapshot_id  
- scope_snapshot_id  
- participant_snapshot  
- candidate_snapshot  
- state (ResolutionState enum)  
- superseded_by (nullable)  
- cross_area_references (optional)  
- created_at  
- annotations (optional)  
- schema_version  

Constraints:

- participant_snapshot must exactly match active epoch set at acceptance  
- Snapshot participant_id values must not have been reused  
- superseded_by must reference same-area Resolution  
- Unknown enum values → `UNSUPPORTED_SCHEMA_VERSION`  

---

# 11. Supersession Encoding

- Represented via `superseded_by`  
- Explicit, immutable, Area-local  
- Occurs only via acceptance  
- Cross-area references must never participate  
- Enum expansion requires major version bump  

---

# 12. Receipt Integration

- Receipts are immutable integrity artifacts  
- One receipt per CLOSED or ACCEPTED session  
- Must match deterministic canonical snapshot  
- Regeneration prohibited  
- Missing or mismatched receipts trigger restore halt  

Receipt canonicalization must include only recognized structural fields.

Unknown informational fields must be excluded from canonical hashing.

---

# 13. Deterministic Encoding Requirements

- Field ordering deterministic  
- Enum string values deterministic  
- Set ordering lexicographically sorted by UUID  
- Timestamps informational only  
- UUID ordering must never affect legitimacy  
- Canonical JSON serialization mandatory  
- Canonical hashing must include only recognized structural fields  
- Unknown informational fields excluded from canonical hash  

---

# 14. Engine Invariants

- Exactly one Area active at runtime  
- Domain graph contains a single `area_id`  
- Governance slots exclusive  
- Structural references resolve and are Area-local  
- Cross-area references non-structural  
- All domain IDs engine-generated UUIDv7  
- Participant identity session-scoped and epoch-based  
- participant_id never reused within Session  
- display_name unique among active Participants  
- Participant snapshots immutable once accepted  
- Lifecycle represented by enums  
- Unknown enum values → `UNSUPPORTED_SCHEMA_VERSION`  
- Unknown structural fields → `UNSUPPORTED_SCHEMA_VERSION`  
- Minor versions additive only  
- Major version mismatch → restore failure  
- Deterministic encoding mandatory  
- Legitimacy never depends on timestamps, ordering, cross-area state, or unknown fields  

Violation of this schema breaks cross-system determinism and constitutes critical Engine failure.