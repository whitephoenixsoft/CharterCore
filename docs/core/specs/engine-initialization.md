# ENG-ENGINE-INITIALIZATION — Engine Initialization & Readiness Specification (v4)  
Status: FROZEN (v4 – Governance Completeness Enforcement)  
Applies to: Engine Core (V1/V2+)  
Scope: Structural Engine Initialization & Area Activation  

---

# 1. Purpose

This document defines how the Engine Core:

- Receives domain objects  
- Validates structural readiness  
- Enforces governance slot completeness  
- Ensures deterministic eligibility for legitimacy evaluation  
- Supports Area Activation (restore) and minimal evaluation contexts  

Initialization is structural verification, not state evolution.

The Engine:

- Does not attach to storage  
- Does not traverse persistence  
- Does not compute hashes  
- Does not load refs  
- Does not perform migration  
- Does not repair governance  

---

# 2. Engine Boundary

## ENG-INIT-01 — Host-Provided Domain Graph

The host must provide:

- Complete domain objects required for evaluation (Minimal Evaluation Mode)  
- Or complete Area object graph (Area Activation Mode)  
- All referenced objects  
- Current session set  
- Authority and Scope objects present in graph (if required by mode)  
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

- Change session states  
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

1. Minimal Evaluation Mode  
   - Graph includes only objects relevant to a specific evaluation  
   - Must satisfy invariants within provided subset  
   - Governance completeness enforced only if evaluation requires acceptance  

2. Area Activation Mode (Restore)  
   - Graph includes complete Area object graph  
   - All sessions, resolutions, Authority, Scope, candidates present  
   - Full invariant enforcement applied  

Fail if:

- Partial graph provided in Area Activation Mode  
- Structural invariants violated  
- Governance completeness violated in Area Activation Mode  

---

# 5. Structural Validation Phase

## ENG-INIT-04 — Identity Validity

- All object identifiers are valid UUIDv7  
- No duplicate identifiers exist within type namespace  
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

- SUPERSEDED resolution missing superseded_by  
- Non-SUPERSEDED resolution has superseded_by  
- ACCEPTED session lacks corresponding Resolution  
- TERMINAL phase conflicts with session state  

---

## ENG-INIT-06 — Governance Slot Enforcement

Per Area, governance slots must satisfy:

Area Activation Mode:

- Exactly one ACTIVE Authority  
- Exactly one ACTIVE Scope  

Fail if:

- No ACTIVE Authority present  
- Multiple ACTIVE Authorities present  
- No ACTIVE Scope present  
- Multiple ACTIVE Scopes present  

Minimal Evaluation Mode:

- If Area contains sessions and acceptance evaluation is possible:
  - Exactly one ACTIVE Authority required  
  - Exactly one ACTIVE Scope required  

Fail if:

- Area contains sessions but lacks Authority  
- Area contains sessions requiring Scope but Scope missing  
- Multiple ACTIVE Authority or Scope detected  

Derived governance state is not stored but must be structurally valid.

Initialization must not:

- Infer missing Scope  
- Infer missing Authority  
- Promote UNDER_REVIEW objects  
- Repair slot violations  

Slot violations constitute StructuralIntegrityFailure.

---

## ENG-INIT-07 — Participant Integrity

- Every session must have ≥1 participant  
- ACCEPTED sessions must have at least one vote recorded  
- Resolution participant_snapshot must equal session participant set at acceptance  
- Resolution candidate_snapshot must exist and match accepted candidate  

Fail if:

- Empty participant sets  
- ACCEPTED session missing vote(s)  
- Resolution snapshot incomplete  

---

## 5.8 Supersession Graph Integrity

- Graph must be acyclic  
- Superseded objects must not be ACTIVE  
- Supersession edges immutable  
- Governance objects obey supersession rules  

Fail if:

- Cycle detected  
- Supersession conflicts with state  

---

## ENG-INIT-09 — UNDER_REVIEW / RETIRED Validation

- UNDER_REVIEW and RETIRED remain structurally valid  
- May be superseded  
- Must not break slot exclusivity  
- Must not alter ACTIVE slot count  

Fail if:

- Governance slot count ambiguous due to state misuse  
- Resolution state mutates during initialization  

---

# 6. Governance Hygiene Enforcement

## ENG-INIT-10 — BLOCK_PERMANENT Enforcement

If any session exists with state = BLOCK_PERMANENT:

- Acceptance attempts must fail  
- No automatic closure permitted  
- No auto-resume permitted  

Initialization must not:

- Change BLOCK_PERMANENT state  
- Clear blocked sessions  

---

# 7. Deterministic Readiness Guarantee

## ENG-INIT-11 — Pure Deterministic Initialization

Given identical:

- Domain objects  
- Authority resolution  
- Scope resolution  
- Session set  

Initialization must produce identical:

- Validation results  
- Governance slot evaluation  
- Evaluation eligibility  
- Invariant checks  

Fail if:

- Readiness depends on storage order  
- Timestamp ordering influences validation  
- Runtime environment influences result  

---

# 8. Failure Semantics

## ENG-INIT-12 — Fail Loud, Fail Pure

Any of the following results in failure:

- Governance slot absence where required  
- Governance slot multiplicity  
- Structural invariant violation  
- Snapshot inconsistency  
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

## ENG-INIT-13 — No Implicit Migration

During initialization, the Engine must not:

- Rewrite objects  
- Upgrade schema versions  
- Modify identifiers  
- Rebind supersession edges  
- Normalize structure  
- Promote governance state  

All migration must occur through explicit operator command.

Fail if:

- Engine mutates domain objects  

---

# 10. Readiness Definition

The Engine is ready when:

- Structural validation passes  
- Supersession graph integrity holds  
- Governance slots valid and exclusive  
- Governance completeness satisfied (if required by mode)  
- Participant and snapshot invariants satisfied  
- No implicit mutation occurred  

Readiness does not imply:

- Legitimacy success  
- Session acceptability  
- Political validity  

---

# 11. Mental Model

- Host supplies facts  
- Engine validates structure  
- Governance must be structurally complete  
- Engine enforces invariants  
- Engine evaluates legitimacy  
- Nothing is repaired implicitly  

Initialization is structural verification only.

---

# 12. Constitutional Alignment

This document conforms to:

- ENG-INTEGRITY  
- ENG-DOMAIN  
- ENG-DECISION  
- ENG-REVIEW-RETIRED  
- ENG-SUPERSESSION  
- ENG-API  
- ENG-IMPORT  

Violation constitutes structural engine failure.