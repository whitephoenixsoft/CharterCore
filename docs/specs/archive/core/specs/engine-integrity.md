# ENG-INTEGRITY — Engine Integrity, Runtime Safety & Structural Halt Guarantees

Status: REFACTORED (v19 – Determinism Closure & Policy Elimination)  
Applies to: Engine Core (V1/V2+)  

Authority: Foundational runtime authority for structural validity, halt conditions, degraded mode, and single-Area enforcement.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-STRUCTURE
- ENG-USABILITY
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-PERSISTENCE
- ENG-COMPILATION

---

# 1. Purpose

ENG-INTEGRITY defines the global runtime integrity rules of the Engine Core.

It is the authoritative specification for:

- structural halt conditions
- single-Area runtime enforcement
- rehydration safety validation
- schema compatibility enforcement
- structural reference validity
- governance slot exclusivity enforcement
- participant epoch integrity validation
- receipt integrity validation at runtime
- fatal vs degraded mode boundaries
- resource safety and atomic failure requirements
- legitimacy compiler runtime doctrine

ENG-INTEGRITY does not redefine:

- object schemas
- structural graph semantics or ACTIVE derivation
- ON_HOLD / RETIRED usability semantics
- receipt structure
- canonical serialization format
- atomic persistence transaction boundaries
- specification manifest computation

Those are defined respectively in:

- ENG-DOMAIN
- ENG-STRUCTURE
- ENG-USABILITY
- ENG-RECEIPT
- ENG-CANON
- ENG-PERSISTENCE
- ENG-SPECVERIFY

ENG-INTEGRITY is the authority for determining whether the Engine may run, must degrade, or must halt.

---

# 1a. Relationship to Initialization

## ENG-INTEGRITY-00 — Initialization Consumes Integrity Outcomes

ENG-INTEGRITY does not establish runtime entry.

It produces authoritative validation outcomes consumed by:

- ENG-INITIALIZATION → runtime entry decision
- ENG-API → rehydration invocation surface

ENG-INTEGRITY determines:

- whether structural integrity holds
- whether runtime trust is sufficient
- whether degraded mode is required
- whether fatal halt conditions exist

ENG-INITIALIZATION determines:

- whether runtime entry occurs
- which runtime mode is entered

ENG-INTEGRITY is therefore a validation authority, not a runtime entry coordinator.

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
If structural integrity is intact but runtime trust conditions fail, the Engine may enter degraded read-only mode.

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
3. structural graph reconstruction
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
- structural reference resolution (including candidate action targets)
- governance slot exclusivity
- participant epoch integrity
- receipt integrity at runtime
- structural readiness for graph reconstruction

ENG-INTEGRITY does not define how the structural graph works or how usability states are interpreted.  
It consumes those results from:

- ENG-STRUCTURE
- ENG-USABILITY

It determines whether the runtime can safely proceed with them.

---

## ENG-INTEGRITY-06A — Integrity Is Validation Only

ENG-INTEGRITY must not:

- mutate domain objects
- repair structural inconsistencies
- reconstruct missing artifacts
- regenerate receipts
- alter governance structure
- resolve conflicts automatically

ENG-INTEGRITY is purely evaluative.

All failures are descriptive and must be surfaced through ENG-ERROR.

Any repair must occur outside the Engine or through explicit Engine operations governed by other specifications.

---

## ENG-INTEGRITY-06B — Complete Validation Requirement

ENG-INTEGRITY must detect and report all applicable structural and runtime integrity violations defined by governing specifications.

Implementations must not omit applicable violations.

---

# 7. Structural vs Informational References

## ENG-INTEGRITY-07 — Structural References Must Resolve

Structural references:

- affect legitimacy
- affect structural graph behavior
- affect ACTIVE derivation
- affect restore safety

Structural references must resolve locally within the active Area.

Missing structural references must cause deterministic failure.

Cross-area structural edges are prohibited.

Reference classification is defined in ENG-DOMAIN.

---

## ENG-INTEGRITY-07A — Informational References Are Non-Structural but Must Be Consistent

Informational references:

- do not affect legitimacy
- do not affect ACTIVE derivation
- must not be interpreted as structural edges

### Cross-Area Informational References

- are metadata only
- are not required to resolve locally

### Intra-Area Informational Resolution References

- must reference existing Resolution objects within the same Area

If such a reference does not resolve:

- initialization must fail deterministically

This requirement is a structural integrity constraint defined by ENG-DOMAIN, not a semantic interpretation of the reference.

---

# 8. Governance Slot Enforcement

## ENG-INTEGRITY-08 — Governance Slots Must Be Structurally Valid

ENG-INTEGRITY must validate:

- Authority slot exclusivity
- Scope slot exclusivity
- no structurally invalid governance multiplicity
- no structurally unresolved governance references

If structural governance validity cannot be proven, runtime must not proceed.

