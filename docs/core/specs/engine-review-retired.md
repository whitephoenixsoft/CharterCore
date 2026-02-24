# ENG-REVIEW-RETIRED  
Dynamic Legitimacy Suspension & Deprecation Model  
Status: DRAFT (Pre-Freeze)  
Applies to: Engine Core (V1/V2+)  

This document must be interpreted in conjunction with:

- ENG-DOMAIN  
- ENG-DECISION  
- ENG-SUPERSESSION  
- ENG-INTEGRITY  

If conflict exists, ENG-INTEGRITY runtime guarantees take precedence.

---

# 1. Purpose

This document defines the semantics of UNDER_REVIEW and RETIRED states for:

- Resolution  
- Scope  

These states govern:

- Allowed state transitions  
- Blocking behavior  
- Resume confirmation rules  
- Interaction with sessions  
- Interaction with evaluation  
- Legitimacy neutrality guarantees  

UNDER_REVIEW is an administrative suspension mechanism.  
RETIRED is a governance-level deprecation mechanism.  

Neither:

- Creates legitimacy  
- Revokes past legitimacy  
- Modifies supersession edges  
- Alters structural ACTIVE derivation  

SUPERSEDED remains the only graph-altering state.

---

# 2. Reviewable Objects

- Resolution supports: ACTIVE, UNDER_REVIEW, RETIRED, SUPERSEDED  
- Scope supports: ACTIVE, UNDER_REVIEW, SUPERSEDED  

Authority does not support UNDER_REVIEW or RETIRED.

Authority changes require supersession and are governed exclusively by ENG-SUPERSESSION.

---

# 3. Structural vs Usability Distinction

## 3.1 Structural ACTIVE

Structural ACTIVE is derived solely from supersession edges (ENG-SUPERSESSION).

UNDER_REVIEW and RETIRED do not affect structural ACTIVE derivation.

---

## 3.2 Usable for Legitimacy

A Resolution or Scope is usable for legitimacy evaluation only if:

- Structurally ACTIVE  
- Not UNDER_REVIEW  
- Not RETIRED  

Usability is evaluated at runtime.

---

# 4. Allowed State Transitions

## 4.1 Resolution

- ACTIVE ↔ UNDER_REVIEW (no session required)  
- ACTIVE ↔ RETIRED (requires session)  
- ACTIVE → SUPERSEDED (requires session)  
- UNDER_REVIEW → SUPERSEDED (requires session)  
- RETIRED → SUPERSEDED (requires session)  

SUPERSEDED is structural and terminal.

---

## 4.2 Scope

- ACTIVE ↔ UNDER_REVIEW (no session required)  
- ACTIVE → SUPERSEDED (requires session)  
- UNDER_REVIEW → SUPERSEDED (requires session)  

SUPERSEDED is terminal.

---

# 5. Legitimacy Neutrality

UNDER_REVIEW:

- Does not create a new Resolution  
- Does not modify supersession edges  
- Does not alter structural ACTIVE  
- Does not affect historical legitimacy  
- Suspends forward usability only  

RETIRED:

- Does not create a new Resolution  
- Does not modify supersession edges  
- Does not alter structural ACTIVE  
- Does not affect historical legitimacy  
- Marks Resolution deprecated  
- Suspends forward usability  

SUPERSEDED:

- Creates supersession edge  
- Alters structural ACTIVE  
- Terminal  

States must remain distinct.

---

# 6. Blocking Matrix

## 6.1 Resolution UNDER_REVIEW

Referencing sessions:

- Transition → BLOCK_TEMPORARY  
- Votes cleared  
- Phase → PRE_STANCE  
- Resume required  
- Acceptance prohibited while UNDER_REVIEW  

---

## 6.2 Resolution RETIRED

Referencing sessions:

- Transition → BLOCK_TEMPORARY  
- Votes cleared  
- Phase → PRE_STANCE  
- Resume required  
- Acceptance prohibited while RETIRED  

Forward progress requires:

- Session reactivating Resolution  
  OR  
- Session superseding Resolution  

---

## 6.3 Scope UNDER_REVIEW

All sessions in Area:

- Transition → BLOCK_TEMPORARY  
- Votes cleared  
- Acceptance prohibited until ACTIVE  

Resume permitted once Scope returns to ACTIVE.

---

## 6.4 Resolution SUPERSEDED

Referencing sessions:

- Transition → BLOCK_PERMANENT  
- Resume prohibited  
- Acceptance permanently impossible  

User must explicitly close session or restart-from.

Area-level acceptance blocking enforced by ENG-INTEGRITY.

---

## 6.5 Scope SUPERSEDED

All sessions in Area:

- Transition → BLOCK_PERMANENT  
- Resume prohibited  
- Acceptance permanently impossible  

User must explicitly close sessions.

Area-level acceptance blocking enforced by ENG-INTEGRITY.

---

# 7. Resume Confirmation Rules

For any BLOCK_TEMPORARY:

- Votes cleared  
- Phase → PRE_STANCE  
- Participants re-specified  
- Governance re-established before voting resumes  
- Resume event recorded in session history  

Resuming:

- Never restores prior votes  
- Never overrides UNDER_REVIEW or RETIRED state  
- Never bypasses usability constraints  

---

# 8. Acceptance Guard

Acceptance must fail deterministically if any referenced object is not usable:

- Authority  
- Scope  
- Referenced Resolution  

Usable = structurally ACTIVE and not UNDER_REVIEW or RETIRED.

No partial validation allowed.

Atomicity governed by ENG-DECISION.

---

# 9. Concurrency Requirements

Transitions to UNDER_REVIEW or RETIRED must:

- Be atomic  
- Trigger deterministic re-evaluation of affected sessions  
- Not partially update session states  

If state change occurs during acceptance:

- Acceptance fails before commit  
- No partial mutation occurs  

Supersession concurrency governed by ENG-SUPERSESSION.

---

# 10. Engine Invariants

- UNDER_REVIEW never alters supersession edges  
- RETIRED never alters supersession edges  
- SUPERSEDED alters structural ACTIVE permanently  
- Temporary suspension never creates legitimacy  
- Temporary suspension never destroys historical legitimacy  
- SUPERSEDED causes BLOCK_PERMANENT (not auto-closure)  
- Closure always requires explicit user action  
- Acceptance requires all referenced objects usable  

Structural inconsistency detected during restore must halt engine (ENG-INTEGRITY).

---

# 11. Relationship to Other Specifications

ENG-DECISION governs:

- Session lifecycle  
- Blocking state transitions  
- Acceptance mechanics  

ENG-SUPERSESSION governs:

- Graph structure  
- Structural ACTIVE derivation  
- Supersession integrity  

ENG-INTEGRITY governs:

- Engine halt conditions  
- Area-level acceptance guards  
- Structural failure handling  

ENG-REVIEW-RETIRED governs:

- Suspension semantics  
- Deprecation semantics  
- Temporary legitimacy interruption  

Together, these define legitimacy creation, suspension, deprecation, and structural evolution.