# ENG-INTEGRITY — Engine Integrity, Runtime Safety & Structural Halt Guarantees

Status: REFACTORED (v15 – Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)  

Authority: Foundational runtime authority for structural validity, halt conditions, degraded mode, and single-Area enforcement.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-PERSISTENCE
- ENG-COMPILATION

---

# 1. Purpose

ENG-INTEGRITY defines the global runtime integrity rules of the Engine Core.

It is the authoritative specification for:

- Structural halt conditions
- Single-Area runtime enforcement
- Rehydration safety
- Schema compatibility enforcement
- Structural reference validity
- Governance slot exclusivity enforcement
- Participant epoch integrity validation
- Receipt integrity validation at runtime
- Fatal vs degraded mode boundaries
- Resource safety and atomic failure requirements
- Legitimacy compiler runtime doctrine

ENG-INTEGRITY does not redefine:

- object schemas
- supersession graph semantics
- review / retired state semantics
- receipt structure
- canonical serialization format
- atomic persistence transaction boundaries
- specification manifest computation

Those are defined respectively in:

- ENG-DOMAIN
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-RECEIPT
- ENG-CANON
- ENG-PERSISTENCE
- ENG-SPECVERIFY

ENG-INTEGRITY is the authority for deciding whether the Engine may run, must degrade, or must halt.

---

# 2. Legitimacy Compiler Doctrine

## ENG-INTEGRITY-01 — Runtime Legitimacy Compiler

The Engine is a legitimacy compiler.

It:

- operates on exactly one Area at a time
- compiles legitimacy only from successfully rehydrated domain graphs
- does not infer legitimacy
- does not repair legitimacy
- does not partially evaluate structurally corrupted graphs
- does not traverse external Areas
- does not evaluate foreign DAGs
- does not expose evaluation or mutation APIs prior to successful rehydration

Legitimacy must remain:

- explicit
- deterministic
- structurally verifiable
- mechanically reproducible
- strictly Area-local

If structural integrity cannot be proven, the Engine must halt.  
If structural integrity is intact but non-fatal runtime trust conditions fail, the Engine may enter degraded read-only mode.

---

# 3. Schema Compatibility Enforcement

## ENG-INTEGRITY-02 — Schema Compatibility Is Mandatory

Schema compatibility rules are defined in ENG-DOMAIN.

ENG-INTEGRITY is responsible for enforcing those rules during rehydration and restore.

If schema compatibility cannot be verified deterministically:

- initialization fails
- runtime does not start

Degraded mode is not permitted for schema incompatibility.

Examples include:

- unsupported major schema version
- unknown structural enum
- unknown structural field
- missing required structural field

Error reporting details are defined in ENG-ERROR.

---

# 4. No Foreign DAG Evaluation

## ENG-INTEGRITY-03 — Rehydration Is Required

The Engine exposes no legitimacy computation for arbitrary foreign graphs.

All legitimacy derivation requires:

1. successful rehydrate_engine
2. structural validation
3. supersession reconstruction
4. governance slot derivation

No relaxed structural mode exists for legitimacy evaluation.

The Engine may store or export opaque artifacts under other specifications, but must not interpret legitimacy for a graph that has not passed integrity validation.

---

# 5. Single-Area Runtime Enforcement

## ENG-INTEGRITY-04 — Exactly One Area

An Engine instance must contain exactly one active Area.

During rehydration:

- all structural objects must share the same area_id
- mixed-area structural graphs are invalid
- cross-area references may exist only as informational metadata

Mixed-area structural graphs must cause deterministic failure.

---

## ENG-INTEGRITY-05 — Rehydration Replaces Runtime State

Calling rehydrate_engine:

- replaces previously loaded Area state
- discards prior runtime legitimacy state
- establishes a new single active Area

The Engine must never merge structural state from multiple Areas.

---

# 6. Structural Validation Authority

## ENG-INTEGRITY-06 — Integrity Is the Runtime Validation Authority

ENG-INTEGRITY is the authority for runtime structural validation.

During rehydration and restore it must validate:

- schema compatibility
- single-Area graph validity
- structural reference resolution
- governance slot exclusivity
- participant epoch integrity
- receipt integrity at runtime
- structural readiness for supersession reconstruction

