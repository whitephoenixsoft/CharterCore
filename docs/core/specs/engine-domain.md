# ENG-DOMAIN — Domain Object Schema (Rewritten v8)
Canonical Engine Object Definitions  
Status: FROZEN (v8 – Single-Area Runtime, Receipt & Degraded Mode Integrated)  
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

Behavioral rules are defined in:

- ENG-DECISION  
- ENG-REVIEW-RETIRED  
- ENG-SUPERSESSION  
- ENG-API  
- ENG-INTEGRITY  
- ENG-ERROR  

This document defines **structure only**.

---

# 2. General Principles

2.1 All persisted domain objects are immutable once written.

2.2 All objects must use canonical JSON encoding:

- Deterministic field ordering  
- Deterministic enum values  
- No implicit defaults  
- Nullability explicit  

2.3 Optional annotations and informational references may exist but **must not** affect object identity or hash unless explicitly declared.

2.4 Lifecycle state must be represented as a single enum field.

2.5 Schema changes require explicit versioning.

2.6 All engine-generated identifiers must be UUIDv7:

- Engine exclusively generates IDs  
- Caller-provided IDs prohibited  
- UUID timestamp components **must not** influence legitimacy, precedence, or restore  

2.7 `area_id` is externally provided, opaque:

- Used solely for scoping evaluation  
- Does not imply multi-Area hosting  
- Engine does not manage Area lifecycle or ownership  

---

# 3. Single-Area Structural Model

The Engine operates on exactly one Area at a time.

Implications:

- Domain graph must contain objects from exactly one `area_id`  
- Mixed-area graphs are invalid and trigger **StructuralIntegrityFailure**  
- Governance slot evaluation is Area-local  
- Supersession reconstruction is Area-local  

Cross-area references are **informational only** and do not constitute multi-Area hosting.  
Multi-Area orchestration is the host’s responsibility.

Degraded/read-only mode may activate if:

- Receipt integrity is compromised  
- Structural corruption is detected but exportable  

---

# 4. Governance Slot Model

## 4.1 Exclusive Governance Slots

Each Area contains two exclusive legitimacy slots:

- Authority slot (Resolution objects)  
- Scope slot (Scope objects)  

Invariants:

- At most one ACTIVE Authority per Area  
- At most one ACTIVE Scope per Area  
- Slot exclusivity is **structural** and validated at restore  
- Authority and Scope participate in standard supersession rules  

Violations:

- Multiple ACTIVE objects → structural integrity failure  
- Empty slot after initial definition → structural integrity failure  

---

# 5. Structural vs Informational References

## 5.1 Structural References

Structural references **affect legitimacy**:

- Must resolve during restore  
- Must reference objects in the same `area_id`  

Permitted:

- Session → Authority (Resolution)  
- Session → Scope  
- Resolution → originating Session  
- Resolution → superseded Resolution  
- Scope → superseded Scope  

Missing references → structural failure.

## 5.2 Informational Cross-Area References

Cross-area references:

- May reference external Areas or Resolutions  
- Must not affect legitimacy, ACTIVE derivation, or supersession  
- Must not trigger restore failure  
- Excluded from orphan detection  

Immutable and treated as opaque metadata.

---

# 6. CrossAreaReference Schema

- `external_area_id` (opaque UUIDv7 recommended)  
- `external_area_label` (snapshot)  
- `external_resolution_id` (nullable)  
- `external_resolution_label` (nullable)  
- `created_at` (timestamp)  
- `schema_version` (string)  

Rules:

- Engine does not dereference or update external identifiers  
- Absence of external references does not affect evaluation  

---

# 7. Session Schema

A Session object contains:

- `session_id` (UUIDv7)  
- `area_id`  
- `session_type` (AUTHORITY | SCOPE | REGULAR)  
- `authority_id` (Resolution reference)  
- `scope_id` (Scope reference, nullable before governance initialization)  
- `phase` (SessionPhase enum)  
- `state` (SessionState enum)  
- `participants` (current IDs)  
- `candidates` (Candidate objects)  
- `constraints` (Constraint objects)  
- `votes` (Vote objects)  
- `receipts` (immutable list, one per CLOSED or ACCEPTED session)  
- `cross_area_references` (optional)  
- `annotations` (optional)  
- `created_at` / `updated_at`  
- `schema_version`  

Constraints:

- Structural references must share the Session’s `area_id`  
- Receipts must exist for CLOSED/ACCEPTED sessions and match canonical snapshot  
- Cross-area references do not affect evaluation  

---

# 8. Resolution Schema

A Resolution object contains:

- `resolution_id` (UUIDv7)  
- `area_id`  
- `originating_session_id`  
- `authority_snapshot_id`  
- `scope_snapshot_id`  
- `participant_snapshot` (set at acceptance)  
- `candidate_snapshot` (complete accepted candidate content)  
- `state` (ResolutionState enum)  
- `superseded_by` (nullable, Resolution ID in same Area)  
- `cross_area_references` (optional)  
- `receipts` (optional, for validation only)  
- `created_at`  
- `annotations` (optional)  
- `schema_version`  

Constraints:

- `superseded_by` must reference Resolution in same `area_id`  
- Receipts must match CLOSED/ACCEPTED sessions  
- Cross-area references are informational  

---

# 9. Supersession Encoding

- Supersession represented via `superseded_by` reference  
- Explicit, immutable, Area-local  
- Graph-altering for superseded object  
- Only occurs via session acceptance  
- Cross-area references must **never** participate  

---

# 10. Receipt Integration

- Receipts are persistent, immutable integrity artifacts  
- One receipt per CLOSED or ACCEPTED session  
- Must match deterministic canonical snapshot  
- Regeneration is prohibited  
- Missing or mismatched receipts trigger **restore halt**  
- Reproducing a receipt implies tampering  
- Engine may enter **degraded mode** if receipts are invalid but DAG export is possible  

---

# 11. Deterministic Encoding Requirements

- Field ordering deterministic  
- Enum string values deterministic  
- Set ordering lexicographically sorted  
- Timestamps stable but not used for legitimacy  
- UUID ordering must **never** affect legitimacy  
- Canonical JSON serialization mandatory  
- Cross-area references serialized deterministically but excluded from legitimacy  

---

# 12. Schema Versioning

- All persisted objects must include `schema_version`  
- Backward-incompatible changes require version increment  
- Migrations must be explicit and deterministic  
- Multiple schema versions may coexist  
- No implicit migration allowed  

---

# 13. Engine Invariants

- Engine operates on exactly one Area at a time  
- Domain graph must contain a single `area_id`  
- Governance slots are exclusive per Area  
- Governance state derived, never stored  
- Structural references must resolve and be Area-local  
- Cross-area references **never** validated for existence  
- All domain object IDs are engine-generated UUIDv7  
- Lifecycle represented by enums, never flags  
- Receipts mandatory for CLOSED/ACCEPTED sessions  
- BLOCK_PERMANENT explicit  
- Participant and candidate mutability restricted to PRE_STANCE  
- Snapshots complete and immutable  
- Supersession structurally explicit and Area-local  
- Deterministic encoding mandatory  
- Legitimacy never depends on timestamps, ordering, or cross-area state  
- Degraded mode enables DAG export when structural or receipt integrity is compromised  

Violation of this schema breaks cross-system determinism and constitutes critical Engine failure.