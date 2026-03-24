# ENG-INITIALIZATION — Engine Rehydration & Runtime Entry Specification

Status: REFACTORED (v10 – Informational Reference Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Engine rehydration, structural runtime entry, and runtime mode establishment

Authority: Runtime entry specification subordinate to ENG-API and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-INTEGRITY
- ENG-SUPERSESSION
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
- supersession graph semantics
- receipt structure
- canonical serialization rules
- specification identity semantics

Those are defined respectively in:

- ENG-INTEGRITY
- ENG-DOMAIN
- ENG-SUPERSESSION
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
- Annotations

All structural references required by the loaded graph must already be present in the provided graph.

Informational references may also be present according to ENG-DOMAIN.  
If present, they are consumed according to their declared class and must not be reinterpreted during initialization.

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
- rewrite supersession structure
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
- modify supersession edges
- promote or demote governance state
- normalize or migrate structural content implicitly

Mutation prohibition during runtime entry is absolute.

Any required migration or repair must occur outside initialization and outside this specification.

---

# 4. Initialization Responsibility Boundary

## ENG-INIT-04 — Initialization Consumes Authoritative Validators

ENG-INITIALIZATION does not define its own independent integrity doctrine.

Instead, it consumes authoritative outcomes from:

- ENG-INTEGRITY → structural safety, halt / degrade outcomes, runtime trust
- ENG-SUPERSESSION → graph reconstruction and structural ACTIVE derivation
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
5. supersession graph reconstruction
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

ENG-INITIALIZATION defines only that runtime entry must resolve to one of these mutually exclusive outcomes.

---

## ENG-INIT-08 — NORMAL_RUNTIME

NORMAL_RUNTIME is entered only when all required structural and runtime trust validations succeed according to ENG-INTEGRITY and the authorities it consumes.

In NORMAL_RUNTIME:

- runtime APIs may proceed subject to their own behavioral specifications
- legitimacy evaluation may occur
- mutating operations may occur if otherwise permitted

---

## ENG-INIT-09 — DEGRADED_READ_ONLY

DEGRADED_READ_ONLY is entered only when ENG-INTEGRITY determines that:

- structural graph validity is sufficient for safe read-only reasoning
- mutation and legitimacy extension are not safe

In DEGRADED_READ_ONLY:

- read-only operations may proceed as allowed elsewhere
- mutation must remain disabled
- legitimacy creation must remain disabled

ENG-INITIALIZATION must not reinterpret degraded mode policy.  
It must only apply it.

---

## ENG-INIT-10 — INITIALIZATION_FAILURE

INITIALIZATION_FAILURE occurs when runtime entry safety cannot be established.

If initialization fails:

- the Engine must not enter runtime
- runtime APIs that require successful rehydration must not execute
- no implicit repair or fallback legitimacy mode is permitted

Failure semantics are governed by ENG-INTEGRITY and ENG-ERROR.

---

# 7. Relationship to Structural Validation

## ENG-INIT-11 — Structural Validation Belongs to ENG-INTEGRITY

ENG-INITIALIZATION must not duplicate structural integrity doctrine.

Structural validation, including but not limited to:

- mixed-area graph detection
- schema incompatibility
- unresolved structural references
- invalid informational references where their governing specification requires local resolution
- governance slot invalidity
- participant epoch violations
- receipt trust failures
- partial mutation trust failures

belongs to ENG-INTEGRITY.

ENG-INITIALIZATION consumes the result of those validations to decide whether runtime entry is possible.

---

# 8. Relationship to Supersession Reconstruction

## ENG-INIT-12 — Graph Reconstruction Belongs to ENG-SUPERSESSION

ENG-INITIALIZATION must not define supersession graph semantics.

It depends on ENG-SUPERSESSION for:

- graph reconstruction
- acyclicity truth
- structural ACTIVE derivation
- graph-valid governance slot participation outcomes

ENG-INITIALIZATION uses those results only to establish runtime readiness.

---

# 9. Relationship to Receipts

## ENG-INIT-13 — Receipt Structure and Integrity Are Consumed, Not Defined, Here

ENG-INITIALIZATION does not define:

- receipt schema
- receipt round structure
- content_hash semantics
- canonical serialization rules
- rule identity semantics

It depends on:

- ENG-RECEIPT for receipt artifact structure
- ENG-CANON for canonical serialization and hash input rules
- ENG-SPECVERIFY for rule provenance validation
- ENG-INTEGRITY for runtime consequence of any failure

Initialization must therefore treat receipts as already-defined artifacts whose validity is externally determined.

---

# 10. Relationship to Domain Schema

## ENG-INIT-14 — Domain Objects Are Immutable Inputs

All domain objects presented during initialization are treated as immutable historical inputs.

Initialization must not:

- rewrite objects
- upgrade schema versions
- modify identifiers
- normalize structure
- alter provenance fields
- rewrite cross references

ENG-DOMAIN defines object shape.  
ENG-INITIALIZATION consumes those objects exactly as provided.

---

# 11. No Migration Rule

## ENG-INIT-15 — Initialization Is Never a Migration Mechanism

Initialization must not function as:

- a migration layer
- a normalization layer
- a repair pipeline
- a schema upgrade pipeline

If the graph is not acceptable in its provided structural form, runtime entry must fail or degrade according to ENG-INTEGRITY.

---

# 12. Determinism Guarantee

## ENG-INIT-16 — Identical Input Graph, Identical Runtime Entry Outcome

Given identical provided domain objects, initialization must produce identical:

- structural validation results
- runtime mode outcome
- supersession reconstruction inputs consumed
- receipt validation inputs consumed
- rule identity verification inputs consumed

Initialization must not depend on:

- storage ordering
- file ordering
- external timestamps
- host environment differences
- runtime iteration order

This is a runtime-entry determinism guarantee, not an independent structural validation doctrine.

---

# 13. Readiness Definition

## ENG-INIT-17 — Runtime Readiness Is a Derived Result

The Engine is ready for runtime only when initialization completes successfully into a valid runtime mode.

Readiness means:

- the provided graph has passed required runtime-entry validation
- the Engine has a coherent single-Area structural universe
- dependent authorities have supplied acceptable results
- runtime mode has been established deterministically

Readiness does not imply:

- that any particular session can accept
- political or social legitimacy
- governance approval beyond structural readiness
- future mutation success

Initialization grants runtime entry, not outcome success.

---

# 14. Relationship to API

## ENG-INIT-18 — Runtime Entry Is Consumed Through ENG-API

ENG-INITIALIZATION does not define a separate public execution path.

Runtime entry is consumed through the API boundary defined in ENG-API, including rehydrate_engine and any read-only runtime mode behavior exposed there.

ENG-INITIALIZATION defines runtime-entry semantics.  
ENG-API defines how callers invoke them.

---

# 15. Mental Model

ENG-INITIALIZATION is the runtime-entry layer.

It answers:

- how the Engine starts from provided structural history
- how rehydration is bounded
- how runtime mode is established
- how authoritative validation results are consumed before runtime begins

It does not answer:

- whether the graph is structurally valid in detail
- how supersession works
- how receipts are canonically hashed
- whether the Engine must halt versus degrade
- whether a session may accept

Those belong elsewhere.

ENG-INITIALIZATION coordinates runtime entry.  
It does not redefine the authorities it depends on.