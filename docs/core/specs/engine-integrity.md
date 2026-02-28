# ENG-INTEGRITY  
Engine Integrity & Runtime Guarantees  
Status: FROZEN (v10 – Single-Area Runtime, No Foreign DAG Evaluation)  
Applies to: Engine Core (V1/V2+)  

---

# 1. Purpose

ENG-INTEGRITY defines the global runtime integrity rules of the Engine Core.

It governs:

- Engine initialization guarantees  
- Structural invariant enforcement  
- Governance bootstrap invariants (Authority & Scope)  
- Area-level acceptance guards  
- Single-Area runtime enforcement  
- Orphan object detection and graph completeness  
- Receipt integrity and rehydration  
- Structural vs informational reference validation  
- Fatal vs degraded failure semantics  
- Atomic commit semantics  
- Legitimacy compiler doctrine  

It does **not** define:

- Session mechanics (ENG-DECISION)  
- Supersession graph structure (ENG-SUPERSESSION)  
- Object schemas (ENG-DOMAIN)  

This specification defines halting conditions, runtime enforcement behavior, and the limits of degraded operation.

---

# 2. Legitimacy Compiler Doctrine

The Engine is a legitimacy compiler.

It:

- Operates on exactly one Area at a time  
- Compiles legitimacy only from successfully rehydrated domain graphs  
- Does not infer legitimacy  
- Does not repair legitimacy  
- Does not partially evaluate corrupted graphs  
- Does not traverse external Areas  
- Does not evaluate foreign DAGs  
- Does not expose validation or evaluation APIs for un-rehydrated graphs  

Legitimacy is:

- Explicit  
- Deterministic  
- Structurally verifiable  
- Mechanically reproducible  
- Strictly Area-local  

If structural integrity of required domain objects cannot be proven,
the Engine must halt or enter degraded read-only mode (as strictly defined below).

Convenience never overrides legitimacy invariants.

---

# 3. No Foreign DAG Evaluation

The Engine exposes no API for:

- Evaluating an arbitrary domain graph without rehydration  
- Validating legitimacy of a foreign DAG outside the active Area runtime  
- Computing ACTIVE sets without successful initialization  

All legitimacy derivation requires:

1. Successful `rehydrate_engine(domain_graph)`  
2. Single-Area structural validation  
3. Deterministic reconstruction of supersession graph and governance state  

There is no “import preview” mode.
There is no relaxed validation mode.
There is no parallel evaluation path.

If rehydration fails, legitimacy evaluation is not permitted.

Any import, transformation, or normalization of external artifacts
is the responsibility of the host system and occurs strictly outside the Engine boundary.

---

# 4. Single-Area Runtime Enforcement

## 4.1 Single-Area Initialization Rule

At any moment, an Engine instance must contain exactly one active Area.

During rehydration:

- All structural domain objects must share identical `area_id`.  
- Mixed-area graphs are prohibited.  
- Cross-area references do not count as multi-Area hosting.  

If multiple structural `area_id` values are detected:

- Initialization must fail with StructuralIntegrityFailure.  
- Engine must halt.

## 4.2 Rehydration Replacement Rule

Calling `rehydrate_engine`:

- Replaces any previously loaded Area state  
- Discards prior legitimacy state entirely  
- Establishes a new single active Area  

The Engine must not:

- Merge Areas  
- Retain governance slot memory from prior Area  
- Preserve supersession state across Areas  

Area switching is exclusively a host responsibility.

---

# 5. Deterministic Restore

On rehydration, the Engine must:

- Load all provided structural domain objects  
- Verify single-area consistency  
- Reconstruct Area-local supersession graph  
- Validate acyclicity  
- Recompute structurally ACTIVE sets  
- Validate exclusive governance slots  
- Validate session and resolution consistency  
- Validate receipt integrity (if receipts provided)  

Restore must be deterministic across implementations.

Cross-area references must never be traversed during restore.

If required structural invariants cannot be reconstructed deterministically,
initialization must fail.

---

# 6. Structural vs Informational References

## 6.1 Structural References

Structural references:

- Affect legitimacy  
- Affect supersession  
- Affect ACTIVE derivation  
- Affect governance slot enforcement  

They must:

- Resolve within the active Area  
- Be validated during restore  
- Be Area-local  

Missing structural references → StructuralIntegrityFailure.

Cross-area structural references are prohibited.

## 6.2 Informational Cross-Area References

Cross-area references:

- Are metadata only  
- Must not affect legitimacy  
- Must not affect ACTIVE derivation  
- Must not be traversed  
- Must not trigger restore failure if unresolved  

They are excluded from structural integrity checks.

---

# 7. Receipt Integrity Rules

Receipts are integrity artifacts.

They:

- Do not create legitimacy  
- Prove that an acceptance or closure event was recorded  

Receipt rules:

- Missing receipts do not invalidate structural legitimacy  
- Missing receipts prevent future acceptance operations  
- Orphaned receipts (no matching session or resolution) → StructuralIntegrityFailure  
- Receipt content hash mismatch → StructuralIntegrityFailure  
- Receipts are immutable  

Receipt verification occurs only after successful structural restore.

Receipt failure must never be downgraded to degraded mode.

---

# 8. Degraded Read-Only Mode

Degraded mode exists solely for controlled recovery scenarios.

It may activate only if:

- Structural domain objects are internally consistent  
- Supersession graph is reconstructable  
- Governance slots are derivable  
- No fatal structural invariant is violated  
- But acceptance safety cannot be guaranteed due to missing non-fatal artifacts (e.g., missing receipts)

In degraded mode:

- No mutating operations are permitted  
- No acceptance is permitted  
- Evaluation may be permitted for informational purposes only  
- `export_area_dag` is permitted  
- All mutating API calls must return `DEGRADED_MODE_ACTIVE`  

Degraded mode must never:

- Mask structural corruption  
- Allow acceptance  
- Allow governance mutation  
- Allow partial legitimacy compilation  

If structural corruption is detected, degraded mode is not allowed — the Engine must halt.

---

# 9. Fatal Structural Integrity Failure

The Engine must halt if any of the following occur:

- Supersession cycle  
- Mixed-area structural object graph  
- Cross-area structural supersession  
- Invalid structural references  
- Governance slot multiplicity or emptiness  
- Schema incompatibility preventing deterministic reconstruction  
- Orphaned receipts  
- Receipt hash mismatch  
- Snapshot inconsistency  
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-DECISION  

Halt behavior:

- No evaluation  
- No mutation  
- No degraded mode  
- Clear invariant violation classification  

No automatic repair permitted.

---

# 10. Determinism Guarantee

- Restore must be deterministic  
- ACTIVE derivation must be deterministic  
- Governance slot evaluation must be deterministic  
- No timestamp-based precedence  
- No UUID-time precedence  
- No ordering-based inference  
- No cross-area influence  

Identical inputs must produce identical runtime state across implementations.

---

# 11. Engine Invariants

- Exactly one Area active at runtime  
- No foreign DAG evaluation  
- No relaxed import mode  
- No partial restore mode  
- Legitimacy only compiled after successful rehydration  
- Structural references must resolve  
- Cross-area references are non-structural  
- Governance slots exclusive  
- Supersession strictly Area-local  
- Determinism mandatory  
- Halt preferred to ambiguity  

---

# 12. Compiler Halt Principle

If legitimacy cannot be mechanically proven from structural domain objects,
the Engine must prefer halt over ambiguity.

The Engine is not a repair tool.
It is a deterministic legitimacy compiler.