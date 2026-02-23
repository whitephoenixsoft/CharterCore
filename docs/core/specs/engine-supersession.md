# ENG-SUPERSESSION  
Supersession & Conflict Model  
Status: DRAFT (Pre-Freeze)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

This document defines the Resolution graph model and all supersession and conflict semantics.

It governs:

- Supersession linking rules
- Active resolution definition
- Conflict detection
- Race condition semantics
- Permanent blocking behavior
- Revalidation triggers
- Under-review graph interaction
- Deterministic restore guarantees

This document does not define session lifecycle mechanics.  
It defines graph integrity and legitimacy conflict behavior.

---

# 2. Resolution Graph Model

## 2.1 Directed Supersession

A Resolution may supersede zero or more prior Resolutions.

Supersession is:

- Explicit
- Directional
- Immutable
- Recorded at acceptance time only

If Resolution B supersedes Resolution A:

- B contains an immutable reference to A.
- A does not mutate.
- A remains in history.

---

## 2.2 Acyclic Requirement

The supersession graph must remain acyclic.

A Resolution may not:

- Supersede itself.
- Create a cycle directly or indirectly.

Cycle detection must be enforced at acceptance time.

Violation results in deterministic acceptance failure.

---

## 2.3 Area Locality

Supersession is scoped to an Area.

A Resolution may supersede only Resolutions within the same Area.

Cross-area references are informational only and have no supersession effect.

---

# 3. Active Resolution Definition

A Resolution is ACTIVE if:

- It has not been superseded by another accepted Resolution in the same Area.
- It is not marked SUPERSEDED.
- It is not logically invalidated by graph rules.

Only one ACTIVE Resolution may exist per legitimacy slot where exclusivity is required.

Authority and Scope are exclusive legitimacy slots.

If multiple active successors would result from acceptance, acceptance must fail.

---

# 4. Referencing Rules

## 4.1 Legitimate Session References

A session may reference:

- The current ACTIVE Authority.
- The current ACTIVE Scope.
- Zero or one ACTIVE Resolution for supersession purposes.

A session must not reference:

- A Resolution that is already superseded.
- A Resolution outside its Area for legitimacy purposes.

If a session attempts to reference a superseded Resolution for legitimacy:

- The session transitions to BLOCK_PERMANENT.

---

## 4.2 Informational References

Sessions may reference:

- Other Areas
- Other Area Resolutions

These references are informational only and do not affect legitimacy.

Informational references have no supersession power.

---

# 5. First-Accept Wins Rule

If two or more sessions attempt to supersede the same ACTIVE Resolution:

- The first successful acceptance creates the successor Resolution.
- The referenced Resolution becomes non-ACTIVE.
- All other sessions referencing that Resolution for supersession become BLOCK_PERMANENT.

There is no automatic branch merging.

There is no implicit precedence rule.

Only explicit supersession edges determine graph structure.

---

# 6. Conflict Detection Semantics

A supersession conflict exists if:

- A session references a Resolution that is no longer ACTIVE.
- A session references an Authority that has been superseded.
- A session references a Scope that has been superseded.
- Acceptance would create multiple ACTIVE successors in an exclusive legitimacy slot.
- Acceptance would introduce a cycle.

Conflict detection must occur:

- During evaluation.
- Immediately before acceptance commit.

If conflict is irreversible, session becomes BLOCK_PERMANENT.

If conflict is reversible (e.g., UNDER_REVIEW), session becomes BLOCK_TEMPORARY.

---

# 7. Revalidation Triggers

The engine must re-evaluate sessions when any of the following occur:

- A referenced Resolution is superseded.
- Authority is superseded.
- Scope is superseded.
- A referenced Resolution enters UNDER_REVIEW.
- Scope enters UNDER_REVIEW.

Effects:

Resolution superseded:
- Referencing sessions → BLOCK_PERMANENT.

Authority superseded:
- All sessions in Area → BLOCK_PERMANENT.

Scope superseded:
- All sessions in Area → BLOCK_PERMANENT.

Resolution UNDER_REVIEW:
- Referencing sessions → BLOCK_TEMPORARY.

Scope UNDER_REVIEW:
- All sessions in Area → BLOCK_TEMPORARY.
- Acceptance is not permitted while Scope is UNDER_REVIEW.

Resolution RETIRED:
- Referencing sessions → BLOCK_TEMPORARY.

Returning from UNDER_REVIEW to ACTIVE does not require a session.

Supersession always requires a session.

---

# 8. Under-Review Graph Semantics

UNDER_REVIEW:

- Does not create a new Resolution.
- Does not modify supersession edges.
- Does not alter historical relationships.
- Temporarily suspends ACTIVE usability for legitimacy evaluation.

UNDER_REVIEW is reversible.

SUPERSEDED is permanent and graph-altering.

These states must not be conflated.

---

# 9. Atomicity and Concurrency

Acceptance must atomically verify:

- Referenced Resolution is ACTIVE.
- Authority is ACTIVE.
- Scope is ACTIVE.
- No supersession conflict exists.
- No cycle would be introduced.

Acceptance must acquire a legitimacy lock covering all affected graph nodes.

If any referenced node changes before commit:

- Acceptance fails deterministically.
- No partial graph mutation occurs.

The first successful acceptance establishes graph truth.

Subsequent conflicting attempts must fail.

---

# 10. Deterministic Restore Guarantee

Given identical Resolution records and supersession edges:

- Independent engine implementations must produce identical ACTIVE sets.
- Graph traversal must produce identical outcomes.
- No timestamp-based or heuristic selection is permitted.
- No implicit "latest wins" logic is allowed.

Supersession is determined solely by explicit edges.

If restore would produce multiple ACTIVE successors in an exclusive slot:

- Restore must fail deterministically.

---

# 11. Permanent Blocking Doctrine

A session must transition to BLOCK_PERMANENT if:

- Its referenced Resolution is superseded.
- Its Authority is superseded.
- Its Scope is superseded.
- Its acceptance would create structural graph violation.
- Its legitimacy reference is no longer valid and cannot be restored.

Permanent blocks cannot be resumed.

Forward motion requires a new session.

---

# 12. Engine Invariants

- Supersession edges are immutable once created.
- The graph must remain acyclic.
- Only explicit supersession defines precedence.
- No implicit authority inheritance exists.
- No automatic conflict resolution occurs.
- Graph integrity must be deterministic across implementations.
- First-accept wins is absolute.

---

# 13. Relationship to ENG-DECISION

ENG-DECISION governs:

- Session lifecycle
- Governance mutation
- Acceptance transaction

ENG-SUPERSESSION governs:

- Resolution graph structure
- Conflict semantics
- Supersession integrity

Both documents together define complete legitimacy mechanics.