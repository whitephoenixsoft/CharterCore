# ENG-INITIALIZATION — Engine Initialization & Readiness Specification (v5)  
Status: FROZEN (v5 – Governance & Participant Epoch Enforcement)  
Applies to: Engine Core (V1/V2+)  
Scope: Structural Engine Initialization, Area Activation, and Deterministic Readiness  

---

# 1. Purpose

This document defines how the Engine Core:

- Receives domain objects  
- Validates structural readiness  
- Enforces governance slot completeness  
- Ensures deterministic eligibility for legitimacy evaluation  
- Validates participant epoch integrity  
- Supports Area Activation (restore) and minimal evaluation contexts  

Initialization is **structural verification only**, not state evolution.

The Engine:

- Does not attach to storage  
- Does not traverse persistence  
- Does not compute hashes  
- Does not load external references  
- Does not perform migration  
- Does not repair governance or session state  

---

# 2. Engine Boundary

## ENG-INIT-01 — Host-Provided Domain Graph

The host must provide:

- Complete domain objects for evaluation (Minimal Evaluation Mode)  
- Or complete Area object graph (Area Activation Mode)  
- All referenced objects including sessions, candidates, Authority, and Scope  
- Explicit evaluation context  

The Engine:

- Must not discover missing objects  
- Must not traverse storage  
- Must not infer references  
- Must not reconstruct missing governance  

Fail if:

- Evaluation depends on objects not explicitly provided  
- Engine attempts to resolve storage paths  

---

# 3. Initialization Contract

## ENG-INIT-02 — Deterministic, Side-Effect Free Construction

Engine initialization must be:

- Deterministic  
- Side-effect free  
- Non-mutating  

Initialization must not:

- Change session states or phases  
- Alter resolution states  
- Create or delete objects  
- Emit legitimacy events  
- Promote or demote governance state  

Fail if:

- Object state changes during initialization  
- Supersession edges are modified  
- Governance is implicitly repaired  

---

# 4. Modes of Initialization

## ENG-INIT-03 — Two Modes

1. **Minimal Evaluation Mode**  
   - Graph includes only objects relevant to a specific evaluation  
   - Must satisfy invariants within provided subset  
   - Governance completeness enforced only if acceptance evaluation is possible  

2. **Area Activation Mode (Restore)**  
   - Graph includes complete Area object graph  
   - All sessions, resolutions, Authority, Scope, and candidates present  
   - Full invariant enforcement applied  

Fail if:

- Partial graph provided in Area Activation Mode  
- Structural invariants violated  
- Governance completeness violated in Area Activation Mode  

---

# 5. Structural Validation Phase

## ENG-INIT-04 — Identity Validity

- All object identifiers must be valid UUIDv7  
- No duplicate identifiers exist within the same type namespace  
- All cross-object references resolve to provided objects  

Fail if:

- Missing references  
- Invalid UUID format  
- Duplicate IDs exist  

---

## ENG-INIT-05 — Lifecycle Consistency

- Session state vs phase consistency  
- Resolution state consistency  
- Supersession invariants  
- Snapshot completeness for ACCEPTED sessions  
- BLOCK_PERMANENT invariants (structural only)  

Fail if:

- SUPERSEDED resolution missing `superseded_by`  
- Non-SUPERSEDED resolution has `superseded_by`  
- ACCEPTED session lacks corresponding Resolution  
- TERMINAL phase conflicts with session state  

---

## ENG-INIT-06 — Governance Slot Enforcement

Per Area, governance slots must satisfy:

- **Area Activation Mode:** exactly one ACTIVE Authority and one ACTIVE Scope  
- **Minimal Evaluation Mode:** required if sessions exist and acceptance evaluation is possible  

Fail if:

- No ACTIVE Authority or Scope present  
- Multiple ACTIVE Authorities or Scopes detected  

Initialization must not:

- Infer missing Scope or Authority  
- Promote UNDER_REVIEW objects  
- Repair slot violations  

Slot violations constitute **StructuralIntegrityFailure**.

---

## ENG-INIT-07 — Participant Epoch Integrity

