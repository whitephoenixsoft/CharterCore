# ENG-INTEGRITY  
Engine Integrity, Runtime, Resource & Compatibility Guarantees  
Status: FROZEN (v13 – Schema Compatibility & Version Enforcement Integrated)  
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
- Schema compatibility enforcement  
- Legitimacy compiler doctrine  

It does **not** define:

- Session mechanics (ENG-SESSION)  
- Supersession graph structure (ENG-SUPERSESSION)  
- Object schemas (ENG-DOMAIN)  

This specification defines halting conditions, compatibility enforcement, runtime guarantees, and the limits of degraded and resource-constrained operation.

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

If structural integrity or schema compatibility cannot be proven,
the Engine must halt or enter degraded read-only mode (strictly defined below).

Convenience never overrides legitimacy invariants.

---

# 3. Schema Compatibility Enforcement

Schema compatibility is enforced during rehydration.

## 3.1 Major Version Enforcement

If any structural domain object contains a `schema_version`
whose **major version** exceeds the Engine’s supported major version:

- Rehydration must fail deterministically.
- Error classification must be `UNSUPPORTED_SCHEMA_VERSION`.
- Degraded mode is not permitted.

Major version mismatches are fatal because structural semantics may differ.

---

## 3.2 Unknown Enum Handling

If any structural enum field contains an unknown value:

- Rehydration must fail deterministically.
- Error classification must be `UNSUPPORTED_SCHEMA_VERSION`.
- Degraded mode is not permitted.

Unknown enum values must never:

- Be ignored  
- Be coerced  
- Be mapped to defaults  
- Be treated as informational  

Enums are structural.

---

## 3.3 Unknown Field Handling

During rehydration:

- Unknown structural fields → `UNSUPPORTED_SCHEMA_VERSION`
- Unknown informational fields → ignored

Unknown informational fields must:

- Not participate in canonical hashing  
- Not affect legitimacy  
- Not affect restore validation  
- Not alter ACTIVE derivation  

Canonical reconstruction must include only recognized structural fields.

If structural/informational classification cannot be determined deterministically,
restore must fail.

---

## 3.4 Minor Version Forward Compatibility

Minor version increments must be strictly additive.

If a domain object’s minor version exceeds the Engine’s supported minor version:

- Rehydration may proceed only if all new fields are informational.
- If new structural meaning is detected, restore must fail.

Enum expansion requires a major version bump.

Structural interpretation must never be forward-inferred.

---

# 4. No Foreign DAG Evaluation

The Engine exposes no API for:

- Evaluating an arbitrary domain graph without rehydration  
- Validating legitimacy of a foreign DAG outside the active Area runtime  
- Computing ACTIVE sets without successful initialization  

All legitimacy derivation requires:

1. Successful `rehydrate_engine(domain_graph)`  
2. Single-Area structural and schema validation  
3. Deterministic reconstruction of supersession graph and governance state  

There is no relaxed validation mode.  
There is no parallel evaluation path.  

If rehydration fails, legitimacy evaluation is not permitted.

All import, transformation, normalization, or pre-validation of external artifacts
is the responsibility of the host system and occurs strictly outside the Engine boundary.

---

# 5. Single-Area Runtime Enforcement

## 5.1 Single-Area Initialization Rule

At any moment, an Engine instance must contain exactly one active Area.

During rehydration:

- All structural domain objects must share identical `area_id`.  
- Mixed-area graphs are prohibited.  
- Cross-area references do not count as multi-Area hosting.  

If multiple structural `area_id` values are detected:

- Initialization must fail with StructuralIntegrityFailure.  
- Engine must halt.

## 5.2 Rehydration Replacement Rule

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

# 6. Deterministic Restore

On rehydration, the Engine must:

- Load all provided structural domain objects  
- Validate schema compatibility  
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

# 7. Structural vs Informational References

## 7.1 Structural References

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

## 7.2 Informational Cross-Area References

Cross-area references:

- Are metadata only  
- Must not affect legitimacy  
- Must not affect ACTIVE derivation  
- Must not be traversed  
- Must not trigger restore failure if unresolved  

They are excluded from structural integrity checks.

