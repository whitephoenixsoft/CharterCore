# ENG-SUPERSESSION — Supersession & Conflict Model (Rewritten v5)
Status: FROZEN (v5 – Single-Area Runtime, Receipt & Degraded Mode Integrated)
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines:

- Resolution graph structure  
- Supersession linking rules  
- ACTIVE-set derivation  
- Governance slot participation in supersession  
- Conflict detection  
- Race condition semantics  
- Permanent and temporary blocking triggers  
- Receipt validation at acceptance and restore  
- Degraded/read-only mode activation  
- Deterministic restore guarantees  
- Area-local supersession sovereignty  

Supersession evaluation occurs **strictly within the single active Area**.

Mixed-area resolution graphs are prohibited (StructuralIntegrityFailure per ENG-INTEGRITY).

Session lifecycle mechanics are **not** defined here.

---

# 2. Single-Area Supersession Model

## 2.1 Area-Local Graph Sovereignty

- Supersession graph is scoped to the active Area.  
- Engine operates on **exactly one Area** at a time.  
- Must not evaluate supersession across Areas.  
- Must not merge or compose resolution graphs from different Areas.  
- Must not derive ACTIVE sets across Area boundaries.  

Cross-area references are informational metadata only.

Degraded/read-only mode may activate if:

- Receipt integrity is compromised  
- Structural corruption is detected but exportable  

---

# 3. Resolution Graph Model

## 3.1 Directed Supersession

- Resolutions may supersede zero or more prior Resolutions within the same Area.  
- Authority and Scope participate like normal Resolutions.  
- Supersession is explicit, directional, immutable, recorded at acceptance, Area-local.  
- Cross-area supersession is **prohibited**.  

Acceptance with invalid receipts or receipt mismatch:

- Must halt normally (StructuralIntegrityFailure)  
- May optionally allow read-only DAG export if configured  

## 3.2 Acyclic Requirement

- Supersession graph must remain acyclic.  
- Cycle detection occurs at acceptance or restore.  
- Violations → acceptance fail or restore halt.  

## 3.3 Area Consistency

- `resolution.area_id` must equal `superseded_resolution.area_id`  
- Cross-area attempts → deterministic failure  
- Restore-time detection → Engine halt  

---

# 4. ACTIVE Resolution Derivation

## 4.1 Structural ACTIVE

Resolution is structurally ACTIVE if:

- No accepted successor exists in the active Area  
- Not structurally inconsistent  
- Receipt integrity verified (CLOSED/ACCEPTED sessions only)  

## 4.2 Legitimacy Usability

A Resolution is usable if:

- Structurally ACTIVE  
- State is not UNDER_REVIEW or RETIRED  
- Receipt exists and matches canonical snapshot  

External Areas do **not** influence usability.

---

## 4.3 Exclusive Governance Slots

- Exactly one structurally ACTIVE Authority per Area  
- Exactly one structurally ACTIVE Scope per Area (once defined)  

Violations (slot multiplicity, empty slot, missing receipt) → deterministic acceptance failure or restore halt.

Degraded/read-only mode may activate if receipt verification fails but DAG export is possible.

---

# 5. Structural vs Informational References

## 5.1 Structural References

- Resolution → superseded Resolution (same Area only)  
- Must resolve during acceptance and restore  
- Missing references → StructuralIntegrityFailure  
- Must not cross Areas  

## 5.2 Informational Cross-Area References

- May reference external Areas/Resolutions  
- Must not be interpreted as supersession edges  
- Must not influence ACTIVE derivation, governance slot evaluation, or conflict detection  

---

# 6. First-Accept Wins Rule

- First successful acceptance creates the supersession edge  
- Competing sessions referencing the same Resolution → BLOCK_PERMANENT  
- No automatic branch merge, timestamp, or cross-area arbitration  

---

# 7. Governance Supersession Effects

## 7.1 Authority Supersession

- Prior Authority becomes non-structurally-ACTIVE  
- All sessions → BLOCK_PERMANENT  
- Acceptance prohibited until explicit closure or restart  

Missing or invalid Authority receipts:

- Halt restore / acceptance  
- May trigger degraded mode with read-only DAG export  

## 7.2 Scope Supersession

- Prior Scope becomes non-structurally-ACTIVE  
- Sessions → BLOCK_PERMANENT or BLOCK_TEMPORARY if UNDER_REVIEW  
- Must not leave Scope slot empty once defined  

Cross-area state does **not** affect blocking.

---

# 8. Conflict Detection

Conflict exists if:

- Session references non-structurally ACTIVE Resolution  
- Session references Authority/Scope not usable  
- Acceptance would introduce cycle  
- Acceptance violates slot exclusivity or emptiness  
- Acceptance attempts cross-area supersession  
- Receipt missing or mismatched  

Conflict detection occurs:

- During evaluation  
- Immediately before acceptance commit  

Irreversible conflict → BLOCK_PERMANENT  
Reversible conflict (UNDER_REVIEW, RETIRED) → BLOCK_TEMPORARY  

Degraded/read-only mode may be triggered for structural/receipt corruption.

---

# 9. Deterministic Restore Guarantee

Given identical persisted objects and supersession edges:

- Structural ACTIVE sets must be identical  
- Governance slot evaluation identical  
- No heuristic, timestamp, ordering, or cross-area logic  

Restore fails / Engine halts if:

- Cycle detected  
- Multiple structurally ACTIVE Authorities or Scopes  
- Zero structurally ACTIVE Authority/Scope  
- Invalid supersession references  
- Cross-area supersession edges  
- Mixed-area structural object graphs  
- Missing or mismatched receipts  

Degraded/read-only mode may allow DAG export without mutations.

No automatic repair permitted.

---

# 10. Engine Invariants

- Supersession edges immutable  
- Graph strictly Area-local  
- Engine operates on exactly one Area  
- Mixed-area graphs prohibited  
- Graph must remain acyclic  
- Structural ACTIVE derivation deterministic  
- Authority and Scope fully participate  
- Governance slots never empty or multiply ACTIVE  
- Cross-area references never structural  
- First-accept wins absolute  
- No implicit conflict resolution  
- Structural inconsistency → Engine halt  
- Receipt integrity mandatory for CLOSED/ACCEPTED sessions  
- Degraded mode optional for read-only DAG export on restore/receipt failure  

---

# 11. Relationship to Other Specifications

- **ENG-DECISION:** session lifecycle, governance mutation, acceptance transaction  
- **ENG-DOMAIN:** domain object structure, structural vs cross-area references, receipt presence  
- **ENG-INTEGRITY:** Engine halt conditions, single-Area enforcement, structural failure handling  
- **ENG-ERROR:** deterministic error classification for receipt, structural, and block failures  

Together, they define deterministic, single-Area legitimacy compilation with receipt enforcement and optional degraded-mode recovery.