ENG-INTEGRITY does not define how supersession works or how usability states are interpreted.  
It consumes those results from:

- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED

It determines whether the runtime can safely proceed with them.

---

# 7. Structural vs Informational References

## ENG-INTEGRITY-07 — Structural References Must Resolve

Structural references:

- affect legitimacy
- affect supersession
- affect ACTIVE derivation
- affect restore safety

Structural references must resolve locally within the active Area.

Missing structural references must cause deterministic failure.

Cross-area structural references are prohibited.

Informational cross-area references:

- are metadata only
- must not affect legitimacy
- must not affect ACTIVE derivation
- must not be traversed for restore or runtime validation

Reference classification is defined structurally in ENG-DOMAIN.

---

# 8. Governance Slot Enforcement

## ENG-INTEGRITY-08 — Governance Slots Must Be Structurally Valid

Governance slot structure is defined in ENG-DOMAIN.  
ACTIVE derivation is defined in ENG-SUPERSESSION.  
Usability suspension semantics are defined in ENG-REVIEW-RETIRED.

ENG-INTEGRITY is responsible for ensuring the runtime sees a structurally valid governance configuration.

It must validate:

- Authority slot exclusivity
- Scope slot exclusivity
- no structurally invalid governance multiplicity
- no structurally unresolved governance references

If structural governance validity cannot be proven, runtime must not proceed.

ENG-INTEGRITY does not redefine Authority or Scope usability rules.  
It enforces that runtime operation cannot continue if those rules produce unsafe governance state.

---

# 9. Participant Epoch Integrity

## ENG-INTEGRITY-09 — Epoch Integrity Is Structural

Participant identity is session-scoped and epoch-based.

ENG-INTEGRITY must validate:

- no participant_id reuse within a session
- no merge of historical participation epochs
- round snapshots align with frozen session history as represented in receipts
- epoch boundaries remain deterministic across restore

Participant schema is defined in ENG-DOMAIN.  
Round and receipt structure are defined in ENG-RECEIPT.  
Session lifecycle boundaries are defined in ENG-SESSION.

ENG-INTEGRITY is the runtime authority that determines whether those structures are safe and valid.

---

# 10. Receipt Integrity Rules

## ENG-INTEGRITY-10 — Receipt Integrity Is Runtime-Critical

Receipts are integrity artifacts.  
They do not create legitimacy, but runtime must verify them where required.

ENG-INTEGRITY must validate, for runtime safety:

- terminal sessions have exactly one receipt
- no orphaned receipts exist where prohibited
- receipt content_hash verifies
- canonical serialization is valid
- declared hash_algorithm is valid
- receipt snapshots are structurally consistent
- rule identity fields are acceptable for runtime interpretation under ENG-SPECVERIFY

Receipt structure is defined in ENG-RECEIPT.  
Canonical encoding is defined in ENG-CANON.  
Rule identity semantics are defined in ENG-SPECVERIFY.

ENG-INTEGRITY is responsible for deciding whether receipt validation success, degraded mode, or halt follows.

Historical receipts remain authoritative even if later Resolution usability changes under ENG-REVIEW-RETIRED.

---

# 11. ACTIVE Derivation & Usability Consumption

## ENG-INTEGRITY-11 — Integrity Consumes, It Does Not Define, ACTIVE and Usability

Structural ACTIVE derivation belongs to ENG-SUPERSESSION.  
UNDER_REVIEW / RETIRED usability semantics belong to ENG-REVIEW-RETIRED.

ENG-INTEGRITY consumes those outcomes to determine whether runtime can proceed safely.

This includes:

- governance slot readiness
- session evaluation readiness
- restore-time runtime safety
- acceptance guard safety

ENG-INTEGRITY must never redefine graph semantics or administrative usability semantics independently.

---

# 12. Degraded Read-Only Mode

## ENG-INTEGRITY-12 — Degraded Mode Is Contained Runtime Damage

Degraded mode may activate only if:

- structural graph is internally consistent
- schema compatibility is satisfied
- supersession graph can be reconstructed
- governance slots can be derived
- runtime trust or completeness is insufficient for safe mutation or acceptance

Examples may include:

- non-fatal receipt trust issues as permitted by runtime policy
- missing optional non-structural artifacts
- host-configured artifacts unavailable where not structurally required

In degraded mode:

