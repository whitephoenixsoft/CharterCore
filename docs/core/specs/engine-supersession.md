# ENG-SUPERSESSION  
Supersession & Conflict Model  
Status: FROZEN (v2 – Governance Slot Enforcement Integrated)  
Applies to: Engine Core (V1/V2+)  

This document must be interpreted in conjunction with:

- ENG-DOMAIN  
- ENG-DECISION  
- ENG-REVIEW-RETIRED  
- ENG-INTEGRITY  

If conflict exists, ENG-INTEGRITY runtime guarantees take precedence.

---

# 1. Purpose

This document defines:

- Resolution graph structure  
- Supersession linking rules  
- ACTIVE-set derivation  
- Governance slot participation in supersession  
- Conflict detection  
- Race condition semantics  
- Permanent blocking triggers  
- Revalidation requirements  
- Deterministic restore guarantees  

This document does not define session lifecycle mechanics.

It defines graph integrity and legitimacy conflict behavior.

---

# 2. Resolution Graph Model

## 2.1 Directed Supersession

A Resolution may supersede zero or more prior Resolutions.

Authority and Scope participate in the supersession graph exactly like any other Resolution within their respective slots.

Supersession is:

- Explicit  
- Directional  
- Immutable  
- Recorded at acceptance time only  

If Resolution B supersedes Resolution A:

- B contains an immutable reference to A.  
- A does not mutate structurally.  
- A remains part of historical graph.  

Supersession edges are permanent once created.

---

## 2.2 Acyclic Requirement

The supersession graph must remain acyclic.

A Resolution may not:

- Supersede itself.  
- Introduce a cycle directly or indirectly.  

Cycle detection must occur at acceptance time.

If acceptance would introduce a cycle:

- Acceptance must fail deterministically.  
- No graph mutation may occur.

If a cycle is detected during restore:

- Engine initialization must fail (StructuralIntegrityFailure per ENG-INTEGRITY).  
- Engine must halt.

---

## 2.3 Area Locality

Supersession is scoped to an Area.

A Resolution may supersede only Resolutions within the same Area.

Cross-area references are informational only and have no supersession effect.

Violation at acceptance time → deterministic acceptance failure.  
Violation detected at restore → StructuralIntegrityFailure.

---

# 3. ACTIVE Resolution Derivation

## 3.1 Structural ACTIVE

A Resolution is structurally ACTIVE if:

- It has no accepted successor in the same Area.  
- It is not structurally inconsistent.  

Structural ACTIVE is derived solely from supersession edges.

Structural ACTIVE does not consider UNDER_REVIEW or RETIRED.

Authority and Scope ACTIVE status is derived the same way.

---

## 3.2 Legitimacy Usability

A Resolution is usable for legitimacy evaluation only if:

- Structurally ACTIVE  
- State is not UNDER_REVIEW  
- State is not RETIRED  

Legitimacy usability is evaluated at runtime.

---

## 3.3 Exclusive Governance Slots

Authority and Scope are exclusive governance slots per Area.

For each Area:

- Exactly one structurally ACTIVE Authority must exist.  
- Exactly one structurally ACTIVE Scope must exist (once Scope is defined).  

Supersession must never result in:

- Zero structurally ACTIVE Authority.  
- Zero structurally ACTIVE Scope (after initial definition).  
- More than one structurally ACTIVE Authority.  
- More than one structurally ACTIVE Scope.  

If acceptance would violate slot exclusivity or leave a slot empty:

- Acceptance must fail deterministically.  
- No graph mutation may occur.  

If restore produces slot multiplicity or emptiness:

- Engine initialization must fail (StructuralIntegrityFailure).  
- Engine must halt.  

No automatic repair is permitted.

---

# 4. Referencing Rules

## 4.1 Legitimate Session References

A session may reference:

- The current usable Authority.  
- The current usable Scope.  
- Zero or more structurally ACTIVE Resolutions for supersession purposes (as permitted by ENG-DECISION).

A session must not reference:

- A Resolution that is not structurally ACTIVE for supersession.  
- A Resolution outside its Area for legitimacy.  

If a session references a Resolution that becomes non-usable:

- Session transitions per ENG-DECISION (BLOCK_TEMPORARY or BLOCK_PERMANENT).  

Area-level acceptance blocking is governed by ENG-INTEGRITY.

---

## 4.2 Informational References

Sessions may reference other Areas or Resolutions for informational context.

These references:

- Do not affect legitimacy.  
- Do not create supersession edges.  
- Have no impact on ACTIVE derivation.

---

# 5. First-Accept Wins Rule

If multiple sessions attempt to supersede the same structurally ACTIVE Resolution:

- The first successful acceptance creates the successor edge.  
- The referenced Resolution becomes non-structurally-ACTIVE.  
- Competing sessions referencing that Resolution must transition to BLOCK_PERMANENT (ENG-DECISION).

There is:

- No automatic branch merging.  
- No implicit precedence rule.  
- No timestamp-based arbitration.

