# ENG-REVIEW-RETIRED
UNDER_REVIEW Semantics & Dynamic Legitimacy Suspension  
Status: DRAFT (Pre-Freeze)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

This document defines the semantics of UNDER_REVIEW and RETIRED states for:

- Resolutions
- Scope

They both govern:

- Allowed state transitions
- Blocking behavior
- Resume confirmation rules
- Interaction with sessions
- Interaction with evaluation
- Legitimacy neutrality guarantees

UNDER_REVIEW is a dynamic legitimacy suspension mechanism.
RETIRED is 

It does not create legitimacy.
It does not revoke legitimacy.
It does not modify supersession edges.

It temporarily suspends usability for decision purposes.

---

# 2. Reviewable Objects

The following objects may enter UNDER_REVIEW:

- Resolution
- Scope

Authority does not support UNDER_REVIEW.
Authority changes require supersession and result in permanent blocking per ENG-SUPERSESSION.

---

# 3. Allowed States

For Resolution:

- ACTIVE
- UNDER_REVIEW
- SUPERSEDED
- RETIRED 

For Scope:

- ACTIVE
- UNDER_REVIEW
- SUPERSEDED

No additional states are permitted.

---

# 4. Allowed State Transitions

## 4.1 Resolution

ACTIVE → UNDER_REVIEW  
UNDER_REVIEW → ACTIVE  
ACTIVE → SUPERSEDED  
ACTIVE → RETIRED 
RETIRED → ACTIVE 
UNDER_REVIEW → SUPERSEDED  
RETIRED → SUPERSEDED

SUPERSEDED is terminal.

UNDER_REVIEW → SUPERSEDED must occur via a valid session acceptance.

Returning UNDER_REVIEWto ACTIVE does not require a session.

Any state changes to and from RETIRED must occur via a valid session acceptance.

---

## 4.2 Scope

ACTIVE → UNDER_REVIEW  
UNDER_REVIEW → ACTIVE  
ACTIVE → SUPERSEDED  
UNDER_REVIEW → SUPERSEDED  

SUPERSEDED is terminal.

Scope supersession must occur via a valid session acceptance.

Returning to ACTIVE does not require a session.

---

# 5. Legitimacy Neutrality

UNDER_REVIEW:

- Does not create a new Resolution.
- Does not modify supersession edges.
- Does not change graph history.
- Does not invalidate past legitimacy.
- Does not alter ACTIVE status historically.

UNDER_REVIEW only affects forward decision usability.

SUPERSEDED is permanent and graph-altering.
UNDER_REVIEW is temporary and graph-neutral.

These states must not be conflated.

---

# 6. Blocking Matrix

The following matrix defines required engine behavior.

## 6.1 Resolution UNDER_REVIEW

If a session references a Resolution that enters UNDER_REVIEW:

- Session transitions to BLOCK_TEMPORARY.
- All active votes are cleared.
- Session phase returns to PRE_STANCE.
- Resume is required.
- Participants must be explicitly re-specified before voting restarts.

---

## 6.2 Scope UNDER_REVIEW

If Scope enters UNDER_REVIEW:

- All ACTIVE sessions in the Area transition to BLOCK_TEMPORARY.
- All active votes in those sessions are cleared.
- Acceptance is not permitted for any session in the Area.
- Resume is required after Scope returns to ACTIVE.

Participants must be explicitly re-specified upon resume.

---

## 6.3 Resolution SUPERSEDED

If a referenced Resolution becomes SUPERSEDED:

- Referencing sessions transition to BLOCK_PERMANENT.
- Session cannot resume.
- Forward motion requires a new session.

---

## 6.4 Scope SUPERSEDED

If Scope becomes SUPERSEDED:

- All sessions in the Area transition to BLOCK_PERMANENT.
- Sessions cannot resume.
- Forward motion requires new sessions under the new Scope.

---
## 6.5 Resolution RETIRED

If a session references a Resolution that enters RETIRED:

- Session transitions to BLOCK_TEMPORARY.
- All active votes are cleared.
- Session phase returns to PRE_STANCE.
- Resume is required.
- Participants must be explicitly re-specified before voting restarts.
- May close the session if session is not longer valid.

---

# 7. Resume Confirmation Rules

For any BLOCK_TEMPORARY caused by UNDER_REVIEW:

- Votes are cleared.
- Session phase returns to PRE_STANCE.
- Participants must be explicitly re-specified.
- Governance must be re-established before voting resumes.
- Resume event must be recorded in session history for receipt derivation.

Resuming does not restore prior voting state.

Blocking resets voting because context has been interrupted.

---

# 8. Interaction With ENG-DECISION

UNDER_REVIEW interacts with session mechanics as follows:

- Blocking due to UNDER_REVIEW follows standard BLOCK_TEMPORARY behavior.
- Acceptance validation must initially fail while referenced objects are UNDER_REVIEW. This is to cause a temporary block only.
- Evaluation must detect UNDER_REVIEW state deterministically.
- Governance immutability rules remain unchanged.

UNDER_REVIEW does not modify session governance rules.
It only affects acceptability.

---

# 9. Interaction With ENG-SUPERSESSION

UNDER_REVIEW:

- Does not create supersession edges.
- Does not change ACTIVE lineage.
- Does not affect acyclicity.
- Does not determine graph precedence.

SUPERSEDED remains governed exclusively by ENG-SUPERSESSION.

---

# 10. Evaluation API Requirements

Evaluation must:

- Detect if referenced Resolution is UNDER_REVIEW.
- Detect if Scope is UNDER_REVIEW.
- Indicate that acceptance is currently impossible.
- Classify blocking as temporary.

Evaluation must return structured, machine-readable reason codes.

Evaluation must not mutate state.

If object returns to ACTIVE, evaluation must reflect that deterministically.

---

# 11. Concurrency Requirements

State transitions to UNDER_REVIEW must:

- Be atomic.
- Trigger immediate deterministic re-evaluation of affected sessions.
- Not partially update session states.

If UNDER_REVIEW occurs during an acceptance attempt:

- Acceptance must fail deterministically before commit.
- No partial mutation is permitted.

---

# 12. Engine Invariants

- UNDER_REVIEW never alters supersession edges.
- UNDER_REVIEW never creates legitimacy.
- UNDER_REVIEW never revokes past legitimacy.
- SUPERSEDED is permanent and terminal.
- Blocking caused by UNDER_REVIEW is always temporary.
- Blocking caused by SUPERSEDED is always permanent.
- Resume always requires participant reconfirmation.
- Voting state is always cleared after temporary block.

---

# 13. Relationship to Other Specifications

ENG-DECISION governs:

- Session lifecycle
- Acceptance mechanics
- Blocking reset behavior

ENG-SUPERSESSION governs:

- Graph structure
- Supersession edges
- Permanent structural conflicts

ENG-REVIEW governs:

- Temporary suspension semantics
- UNDER_REVIEW state behavior
- Dynamic session interruption rules

Together, these documents fully define legitimacy creation, evolution, and temporary suspension within the engine.