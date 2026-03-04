# ENG-INTEGRITY  
Engine Integrity, Runtime & Resource Guarantees  
Status: FROZEN (v12 – Resource Exhaustion & Atomic Failure Integrated)  
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
- Participant epoch integrity invariants  
- Fatal vs degraded failure semantics  
- Atomic commit semantics  
- Resource exhaustion semantics  
- Legitimacy compiler doctrine  

It does **not** define:

- Session mechanics (ENG-SESSION)  
- Supersession graph structure (ENG-SUPERSESSION)  
- Object schemas (ENG-DOMAIN)  

This specification defines halting conditions, runtime enforcement behavior, and the limits of degraded and resource-constrained operation.

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
the Engine must halt or enter degraded read-only mode (strictly defined below).

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

There is no relaxed validation mode.  
There is no parallel evaluation path.  

If rehydration fails, legitimacy evaluation is not permitted.

All import, transformation, normalization, or pre-validation of external artifacts
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
- Validate participant epoch invariants  
- Validate receipt integrity (if receipts provided)  

Restore must be deterministic across implementations within a sufficient resource envelope.

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

# 7. Participant Epoch Integrity Invariants

Participant identity is session-scoped and epoch-based.

The following invariants are structural and must be validated during restore:

1. **No participant_id reuse within a Session**

   - A `participant_id` must appear at most once within a Session’s lifecycle.  
   - A `participant_id` must never be reassigned to represent a different participation epoch.  
   - Duplicate or conflicting reuse → StructuralIntegrityFailure.

2. **Snapshot participant IDs must match session state at acceptance**

   For any ACCEPTED session:

   - The `participant_snapshot` recorded in the receipt must exactly match  
     the final active participant epoch set at the moment of acceptance.  
   - No missing participant_id values.  
   - No additional participant_id values.  
   - No rewritten or merged epochs.  

   Any mismatch between:

   - Frozen session state  
   - Resolution snapshot  
   - Receipt participant_snapshot  

   → StructuralIntegrityFailure.

3. **No historical epoch merging**

   - Participation epochs terminated by resume or removal must not reappear.  
   - Receipt snapshots must reference only the final epoch set.  

These invariants ensure deterministic reproduction of session state and receipt artifacts.

---

# 8. Receipt Integrity Rules

Receipts are integrity artifacts.

They:

- Do not create legitimacy  
- Prove that an acceptance or closure event was recorded  

Receipt rules:

- Missing receipts do not invalidate structural legitimacy  
- Missing receipts prevent future acceptance operations  
- Orphaned receipts (no matching session or resolution) → StructuralIntegrityFailure  
- Receipt content hash mismatch → StructuralIntegrityFailure  
- Snapshot mismatch → StructuralIntegrityFailure  
- Receipts are immutable  

Receipt verification occurs only after successful structural restore.

Receipt failure must never be downgraded to degraded mode.

---

# 9. Degraded Read-Only Mode

Degraded mode exists solely for controlled recovery scenarios.

It may activate only if:

- Structural domain objects are internally consistent  
- Supersession graph is reconstructable  
- Governance slots are derivable  
- No fatal structural invariant is violated  
- Acceptance safety cannot be guaranteed due to missing non-fatal artifacts (e.g., missing receipts)

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

# 10. Resource Exhaustion & Atomic Failure

## 10.1 Resource Envelope Assumption

The Engine guarantees deterministic behavior within a sufficient resource envelope.

The specification does not define:

- Maximum graph size  
- Maximum session count  
- Maximum receipt size  
- Memory ceilings  
- CPU ceilings  

Resource limits are implementation-defined.

Logical determinism applies only when sufficient resources are available.

---

## 10.2 Atomic Failure Requirement

If resource exhaustion occurs during:

- Rehydration  
- ACTIVE derivation  
- Acceptance  
- Receipt emission  
- Supersession application  
- Canonical serialization  
- Hash computation  

The Engine must:

- Fail explicitly  
- Abort the operation atomically  
- Leave all structural domain objects unchanged  
- Emit no partial receipts  
- Emit no partial state transitions  

The Engine must not:

- Partially mutate domain state  
- Partially emit legitimacy artifacts  
- Silently truncate graph validation  
- Succeed with incomplete validation  

Resource exhaustion must never result in:

- Divergent legitimacy state  
- Partially applied acceptance  
- Partially reconstructed supersession graph  

If atomicity cannot be guaranteed, the Engine must halt.

---

# 11. Fatal Structural Integrity Failure

The Engine must halt if any of the following occur:

- Supersession cycle  
- Mixed-area structural object graph  
- Cross-area structural supersession  
- Invalid structural references  
- Governance slot multiplicity or emptiness  
- Schema incompatibility preventing deterministic reconstruction  
- Orphaned receipts  
- Receipt hash mismatch  
- Snapshot inconsistency (including participant epoch mismatch)  
- participant_id reuse within a session  
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-SESSION  
- Any resource exhaustion that leaves state partially mutated  

Halt behavior:

- No evaluation  
- No mutation  
- No degraded mode  
- Clear invariant violation classification  

No automatic repair permitted.

---

# 12. Determinism Guarantee

Within the resource envelope:

- Restore must be deterministic  
- ACTIVE derivation must be deterministic  
- Governance slot evaluation must be deterministic  
- Participant epoch validation deterministic  
- No timestamp-based precedence  
- No UUID-time precedence  
- No ordering-based inference  
- No cross-area influence  

Identical inputs and identical resource limits must produce identical runtime state across implementations.

---

# 13. Engine Invariants

- Exactly one Area active at runtime  
- No foreign DAG evaluation  
- No relaxed import mode  
- No partial restore mode  
- Legitimacy only compiled after successful rehydration  
- Structural references must resolve  
- Cross-area references are non-structural  
- Governance slots exclusive  
- Supersession strictly Area-local  
- Participant epochs strictly enforced  
- Determinism mandatory  
- Resource failure atomic  
- Halt preferred to ambiguity  

---

# 14. Compiler Halt Principle

If legitimacy cannot be mechanically proven from structural domain objects,
the Engine must prefer halt over ambiguity.

If atomic safety cannot be guaranteed under resource exhaustion,
the Engine must prefer halt over partial mutation.

The Engine is not a repair tool.  
It is a deterministic legitimacy compiler operating within a resource envelope.