Explicit supersession edges define graph truth.

---

# 6. Governance Supersession Effects

## 6.1 Authority Supersession

When an Authority Resolution is superseded:

- The prior Authority becomes non-structurally-ACTIVE.  
- All sessions in the Area must transition to BLOCK_PERMANENT.  
- Acceptance in the Area is prohibited until sessions are explicitly closed or restarted.

Authority supersession must not leave the Authority slot empty.

If acceptance would remove the only structurally ACTIVE Authority without establishing a successor:

- Acceptance must fail.

---

## 6.2 Scope Supersession

When a Scope is superseded:

- The prior Scope becomes non-structurally-ACTIVE.  
- All sessions in the Area must transition to BLOCK_PERMANENT.

If Scope enters UNDER_REVIEW:

- All sessions in the Area transition to BLOCK_TEMPORARY.

Scope supersession must not leave the Scope slot empty once defined.

If acceptance would result in zero structurally ACTIVE Scope:

- Acceptance must fail.

---

# 7. Conflict Detection

A supersession conflict exists if:

- A session references a Resolution that is no longer structurally ACTIVE.  
- A session references an Authority or Scope that is no longer usable.  
- Acceptance would introduce a cycle.  
- Acceptance would violate exclusive governance slot constraints.  
- Acceptance would leave a governance slot empty.

Conflict detection must occur:

- During evaluation.  
- Immediately before acceptance commit.

Irreversible conflict → BLOCK_PERMANENT (session-level).  
Reversible conflict (UNDER_REVIEW, RETIRED) → BLOCK_TEMPORARY.

---

# 8. Revalidation Triggers

The engine must re-evaluate sessions when:

- A referenced Resolution is superseded.  
- Authority is superseded.  
- Scope is superseded.  
- A referenced Resolution enters UNDER_REVIEW.  
- A referenced Resolution enters RETIRED.  
- Scope enters UNDER_REVIEW.

Effects:

Resolution SUPERSEDED → Referencing sessions → BLOCK_PERMANENT  
Authority SUPERSEDED → All sessions in Area → BLOCK_PERMANENT  
Scope SUPERSEDED → All sessions in Area → BLOCK_PERMANENT  
Resolution UNDER_REVIEW → Referencing sessions → BLOCK_TEMPORARY  
Resolution RETIRED → Referencing sessions → BLOCK_TEMPORARY  
Scope UNDER_REVIEW → All sessions in Area → BLOCK_TEMPORARY  

Area-level acceptance blocking behavior enforced by ENG-INTEGRITY.

---

# 9. Atomicity and Concurrency

Acceptance must atomically verify:

- Referenced Resolution structurally ACTIVE.  
- Authority usable.  
- Scope usable.  
- Governance slots remain exactly one ACTIVE each.  
- No cycle introduction.

If any check fails:

- Acceptance fails deterministically.  
- No partial graph mutation occurs.

First successful acceptance establishes graph truth.

Subsequent conflicting attempts must fail.

---

# 10. Deterministic Restore Guarantee

Given identical persisted objects and supersession edges:

- Independent implementations must derive identical structural ACTIVE sets.  
- Governance slot evaluation must produce identical results.  
- No heuristic, timestamp, or ordering-based logic permitted.  

If restore produces:

- A cycle  
- Multiple structurally ACTIVE Authorities  
- Multiple structurally ACTIVE Scopes  
- Zero structurally ACTIVE Authority  
- Zero structurally ACTIVE Scope (after definition)  
- Invalid supersession references  

Engine initialization must fail (StructuralIntegrityFailure per ENG-INTEGRITY).

The engine must halt.

No automatic repair is permitted.

---

# 11. Permanent Blocking Doctrine

A session must transition to BLOCK_PERMANENT if:

- Its referenced Resolution becomes non-structurally-ACTIVE.  
- Its Authority is superseded.  
- Its Scope is superseded.  
- Acceptance would introduce structural violation.  

Permanent blocks:

- Cannot resume.  
- Require explicit closure.  
- May be restarted via explicit operator action.

Area-level acceptance prohibition while permanent blocks exist is defined in ENG-INTEGRITY.

---

# 12. Engine Invariants

- Supersession edges are immutable.  
- Graph must remain acyclic.  
- Structural ACTIVE derivation is deterministic.  
- Authority and Scope participate fully in supersession graph.  
- Governance slots must never be empty or multiply ACTIVE.  
- No implicit conflict resolution exists.  
- First-accept wins is absolute.  
- Structural inconsistency must halt the engine.

---

# 13. Relationship to Other Specifications

ENG-DECISION governs:

- Session lifecycle  
- Governance mutation  
- Acceptance transaction  

ENG-SUPERSESSION governs:

- Resolution graph structure  
- Governance slot participation  
- Structural ACTIVE derivation  
- Supersession integrity  
- Conflict semantics  

ENG-INTEGRITY governs:

- Engine halt conditions  
- Area-level acceptance guards  
- Structural failure handling  

Together they define complete legitimacy mechanics.