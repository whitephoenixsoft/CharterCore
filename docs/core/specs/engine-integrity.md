# ENG-INTEGRITY  
Engine Integrity & Runtime Guarantees  
Status: DRAFT (Pre-Freeze)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

This document defines the global runtime integrity rules of the engine.

It governs:

- Engine initialization guarantees
- Structural invariant enforcement
- Area-level acceptance guards
- Fatal failure semantics
- Cross-document invariant precedence
- Legitimacy compiler doctrine

This document does not define:

- Session mechanics (ENG-DECISION)
- Supersession graph structure (ENG-SUPERSESSION)
- Suspension and deprecation semantics (ENG-REVIEW-RETIRED)
- Object schemas (ENG-DOMAIN)

It defines system-level halting conditions and runtime enforcement behavior.

---

# 2. Legitimacy Compiler Doctrine

The engine is a legitimacy compiler.

It does not infer legitimacy.
It does not repair legitimacy.
It does not auto-resolve ambiguity.

Legitimacy is:

- Explicit
- Deterministic
- Structurally verifiable
- Mechanically reproducible

If structural integrity cannot be proven, the engine must halt.

Convenience never overrides legitimacy invariants.

---

# 3. Engine Initialization Guarantees

## 3.1 Deterministic Restore

On startup, the engine must:

- Load all persisted domain objects.
- Reconstruct supersession graph.
- Recompute ACTIVE sets.
- Validate exclusive legitimacy slots.
- Validate acyclic supersession graph.
- Validate state consistency across objects.
- Validate schema versions.

Restore must be deterministic across implementations.

---

## 3.2 Fatal Structural Integrity Failure

Engine initialization must fail deterministically if any of the following are detected:

- Supersession cycle
- Multiple ACTIVE successors in an exclusive legitimacy slot
- Invalid superseded_by references
- Resolution state inconsistent with supersession structure
- Scope state inconsistent with supersession structure
- Cross-area supersession violation
- Schema mismatch that prevents deterministic reconstruction
- Any invariant violation defined in ENG-DOMAIN, ENG-SUPERSESSION, or ENG-DECISION

Failure behavior:

- Engine must not enter interactive mode.
- No session evaluation permitted.
- No acceptance permitted.
- Error must clearly identify invariant violation class.

No automatic repair is allowed.

Forward motion requires explicit consolidation through baseline review or corrective session.

---

# 4. Area Acceptance Guard

## 4.1 BLOCK_PERMANENT Enforcement

If any session in an Area is in state BLOCK_PERMANENT:

- Acceptance of any session in that Area must fail.
- Evaluation must report area_governance_blocked.
- The blocking session(s) must be explicitly closed before acceptance can proceed.

This rule enforces legitimacy hygiene.

---

## 4.2 Scope and Authority Supersession

If Authority or Scope in an Area is SUPERSEDED:

- All sessions in the Area transition to BLOCK_PERMANENT.
- Area acceptance is prohibited.
- Explicit closure or restart-from is required.

---

## 4.3 Scope UNDER_REVIEW

If Scope is UNDER_REVIEW:

- All sessions in the Area transition to BLOCK_TEMPORARY.
- Acceptance in the Area is prohibited.
- Resume is permitted once Scope returns to ACTIVE.

---

# 5. No Implicit Repair Rule

The engine must never:

- Auto-close sessions due to governance conflict.
- Auto-resolve supersession conflicts.
- Auto-merge branches.
- Auto-reactivate RETIRED resolutions.
- Auto-resume sessions after interruption.
- Modify domain objects during restore.

All corrective actions require explicit user command.

---

# 6. Failure Class Hierarchy

## 6.1 StructuralIntegrityFailure (Fatal)

Occurs during initialization.

Halts engine completely.

Examples:

- Graph cycle
- Multiple ACTIVE successors
- Corrupted supersession chain

---

## 6.2 AreaGovernanceBlocked (Non-Fatal Runtime Guard)

Occurs when:

- BLOCK_PERMANENT session exists in Area

Prevents acceptance in that Area.

Resolved only by explicit closure.

---

## 6.3 SessionGovernanceBlocked

Occurs when:

- Session in BLOCK_TEMPORARY or BLOCK_PERMANENT

Prevents acceptance of that session.

---

# 7. Deterministic Enforcement

All integrity checks must be:

- Deterministic
- Implementation-independent
- Side-effect-free until commit
- Reproducible from persisted data

No timestamp precedence.
No heuristic ordering.
No implementation-specific fallback.

---

# 8. Cross-Document Invariant Precedence

If conflict arises between documents:

1. ENG-INTEGRITY overrides runtime behavior.
2. ENG-SUPERSESSION governs graph structure.
3. ENG-DECISION governs session mechanics.
4. ENG-REVIEW-RETIRED governs suspension semantics.
5. ENG-DOMAIN governs structural encoding.

Runtime must never violate structural invariants.

---

# 9. Compiler Halt Principle

If legitimacy structure is ambiguous,
the engine must halt.

If supersession graph is inconsistent,
the engine must halt.

If exclusive legitimacy slots contain multiple ACTIVE objects,
the engine must halt.

If structural invariants cannot be proven,
the engine must halt.

The engine prefers halt over ambiguity.

---

# 10. Explicit Consolidation Doctrine

When initialization fails due to structural violation:

- User must perform baseline review.
- Consolidation must be explicit.
- No hidden repair path is allowed.
- Post-consolidation state must be fully deterministic.

Restore integrity before forward legitimacy motion.

---

# 11. Engine Invariants

- Legitimacy is compiled, not inferred.
- Structural integrity precedes usability.
- Explicit closure is required for permanent conflicts.
- Deterministic restore is mandatory.
- Area hygiene is enforced mechanically.
- No silent mutation is allowed.
- Halt is preferable to ambiguity.

---

This document establishes the runtime integrity contract of the engine.

All other specifications must conform to these guarantees.