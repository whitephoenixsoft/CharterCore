# ENG-INTEGRITY  
Engine Integrity & Runtime Guarantees  
Status: FROZEN (v6 – Integrity, Governance Bootstrap, Atomicity, Time & Identity Semantics)  
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

Legitimacy is:

- Explicit  
- Deterministic  
- Structurally verifiable  
- Mechanically reproducible  

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

## 3.2 Governance Preconditions

The following structural prerequisites apply:

1. Scope Acceptance  
   - Requires exactly one ACTIVE Authority.

2. Regular Session Creation  
   - Requires exactly one ACTIVE Authority.  
   - Requires exactly one ACTIVE Scope.

3. Area Restore  
   - If an Area contains regular sessions, it must contain exactly one ACTIVE Authority and one ACTIVE Scope.

If governance prerequisites are not satisfied:

- Session creation must fail deterministically.
- Acceptance must not proceed.
- Restore must halt if invariants cannot be proven.

No null Authority is allowed once governance has begun.  
No null Scope is allowed once defined.

---

## 3.3 Supersession Safety for Governance

Supersession rules apply to Authority and Scope identically to other resolutions.

However:

- Supersession must never result in zero ACTIVE Authority in a governed Area.
- Supersession must never result in zero ACTIVE Scope in an Area containing regular sessions.
- Multiple ACTIVE governance objects in a slot triggers StructuralIntegrityFailure.
- Governance supersession may trigger BLOCK_PERMANENT or BLOCK_TEMPORARY per defined rules.

Governance integrity is structural, not advisory.

---

# 4. Engine Initialization Guarantees

## 4.1 Deterministic Restore

On startup, the Engine must:

- Load all persisted domain objects  
- Reconstruct the supersession graph  
- Recompute ACTIVE sets  
- Validate exclusive legitimacy slots (including governance slots)  
- Validate acyclic supersession graph  
- Validate session, candidate, and vote consistency  
- Validate schema versions  

Restore must be deterministic across implementations.

---

## 4.2 Orphan Object & Graph Completeness

- Every referenced object ID must exist in the imported graph or persisted store (unless operating in a relaxed import mode defined by the host).  
- Superseded, retired, or active objects must be fully present to validate legitimacy.  
- Orphan detection triggers StructuralIntegrityFailure.

Relaxed import mode:

- May relax structural completeness for ingestion of external proposals.
- Must not create legitimacy automatically.
- Full legitimacy evaluation requires standard session mechanics.

Fail if:

- Missing referenced IDs outside relaxed import mode  
- Supersession edges invalid or cyclic  
- Participant snapshots incomplete  
- Candidate or vote snapshots incomplete  

---

## 4.3 Fatal Structural Integrity Failure

Engine initialization must fail deterministically if any of the following are detected:

- Supersession cycle  
- Multiple ACTIVE successors in an exclusive legitimacy slot  
- Missing required governance objects  
- Invalid superseded_by references  
- Resolution or scope state inconsistent with supersession  
- Cross-area supersession violation  
- Schema mismatch preventing deterministic reconstruction  
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-DECISION  

Failure behavior:

- Engine must halt completely  
- No session evaluation permitted  
- No acceptance permitted  
- Error must clearly identify invariant violation class  

No automatic repair is allowed. Forward motion requires explicit host action.

---

# 5. Area Acceptance Guard

## 5.1 BLOCK_PERMANENT Enforcement

If any session in an Area is in BLOCK_PERMANENT:

- Acceptance of any session in that Area fails  
- Evaluation reports area_governance_blocked  
- Blocking session(s) must be explicitly closed before acceptance  

## 5.2 Authority or Scope Supersession

If Authority or Scope is SUPERSEDED:

- All Area sessions transition to BLOCK_PERMANENT  
- Acceptance is prohibited  
- Explicit closure or restart-from required  

## 5.3 Scope UNDER_REVIEW

If Scope is UNDER_REVIEW:

- All Area sessions transition to BLOCK_TEMPORARY  
- Acceptance prohibited until Scope returns to ACTIVE  
- Resume permitted  

---

# 6. No Implicit Repair Rule

The Engine must never:

