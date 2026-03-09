# ENG-SUPERSESSION — Supersession & Conflict Model
Status: DRAFT (Adjusted for V3 Governance & Incremental Compilation)
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines:

- Resolution supersession graph structure  
- Supersession linking rules  
- ACTIVE resolution derivation including UNDER_REVIEW/RETIRED semantics  
- Governance slot participation in supersession  
- Conflict detection  
- Acceptance race semantics  
- Blocking triggers  
- Receipt validation during acceptance and restore  
- Deterministic restore guarantees  
- Area-local supersession sovereignty  
- Incremental compilation integration

Supersession evaluation occurs strictly within a single active Area.  
Mixed-area resolution graphs are prohibited (StructuralIntegrityFailure per ENG-INTEGRITY).  

Session lifecycle mechanics defined in ENG-SESSION and ENG-DECISION.

---

# 2. Single-Area Supersession Model

## 2.1 Area-Local Graph Sovereignty

- Supersession graph scoped to exactly one Area  
- Engine operates on exactly one Area at runtime  
- Supersession evaluation must not cross Area boundaries  
- Resolution graphs must not contain structural objects from multiple Areas  
- ACTIVE derivation strictly within Area  

Cross-area references are informational only.  

Mixed-area structural objects → StructuralIntegrityFailure during acceptance or restore.

---

# 3. Resolution Graph Model

## 3.1 Directed Supersession

- Resolutions may supersede zero or more prior Resolutions within the same Area  
- Supersession edges are explicit, directional, immutable  
- Supersession edges recorded during session acceptance  
- Supersession edges remain Area-local  

Cross-area supersession prohibited.

---

## 3.2 Acyclic Requirement

Supersession graph must remain acyclic.  

Cycle detection occurs:

- During acceptance  
- During restore  

Failure → acceptance rejection or Engine restore halt.  

No automatic repair permitted.

---

## 3.3 Area Consistency

All supersession edges must satisfy:

resolution.area_id == superseded_resolution.area_id  

Cross-area supersession → deterministic failure.

---

# 4. ACTIVE Resolution Derivation

## 4.1 Structural ACTIVE

A Resolution is structurally ACTIVE if:

- Its state == ACTIVE  
- No accepted successor exists within the Area  

UNDER_REVIEW and RETIRED do not alter structural ACTIVE but **suspend usability** in legitimacy evaluation.

Superseded Resolutions are not ACTIVE.

ACTIVE derivation deterministic across implementations.

---

## 4.2 Governance Slot Participation

Authority and Scope Resolutions participate fully in the supersession graph.

- Authority must always be ACTIVE for legitimacy evaluation  
- Scope may enter UNDER_REVIEW, which blocks dependent sessions  
- Scope RETIRED is structural SUPERSSEDED equivalent for usage but does not alter supersession  

Violations → StructuralIntegrityFailure or deterministic acceptance failure.

---

# 5. Structural vs Informational References

## 5.1 Structural References

Structural references affect legitimacy and must resolve:

- Resolution → superseded Resolution  

Rules:

- Must resolve during acceptance and restore  
- Must remain Area-local  
- Missing references → StructuralIntegrityFailure

## 5.2 Informational Cross-Area References

- May reference external Areas or Resolutions  
- Must not be interpreted as supersession edges  
- Must not affect ACTIVE derivation or governance slot evaluation  

---

# 6. Acceptance Race Semantics

Multiple sessions may attempt to supersede the same Resolution.

- First-successful-accept rule enforced  
- Competing sessions transition to BLOCK_PERMANENT  
- Incremental compilation sessions consult index to resolve historical conflicts  

No branch merge permitted.  
No timestamp arbitration for live sessions; only index or canonical order used.

---

# 7. Governance Supersession Effects

## 7.1 Authority Supersession

- Prior Authority becomes SUPERSEDED  
- ACTIVE Authority changes deterministically  
- Initial bootstrapping may assume SOLE_ACTOR if no Authority exists, but rehydrated DAG requires explicit Authority  
- Sessions under prior Authority → BLOCK_PERMANENT  

## 7.2 Scope Supersession

- Prior Scope becomes SUPERSEDED  
- ACTIVE Scope changes deterministically  
- Sessions referencing prior Scope may → BLOCK_PERMANENT depending on evaluation rules (ENG-DECISION)  
- Scope RETIRED blocks forward usability, cannot be reactivated  

---

# 8. Resolution States & Incremental Compilation

- ACTIVE: normal, structurally usable  
- UNDER_REVIEW: administrative suspension, blocks dependent sessions, does not alter structural ACTIVE  
- RETIRED: deprecated, blocks forward usability, cannot transition to SUPERSEDED  
- SUPERSEDED: terminal, structural, blocks all dependent sessions permanently  

Incremental compilation:

- Consults resolution index with acceptance timestamps  
- Determines canonical order for historical replays  
- Early acceptance → wins; later acceptance → rejected  
- Engine enforces deterministic index resolution  

---

# 9. Conflict Detection

Conflict exists if acceptance would:

- Reference non-ACTIVE Resolution  
- Introduce supersession cycle  
- Violate governance slot invariants  
- Introduce cross-area supersession  
- Reference missing structural objects  
- Reference receipts failing integrity validation  

Conflict detection occurs:

- During evaluation  
- Immediately before acceptance commit  

Irreversible conflicts → BLOCK_PERMANENT.

---

# 10. Receipt Validation

Receipt integrity validated during:

- Acceptance evaluation  
- Engine restore  

Checks:

- Receipt exists for terminal sessions  
- Canonical serialization valid  
- content_hash recomputation matches stored value  
- spec_set_hash matches executing Engine manifest  

Receipt mismatch → StructuralIntegrityFailure.

---

# 11. Deterministic Restore Guarantee

Given identical persisted objects and supersession edges:

- ACTIVE resolution sets identical  
- Governance slot derivation identical  
- Supersession graph evaluation identical  

Restore fails if:

- Cycles exist  
- Cross-area supersession exists  
- Structural references missing  
- Receipt integrity fails  
- Governance slot invariants violated  

No heuristic ordering, no timestamp ordering, no automatic repair.

Incremental compilation index consulted for historical determinism.

---

# 12. Engine Invariants

- Supersession edges immutable  
- Supersession graph strictly Area-local  
- Engine operates on exactly one Area  
- Graph remains acyclic  
- ACTIVE derivation deterministic  
- Governance slot invariants enforced  
- Cross-area references informational only  
- First successful acceptance wins  
- Receipt integrity validated  
- Canonical hashing enforced  
- Structural corruption → deterministic failure  
- UNDER_REVIEW/RETIRED semantics enforced  
- Incremental compilation consults resolution index  

---

# 13. Relationship to Other Specifications

ENG-DECISION: acceptance transactions, blocking semantics, legitimacy lock behavior.  
ENG-DOMAIN: structural schemas, governance slot invariants.  
ENG-INTEGRITY: engine halt conditions, degraded read-only mode, incremental compilation index enforcement.  
ENG-CANON: canonical serialization and hash computation rules.  
ENG-RECEIPT: receipt structure and canonical integrity verification.  
ENG-SPECVERIFY: rule identity binding via spec_set_hash.