---

# 8. Participant Epoch Integrity Invariants

Participant identity is session-scoped and epoch-based.

The following invariants are structural and must be validated during restore:

1. No `participant_id` reuse within a Session.  
2. Snapshot participant IDs must exactly match final active epoch set at acceptance.  
3. No historical epoch merging permitted.  

Any violation → StructuralIntegrityFailure.

These invariants ensure deterministic reproduction of session state and receipt artifacts.

---

# 9. Receipt Integrity Rules

Receipts are integrity artifacts.

They:

- Do not create legitimacy  
- Prove that an acceptance or closure event was recorded  

Receipt rules:

- Missing receipts do not invalidate structural legitimacy  
- Missing receipts prevent future acceptance operations  
- Orphaned receipts → StructuralIntegrityFailure  
- Receipt content hash mismatch → StructuralIntegrityFailure  
- Snapshot mismatch → StructuralIntegrityFailure  
- Receipts are immutable  

Receipt verification occurs only after successful structural and schema restore.

Receipt failure must never be downgraded to degraded mode.

---

# 10. Degraded Read-Only Mode

Degraded mode exists solely for controlled recovery scenarios.

It may activate only if:

- Structural domain objects are internally consistent  
- Schema compatibility is satisfied  
- Supersession graph is reconstructable  
- Governance slots are derivable  
- No fatal structural invariant is violated  
- Acceptance safety cannot be guaranteed due to missing non-fatal artifacts  

In degraded mode:

- No mutating operations are permitted  
- No acceptance is permitted  
- Evaluation may be permitted for informational purposes only  
- `export_area_dag` is permitted  
- All mutating API calls must return `DEGRADED_MODE_ACTIVE`  

Degraded mode must never:

- Mask structural corruption  
- Mask schema incompatibility  
- Allow acceptance  
- Allow governance mutation  

If structural corruption or schema incompatibility is detected,
the Engine must halt.

---

# 11. Resource Exhaustion & Atomic Failure

## 11.1 Resource Envelope Assumption

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

## 11.2 Atomic Failure Requirement

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

Resource exhaustion must never result in:

- Divergent legitimacy state  
- Partially applied acceptance  
- Partially reconstructed supersession graph  

If atomicity cannot be guaranteed, the Engine must halt.

---

# 12. Fatal Structural Integrity Failure

The Engine must halt if any of the following occur:

- Supersession cycle  
- Mixed-area structural object graph  
- Cross-area structural supersession  
- Invalid structural references  
- Governance slot multiplicity or emptiness  
- Unsupported schema version  
- Unknown structural enum value  
- Unknown structural field  
- Orphaned receipts  
- Receipt hash mismatch  
- Snapshot inconsistency  
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

# 13. Determinism Guarantee

Within the resource envelope and schema compatibility boundary:

- Restore must be deterministic  
- ACTIVE derivation must be deterministic  
- Governance slot evaluation must be deterministic  
- Participant epoch validation deterministic  
- Schema enforcement deterministic  
- No timestamp-based precedence  
- No UUID-time precedence  
- No ordering-based inference  
- No cross-area influence  

Identical inputs, identical schema support, and identical resource limits must produce identical runtime state across implementations.

---

# 14. Engine Invariants

- Exactly one Area active at runtime  
- No foreign DAG evaluation  
- No relaxed import mode  
- No partial restore mode  
- Legitimacy only compiled after successful rehydration  
- Schema compatibility enforced before legitimacy compilation  
- Structural references must resolve  
- Cross-area references are non-structural  
- Governance slots exclusive  
- Supersession strictly Area-local  
- Participant epochs strictly enforced  
- Determinism mandatory  
- Resource failure atomic  
- Halt preferred to ambiguity  

---

# 15. Compiler Halt Principle

If legitimacy or schema compatibility cannot be mechanically proven from structural domain objects,
the Engine must prefer halt over ambiguity.

If atomic safety cannot be guaranteed under resource exhaustion,
the Engine must prefer halt over partial mutation.

The Engine is not a repair tool.  
It is a deterministic legitimacy compiler operating within a schema boundary and resource envelope.