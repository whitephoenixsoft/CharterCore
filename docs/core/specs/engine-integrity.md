# ENG-INTEGRITY  
Engine Integrity & Runtime Guarantees  
Status: FROZEN (v7 – Cross-Area Structural Boundary Formalized)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

ENG-INTEGRITY defines the global runtime integrity rules of the Engine Core.

It governs:

- Engine initialization guarantees  
- Structural invariant enforcement  
- Governance bootstrap invariants (Authority & Scope)  
- Area-level acceptance guards  
- Orphan object detection and graph completeness  
- Structural vs informational reference validation  
- Crash and persistence boundaries  
- Atomic commit semantics  
- Time semantics for engine operations  
- Identity and hash separation rules  
- Fatal failure semantics  
- Cross-document invariant precedence  
- Legitimacy compiler doctrine  

It does **not** define:

- Session mechanics (ENG-DECISION)  
- Supersession graph structure (ENG-SUPERSESSION)  
- Suspension and deprecation semantics (ENG-REVIEW-RETIRED)  
- Object schemas (ENG-DOMAIN)  

This specification defines system-level halting conditions and runtime enforcement behavior.

---

# 2. Legitimacy Compiler Doctrine

The Engine is a legitimacy compiler:

- Does not infer legitimacy  
- Does not repair legitimacy  
- Does not auto-resolve ambiguity  
- Does not continue history from corrupted or incomplete data  
- Does not implicitly create governance structures  
- Does not traverse external Areas  

Legitimacy is:

- Explicit  
- Deterministic  
- Structurally verifiable  
- Mechanically reproducible  
- Area-local  

If structural integrity cannot be proven, the Engine must halt.

Convenience never overrides legitimacy invariants.

---

# 3. Governance Bootstrap Invariants

## 3.1 Exclusive Governance Slots

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

# 4. Engine Initialization Guarantees

## 4.1 Deterministic Restore

On startup, the Engine must:

- Load all persisted domain objects for the Area  
- Reconstruct the supersession graph (Area-local only)  
- Recompute ACTIVE sets  
- Validate exclusive legitimacy slots  
- Validate acyclic supersession graph  
- Validate session, candidate, and vote consistency  
- Validate schema versions  

Restore must be deterministic across implementations.

Cross-area references must not be traversed during restore.

---

## 4.2 Structural vs Informational References

### Structural References

Structural references are those that:

- Affect legitimacy
- Affect ACTIVE derivation
- Affect supersession
- Affect governance slot evaluation

Structural references must:

- Resolve to objects present in the imported Area graph  
- Be validated during restore  
- Be validated during acceptance  

Missing structural references trigger StructuralIntegrityFailure.

Structural references include:

- superseded_by edges (same Area only)
- Session → Authority
- Session → Scope
- Resolution → originating Session

---

### Informational Cross-Area References

Cross-area references (as defined in ENG-DOMAIN):

- Reference external Areas or external Resolutions
- Are informational only
- Do not affect legitimacy
- Do not affect ACTIVE derivation
- Do not affect governance slot evaluation
- Do not affect conflict detection
- Do not affect restore success

The Engine must not:

- Dereference cross-area references
- Validate their existence
- Treat missing external targets as orphaned
- Fail restore due to unresolved cross-area references

Deletion or absence of an externally referenced Area or Resolution must not alter:

- Structural validity
- Blocking state
- Acceptance eligibility
- Governance state

Cross-area references are excluded from orphan detection.

---

## 4.3 Orphan Object & Graph Completeness

Orphan detection applies only to structural references.

Every structural referenced object ID must exist in the imported Area graph (unless operating in relaxed import mode defined by the host).

Relaxed import mode:

- May relax structural completeness for ingestion of external proposals.
- Must not create legitimacy automatically.
- Full legitimacy evaluation requires standard session mechanics.

Fail if:

- Missing structural referenced IDs outside relaxed import mode  
- Supersession edges invalid or cyclic  
- Participant snapshots incomplete  
- Candidate or vote snapshots incomplete  

Do not fail if:

- Cross-area references are unresolved  
- External Areas are absent  
- External Resolutions are absent  

---

# 5. Fatal Structural Integrity Failure

Engine initialization must fail deterministically if any of the following are detected:

- Supersession cycle (Area-local)  
- Multiple ACTIVE successors in an exclusive legitimacy slot  
- Missing required governance objects  
- Invalid structural superseded_by references  
- Resolution or scope state inconsistent with supersession  
- Cross-area supersession attempt  
- Schema mismatch preventing deterministic reconstruction  
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-DECISION  

Failure behavior:

- Engine must halt completely  
- No session evaluation permitted  
- No acceptance permitted  
- Error must clearly identify invariant violation class  

No automatic repair is allowed.

---

# 6. Area Acceptance Guard

## 6.1 BLOCK_PERMANENT Enforcement

If any session in an Area is in BLOCK_PERMANENT:

- Acceptance of any session in that Area fails  
- Evaluation reports area_governance_blocked  
- Blocking session(s) must be explicitly closed before acceptance  

## 6.2 Authority or Scope Supersession

If Authority or Scope is SUPERSEDED:

- All Area sessions transition to BLOCK_PERMANENT  
- Acceptance is prohibited  
- Explicit closure or restart required  

## 6.3 Scope UNDER_REVIEW

If Scope is UNDER_REVIEW:

- All Area sessions transition to BLOCK_TEMPORARY  
- Acceptance prohibited until Scope returns to ACTIVE  
- Resume permitted  

Cross-area state changes must not trigger blocking behavior.

---

# 7. No Implicit Repair Rule

The Engine must never:

- Auto-close sessions due to governance conflict  
- Auto-resolve supersession conflicts  
- Auto-merge branches  
- Auto-reactivate RETIRED resolutions  
- Auto-resume sessions after interruption  
- Modify domain objects during restore  
- Auto-create Authority or Scope  
- Attempt to reconcile cross-area references  

All corrective actions require explicit user or host command.

---

# 8. Deterministic Enforcement

- All structural checks are deterministic and reproducible from Area data  
- No timestamp precedence or heuristic ordering  
- No implicit ordering based on UUID time component  
- No cross-area topology influence  
- Side-effect-free until commit  
- Engine halts on structural ambiguity  

---

# 9. Time & Identity Semantics

Timestamps and UUIDv7 time components:

- Are informational only  
- Must never influence evaluation, restore, acceptance, or supersession  
- Must never determine cross-area precedence  

Identity is Area-local and UUID-exclusive.

---

# 10. Compiler Halt Principle

The Engine prefers halt over ambiguity.

Structural ambiguity includes:

- Orphaned structural references  
- Governance slot violation  
- Multiple ACTIVE objects in exclusive slots  
- Supersession graph inconsistency  
- Cross-area supersession edge  

Integrity always preferred over convenience.

---

# 11. Engine Invariants

- Legitimacy is compiled, not inferred  
- Structural integrity precedes usability  
- Governance must be explicitly initialized  
- Exactly one ACTIVE Authority per governed Area  
- Exactly one ACTIVE Scope per governed Area  
- Structural references must resolve  
- Cross-area references must not be validated  
- Deterministic restore is mandatory  
- Area sovereignty preserved  
- UUID is the sole identity authority  
- Hashes are non-semantic integrity metadata  
- No silent mutation allowed  
- Halt preferred to ambiguity  

---

This specification establishes the runtime integrity, governance bootstrap, structural boundary, atomicity, time semantics, and identity guarantees of the Engine Core.

All other specifications must conform to these guarantees.