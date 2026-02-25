# ENG-ENGINE-INITIALIZATION — Engine Initialization & Readiness Specification (v3)
Status: FROZEN (v3)
Applies to: Engine Core (V1/V2+)
Scope: Structural Engine Initialization & Area Activation

---

# 1. Purpose

This document defines how the Engine Core:

- Receives domain objects
- Validates structural readiness
- Ensures deterministic eligibility for legitimacy evaluation
- Supports Area Activation (restore) and minimal evaluation contexts

Initialization replaces legacy storage-coupled boot logic.  

The Engine:

- Does not attach to storage
- Does not traverse persistence
- Does not compute hashes
- Does not load refs
- Does not perform migration

Initialization is purely structural verification, not state evolution.

---

# 2. Engine Boundary

## ENG-INIT-01 — Host-Provided Domain Graph

The host (CLI or embedding system) must provide:

- Complete domain objects required for evaluation (minimal evaluation)
- Or complete Area object graph (Area Activation / restore)
- All referenced objects
- Current session set
- Current authority and scope resolutions
- Explicit evaluation request

The Engine:

- Must not discover missing objects
- Must not traverse storage
- Must not infer references

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

Fail if:

- Object state changes during initialization
- Supersession edges are modified
- Audit or legitimacy artifacts are created implicitly

---

# 4. Modes of Initialization

## ENG-INIT-03 — Two Modes

1. Minimal Evaluation Mode  
   - Graph includes only objects relevant to a specific evaluation request  
   - Only referenced sessions, resolutions, and snapshots  
   - Must satisfy structural invariants within the subset provided

2. Area Activation Mode (Restore)  
   - Graph includes complete Area object graph  
   - All sessions, resolutions, Authority, Scope, and candidates present  
   - Full invariant enforcement applied (exclusive slots, snapshot completeness, participant integrity)

Fail if:

- Partial graph provided in Area Activation mode  
- Structural invariants violated

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

## ENG-INIT-06 — Exclusive Slot Enforcement

- Exactly one ACTIVE Authority per Area
- Exactly one ACTIVE Scope per Area

Fail if:

- Multiple ACTIVE objects exist in the same Area
- Exclusive slot conflicts present

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

## ENG-INIT-08 — Supersession Graph Integrity

- Graph must be acyclic
- Superseded objects are not ACTIVE
- Supersession edges are immutable

Fail if:

- Cycle detected
- Supersession structure conflicts with state

---

## ENG-INIT-09 — UNDER_REVIEW / RETIRED Resolution Validation

- Remain structurally ACTIVE in supersession graph
- May be superseded
- Do not break graph integrity

Fail if:

- Resolution state mutates during initialization
- Graph integrity violated

---

# 6. Governance Hygiene Enforcement

## ENG-INIT-10 — BLOCK_PERMANENT Enforcement

If any session exists with state = BLOCK_PERMANENT:

- No new session acceptance may succeed
- Engine must reject acceptance attempts
- No automatic closure or auto-resume permitted

Explicit operator action is required.

Fail if:

- Acceptance bypasses BLOCK_PERMANENT
- Session state changes implicitly

---

# 7. Deterministic Readiness Guarantee

## ENG-INIT-11 — Pure Deterministic Initialization

Given identical:

- Domain objects
- Authority resolution
- Scope resolution
- Session set

Initialization must produce:

- Identical validation results
- Identical evaluation eligibility
- Identical invariant checks

Fail if:

- Readiness depends on storage order, import source, timestamp ordering, or runtime environment

---

# 8. Failure Semantics

## ENG-INIT-12 — Fail Loud, Fail Pure

- Explicit failure on any invariant violation
- No partial readiness allowed
- No degraded evaluation mode
- No silent recovery or repair

---

# 9. No Migration Rule

## ENG-INIT-13 — No Implicit Migration

During initialization, the Engine must not:

- Rewrite objects
- Upgrade schema versions
- Modify identifiers
- Rebind supersession edges
- Normalize structure

All migration must occur through explicit operator command.

Fail if:

- Engine mutates domain objects

---

# 10. Readiness Definition

The Engine is considered ready when:

- Structural validation passes
- Supersession graph integrity holds
- Exclusive slot enforcement passed
- Participant and snapshot invariants satisfied
- No implicit mutation occurred

Readiness does not imply:

- Legitimacy success
- Session acceptability
- Governance validity

---

# 11. Mental Model

- Host supplies facts
- Engine validates structure
- Engine enforces invariants
- Engine evaluates legitimacy
- Nothing changes unless explicitly commanded

Initialization is structural verification, not state evolution.

---

# 12. Constitutional Alignment

This document conforms to:

- ENG-CORE-PURITY
- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION
- ENG-AUD

Violation of this specification constitutes a structural engine failure.