- no mutating operations allowed
- no acceptance allowed
- no incremental legitimacy extension allowed
- evaluation permitted for informational use only where safe
- DAG export permitted

Degraded mode must never mask structural corruption.

If structural corruption exists, the Engine must halt instead.

---

# 13. Resource Exhaustion & Atomic Failure

## ENG-INTEGRITY-13 — Determinism Exists Within a Resource Envelope

The Engine guarantees deterministic behavior only within a sufficient resource envelope.

Resource ceilings are implementation-defined.

---

## ENG-INTEGRITY-14 — Resource Failure Must Be Atomic or Fatal

If resource exhaustion occurs during:

- rehydration
- restore validation
- ACTIVE derivation
- acceptance
- supersession application
- receipt verification
- canonical serialization
- hash computation

the Engine must:

- abort the operation atomically
- leave structural state unchanged
- emit no partial structural results
- emit no partial transitions

If atomic safety cannot be guaranteed, the Engine must halt.

Atomic persistence boundaries themselves are defined in ENG-PERSISTENCE.  
ENG-INTEGRITY governs runtime safety if those guarantees cannot be maintained.

---

# 14. Fatal Structural Integrity Failure

## ENG-INTEGRITY-15 — Halt Conditions

The Engine must halt if structural legitimacy cannot be proven.

Fatal structural failures include, at minimum:

- supersession cycle detected
- mixed-area structural graph detected
- cross-area structural supersession detected
- unresolved structural references
- unsupported schema version
- unknown structural enum or field
- governance slot multiplicity or structurally invalid emptiness
- participant epoch reuse
- terminal receipt missing where required
- receipt snapshot mismatch where runtime policy makes it fatal
- receipt hash mismatch where runtime policy makes it fatal
- partial mutation after resource failure
- any structural inconsistency that prevents safe legitimacy compilation

No automatic repair is permitted.

ENG-ERROR defines reporting form.  
ENG-INTEGRITY defines whether runtime may continue.

---

# 15. Determinism Guarantee

## ENG-INTEGRITY-16 — Runtime Structural Determinism

Within schema compatibility and resource envelope constraints, ENG-INTEGRITY must guarantee deterministic runtime outcomes for:

- restore validation
- structural readiness determination
- governance slot validity
- participant epoch validation
- receipt runtime integrity validation
- runtime mode selection (normal, degraded, halt)

ENG-INTEGRITY does not define historical replay ordering or graph precedence rules.  
Those belong to ENG-COMPILATION and ENG-SUPERSESSION.

It must, however, apply their outputs deterministically.

No storage order, environment variation, or implicit timestamp ordering may alter integrity outcomes.

---

# 16. Relationship to Atomic Persistence

## ENG-INTEGRITY-17 — Integrity Requires Persistence Guarantees but Does Not Define Them

Atomic commit boundaries are defined in ENG-PERSISTENCE.

ENG-INTEGRITY requires that runtime trust in legitimacy artifacts depends on those guarantees remaining true.

If runtime evidence indicates those guarantees were violated:

- degraded mode or halt must result, depending on whether structural legitimacy can still be safely reasoned about

ENG-INTEGRITY does not redefine the persistence transaction model.

---

# 17. Engine Invariants

- exactly one Area active at runtime
- no foreign DAG legitimacy evaluation
- no partial restore mode for legitimacy
- schema compatibility enforced before runtime legitimacy compilation
- structural references must resolve
- governance slots structurally valid
- participant epochs structurally enforced
- runtime determinism mandatory
- resource failure atomic or fatal
- ACTIVE derivation consumed from ENG-SUPERSESSION
- UNDER_REVIEW / RETIRED usability consumed from ENG-REVIEW-RETIRED
- receipt integrity validated through ENG-RECEIPT + ENG-CANON + ENG-SPECVERIFY
- atomic persistence guarantees assumed from ENG-PERSISTENCE and enforced as runtime trust conditions

---

# 18. Compiler Halt Principle

If legitimacy cannot be mechanically proven from structural domain objects, the Engine must halt.

If atomic runtime safety cannot be guaranteed, the Engine must halt.

If structural validity is preserved but runtime trust is insufficient for mutation, the Engine may degrade to read-only mode.

ENG-INTEGRITY is the final runtime authority for making that distinction.