# ENG-ENGINE-INITIALIZATION — Engine Initialization & Readiness Specification  
Status: FROZEN (v2)  
Applies to: Engine Core (V1/V2+)  
Scope: Pure Engine Initialization (Storage-Agnostic)  

---

# 1. Purpose

This document defines how the Engine Core:

- Is instantiated
- Receives domain objects
- Validates structural readiness
- Becomes eligible to evaluate legitimacy

This specification replaces legacy storage-coupled boot logic.

The Engine:

- Does not attach to storage
- Does not traverse persistence
- Does not compute hashes
- Does not load refs
- Does not perform migration

Initialization is purely structural.

---

# 2. Engine Boundary

## ENG-INIT-01 — Host-Provided Domain Graph

The host (CLI or embedding system) must provide:

- Complete domain objects required for evaluation
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

## ENG-INIT-02 — Deterministic Construction

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

# 4. Structural Validation Phase

Upon initialization, the Engine must validate:

### ENG-INIT-03 — Identity Validity

- All object identifiers are valid UUIDv7
- No duplicate identifiers exist within type namespace
- All cross-object references resolve to provided objects

Fail if:

- An object references a missing identifier
- Identifier format is invalid
- Duplicate IDs exist

---

### ENG-INIT-04 — Lifecycle Consistency

The Engine must validate:

- Session state and phase consistency
- Resolution state consistency
- Supersession invariants
- Snapshot completeness for accepted sessions
- BLOCK_PERMANENT invariants (structural only)

Fail if:

- A resolution marked SUPERSEDED lacks superseded_by
- A non-SUPERSEDED resolution has superseded_by
- ACCEPTED session lacks originating resolution
- TERMINAL phase conflicts with session state

---

### ENG-INIT-05 — Supersession Graph Integrity

The Engine must verify:

- Supersession graph is acyclic
- Superseded objects are not ACTIVE
- Supersession edges are immutable

Fail if:

- Cycle detected
- Supersession structure conflicts with state

---

# 5. Governance Hygiene Enforcement

Initialization must not alter state, but must enforce invariants at evaluation time.

## ENG-INIT-06 — BLOCK_PERMANENT Enforcement

If any session exists with:

state = BLOCK_PERMANENT

Then:

- No new session acceptance may succeed
- Engine must reject acceptance attempts
- No automatic closure is permitted

The Engine must not:

- Auto-close sessions
- Auto-resume sessions
- Auto-consolidate sessions

Explicit operator action is required.

Fail if:

- Acceptance bypasses BLOCK_PERMANENT
- Session state changes implicitly

---

# 6. Resolution State Integrity

## ENG-INIT-07 — UNDER_REVIEW and RETIRED

Initialization must verify:

- UNDER_REVIEW and RETIRED resolutions:
  - Remain structurally ACTIVE in supersession graph
  - May still be superseded
  - Do not break graph integrity

The Engine must not:

- Alter resolution state during initialization
- Infer deactivation beyond explicit state

Fail if:

- Resolution state mutates during startup

---

# 7. Determinism Guarantee

## ENG-INIT-08 — Pure Deterministic Readiness

Given identical:

- Domain objects
- Authority resolution
- Scope resolution
- Session set

Initialization must produce:

- Identical validation results
- Identical evaluation eligibility
- Identical invariant checks

Initialization must not depend on:

- Storage order
- Import source
- Timestamp ordering
- Runtime environment

Fail if:

- Two independent implementations produce different readiness outcomes

---

# 8. Failure Semantics

## ENG-INIT-09 — Fail Loud, Fail Pure

If structural invariants are violated:

- Initialization must fail explicitly
- No partial readiness allowed
- No degraded evaluation mode
- No silent recovery

The Engine does not repair.

The Engine rejects invalid structure.

---

# 9. No Migration Rule

## ENG-INIT-10 — No Implicit Migration

During initialization, the Engine must not:

- Rewrite objects
- Upgrade schema versions
- Modify identifiers
- Rebind supersession edges
- Normalize structure

All migration must occur outside the engine through explicit commands.

Fail if:

- Engine mutates domain objects during initialization

---

# 10. What Initialization Is Not

Initialization is not:

- Storage verification
- Hash validation
- Envelope validation
- Import logic
- Ref resolution
- Area lifecycle management

Those responsibilities belong to the host layer.

---

# 11. Readiness Definition

The Engine is considered ready when:

- Structural validation passes
- Supersession graph integrity holds
- Lifecycle invariants hold
- No implicit mutation occurred

Readiness does not imply:

- Legitimacy success
- Session acceptability
- Governance validity

Readiness only guarantees:

The Engine may now evaluate legitimacy deterministically.

---

# 12. Mental Model

- Host supplies facts.
- Engine validates structure.
- Engine enforces invariants.
- Engine evaluates legitimacy.
- Nothing changes unless explicitly commanded.

Initialization is structural verification, not state evolution.

---

# Constitutional Alignment

This document conforms to:

- ENG-CORE-PURITY
- ENG-DOMAIN
- ENG-DECISION
- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION
- ENG-AUD

Violation of this specification constitutes a structural engine failure.