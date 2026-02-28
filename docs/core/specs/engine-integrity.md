# ENG-INTEGRITY  
Engine Integrity & Runtime Guarantees  
Status: FROZEN (v9 – Single-Area Runtime with Degraded Mode)  
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
- Degraded read-only mode  
- Crash and persistence boundaries  
- Atomic commit semantics  
- Time semantics for engine operations  
- Identity and hash separation rules  
- Fatal vs degraded failure semantics  
- Cross-document invariant precedence  
- Legitimacy compiler doctrine  

It does **not** define:

- Session mechanics (ENG-DECISION)  
- Supersession graph structure (ENG-SUPERSESSION)  
- Suspension and deprecation semantics (ENG-REVIEW-RETIRED)  
- Object schemas (ENG-DOMAIN)  

This specification defines system-level halting conditions, runtime enforcement behavior, and degraded mode rules.

---

# 2. Legitimacy Compiler Doctrine

The Engine is a legitimacy compiler.

It:

- Operates on exactly one Area at a time  
- Does not infer legitimacy  
- Does not repair legitimacy  
- Does not auto-resolve ambiguity  
- Does not continue history from corrupted or incomplete data  
- Does not implicitly create governance structures  
- Does not traverse external Areas  
- Does not retain cross-Area legitimacy state  

Legitimacy is:

- Explicit  
- Deterministic  
- Structurally verifiable  
- Mechanically reproducible  
- Strictly Area-local  

If **structural integrity of primary domain objects** cannot be proven, the Engine must halt.

Convenience never overrides legitimacy invariants.

Receipts are **integrity artifacts**:

- Missing receipts **do not trigger fatal failure**, but may prevent acceptance operations.  
- Receipts without matching sessions or with mismatched content hash **are fatal structural failures**.  
- Receipts are read-only and may be included in **DAG export** for audit or consolidation.

---

# 3. Single-Area Runtime Enforcement

## 3.1 Single-Area Initialization Rule

At any moment in time, an Engine instance must contain exactly one active Area.

During initialization or rehydration:

- All loaded **structural domain objects** must share the same area_id.  
- Mixed-area object graphs are prohibited.  
- Cross-area references do not count as multi-Area hosting.  

If multiple distinct area_id values are detected among structural domain objects:

- Engine initialization must fail with StructuralIntegrityFailure.  
- Engine must halt.

## 3.2 Rehydration Replacement Rule

Calling `rehydrate_engine`:

- Replaces any previously loaded Area state.  
- Discards prior Area legitimacy state entirely.  
- Establishes a new single active Area.

The Engine must not:

- Merge Areas.  
- Retain governance slot memory from prior Area.  
- Preserve supersession graph from prior Area.

Area switching is exclusively a host responsibility.

---

# 4. Governance Bootstrap Invariants

## 4.1 Exclusive Governance Slots

Each Area contains two exclusive legitimacy slots:

- Authority slot  
- Scope slot  

Invariants:

- At most one ACTIVE Authority per Area.  
- At most one ACTIVE Scope per Area.  
- Authority and Scope participate in standard supersession rules.  
- Governance objects are created and modified only via standard session mechanics.  

The Engine must never implicitly create Authority or Scope.

---

# 5. Engine Initialization Guarantees

## 5.1 Deterministic Restore

On startup or rehydration, the Engine must:

- Load all persisted domain objects for the Area  
- Verify all objects share identical area_id  
- Reconstruct the supersession graph (Area-local only)  
- Recompute ACTIVE sets  
- Validate exclusive legitimacy slots  
- Validate acyclic supersession graph  
- Validate session, candidate, and vote consistency  
- Validate schema versions  
- Validate receipt integrity (hash consistency)  

Restore must be deterministic across implementations.

Cross-area references must not be traversed during restore.

## 5.2 Structural vs Informational References

### Structural References

- Affect legitimacy, ACTIVE derivation, supersession, and governance evaluation  
- Must resolve to objects present in the imported Area graph  
- Must be Area-local  
- Missing structural references **trigger StructuralIntegrityFailure**  

Structural references include:

- superseded_by edges (same Area only)  
- Session → Authority  
- Session → Scope  
- Resolution → originating Session  

Cross-area structural references are prohibited.

### Informational Cross-Area References

- Reference external Areas or Resolutions  
- Informational only: no effect on legitimacy, ACTIVE derivation, supersession, or governance  
- Must not trigger restore failure  
- Excluded from orphan detection  

