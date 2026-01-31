# Charter Foundation — Resolution & Session Status Specification

Status: FOUNDATIONAL  
Layer: Engine & Guidance Foundation  
Authority: Subordinate to Charter Core Engine Invariants and V5 Guidance Invariants  
Purpose: Define valid states and transitions for Resolutions and Sessions, preserve legitimacy, and clearly distinguish canonical engine review from baseline review.

---

## Overview

Charter separates:

- **Legitimacy-bearing states** (engine truth)
- **Exploratory or preparatory states** (baseline review)

This document defines which states exist, what they mean, and what they *cannot* do.

No state exists to infer intent.  
No state exists to “soften” legitimacy.

---

## I. Resolution Statuses (Canonical Engine)

### 1. ACTIVE

**Definition**  
The resolution is fully accepted and governs legitimacy according to the Authority and Scope recorded at acceptance time.

**Rules**

- Only ACTIVE resolutions create legitimacy.
- ACTIVE resolutions may transition only via:
  - Explicit supersession
  - Explicit retirement (if permitted)
- Content, annotations, and acceptance context are immutable.

**Fail if**

- ACTIVE content or context is modified.
- ACTIVE resolution re-enters UNDER_REVIEW.
- ACTIVE status is ambiguous or inferred.

---

### 2. UNDER_REVIEW (Engine-Level)

**Definition**  
A temporary blocking state indicating that a governing resolution is being reviewed and must not influence legitimacy.

**Rules**

- Only **Scope** resolutions may enter UNDER_REVIEW.
- **Authority resolutions MUST NOT** enter UNDER_REVIEW.
- UNDER_REVIEW resolutions have **no governing power**.
- Any session whose legitimacy depends on an UNDER_REVIEW resolution MUST be BLOCKED.
- Entering or exiting UNDER_REVIEW MUST be explicit and auditable.

**Fail if**

- Governing behavior changes implicitly.
- Sessions proceed or accept resolutions while dependency is UNDER_REVIEW.
- Review state is inferred or silent.

**Notes**

This is a **canonical engine construct**.  
It exists to block legitimacy without mutating history.

---

### 3. SUPERSEDED

**Definition**  
The resolution has been explicitly replaced by a newer resolution.

**Rules**

- Supersession is directional and irreversible.
- Historical content and acceptance context are preserved.
- A superseded resolution MUST NOT regain ACTIVE status.

**Fail if**

- Supersession is reversed or toggled.
- A superseded resolution becomes ACTIVE again.

---

### 4. RETIRED

**Definition**  
The resolution is permanently inactive and removed from governing consideration.

**Rules**

- Retired resolutions do not affect legitimacy.
- **Authority and Scope resolutions MUST NOT be retired.**

**Fail if**

- Authority or Scope is retired.
- A retired resolution influences legitimacy.

---

### 5. UNDER_REVIEW (Baseline Review)

**Definition**  
A **non-canonical**, exploratory review state used exclusively within baseline review workspaces.

**Rules**

- Baseline UNDER_REVIEW:
  - Does NOT block engine legitimacy
  - Does NOT alter canonical resolution states
  - Does NOT affect sessions or acceptance
- Used for:
  - Import reconciliation
  - Exploratory comparison
  - Deliberative synthesis

**Rationale**

This distinction prevents conflating *review* with *governance*.  
Baseline review supports human workflow without touching engine truth.

---

## II. Session Statuses (Canonical Engine)

### 1. ACTIVE

**Definition**  
The session is open and may develop candidates, record stances, or accept resolutions.

**Rules**

- Participants and constraints freeze on first stance.
- Legitimacy may only be created via explicit acceptance.
- Authority and Scope must be active and not under review.

**Fail if**

- Legitimacy is created implicitly.
- Frozen context is ignored or altered.

---

### 2. PAUSED

**Definition**  
The session is intentionally halted without blocking dependencies.

**Rules**

- No legitimacy actions may occur.
- Context (Authority, Scope, participants, constraints) remains unchanged.
- Resume requires explicit action.

**Fail if**

- Session resumes legitimacy actions automatically.
- Context mutates while paused.

---

### 3. BLOCKED

**Definition**  
The session cannot proceed due to unresolved legitimacy dependencies.

**Examples**

- Scope is UNDER_REVIEW
- Governing resolution was superseded mid-session

**Rules**

- Acceptance is disallowed.
- Session may remain visible and inspectable.
- Resume requires explicit revalidation of governing context **or**
  a new session must be started.

**Fail if**

- Legitimacy is created while blocked.
- Dependency resolution is bypassed or inferred.

---

### 4. CLOSED

**Definition**  
The session has completed all actions and is sealed.

**Rules**

- No further stances, candidates, or acceptances.
- All history is immutable and auditable.

**Fail if**

- Closed session mutates history or legitimacy.

---

## III. Governing Principles

### Legitimacy Separation

- Engine-level UNDER_REVIEW is **blocking**
- Baseline UNDER_REVIEW is **exploratory**
- These states must never be conflated

### Explicit Transitions Only

- No implicit state changes
- No inferred governance
- Every transition is auditable

### Governance Respect

- Authority is never under review or retired
- Scope review blocks dependent legitimacy
- Review never rewrites history

### Audit Integrity

Every status transition must be reconstructible:

- What changed
- When it changed
- Under what authority
- With what dependencies

---

## Summary

Resolution and session statuses exist to:

- Preserve legitimacy
- Prevent silent reinterpretation
- Block unsafe commitment without deleting history
- Support exploratory work without contaminating engine truth

Baseline review helps humans think.  
Sessions help humans commit.  
The engine records what actually happened.

Nothing else.