---

# 9. Participant Epoch Integrity

## ENG-INTEGRITY-09 — Epoch Integrity Is Structural

ENG-INTEGRITY must validate:

- no participant_id reuse within a session
- no merge of historical participation epochs
- round snapshots align with frozen session history
- epoch boundaries remain deterministic across restore

---

# 10. Receipt Integrity Rules

## ENG-INTEGRITY-10 — Receipt Integrity Is Runtime Trust-Critical

ENG-INTEGRITY must validate:

- terminal sessions have exactly one receipt
- no orphaned receipts exist where prohibited
- receipt content_hash verifies
- canonical serialization is valid
- declared hash_algorithm is valid
- receipt snapshots are structurally consistent
- rule identity fields are valid under ENG-SPECVERIFY

Receipt validation failures must be deterministically classified as:

- fatal (halt)
- non-fatal (degraded)

This classification must not depend on runtime configuration or implementation policy.

---

# 11. ACTIVE Derivation & Usability Consumption

## ENG-INTEGRITY-11 — Integrity Consumes Structure and Usability

ENG-INTEGRITY consumes:

- structural ACTIVE derivation from ENG-STRUCTURE
- usability semantics from ENG-USABILITY

It must not redefine either.

---

# 12. Degraded Read-Only Mode

## ENG-INTEGRITY-12 — Degraded Mode Is Deterministic

Degraded mode activation must be fully determined by rule evaluation.

A condition must be classified as either:

- fatal (requires halt), or
- non-fatal (permits degraded mode)

This classification must be:

- explicitly defined by governing specifications
- deterministic for identical inputs
- independent of implementation or configuration

ENG-INTEGRITY must not rely on implementation-defined policy.

In degraded mode:

- no mutating operations allowed
- no acceptance allowed
- no legitimacy extension allowed
- evaluation permitted for informational use only
- DAG export permitted

---

# 13. Resource Exhaustion & Atomic Failure

## ENG-INTEGRITY-13 — Determinism Within Resource Envelope

Within a given implementation and resource configuration:

- identical inputs must produce identical outcomes

---

## ENG-INTEGRITY-14 — Atomic Failure Requirement

If resource exhaustion occurs, the Engine must:

- abort the operation atomically
- leave structural state unchanged
- emit no partial structural results

If atomic safety cannot be guaranteed, the Engine must halt.

The determination of atomic safety must be deterministic for identical inputs and environment conditions.

---

# 14. Fatal Structural Integrity Failure

## ENG-INTEGRITY-15 — Halt Conditions

The Engine must halt if structural legitimacy cannot be proven.

Fatal failures include:

- structural graph cycle detected
- mixed-area structural graph detected
- cross-area structural edge detected
- unresolved structural references
- unsupported schema version
- unknown structural enum or field
- governance slot multiplicity or invalid emptiness
- participant epoch reuse
- terminal receipt missing
- receipt hash mismatch (when classified fatal)
- receipt snapshot mismatch (when classified fatal)
- partial mutation after resource failure

Additionally:

- unresolved intra-Area informational Resolution references must cause initialization failure

No automatic repair is permitted.

---

# 15. Determinism Guarantee

## ENG-INTEGRITY-16 — Runtime Structural Determinism

Within constraints of schema compatibility and resource envelope, ENG-INTEGRITY must produce deterministic outcomes.

---

## ENG-INTEGRITY-16A — Deterministic Input Closure

Deterministic integrity outcomes require identical:

- domain graph (canonical structural content)
- receipt artifacts
- runtime inputs
- spec_set_hash
- schema versions
- canonical serialization rules

Outcomes must not depend on implicit or external context.

---

## ENG-INTEGRITY-16B — Runtime Trust Determinism

Runtime trust must be derived solely from deterministic validation outcomes.

It must not depend on heuristic or implementation-specific interpretation.

---

# 16. Relationship to Atomic Persistence

## ENG-INTEGRITY-17 — Persistence Assumptions

ENG-INTEGRITY consumes atomic persistence guarantees from ENG-PERSISTENCE.

If those guarantees are violated:

- degraded mode or halt must result deterministically

---

# 17. Engine Invariants

- exactly one Area active
- no foreign DAG evaluation
- no partial restore for legitimacy
- schema compatibility enforced
- structural references must resolve
- informational references remain non-structural
- governance slots valid
- participant epochs valid
- receipt integrity validated
- runtime determinism mandatory
- degraded vs halt classification deterministic
- resource failure atomic or fatal
- no repair or mutation by integrity

---

# 18. Compiler Halt Principle

If legitimacy cannot be mechanically proven from structural domain objects, the Engine must halt.

If atomic runtime safety cannot be guaranteed, the Engine must halt.

If structural validity is preserved but trust is insufficient for mutation, the Engine may degrade to read-only mode.

ENG-INTEGRITY is the final runtime authority.