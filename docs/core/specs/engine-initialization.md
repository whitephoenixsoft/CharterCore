# ENG-INITIALIZATION — Engine Rehydration & Runtime Entry Specification

Status: REFACTORED (v11 – Structure/Usability Alignment & Referential Consistency)  
Applies to: Engine Core (V1/V2+)  
Scope: Engine rehydration, structural runtime entry, and runtime mode establishment

Authority: Runtime entry specification subordinate to ENG-API and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-STRUCTURE
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-ERROR
- ENG-API

---

# 1. Purpose

ENG-INITIALIZATION defines how the Engine enters runtime from a host-provided domain graph.

It is the authoritative specification for:

- the rehydration entry contract
- the host / Engine runtime-entry boundary
- deterministic, non-mutating runtime construction
- the sequencing of runtime entry
- runtime mode establishment after rehydration

ENG-INITIALIZATION does not redefine:

- structural validity rules
- halt conditions
- degraded mode eligibility criteria
- domain object schemas
- structural graph semantics or ACTIVE derivation
- receipt structure
- canonical serialization rules
- specification identity semantics

Those are defined respectively in:

- ENG-INTEGRITY
- ENG-DOMAIN
- ENG-STRUCTURE
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

ENG-INITIALIZATION defines how rehydration is invoked and how runtime entry depends on the authoritative validations supplied by those specifications.

---

# 2. Engine Boundary

## ENG-INIT-01 — Host-Provided Domain Graph

The host must provide a complete domain graph for exactly one Area when requesting runtime initialization.

The graph may include:

- Sessions
- Resolutions
- Receipts

All structural references required by the loaded graph must already be present in the provided graph.

Informational references may also be present according to ENG-DOMAIN.

Initialization must:

- consume informational references according to their declared class
- not reinterpret informational references as structural inputs

Validation outcomes for informational references, including failure conditions, are defined by ENG-INTEGRITY.

During initialization, the Engine must not:

- discover missing objects
- traverse storage
- infer missing references
- reconstruct governance state
- merge multiple Area graphs

Reference classification is defined in ENG-DOMAIN.  
Structural validity enforcement is defined in ENG-INTEGRITY.

---

## ENG-INIT-02 — Initialization Is Runtime Entry, Not History Mutation

Initialization verifies provided structural history.

It does not:

- evolve history
- repair history
- create legitimacy artifacts
- modify domain objects
- rewrite structural graph relationships
- emit receipts
- emit audit as a substitute for validation

The host supplies facts.  
The Engine determines whether those facts permit safe runtime entry.

---

# 3. Rehydration Contract

## ENG-INIT-03 — Deterministic, Side-Effect-Free Construction

Engine rehydration must be:

- deterministic
- side-effect free with respect to domain history
- non-mutating

Rehydration must not:

- change session states or phases
- alter Resolution states
- create or delete objects
- modify structural graph relationships
- promote or demote governance state
- normalize or migrate structural content implicitly

Mutation prohibition during runtime entry is absolute.

Any required migration or repair must occur outside initialization.

---

# 4. Initialization Responsibility Boundary

## ENG-INIT-04 — Initialization Consumes Authoritative Validators

ENG-INITIALIZATION does not define its own independent integrity doctrine.

Instead, it consumes authoritative outcomes from:

- ENG-INTEGRITY → structural safety, halt / degrade outcomes, runtime trust
- ENG-STRUCTURE → graph reconstruction and structural ACTIVE derivation
- ENG-RECEIPT → receipt artifact structure
- ENG-CANON → canonical hash input construction
- ENG-SPECVERIFY → rule provenance verification
- ENG-DOMAIN → object schemas and structural field definitions

ENG-INITIALIZATION is the runtime entry coordinator, not the source of structural truth.

---

# 5. Rehydration Sequence

## ENG-INIT-05 — Deterministic Runtime Entry Sequence

Initialization must execute a deterministic runtime entry sequence.

That sequence must, at minimum, obtain authoritative results for:

1. schema compatibility
2. single-Area graph validity
3. structural reference resolution
4. governance slot structural validity
5. structural graph reconstruction
6. structural ACTIVE derivation
7. receipt structural validity
8. canonical receipt integrity validation
9. specification identity validation
10. runtime mode selection