- Every session must have ≥1 participant  
- ACCEPTED sessions must have at least one vote recorded  
- Resolution `participant_snapshot` must equal session participant set at acceptance  
- **Participant IDs represent participation epochs**; no historical epoch merging  
- IDs in snapshot must correspond to the **final participant epoch set**  
- Resolution `candidate_snapshot` must match accepted candidate set  

Fail if:

- Empty participant sets  
- ACCEPTED session missing votes  
- Resolution snapshot incomplete  
- Participant IDs reused across participation epochs  

---

## ENG-INIT-08 — BLOCK_TEMPORARY Reconfirmation

- Sessions in BLOCK_TEMPORARY must terminate prior participant epochs on resume  
- New participant set must be explicitly added  
- New participant IDs must be generated  
- Votes must be cleared  
- Initialization validates that reconfirmation rules are enforceable  

Fail if:

- Participant set persisted without reconfirmation  
- Votes remain from prior epoch  

---

## ENG-INIT-09 — Solo Mode Considerations

- If only one participant exists, an implicit ACCEPT vote is expected  
- Validation ensures deterministic vote insertion if acceptance is evaluated  
- Phase and participant structure conform to SOLO mode invariants  

Fail if:

- Solo mode session lacks participant  
- Implicit vote rules cannot be deterministically enforced  

---

## ENG-INIT-10 — Supersession Graph Integrity

- Graph must be acyclic  
- Superseded objects must not be ACTIVE  
- Supersession edges immutable  
- Governance objects obey supersession rules  

Fail if:

- Cycle detected  
- Supersession conflicts with state  

---

## ENG-INIT-11 — UNDER_REVIEW / RETIRED Validation

- UNDER_REVIEW and RETIRED objects must remain structurally valid  
- Must not break slot exclusivity  
- Must not alter ACTIVE slot count  

Fail if:

- Governance slot count ambiguous  
- Resolution state mutates during initialization  

---

# 6. Governance Hygiene Enforcement

## ENG-INIT-12 — BLOCK_PERMANENT Enforcement

- Acceptance attempts must fail if any session = BLOCK_PERMANENT  
- No automatic closure or resume permitted  

Initialization must not:

- Change BLOCK_PERMANENT state  
- Clear blocked sessions  

---

# 7. Deterministic Readiness Guarantee

## ENG-INIT-13 — Pure Deterministic Initialization

Given identical:

- Domain objects  
- Authority resolution  
- Scope resolution  
- Session set  

Initialization must produce identical:

- Validation results  
- Governance slot evaluation  
- Evaluation eligibility  
- Participant epoch validation  
- Snapshot checks  

Fail if:

- Readiness depends on storage order, timestamp ordering, or environment  

---

# 8. Failure Semantics

## ENG-INIT-14 — Fail Loud, Fail Pure

Any of the following results in failure:

- Governance slot absence or multiplicity  
- Structural invariant violation  
- Participant or snapshot integrity violation  
- Supersession cycle  

Failure must be:

- Explicit  
- Deterministic  
- Non-mutating  

No partial readiness allowed.  
No degraded mode allowed.  
No implicit repair allowed.

---

# 9. No Migration Rule

## ENG-INIT-15 — No Implicit Migration

During initialization, the Engine must not:

- Rewrite objects  
- Upgrade schema versions  
- Modify identifiers  
- Rebind supersession edges  
- Normalize structure  
- Promote governance state  

Fail if:

- Engine mutates domain objects  

---

# 10. Readiness Definition

The Engine is ready when:

- Structural validation passes  
- Supersession graph integrity holds  
- Governance slots valid and exclusive  
- Governance completeness satisfied (if required)  
- Participant epoch and snapshot invariants satisfied  
- No implicit mutation occurred  

Readiness does **not** imply:

- Legitimacy success  
- Session acceptability  
- Political validity  

---

# 11. Mental Model

- Host supplies facts  
- Engine validates structure  
- Governance must be structurally complete  
- Participant epochs must be unique and deterministic  
- Engine enforces invariants  
- Initialization evaluates readiness, not acceptance  
- Nothing is repaired implicitly  

---

# 12. Constitutional Alignment

- ENG-INTEGRITY  
- ENG-DOMAIN  
- ENG-DECISION  
- ENG-REVIEW-RETIRED  
- ENG-SUPERSESSION  
- ENG-API  
- ENG-IMPORT  

Violation constitutes **structural engine failure**.