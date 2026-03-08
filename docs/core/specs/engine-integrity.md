# ENG-INTEGRITY  
Engine Integrity, Runtime, Resource & Compatibility Guarantees  
Status: FROZEN (v14 – Canonical Receipt Verification & Round Snapshot Alignment)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

ENG-INTEGRITY defines the global runtime integrity rules of the Engine Core.

It governs:

- Engine initialization guarantees  
- Structural invariant enforcement  
- Governance bootstrap invariants  
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

Session mechanics are defined in ENG-SESSION.  
Supersession graph structure is defined in ENG-SUPERSESSION.  
Object schemas and compatibility rules are defined in ENG-DOMAIN.

This specification defines halting conditions and runtime guarantees.

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
the Engine must halt or enter degraded read-only mode.

Convenience never overrides legitimacy invariants.

---

# 3. Schema Compatibility Enforcement

Schema compatibility rules are defined in ENG-DOMAIN.

ENG-INTEGRITY enforces those rules during rehydration.

If schema compatibility cannot be verified deterministically,
initialization must fail.

Major version mismatches or unknown structural enums must produce:

UNSUPPORTED_SCHEMA_VERSION

Degraded mode is not permitted for schema incompatibility.

---

# 4. No Foreign DAG Evaluation

The Engine exposes no API for:

- Evaluating arbitrary domain graphs without rehydration  
- Validating legitimacy of foreign DAGs  
- Computing ACTIVE sets without initialization  

All legitimacy derivation requires:

1. Successful `rehydrate_engine`  
2. Structural validation  
3. Supersession reconstruction  
4. Governance slot derivation  

There is no relaxed validation mode.

---

# 5. Single-Area Runtime Enforcement

## 5.1 Single-Area Initialization Rule

An Engine instance must contain exactly one active Area.

During rehydration:

All structural objects must share identical `area_id`.

Mixed-area graphs cause StructuralIntegrityFailure.

Cross-area references are informational only.

---

## 5.2 Rehydration Replacement Rule

Calling `rehydrate_engine`:

- Replaces any previously loaded Area state  
- Discards prior legitimacy state  
- Establishes a new single active Area  

The Engine must never merge Areas.

---

# 6. Deterministic Restore

During restore the Engine must:

- Validate schema compatibility  
- Validate single-area object graph  
- Reconstruct supersession graph  
- Verify graph acyclicity  
- Derive ACTIVE resolutions  
- Validate governance slot exclusivity  
- Validate participant epoch invariants  
- Validate receipt integrity  

Restore must be deterministic across implementations.

Cross-area references must never be traversed.

---

# 7. Structural vs Informational References

Structural references:

- Affect legitimacy  
- Affect supersession  
- Affect ACTIVE derivation  

They must resolve locally within the Area.

Missing structural references → StructuralIntegrityFailure.

Cross-area structural references are prohibited.

Informational cross-area references:

- Are metadata only  
- Must not affect legitimacy  
- Must not be traversed  

---

# 8. Participant Epoch Integrity Invariants

Participant identity is session-scoped and epoch-based.

Restore must validate:

- No participant_id reuse within a session  
- Round snapshots in receipts match the session state frozen at each round boundary  
- No merging of historical participation epochs  

Violation → StructuralIntegrityFailure.

---

# 9. Receipt Integrity Rules

Receipts are integrity artifacts.

They do not create legitimacy.

Rules:

- Terminal sessions must have exactly one receipt  
- Missing terminal receipt → StructuralIntegrityFailure  
- Orphaned receipts → StructuralIntegrityFailure  
- Receipt hash mismatch → StructuralIntegrityFailure  
- Snapshot mismatch → StructuralIntegrityFailure  

Receipt verification must confirm:

- Canonical serialization validity (ENG-CANON)  
- content_hash recomputation matches stored value  
- hash_algorithm declared and valid  
- spec_set_hash matches the rule manifest embedded in the executing Engine  

Receipt verification occurs after structural restore succeeds.

---

# 10. Degraded Read-Only Mode

Degraded mode may activate only if:

- Structural graph is internally consistent  
- Schema compatibility satisfied  
- Supersession graph reconstructable  
- Governance slots derivable  
- Acceptance safety cannot be guaranteed due to missing non-fatal artifacts

Possible triggers include:

- Missing optional artifacts
- Host-configured audit artifacts unavailable

In degraded mode:

- No mutating operations allowed  
- No acceptance allowed  
- Evaluation permitted for informational use  
- DAG export permitted  

Degraded mode must never mask structural corruption.

---

# 11. Resource Exhaustion & Atomic Failure

## 11.1 Resource Envelope Assumption

The Engine guarantees determinism within a sufficient resource envelope.

Resource ceilings are implementation-defined.

---

## 11.2 Atomic Failure Requirement

If resource exhaustion occurs during:

- Rehydration  
- ACTIVE derivation  
- Acceptance  
- Supersession application  
- Receipt emission  
- Canonical serialization  
- Hash computation  

The Engine must:

- Abort the operation atomically  
- Leave all structural objects unchanged  
- Emit no partial receipts  
- Emit no partial transitions  

If atomic safety cannot be guaranteed, the Engine must halt.

---

# 12. Fatal Structural Integrity Failure

The Engine must halt if:

- Supersession cycle detected  
- Mixed-area structural graph detected  
- Cross-area structural supersession detected  
- Invalid structural references detected  
- Governance slot multiplicity or emptiness detected  
- Unsupported schema version detected  
- Unknown structural enum detected  
- Unknown structural field detected  
- Missing terminal receipt detected  
- Receipt hash mismatch detected  
- Snapshot mismatch detected  
- participant_id reuse detected  
- Resource exhaustion leaves partial mutation  

No automatic repair permitted.

---

# 13. Determinism Guarantee

Within schema compatibility and resource envelope:

- Restore deterministic  
- ACTIVE derivation deterministic  
- Governance slot evaluation deterministic  
- Participant epoch validation deterministic  
- Receipt verification deterministic  

No timestamp-based precedence permitted.

---

# 14. Engine Invariants

- Exactly one Area active  
- No foreign DAG evaluation  
- No partial restore mode  
- Schema compatibility enforced before legitimacy compilation  
- Structural references must resolve  
- Governance slots exclusive  
- Supersession strictly Area-local  
- Participant epochs strictly enforced  
- Determinism mandatory  
- Resource failure atomic  

---

# 15. Compiler Halt Principle

If legitimacy cannot be mechanically proven from structural domain objects,
the Engine must halt.

If atomic safety cannot be guaranteed,
the Engine must halt.

The Engine is a deterministic legitimacy compiler operating within a schema boundary and resource envelope.