# ENG-REVIEW-RETIRED  
Dynamic Legitimacy Suspension & Deprecation Model  
Status: DRAFT (Adjusted for V3 Invariants)  
Applies to: Engine Core (V1/V2+)  

This document must be interpreted in conjunction with:

- ENG-DOMAIN  
- ENG-DECISION  
- ENG-SUPERSESSION  
- ENG-INTEGRITY  

If conflict exists, ENG-INTEGRITY runtime guarantees take precedence.

---

# 1. Purpose

This document defines the semantics of UNDER_REVIEW and RETIRED states for Resolutions.

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

The Resolution state enumeration is defined in ENG-DOMAIN.

---

# 2. Governance Control Model

## 2.1 Session Governance Rule

All legitimacy-affecting changes to Resolutions must occur through a successful governance session acceptance.

The only exception is the administrative transition:

ACTIVE ↔ UNDER_REVIEW

This transition may occur without a governance session.

All other transitions require a governance session:

- ACTIVE ↔ RETIRED  
- ACTIVE → SUPERSEDED  
- UNDER_REVIEW → SUPERSEDED  
- RETIRED → SUPERSEDED  

This guarantees:

- Governance evolution is explicit  
- Legitimacy changes are session-mediated  
- Administrative suspension remains lightweight  

---

## 2.2 Authority Primacy Rule

Authority is the foundational governance object for an Area.

Rules:

- Authority must be defined before any other governance object  
- Authority must always be ACTIVE for legitimacy evaluation to proceed  
- If Authority is not ACTIVE, all REGULAR sessions are blocked  
- Authority cannot enter UNDER_REVIEW or RETIRED  
- Authority evolution occurs only via supersession sessions (ENG-SUPERSESSION)

---

## 2.3 Scope Definition Rule

Scope defines the semantic boundaries of an Area.

Rules:

- Scope exists for human governance interpretation  
- Scope must be defined after Authority is established  
- Scope supports ACTIVE, UNDER_REVIEW, SUPERSEDED  
- Scope does not support RETIRED  
- If Scope is undefined or UNDER_REVIEW, all REGULAR sessions are blocked  
- Administrative transitions (UNDER_REVIEW) are explicit and auditable  
- Scope can be superseded by a new Scope via governance session  

---

# 3. Reviewable Objects

Resolution supports:

- ACTIVE  
- UNDER_REVIEW  
- RETIRED  
- SUPERSEDED  

Scope Resolution supports:

- ACTIVE  
- UNDER_REVIEW  
- SUPERSEDED  

Authority Resolution supports:

- ACTIVE  
- SUPERSEDED  

Authority cannot enter UNDER_REVIEW or RETIRED.

---

# 4. Structural vs Usability Distinction

## 4.1 Structural ACTIVE

Structural ACTIVE is derived solely from supersession edges (ENG-SUPERSESSION).  
UNDER_REVIEW and RETIRED do not alter structural ACTIVE.

---

## 4.2 Usable for Legitimacy

A Resolution is usable for legitimacy evaluation only if:

- Structurally ACTIVE  
- Not UNDER_REVIEW  
- Not RETIRED  

A Scope Resolution is usable only if:

- Structurally ACTIVE  
- Not UNDER_REVIEW  

Authority must always be ACTIVE.  
Usability is evaluated per session in ENG-DECISION.

---

# 5. Allowed State Transitions

## 5.1 Resolution

- ACTIVE ↔ UNDER_REVIEW (administrative)  
- ACTIVE ↔ RETIRED (requires session)  
- ACTIVE → SUPERSEDED (requires session)  
- UNDER_REVIEW → SUPERSEDED (requires session)  

SUPERSEDED is structural and terminal.  
RETIRED cannot transition to SUPERSEDED.

---

## 5.2 Scope Resolution

- ACTIVE ↔ UNDER_REVIEW (administrative)  
- ACTIVE → SUPERSEDED (requires session)  
- UNDER_REVIEW → SUPERSEDED (requires session)  

SUPERSEDED is terminal.

---

# 6. Legitimacy Neutrality

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
- Marks the Resolution deprecated  
- Suspends forward usability  

SUPERSEDED:

- Creates supersession edge  
- Alters structural ACTIVE  
- Terminal  

---

# 7. Blocking Matrix

Blocking behavior is enforced during session evaluation per ENG-DECISION.

## 7.1 Resolution UNDER_REVIEW

Referencing sessions:

- Transition → BLOCK_TEMPORARY  
- Votes cleared  
- Phase → PRE_STANCE  
- Resume required  
- Acceptance prohibited  

---

## 7.2 Resolution RETIRED

Referencing sessions:

- Transition → BLOCK_TEMPORARY  
- Votes cleared  
- Phase → PRE_STANCE  
- Resume required  
- Acceptance prohibited  

Forward progress requires:

- Reactivating the Resolution (administrative)  
or  
- Superseding via governance session  

---

## 7.3 Scope UNDER_REVIEW

All sessions in the Area:

- Transition → BLOCK_TEMPORARY  
- Votes cleared  
- Acceptance prohibited until Scope returns to ACTIVE  

---

## 7.4 Resolution SUPERSEDED

Referencing sessions:

- Transition → BLOCK_PERMANENT  
- Resume prohibited  
- Acceptance permanently impossible  

---

## 7.5 Scope SUPERSEDED

All sessions in the Area:

- Transition → BLOCK_PERMANENT  
- Resume prohibited  
- Acceptance permanently impossible  

---

# 8. Resume Confirmation Rules

For any BLOCK_TEMPORARY:

- Votes cleared  
- Phase → PRE_STANCE  
- Participants re-specified  
- Governance re-established before voting resumes  

Resuming:

- Never restores prior votes  
- Never overrides UNDER_REVIEW or RETIRED  
- Never bypasses usability constraints  

---

# 9. Acceptance Guard

Acceptance fails deterministically if any referenced object is not usable.

Usable means:

Resolution:

- Structurally ACTIVE  
- Not UNDER_REVIEW  
- Not RETIRED  

Scope:

- Structurally ACTIVE  
- Not UNDER_REVIEW  

Authority:

- Must be ACTIVE  

Atomicity governed by ENG-DECISION.

---

# 10. Concurrency Requirements

Transitions to UNDER_REVIEW or RETIRED must:

- Be atomic  
- Trigger deterministic re-evaluation of affected sessions  
- Not partially update session states  

If state change occurs during acceptance:

- Acceptance fails before commit  
- No partial mutation occurs  

Supersession concurrency governed by ENG-SUPERSESSION.

---

# 11. Receipt Interaction

Receipts freeze governance context at acceptance time.

Later transitions to UNDER_REVIEW or RETIRED:

- Do not alter historical receipts  
- Do not invalidate past legitimacy  

Receipts remain authoritative records of acceptance events.

---

# 12. Engine Invariants

- UNDER_REVIEW never alters supersession edges  
- RETIRED never alters supersession edges  
- SUPERSEDED alters structural ACTIVE permanently  
- Temporary suspension never creates legitimacy  
- Temporary suspension never destroys historical legitimacy  
- SUPERSEDED causes BLOCK_PERMANENT  
- Closure always requires explicit user action  
- Acceptance requires all referenced objects usable  

Structural inconsistency during restore must halt the engine (ENG-INTEGRITY).

---

# 13. Relationship to Other Specifications

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

ENG-REVIEW-RETIRED governs:

- Suspension semantics  
- Deprecation semantics  
- Temporary legitimacy interruption  

Together these define legitimacy creation, suspension, deprecation, and structural evolution.