The exact validation rules are not defined here.

The authoritative definitions belong to the specifications named above.

---

## ENG-INIT-06 — Deterministic Ordering of Reported Failures

Initialization must produce deterministic reporting order for discovered failures.

Ordering rules for reported errors are defined in ENG-ERROR.

ENG-INITIALIZATION requires only that:

- validation not depend on storage order
- initialization not become nondeterministic due to host environment
- runtime entry outcome be reproducible for identical input graphs

---

# 6. Runtime Entry Outcomes

## ENG-INIT-07 — Exactly One Initialization Outcome

After rehydration attempt, initialization must produce exactly one runtime entry outcome:

- NORMAL_RUNTIME
- DEGRADED_READ_ONLY
- INITIALIZATION_FAILURE

ENG-INITIALIZATION does not independently define when degraded mode or failure applies.  
Those rules belong to ENG-INTEGRITY.

---

## ENG-INIT-08 — NORMAL_RUNTIME

NORMAL_RUNTIME is entered only when all required structural and runtime trust validations succeed.

In NORMAL_RUNTIME:

- runtime APIs may proceed
- legitimacy evaluation may occur
- mutating operations may occur if otherwise permitted

---

## ENG-INIT-09 — DEGRADED_READ_ONLY

DEGRADED_READ_ONLY is entered only when ENG-INTEGRITY determines that:

- structural graph validity is sufficient for safe read-only reasoning
- mutation and legitimacy extension are not safe

In DEGRADED_READ_ONLY:

- read-only operations may proceed
- mutation must remain disabled
- legitimacy creation must remain disabled

---

## ENG-INIT-10 — INITIALIZATION_FAILURE

INITIALIZATION_FAILURE occurs when runtime entry safety cannot be established.

If initialization fails:

- the Engine must not enter runtime
- runtime APIs must not execute
- no implicit repair or fallback mode is permitted

---

# 7. Relationship to Structural Validation

## ENG-INIT-11 — Structural Validation Belongs to ENG-INTEGRITY

All structural validation belongs to ENG-INTEGRITY.

ENG-INITIALIZATION must not duplicate:

- mixed-area detection
- schema incompatibility handling
- structural reference validation
- informational reference validation rules
- governance slot validation
- participant epoch validation
- receipt trust validation

ENG-INITIALIZATION consumes those results only.

---

# 8. Relationship to Structural Graph

## ENG-INIT-12 — Structural Graph Is External Authority

ENG-INITIALIZATION must not define structural graph semantics.

It depends on ENG-STRUCTURE for:

- graph reconstruction
- acyclicity truth
- structural ACTIVE derivation
- governance slot structural outcomes

---

# 9. Relationship to Receipts

## ENG-INIT-13 — Receipt Validation Is External

ENG-INITIALIZATION does not define receipt rules.

It depends on:

- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-INTEGRITY

---

# 10. Domain Object Immutability

## ENG-INIT-14 — Domain Objects Are Immutable Inputs

All domain objects are treated as immutable inputs.

Initialization must not:

- rewrite objects
- upgrade schema versions
- modify identifiers
- normalize structure
- alter provenance fields
- rewrite references

---

# 11. No Migration Rule

## ENG-INIT-15 — Initialization Is Never a Migration Mechanism

Initialization must not function as:

- a migration layer
- a normalization layer
- a repair pipeline
- a schema upgrade pipeline

---

# 12. Determinism Guarantee

## ENG-INIT-16 — Deterministic Runtime Entry

Given identical inputs, initialization must produce identical:

- validation outcomes
- runtime mode
- graph reconstruction inputs
- receipt validation inputs

---

# 13. Readiness Definition

## ENG-INIT-17 — Runtime Readiness Is Derived

The Engine is ready only when initialization completes successfully.

Readiness means:

- structural validation passed
- graph reconstructed
- runtime mode established

---

# 14. Relationship to API

## ENG-INIT-18 — Consumed Through API

Initialization is invoked through ENG-API.

ENG-INITIALIZATION defines semantics.  
ENG-API defines invocation.

---

# 15. Mental Model

ENG-INITIALIZATION is the runtime-entry layer.

It answers:

- how the Engine starts
- how runtime mode is determined
- how validation results are consumed

It does not define validation itself.