- Auto-close sessions due to governance conflict  
- Auto-resolve supersession conflicts  
- Auto-merge branches  
- Auto-reactivate RETIRED resolutions  
- Auto-resume sessions after interruption  
- Modify domain objects during restore  
- Auto-create Authority or Scope  

All corrective actions require explicit user or host command.

---

# 7. Failure Class Hierarchy

## 7.1 StructuralIntegrityFailure (Fatal)

- Occurs during initialization or rehydration  
- Halts Engine completely  
- Triggered by orphaned IDs, corrupted supersession, cycles, governance slot violations, or missing snapshots  

## 7.2 AreaGovernanceBlocked (Non-Fatal Runtime Guard)

- Occurs when any session is BLOCK_PERMANENT  
- Prevents acceptance in that Area  
- Resolved only by explicit closure  

## 7.3 SessionGovernanceBlocked

- Occurs when session is BLOCK_TEMPORARY or BLOCK_PERMANENT  
- Prevents acceptance of that session  

---

# 8. Deterministic Enforcement

- All checks are deterministic and reproducible from persisted data  
- No timestamp precedence or heuristic ordering  
- No implicit ordering based on UUID time component  
- Side-effect-free until commit  
- Engine halts on ambiguity  

---

# 9. Persistence & Atomic Commit Model

- Acceptance is atomic: session state, resolution creation, and legitimacy receipt persist together  
- Audit emission occurs after resolution commit  
- Crash post-commit but pre-audit emission is valid  
- Rehydration is non-mutating  
- Restore or simulation failures do not alter persisted state  
- Engine guarantees transactional integrity for atomic operations  

---

# 10. Time Semantics

- Timestamps are informational only  
- UUIDv7 time components are non-authoritative  
- Timestamps must never influence evaluation, restore, acceptance, or supersession  
- Clock drift or host inconsistencies do not affect correctness  
- Chronology for legitimacy is determined solely by structural rules  

Time is descriptive, never authoritative.

---

# 11. Identity & Hash Semantics

## 11.1 Canonical Identity

- All engine domain objects are identified exclusively by UUIDv7  
- UUID is the sole canonical identity mechanism  
- Identity never depends on content hashing, serialization form, or storage location  

## 11.2 Hash Independence Rule

If content hashes are used:

- Hashes are integrity metadata only  
- Hash changes do not alter object identity  
- Hash algorithm migration does not alter object identity  
- Identity resolution relies exclusively on UUID  

## 11.3 Hash Non-Semantic Guarantee

Hashes must never influence:

- Legitimacy evaluation  
- Acceptance  
- Supersession  
- Restore logic  

Hashes are descriptive, not authoritative.

---

# 12. Cross-Document Invariant Precedence

If conflicts arise:

1. ENG-INTEGRITY → runtime enforcement  
2. ENG-SUPERSESSION → graph structure  
3. ENG-DECISION → session mechanics  
4. ENG-REVIEW-RETIRED → suspension semantics  
5. ENG-DOMAIN → structural encoding  

Runtime must never violate structural invariants.

---

# 13. Compiler Halt Principle

The Engine prefers halt over ambiguity.

- Any structural ambiguity → halt  
- Orphan detection → halt  
- Governance slot violation → halt  
- Multiple ACTIVE objects in exclusive slots → halt  
- Supersession graph inconsistency → halt  

Integrity always preferred over convenience.

---

# 14. Explicit Consolidation Doctrine

When initialization fails:

- User or host performs explicit consolidation  
- Post-consolidation state must be fully deterministic  
- No structural trust is inherited from corrupted sources  
- New legitimacy is created only through standard session mechanics  

---

# 15. Engine Invariants

- Legitimacy is compiled, not inferred  
- Structural integrity precedes usability  
- Governance must be explicitly initialized  
- Exactly one ACTIVE Authority per governed Area  
- Exactly one ACTIVE Scope per governed Area  
- Deterministic restore is mandatory  
- Area hygiene enforced mechanically  
- UUID is the sole identity authority  
- Hashes are non-semantic integrity metadata  
- No silent mutation allowed  
- Halt preferred to ambiguity  

---

This specification establishes the runtime integrity, governance bootstrap, atomicity, time semantics, and identity guarantees of the Engine Core.

All other specifications must conform to these guarantees.