## 5.3 Receipt Rehydration Rules

- Missing receipts: **do not trigger fatal failure**  
- Orphaned receipts (no matching session or resolution): **trigger StructuralIntegrityFailure**  
- Receipt hash mismatch: **trigger StructuralIntegrityFailure**  
- Receipts are read-only; no mutation allowed  
- Receipts are included in DAG export  

## 5.4 Orphan Object & Graph Completeness

- Orphan detection applies only to structural references  
- Missing referenced objects outside relaxed import mode → StructuralIntegrityFailure  
- Supersession edges invalid or cyclic → StructuralIntegrityFailure  
- Participant, candidate, vote snapshots incomplete → StructuralIntegrityFailure  
- Cross-area references unresolved → no failure  

---

# 6. Degraded Read-Only Mode

If any structural integrity violation **does not compromise core domain objects** but prevents acceptance:

- Engine enters **degraded read-only mode**  
- ACCEPTED/CLOSED sessions remain readable, but **mutating operations are prohibited**  
- Only allowed operations:  
  - Read queries (`list_sessions`, `list_resolutions`, `get_session_receipt`)  
  - `export_area_dag` for consolidation or recovery  
- EvaluationReport for any mutating attempt:  
  - `outcome = REJECTED`  
  - `error_code = DEGRADED_MODE_ACTIVE`  
- DAG export includes all objects deterministically, including receipts  

Degraded mode allows recovery workflows without violating deterministic integrity of canonical domain objects.

---

# 7. Fatal Structural Integrity Failure

Engine must halt if any of the following are detected:

- Supersession cycle (Area-local)  
- Multiple ACTIVE successors in an exclusive legitimacy slot  
- Missing required governance objects  
- Invalid structural superseded_by references  
- Resolution or scope state inconsistent with supersession  
- Cross-area supersession attempt  
- Mixed-area structural object graph  
- Schema mismatch preventing deterministic reconstruction  
- Receipt hash mismatch  
- Orphaned receipts (no matching session or resolution)  
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-DECISION  

Halt behavior:

- No session evaluation permitted  
- No acceptance permitted  
- Error must clearly identify invariant violation class  

No automatic repair allowed.

---

# 8. Area Acceptance Guard

## 8.1 BLOCK_PERMANENT Enforcement

- Any session in BLOCK_PERMANENT → acceptance fails  
- Evaluation reports `area_governance_blocked`  
- Explicit closure required before acceptance  

## 8.2 Authority or Scope Supersession

- SUPERSEDED Authority/Scope → all sessions BLOCK_PERMANENT  
- Acceptance prohibited until explicit closure or restart  

## 8.3 Scope UNDER_REVIEW

- Scope UNDER_REVIEW → all sessions BLOCK_TEMPORARY  
- Acceptance prohibited until ACTIVE  

Cross-area changes do not affect blocking.

---

# 9. Deterministic Enforcement

- Structural checks must be deterministic  
- No timestamp or UUID-based precedence  
- Side-effect-free until commit  
- Engine halts on ambiguity  

---

# 10. Time & Identity Semantics

- Timestamps / UUIDv7 time components: informational only  
- Identity is Area-local and UUID-exclusive  

---

# 11. Compiler Halt Principle

- Prefer halt over ambiguity  
- Structural ambiguity includes orphaned references, governance slot violation, multiple ACTIVE objects, supersession inconsistency, cross-area supersession, mixed-area graphs, receipt hash mismatch, orphaned receipts  

---

# 12. Engine Invariants

- Operates on exactly one Area at a time  
- Mixed-area graphs prohibited  
- Legitimacy compiled, not inferred  
- Structural integrity precedes usability  
- Governance must be explicitly initialized  
- Exactly one ACTIVE Authority per Area  
- Exactly one ACTIVE Scope per Area  
- Structural references must resolve and be Area-local  
- Cross-area references not validated  
- Deterministic restore mandatory  
- Area sovereignty preserved  
- UUID is sole identity authority  
- Hashes are non-semantic integrity metadata  
- No silent mutation  
- Halt preferred to ambiguity  
- Degraded read-only mode allowed for DAG export if receipts or non-critical invariants fail  

---

# 13. Receipts & DAG Export

- All receipts must be deterministic and immutable  
- Export includes all domain objects and receipts  
- Consolidation or recovery relies exclusively on exported DAG  

---

This specification introduces **degraded read-only mode** and formalizes **receipt integrity rules** while preserving single-Area, deterministic, and legitimacy-first guarantees.