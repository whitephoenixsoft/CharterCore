# ENG-SUPERSESSION  
Supersession & Conflict Model  
Status: FROZEN (v3 – Cross-Area Structural Boundary Formalized)  
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
- Area-local supersession sovereignty  

This document does not define session lifecycle mechanics.

It defines graph integrity and legitimacy conflict behavior.

---

# 2. Resolution Graph Model

## 2.1 Directed Supersession

A Resolution may supersede zero or more prior Resolutions **within the same Area only**.

Authority and Scope participate in the supersession graph exactly like any other Resolution within their respective slots.

Supersession is:

- Explicit  
- Directional  
- Immutable  
- Recorded at acceptance time only  
- Strictly Area-local  

If Resolution B supersedes Resolution A:

- B contains an immutable structural reference to A.  
- A does not mutate structurally.  
- A remains part of historical graph.  

Supersession edges are permanent once created.

---

## 2.2 Acyclic Requirement

The supersession graph must remain acyclic within each Area.

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

## 2.3 Area Sovereignty

Supersession is strictly scoped to a single Area.

A Resolution may supersede only Resolutions where:

- resolution.area_id == new_resolution.area_id

Cross-area supersession is prohibited.

Violation at acceptance time:

- Deterministic acceptance failure.  
- No graph mutation.

Violation detected at restore:

- StructuralIntegrityFailure.  
- Engine must halt.

Cross-area references (as defined in ENG-DOMAIN) are informational only and must never be interpreted as supersession edges.

---

# 3. ACTIVE Resolution Derivation

## 3.1 Structural ACTIVE

A Resolution is structurally ACTIVE if:

- It has no accepted successor in the same Area.  
- It is not structurally inconsistent.  

Structural ACTIVE is derived solely from supersession edges within the Area.

Cross-area references must not influence ACTIVE derivation.

Authority and Scope ACTIVE status is derived the same way.

---

## 3.2 Legitimacy Usability

A Resolution is usable for legitimacy evaluation only if:

- Structurally ACTIVE  
- State is not UNDER_REVIEW  
- State is not RETIRED  

Legitimacy usability is evaluated at runtime.

External Area state must not influence usability.

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

# 4. Structural vs Informational References

## 4.1 Structural References

Structural references affecting supersession are limited to:

- Resolution → superseded Resolution (same Area only)

These references:

- Must resolve.
- Must be validated during acceptance.
- Must be validated during restore.
- Affect ACTIVE derivation.

Missing structural references constitute structural integrity failure.

---

## 4.2 Informational Cross-Area References

Cross-area references (per ENG-DOMAIN):

- May reference external Areas or Resolutions.
- Must not be interpreted as supersession edges.
- Must not be traversed.
- Must not be validated for existence.
- Must not affect ACTIVE derivation.
- Must not affect governance slot exclusivity.
- Must not cause restore failure if unresolved.

Deletion or absence of a referenced external Area or Resolution must not alter:

- Structural ACTIVE derivation.
- Conflict detection.
- Governance usability.
- Blocking behavior.

They are metadata only.

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
- No cross-area arbitration.

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
- Acceptance attempts cross-area supersession.

Conflict detection must occur:

- During evaluation.  
- Immediately before acceptance commit.

Irreversible conflict → BLOCK_PERMANENT (session-level).  
Reversible conflict (UNDER_REVIEW, RETIRED) → BLOCK_TEMPORARY.

---

# 8. Deterministic Restore Guarantee

Given identical persisted objects and supersession edges:

- Independent implementations must derive identical structural ACTIVE sets per Area.  
- Governance slot evaluation must produce identical results.  
- No heuristic, timestamp, ordering-based, or cross-area logic permitted.  

If restore produces:

- A cycle  
- Multiple structurally ACTIVE Authorities  
- Multiple structurally ACTIVE Scopes  
- Zero structurally ACTIVE Authority  
- Zero structurally ACTIVE Scope (after definition)  
- Invalid structural supersession references  
- Cross-area supersession edges  

Engine initialization must fail (StructuralIntegrityFailure per ENG-INTEGRITY).

The engine must halt.

No automatic repair is permitted.

---

# 9. Permanent Blocking Doctrine

A session must transition to BLOCK_PERMANENT if:

- Its referenced Resolution becomes non-structurally-ACTIVE.  
- Its Authority is superseded.  
- Its Scope is superseded.  
- Acceptance would introduce structural violation.  
- Acceptance attempts cross-area supersession.

Permanent blocks:

- Cannot resume.  
- Require explicit closure.  
- May be restarted via explicit operator action.

Area-level acceptance prohibition while permanent blocks exist is defined in ENG-INTEGRITY.

---

# 10. Engine Invariants

- Supersession edges are immutable.  
- Supersession graph is Area-local.  
- Graph must remain acyclic.  
- Structural ACTIVE derivation is deterministic.  
- Authority and Scope participate fully in supersession graph.  
- Governance slots must never be empty or multiply ACTIVE.  
- Cross-area references are never structural.  
- No implicit conflict resolution exists.  
- First-accept wins is absolute.  
- Structural inconsistency must halt the engine.

---

# 11. Relationship to Other Specifications

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
- Area-local sovereignty  

ENG-INTEGRITY governs:

- Engine halt conditions  
- Area-level acceptance guards  
- Structural failure handling  

Together they define complete legitimacy mechanics.