# ENG-INTEGRITY  
Engine Integrity & Runtime Guarantees  
Status: FROZEN (v8 – Single-Area Runtime Constitutionalized)  
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

If structural integrity cannot be proven, the Engine must halt.

Convenience never overrides legitimacy invariants.

---

# 3. Single-Area Runtime Enforcement

## 3.1 Single-Area Initialization Rule

At any moment in time, an Engine instance must contain exactly one active Area.

During initialization or rehydration:

- All loaded domain objects must share the same area_id.
- Mixed-area object graphs are prohibited.
- Cross-area references do not count as multi-Area hosting.

If multiple distinct area_id values are detected among structural domain objects:

- Engine initialization must fail with StructuralIntegrityFailure.
- Engine must halt.

---

## 3.2 Rehydration Replacement Rule

Calling rehydrate_engine:

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

Restore must be deterministic across implementations.

Cross-area references must not be traversed during restore.

---

## 5.2 Structural vs Informational References

### Structural References

Structural references are those that:

- Affect legitimacy
- Affect ACTIVE derivation
- Affect supersession
- Affect governance slot evaluation

Structural references must:

- Resolve to objects present in the imported Area graph  
- Share the same area_id  
- Be validated during restore  
- Be validated during acceptance  

Missing structural references trigger StructuralIntegrityFailure.

Structural references include:

- superseded_by edges (same Area only)
- Session → Authority
- Session → Scope
- Resolution → originating Session

Cross-area structural references are prohibited.

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
- Do not create multi-Area runtime context

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

## 5.3 Orphan Object & Graph Completeness

Orphan detection applies only to structural references within the active Area.

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
- Mixed-area structural graph detected  

Do not fail if:

- Cross-area references are unresolved  
- External Areas are absent  
- External Resolutions are absent  

---

# 6. Fatal Structural Integrity Failure

Engine initialization must fail deterministically if any of the following are detected:

- Supersession cycle (Area-local)  
- Multiple ACTIVE successors in an exclusive legitimacy slot  
- Missing required governance objects  
- Invalid structural superseded_by references  
- Resolution or scope state inconsistent with supersession  
- Cross-area supersession attempt  
- Mixed-area structural object graph  
- Schema mismatch preventing deterministic reconstruction  
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-DECISION  

Failure behavior:

- Engine must halt completely  
- No session evaluation permitted  
- No acceptance permitted  
- Error must clearly identify invariant violation class  

No automatic repair is allowed.

---

# 7. Area Acceptance Guard

## 7.1 BLOCK_PERMANENT Enforcement

If any session in the active Area is in BLOCK_PERMANENT:

- Acceptance of any session in that Area fails  
- Evaluation reports area_governance_blocked  
- Blocking session(s) must be explicitly closed before acceptance  

Blocking is strictly Area-local.

## 7.2 Authority or Scope Supersession

If Authority or Scope is SUPERSEDED:

- All sessions in the active Area transition to BLOCK_PERMANENT  
- Acceptance is prohibited  
- Explicit closure or restart required  

## 7.3 Scope UNDER_REVIEW

If Scope is UNDER_REVIEW:

- All sessions in the active Area transition to BLOCK_TEMPORARY  
- Acceptance prohibited until Scope returns to ACTIVE  
- Resume permitted  

Cross-area state changes must not trigger blocking behavior.

---

# 8. Deterministic Enforcement

- All structural checks are deterministic and reproducible from active Area data  
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
- Mixed-area structural graph  

Integrity always preferred over convenience.

---

# 11. Engine Invariants

- Engine operates on exactly one Area at a time  
- Mixed-area structural graphs are prohibited  
- Legitimacy is compiled, not inferred  
- Structural integrity precedes usability  
- Governance must be explicitly initialized  
- Exactly one ACTIVE Authority per governed Area  
- Exactly one ACTIVE Scope per governed Area  
- Structural references must resolve and be Area-local  
- Cross-area references must not be validated  
- Deterministic restore is mandatory  
- Area sovereignty preserved  
- UUID is the sole identity authority  
- Hashes are non-semantic integrity metadata  
- No silent mutation allowed  
- Halt preferred to ambiguity  

---

This specification establishes the runtime integrity, governance bootstrap, single-Area enforcement, structural boundary, atomicity, time semantics, and identity guarantees of the Engine Core.

All other specifications must conform to these guarantees.

