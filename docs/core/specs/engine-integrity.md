# ENG-INTEGRITY  
Engine Integrity & Runtime Guarantees  
Status: FROZEN (v4 – Integrity, Orphan Handling, Atomicity & Time Semantics)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

ENG-INTEGRITY defines the global runtime integrity rules of the Engine Core.

It governs:

- Engine initialization guarantees  
- Structural invariant enforcement  
- Area-level acceptance guards  
- Orphan object detection and graph completeness  
- Crash and persistence boundaries  
- Atomic commit semantics  
- Time semantics for engine operations  
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

The Engine is a **legitimacy compiler**:

- Does not infer legitimacy  
- Does not repair legitimacy  
- Does not auto-resolve ambiguity  
- Does not continue history from corrupted or incomplete data  

Legitimacy is:

- Explicit  
- Deterministic  
- Structurally verifiable  
- Mechanically reproducible  

If structural integrity cannot be proven, the Engine must halt.

Convenience never overrides legitimacy invariants.

---

# 3. Engine Initialization Guarantees

## 3.1 Deterministic Restore

On startup, the Engine must:

- Load all persisted domain objects  
- Reconstruct the supersession graph  
- Recompute ACTIVE sets  
- Validate exclusive legitimacy slots  
- Validate acyclic supersession graph  
- Validate session, candidate, and vote consistency  
- Validate schema versions  

Restore must be deterministic across implementations.

## 3.2 Orphan Object & Graph Completeness

- Every referenced object ID must exist in the imported graph or persisted store (unless operating in a **relaxed import mode**, e.g., Charter CLI Baseline Review).  
- Superseded, retired, or active objects must be fully present to validate legitimacy.  
- Orphan detection triggers **StructuralIntegrityFailure**.  

Relaxed import mode (CLI context):

- Flat lists of resolutions or proposals may be imported without full structural graph.  
- Structural reference checks are relaxed; the Engine validates only minimal integrity required to process proposals.  
- No legitimacy or acceptance is created automatically; full evaluation requires standard session mechanics.

Fail if:

- Missing referenced IDs outside relaxed import mode  
- Supersession edges invalid or cyclic  
- Participant snapshots incomplete  
- Candidate or vote snapshots incomplete  

---

## 3.3 Fatal Structural Integrity Failure

Engine initialization must fail deterministically if any of the following are detected:

- Supersession cycle  
- Multiple ACTIVE successors in an exclusive legitimacy slot  
- Invalid superseded_by references  
- Resolution or scope state inconsistent with supersession  
- Cross-area supersession violation  
- Schema mismatch preventing deterministic reconstruction  
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-DECISION  

Failure behavior:

- Engine must **halt completely**  
- No session evaluation permitted  
- No acceptance permitted  
- Error must clearly identify the invariant violation class  

No automatic repair is allowed. Forward motion requires **explicit host action**, e.g., baseline consolidation in CLI.

---

# 4. Area Acceptance Guard

## 4.1 BLOCK_PERMANENT Enforcement

If any session in an Area is in **BLOCK_PERMANENT**:

- Acceptance of any session in that Area fails  
- Evaluation reports `area_governance_blocked`  
- Blocking session(s) must be explicitly closed before acceptance  

## 4.2 Scope and Authority Supersession

- SUPERSEDED Authority or Scope → all Area sessions BLOCK_PERMANENT  
- Acceptance prohibited  
- Explicit closure or restart-from required  

## 4.3 Scope UNDER_REVIEW

- UNDER_REVIEW Scope → all Area sessions BLOCK_TEMPORARY  
- Acceptance prohibited until Scope returns to ACTIVE  
- Resume permitted  

---

# 5. No Implicit Repair Rule

The Engine must never:

- Auto-close sessions due to governance conflict  
- Auto-resolve supersession conflicts  
- Auto-merge branches  
- Auto-reactivate RETIRED resolutions  
- Auto-resume sessions after interruption  
- Modify domain objects during restore  

All corrective actions require **explicit user or host command**.

---

# 6. Failure Class Hierarchy

## 6.1 StructuralIntegrityFailure (Fatal)

- Occurs during initialization or rehydration  
- Halts Engine completely  
- Triggered by orphaned IDs, corrupted supersession, cycles, or missing snapshots  

## 6.2 AreaGovernanceBlocked (Non-Fatal Runtime Guard)

- Occurs when any session is BLOCK_PERMANENT  
- Prevents acceptance in that Area  
- Resolved only by explicit closure  

## 6.3 SessionGovernanceBlocked

- Occurs when session is BLOCK_TEMPORARY or BLOCK_PERMANENT  
- Prevents acceptance of that session  

---

# 7. Deterministic Enforcement

- Checks are deterministic and reproducible from persisted data  
- No timestamp precedence or heuristic ordering  
- Side-effect-free until commit  
- Engine halts on ambiguity  

---

# 8. Persistence & Atomic Commit Model

- Acceptance is atomic: session state, resolution creation, and legitimacy receipt persist together  
- Audit emission occurs **after** resolution commit; crash post-commit but pre-audit emission is allowed  
- Rehydration is non-mutating; restore or simulation failures do not alter persisted state  
- Engine ensures transactional integrity for all atomic operations  

---

# 9. Time Semantics

- Timestamps are **informational only**.  
- UUIDv7 time components may preserve creation order but are **not authoritative** for legitimacy, acceptance, or supersession.  
- Timestamps **must never influence** evaluation, restore, or acceptance rules.  
- Clock drift or host inconsistencies do not affect engine correctness.  
- Audit logs may include timestamps for human reference; chronological ordering for legitimacy is determined solely by deterministic structural rules.  

---

# 10. Cross-Document Invariant Precedence

If conflicts arise:

1. ENG-INTEGRITY → runtime enforcement  
2. ENG-SUPERSESSION → graph structure  
3. ENG-DECISION → session mechanics  
4. ENG-REVIEW-RETIRED → suspension semantics  
5. ENG-DOMAIN → structural encoding  

Runtime must never violate structural invariants.

---

# 11. Compiler Halt Principle

- Any ambiguity → halt  
- Orphan detection → halt  
- Multiple ACTIVE objects in exclusive slots → halt  
- Supersession graph inconsistent → halt  
- Integrity always preferred over convenience  

---

# 12. Explicit Consolidation Doctrine

When initialization fails:

- User or host performs explicit consolidation (e.g., Charter CLI Baseline Review)  
- Post-consolidation state must be fully deterministic  
- Resolutions from untrusted or corrupted sources may be batched as proposals  
- No structural trust is inherited; new legitimacy is created only through standard session mechanics  

---

# 13. Engine Invariants

- Legitimacy is compiled, not inferred  
- Structural integrity precedes usability  
- Explicit closure required for permanent conflicts  
- Deterministic restore is mandatory  
- Area hygiene enforced mechanically  
- No silent mutation allowed  
- Halt preferred to ambiguity  

---

This specification establishes the runtime integrity, orphan-handling, atomic commit, and time semantics contract of the Engine Core.  

All other specifications must conform to these guarantees.