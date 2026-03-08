# ENG-SUPERSESSION — Supersession & Conflict Model
Status: FROZEN (v6 – Canonical Receipt Validation & Domain Alignment)
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines:

- Resolution supersession graph structure
- Supersession linking rules
- ACTIVE resolution derivation
- Governance slot participation in supersession
- Conflict detection
- Acceptance race semantics
- Blocking triggers
- Receipt validation during acceptance and restore
- Deterministic restore guarantees
- Area-local supersession sovereignty

Supersession evaluation occurs strictly within the single active Area.

Mixed-area resolution graphs are prohibited (StructuralIntegrityFailure per ENG-INTEGRITY).

Session lifecycle mechanics are defined in ENG-SESSION and ENG-DECISION.

---

# 2. Single-Area Supersession Model

## 2.1 Area-Local Graph Sovereignty

The supersession graph is scoped to exactly one Area.

Rules:

- The Engine operates on exactly one Area at runtime
- Supersession evaluation must not cross Area boundaries
- Resolution graphs must not contain structural objects from multiple Areas
- ACTIVE derivation must occur strictly within the Area

Cross-area references are informational metadata only.

If mixed-area structural objects are detected:

StructuralIntegrityFailure must occur during acceptance or restore.

---

# 3. Resolution Graph Model

## 3.1 Directed Supersession

Resolutions may supersede zero or more prior Resolutions within the same Area.

Rules:

- Supersession edges are explicit
- Supersession edges are directional
- Supersession edges are immutable
- Supersession edges are recorded during session acceptance
- Supersession edges must remain Area-local

Cross-area supersession is prohibited.

---

## 3.2 Acyclic Requirement

The supersession graph must remain acyclic.

Cycle detection must occur:

- During acceptance
- During restore

Cycle detection failure causes:

- Acceptance rejection  
or
- Engine restore halt

No automatic repair permitted.

---

## 3.3 Area Consistency

All supersession edges must satisfy:

resolution.area_id == superseded_resolution.area_id

Cross-area supersession attempts must deterministically fail.

Restore-time detection must halt Engine initialization.

---

# 4. ACTIVE Resolution Derivation

## 4.1 Structural ACTIVE

A Resolution is structurally ACTIVE if:

- Its state == ACTIVE
- No accepted successor exists within the Area

Superseded Resolutions are not ACTIVE.

ACTIVE derivation must be deterministic across implementations.

---

## 4.2 Governance Slot Participation

Authority and Scope Resolutions participate fully in the supersession graph.

Governance slot invariants are defined in ENG-DOMAIN.

Supersession evaluation must enforce those invariants during acceptance and restore.

Violations cause:

StructuralIntegrityFailure or deterministic acceptance failure.

---

# 5. Structural vs Informational References

## 5.1 Structural References

Structural references affect legitimacy and must resolve.

Examples:

Resolution → superseded Resolution

Rules:

- Must resolve during acceptance
- Must resolve during restore
- Must remain Area-local

Missing structural references cause StructuralIntegrityFailure.

---

## 5.2 Informational Cross-Area References

Cross-area references:

- May reference external Areas
- May reference external Resolutions
- Must not be interpreted as supersession edges
- Must not affect ACTIVE derivation
- Must not influence governance slot evaluation

They are informational metadata only.

---

# 6. Acceptance Race Semantics

Multiple sessions may attempt to supersede the same Resolution.

Race resolution follows the first-successful-accept rule.

Concurrency enforcement is defined in ENG-DECISION via the legitimacy lock.

Outcome:

- First successful acceptance commits the supersession edge
- Competing sessions transition to BLOCK_PERMANENT

No branch merge permitted.

No timestamp arbitration permitted.

---

# 7. Governance Supersession Effects

## 7.1 Authority Supersession

When an Authority Resolution is superseded:

- The prior Authority becomes SUPERSEDED
- ACTIVE Authority changes deterministically

Sessions created under the prior Authority may transition to BLOCK_PERMANENT.

Acceptance under an obsolete Authority must fail during validation.

---

## 7.2 Scope Supersession

When a Scope Resolution is superseded:

- The prior Scope becomes SUPERSEDED
- ACTIVE Scope changes deterministically

Sessions referencing the prior Scope may transition to BLOCK_PERMANENT depending on evaluation rules defined in ENG-DECISION.

---

# 8. Conflict Detection

A supersession conflict exists if acceptance would:

- Reference a non-ACTIVE Resolution
- Introduce a supersession cycle
- Violate governance slot invariants
- Introduce cross-area supersession
- Reference missing structural objects
- Reference receipts that fail integrity validation

Conflict detection must occur:

- During evaluation
- Immediately before acceptance commit

Irreversible conflicts cause BLOCK_PERMANENT.

---

# 9. Receipt Validation

Receipt integrity must be validated during:

- acceptance evaluation
- engine restore

Validation must confirm:

- Receipt exists for terminal sessions
- Canonical serialization is valid
- content_hash recomputation matches stored value
- spec_set_hash matches the specification set embedded in the executing Engine

Canonical rules are defined in ENG-CANON.

Receipt mismatch constitutes StructuralIntegrityFailure.

---

# 10. Deterministic Restore Guarantee

Given identical persisted objects and supersession edges:

- ACTIVE resolution sets must be identical
- Governance slot derivation must be identical
- Supersession graph evaluation must be identical

Restore must fail if:

- Cycles exist
- Cross-area supersession exists
- Structural references are missing
- Receipt integrity validation fails
- Governance slot invariants are violated

No heuristic ordering permitted.

No timestamp ordering permitted.

No automatic repair permitted.

---

# 11. Engine Invariants

- Supersession edges immutable
- Supersession graph strictly Area-local
- Engine operates on exactly one Area
- Graph must remain acyclic
- ACTIVE derivation deterministic
- Governance slot invariants enforced
- Cross-area references informational only
- First successful acceptance wins
- Receipt integrity validated
- Canonical hashing rules enforced
- Structural corruption causes deterministic failure

Optional degraded read-only mode behavior is defined in ENG-INTEGRITY.

---

# 12. Relationship to Other Specifications

ENG-DECISION  
Defines acceptance transactions, blocking semantics, and legitimacy lock behavior.

ENG-DOMAIN  
Defines structural schemas and governance slot invariants.

ENG-INTEGRITY  
Defines engine halt conditions and degraded read-only mode.

ENG-CANON  
Defines canonical serialization and hash computation rules.

ENG-RECEIPT  
Defines receipt structure and canonical integrity verification.

ENG-SPECVERIFY  
Defines rule identity binding through spec_set_hash.