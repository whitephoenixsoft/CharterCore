# ENG-INITIALIZATION — Engine Rehydration & Structural Readiness Specification

Status: FROZEN (v7 – Rehydration, Receipt Verification & Degraded Mode Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Engine rehydration, structural validation, integrity verification, and deterministic runtime readiness

Authority: Subordinate to ENG-DOMAIN, ENG-INTEGRITY, ENG-SESSION, ENG-RECEIPT, ENG-SPECVERIFY, ENG-API

---

# 1. Purpose

This document defines how the Engine Core initializes its runtime by rehydrating a provided domain graph.

Rehydration performs:

- Structural validation of domain objects
- Governance slot validation
- Supersession graph verification
- Receipt integrity verification
- Specification identity verification
- Deterministic readiness evaluation
- Degraded mode determination

Rehydration verifies history.

It does not evolve history.

The Engine:

- Does not attach to storage
- Does not traverse persistence
- Does not discover external objects
- Does not repair corrupted structures
- Does not mutate domain objects
- Does not create legitimacy events

The host provides domain objects.  
The Engine validates them.

---

# 2. Engine Boundary

## ENG-INIT-01 — Host-Provided Domain Graph

The host must provide a complete domain graph for exactly one Area.

The graph may include:

- Sessions
- Resolutions
- Receipts
- Annotations

All structural references must resolve within the provided graph.

The Engine must not:

- Discover missing objects
- Traverse storage
- Infer missing references
- Reconstruct governance state

Fail if:

- Required references are missing
- Objects reference entities not present in the graph

---

# 3. Rehydration Contract

## ENG-INIT-02 — Deterministic, Side-Effect-Free Construction

Engine rehydration must be:

- Deterministic
- Side-effect free
- Non-mutating

Rehydration must not:

- Change session states or phases
- Alter resolution states
- Create or delete objects
- Emit legitimacy events
- Modify supersession edges
- Promote or demote governance state

Fail if any domain object is mutated during rehydration.

---

# 4. Structural Validation Model

## ENG-INIT-03 — Full Structural Validation Pass

Rehydration must execute a full structural validation pass.

The Engine must:

- Evaluate all structural invariants
- Accumulate all violations
- Never short-circuit on first failure
- Produce deterministic ordering of violations

If fatal structural violations exist:

- Rehydration fails
- Engine must not start

Structural violations may include multiple error entries.

---

## ENG-INIT-04 — Deterministic Validation Ordering

Structural validation must occur in the following order:

1. Identity validity
2. Reference resolution
3. Lifecycle consistency
4. Governance slot enforcement
5. Supersession graph integrity
6. Receipt structural validation
7. Receipt integrity verification
8. Specification identity verification
9. Resolution state validation

Within each category:

- Errors must be ordered lexicographically by `error_code`
- Then lexicographically by related object identifiers

No reordering permitted.

---

# 5. Structural Validation Categories

## ENG-INIT-05 — Identity Validity

The Engine must verify:

- All identifiers are valid UUIDv7
- Identifiers are unique within their namespace
- Object references resolve to provided objects

Fail if:

- UUID format invalid
- Duplicate identifiers detected
- Cross-object references unresolved

---

## ENG-INIT-06 — Lifecycle Consistency

The Engine must verify lifecycle correctness for:

- Session states
- Session phases
- Resolution states
- Supersession relationships

Examples of structural violations:

- SUPERSEDED resolution missing `superseded_by`
- Non-SUPERSEDED resolution containing `superseded_by`
- ACCEPTED session missing corresponding Resolution
- Terminal session missing corresponding receipt
- Session phase inconsistent with session state

---

## ENG-INIT-07 — Governance Slot Enforcement

Each Area contains two governance slots:

- Authority slot
- Scope slot

The Engine must verify:

- At most one ACTIVE Authority Resolution
- At most one ACTIVE Scope Resolution

UNDER_REVIEW and RETIRED resolutions must not occupy governance slots.

Fail if:

- Multiple ACTIVE Authorities exist
- Multiple ACTIVE Scopes exist
- Slot exclusivity violated

The Engine must not:

- Infer missing governance
- Promote UNDER_REVIEW objects
- Repair slot violations

---

## ENG-INIT-08 — Supersession Graph Integrity

The supersession graph must be structurally valid.

The Engine must verify:

- Graph is acyclic
- Supersession edges reference valid resolutions
- SUPERSEDED resolutions are not ACTIVE
- Supersession edges remain immutable

Fail if:

- Cycles detected
- Supersession state conflicts with resolution state

---

## ENG-INIT-09 — Receipt Structural Validation

For sessions with terminal states:

ACCEPTED sessions must have:

- Exactly one LEGITIMACY receipt

CLOSED sessions must have:

- Exactly one EXPLORATION receipt

Receipt structure must match session closure semantics.

Fail if:

- Terminal session lacks receipt
- Receipt references nonexistent session
- Receipt type mismatches session terminal state

---

## ENG-INIT-10 — Receipt Integrity Verification

Receipt integrity must be verified deterministically.

The Engine must:

- Recompute `content_hash`
- Verify canonical serialization
- Verify round ordering
- Verify snapshot consistency

Fail if:

- Receipt hash mismatch detected
- Round sequence invalid
- Snapshot structure invalid

Integrity failures do not automatically cause initialization failure.  
They may trigger degraded mode.

---

## ENG-INIT-11 — Specification Identity Verification

Receipts contain rule identity fields:

- `engine_version`
- `spec_set_hash`

The Engine must verify:

- spec_set_hash matches the Engine manifest
- OR spec_set_hash exists within supported legacy specification sets

Possible outcomes:

MATCH  
LEGACY_MATCH  
SPEC_SET_UNKNOWN

SPEC_SET_UNKNOWN must not cause structural failure but must prevent reinterpretation.

---

## ENG-INIT-12 — Resolution State Validation

Resolution lifecycle states must follow the allowed state model:

- ACTIVE
- SUPERSEDED
- UNDER_REVIEW
- RETIRED

The Engine must verify:

- SUPERSEDED resolutions reference `superseded_by`
- RETIRED resolutions are not superseded
- UNDER_REVIEW resolutions do not occupy governance slots

Fail if lifecycle transitions violate state invariants.

---

# 6. Degraded Mode Determination

## ENG-INIT-13 — Integrity Damage Containment

If receipt integrity verification detects non-fatal integrity defects:

The Engine must enter degraded mode.

Examples include:

- Receipt hash mismatch
- Snapshot inconsistency
- Historical artifact divergence

In degraded mode:

- Engine becomes read-only
- Legitimacy creation is prohibited
- Session mutation is prohibited
- Incremental compilation is prohibited

Allowed operations include:

- Read-only queries
- DAG export
- Integrity diagnostics

Degraded mode enables investigation without extending corrupted history.

---

# 7. Deterministic Rehydration Guarantee

## ENG-INIT-14 — Pure Deterministic Initialization

Given identical domain objects, rehydration must produce identical:

- Structural validation results
- Integrity verification results
- Governance slot evaluation
- Runtime mode determination

Validation must not depend on:

- Storage ordering
- Timestamp ordering
- Host environment
- Runtime conditions

---

# 8. Failure Semantics

## ENG-INIT-15 — Fatal Structural Failure

If fatal structural violations are detected:

- Rehydration fails
- Engine must not start
- No runtime APIs may execute

Failures must be:

- Explicit
- Deterministic
- Non-mutating

The Engine must never repair corrupted history.

---

# 9. No Migration Rule

## ENG-INIT-16 — No Implicit Migration

During rehydration the Engine must not:

- Rewrite objects
- Upgrade schema versions
- Modify identifiers
- Rebind supersession edges
- Normalize structure
- Alter governance state

Domain objects are treated as immutable historical artifacts.

---

# 10. Runtime Readiness

## ENG-INIT-17 — Runtime Modes

After rehydration the Engine enters exactly one runtime mode.

NORMAL_RUNTIME

- All structural and integrity checks passed

DEGRADED_READ_ONLY

- Structural graph valid
- Integrity defects detected
- Mutations disabled

Initialization failure

- Fatal structural violations detected
- Engine must not start

---

# 11. Readiness Definition

The Engine is ready for runtime when:

- Structural validation succeeds
- Supersession graph integrity holds
- Governance slots are valid
- Receipts structurally valid

Runtime readiness does not imply:

- Session acceptance success
- Political legitimacy
- Governance approval

The Engine validates structure.  
It does not decide outcomes.

---

# 12. Mental Model

Rehydration reconstructs the historical legitimacy graph.

The host supplies facts.

The Engine verifies:

- structural consistency
- historical integrity
- rule identity

If history is valid, the Engine runs.

If history is damaged, the Engine permits inspection only.

If history is structurally invalid, the Engine refuses to start.

Nothing is repaired implicitly.

---

# 13. Constitutional Alignment

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-SESSION
- ENG-RECEIPT
- ENG-SPECVERIFY
- ENG-API
- ENG-IMPORT

Violation constitutes structural engine failure.