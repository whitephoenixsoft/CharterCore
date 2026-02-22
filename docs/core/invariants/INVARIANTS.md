# Charter Core — Engine Invariants
Status: FROZEN (V2-Minimal)  
Applies to: Charter Core Engine  
Violations indicate an engine correctness failure

---

## Purpose

These invariants define the **non-negotiable mechanical behavior** of the Charter Core engine.  

They exist to preserve:

- legitimacy
- determinism
- auditability
- non-retroactivity

Behavior not guaranteed here must be implemented **outside** the engine.

---

## I. Core Boundary

### ENG-CORE-01 — Charter Core Is a Legitimacy Engine

Charter Core **does not**:

- reason about content
- facilitate workflow
- interpret semantics
- infer intent

Its responsibility is strictly to:

- enforce explicit legitimacy
- maintain deterministic governance rules
- ensure auditability
- prevent retroactive changes

---

## II. Identity & Immutability

### ENG-ID-01 — Canonical Engine Identity

All engine entities (Areas, Sessions, Resolutions, Candidates, Authority, Scope, Participants) must have stable, canonical IDs that survive restarts and export/import.

Fail if identity changes due to relabeling, reorganization, or serialization.

---

### ENG-ID-02 — Accepted Resolutions Are Immutable

Once a resolution is accepted:

- content and acceptance context are immutable
- state changes only via **explicit supersession** or **retirement**

Fail if content or context is edited or reinterpreted.

---

## III. Legitimacy & Decision Semantics

### ENG-LEG-01 — Explicit Decisions Only

Legitimacy arises **only** from explicit acceptance within a session.

---

### ENG-LEG-02 — Sessions Are the Unit of Legitimacy

Sessions define participants, authority, constraints, and outcomes.  
Resolutions outside sessions do not confer legitimacy.

---

### ENG-LEG-03 — Legitimacy Evaluated at Acceptance

Legitimacy is determined **only** by:

- Authority active at acceptance
- Scope active at acceptance
- Constraints at acceptance
- Recorded stances at acceptance

---

### ENG-LEG-04 — Deterministic Evaluation

Identical inputs (participants, stances, authority, constraints) must produce identical outcomes.

---

### ENG-LEG-05 — Explicit Dissent Is First-Class

Abstention is explicit. Silence is meaningless. Absence is not rejection.

---

## IV. Resolution Lifecycle & History

### ENG-HIST-01 — Append-Only History

Resolutions are never edited or deleted. Corrections require **supersession** or **clarifying resolutions**.

---

### ENG-HIST-02 — Explicit States

Valid states: ACTIVE, UNDER_REVIEW, SUPERSEDED, RETIRED.  
UNDER_REVIEW has no governing power. Transitions are explicit and auditable.

---

### ENG-HIST-04 — Supersession Is One-Way

Supersession is irreversible and directional. Superseded resolutions cannot regain ACTIVE status.

---

## V. Areas, Authority, and Scope

### ENG-AREA-01 — Areas Are Governance Boundaries

Each resolution belongs to exactly one Area. Cross-area relevance must be explicit.

---

### ENG-AREA-02 — Area Initialization

An Area requires exactly one ACTIVE Authority and one ACTIVE Scope.  
Fail if decisions occur in an uninitialized Area.

---

### ENG-AREA-03 — Governance Anchors Cannot Be Retired

Authority and Scope **cannot** be retired. Changes occur only via supersession.

---

### ENG-AUTH-01 — Authority Is Mechanical

Authority defines vote evaluation.  

Rules:

- One ACTIVE Authority per Area
- Cannot enter UNDER_REVIEW
- Changes require a session
- Never rewrites history

---

### ENG-SCOPE-01 — Scope Is Descriptive

Scope is first-class but descriptive. Scope may enter **UNDER_REVIEW**, which removes governing power.  
Entering/exiting UNDER_REVIEW must be explicit and auditable.

---

### ENG-CONTEXT-01 — Authority and Scope Permanently Recorded

Every accepted resolution and session permanently records Authority and Scope at acceptance.

---

## VI. Sessions, Candidates, and Constraints

### ENG-SES-01 — Candidates Are Neutral

Candidates are non-legitimacy artifacts. Rejection or abandonment has no effect.

---

### ENG-SES-02 — Candidate Set Freezes

After first stance, candidate content is immutable.

---

### ENG-CON-01 — Constraints Are Engine-Enforced

Constraints are session-scoped and do not modify Authority or Scope.

---

### ENG-SES-03 — Pause, Block, and Resume

Blocked sessions (e.g., due to UNDER_REVIEW Scope) **cannot proceed** until context is explicitly revalidated.

---

## VII. Voting & Acceptance

### ENG-VOTE-01 — Votes Are Evaluative Only

Votes do not automatically create legitimacy.

---

### ENG-ACCEPT-01 — Explicit Acceptance Required

Legitimacy arises only via explicit acceptance in a session under current Authority and Scope.

---

### ENG-ACCEPT-04 — Non-Retroactivity

Changes to votes, authority, or scope after acceptance cannot affect past legitimacy.

---

## VIII. Concurrency & Isolation

### ENG-CONCUR-01 — Concurrent Sessions Are Isolated

Sessions only interfere if Authority or Scope changes, or if a resolution is superseded.  
Affected sessions must pause or block.

---

## IX. Import, Export, and Review

### ENG-EXP-01 — Only Closed Sessions Export Legitimacy

Active or paused sessions do not export legitimacy.

---

### ENG-IMP-01 — Consolidation Preserves Legitimacy Only

Imported resolutions do not affect current legitimacy; imported sessions are read-only.

---

## XIV. Supersession, Review, and Governance Stability

### ENG-SUP-01 — Supersession Is One-Way

Supersession is directional and irreversible.

---

### ENG-SUP-02 — Authority Is Immutable Except by Supersession

Authority cannot enter UNDER_REVIEW; exactly one ACTIVE Authority must exist per Area.

---

### ENG-SUP-03 — Scope May Enter Review State

Scope may enter UNDER_REVIEW. Entering/exiting review is explicit and auditable.

---

### ENG-SUP-04 — Review Blocks Dependent Legitimacy

Sessions depending on a resolution under review are blocked.

---

### ENG-SUP-05 — Context Revalidation After Governance Change

Blocked sessions must revalidate governing context explicitly before proceeding.

---

## XV. Explicit Non-Goals

Charter Core engine does **not** provide:

- reasoning, workflow, facilitation, or semantic interpretation

---

## Lock Statement

These invariants are frozen. Adherence defines engine correctness.