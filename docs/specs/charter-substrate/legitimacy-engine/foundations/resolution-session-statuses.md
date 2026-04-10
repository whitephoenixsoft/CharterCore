# Charter Foundation — Resolution & Session Status Specification

Status: FOUNDATIONAL  
Layer: Engine & Guidance Foundation  
Authority: Subordinate to Charter Core Engine Invariants and V5 Guidance Invariants  
Purpose: Define valid states and transitions for Resolutions and Sessions, preserve legitimacy, and clearly distinguish canonical engine review from Reconciliation review.

---

## Overview

Charter separates:

- **Legitimacy-bearing states** (engine truth)
- **Exploratory or preparatory states** (Reconciliation review)

This document defines which states exist, what they mean, and what they *cannot* do.

No state exists to infer intent.  
No state exists to “soften” legitimacy.

---

## I. Resolution Statuses (Canonical Engine)

### 1. ACTIVE

**Definition**  
The resolution is accepted and currently participates in governance evaluation according to the Authority and Scope recorded at acceptance time.

**Rules**

- Legitimacy is created only through **explicit session acceptance**
- ACTIVE resolutions may participate in governance evaluation
- ACTIVE resolutions may transition only via:
  - Explicit supersession
  - Explicit retirement (if permitted)

- Content, annotations, and acceptance context are immutable

**Fail if**

- ACTIVE content or context is modified
- ACTIVE resolution re-enters ON_HOLD without explicit administrative transition
- ACTIVE status is ambiguous or inferred

---

### 2. ON_HOLD (Engine-Level)

**Definition**  
A temporary usability state indicating that a governance-relevant resolution must not be used for legitimacy evaluation.

**Rules**

- Applies to governance-relevant resolutions as defined by engine rules (commonly Scope)
- ON_HOLD resolutions have **no governing effect**
- Any session depending on an ON_HOLD resolution must be **BLOCKED**
- Entering or exiting ON_HOLD must be explicit and auditable

**Fail if**

- Governing behavior changes implicitly
- Sessions proceed to successful acceptance while dependencies are ON_HOLD
- Review state is inferred or silent

**Notes**

This is a **canonical engine construct**.  
It blocks forward legitimacy without mutating historical truth.

---

### 3. SUPERSEDED

**Definition**  
The resolution has been explicitly replaced by a newer resolution.

**Rules**

- Supersession is directional and irreversible
- Historical content and acceptance context are preserved
- A superseded resolution must not regain ACTIVE status

**Fail if**

- Supersession is reversed or toggled
- A superseded resolution becomes ACTIVE again

---

### 4. RETIRED

**Definition**  
The resolution is permanently inactive and removed from governance consideration.

**Rules**

- Retired resolutions do not participate in governance evaluation
- Authority and Scope resolutions must not be retired

**Fail if**

- Authority or Scope is retired
- A retired resolution influences legitimacy evaluation

---

### 5. ON_HOLD (Reconciliation Review)

**Definition**  
A **non-canonical**, exploratory state used exclusively within Reconciliation review workspaces.

**Rules**

- Reconciliation ON_HOLD:
  - Does NOT block engine legitimacy
  - Does NOT alter canonical resolution states
  - Does NOT affect sessions or acceptance

- Used for:
  - Import reconciliation
  - Exploratory comparison
  - Deliberative synthesis

**Rationale**

This distinction prevents conflating *review* with *governance*.  
Reconciliation review supports human workflow without affecting engine truth.

---

## II. Session Statuses (Canonical Engine)

### 1. ACTIVE

**Definition**  
The session is open and may develop candidates, record stances, or attempt acceptance.

**Rules**

- Participants, candidates, and constraints freeze on first stance
- Legitimacy may only be created through explicit acceptance
- Authority and Scope must be usable according to engine-defined rules

**Fail if**

- Legitimacy is created implicitly
- Frozen context is ignored or altered

---

### 2. PAUSED

**Definition**  
The session is intentionally halted without unresolved dependencies.

**Rules**

- Acceptance must not succeed while paused
- Context (Authority, Scope, participants, constraints) remains unchanged
- Resume requires explicit action

**Fail if**

- Session resumes legitimacy actions automatically
- Context mutates while paused

---

### 3. BLOCKED

**Definition**  
The session cannot proceed due to unresolved legitimacy dependencies.

Blocking may be **temporary or permanent** as determined by engine rules.

**Examples**

- Scope is ON_HOLD
- Governing resolution becomes unusable
- Structural or usability conditions prevent forward progress

**Rules**

- Acceptance must not succeed while blocked
- Session remains visible and inspectable
- Resume requires resolution of blocking conditions and explicit revalidation under current rules

**Fail if**

- Legitimacy is created while blocked
- Dependency resolution is bypassed or inferred

---

### 4. CLOSED

**Definition**  
The session has completed and is terminal.

**Rules**

- No further stances, candidates, or acceptances
- All history is immutable and auditable

**Fail if**

- Closed session mutates history or legitimacy

---

## III. Governing Principles

### Legitimacy Separation

- Engine-level ON_HOLD is **blocking**
- Reconciliation ON_HOLD is **exploratory**
- These states must never be conflated

---

### Explicit Transitions Only

- No implicit state changes
- No inferred governance
- Every transition is auditable

---

### Governance Respect

- Authority is never placed into a usability state that invalidates governance definition
- Scope usability directly affects dependent sessions
- Usability changes do not rewrite historical legitimacy

---

### Informational Fields Are Non-Semantic

Fields classified as informational or intent metadata (including annotations, reversibility_intent, and cross-area references):

- Must not influence structural validation
- Must not influence decision evaluation
- Must not influence acceptance eligibility
- Must not influence legitimacy determination

These fields exist solely to support human understanding and guidance.

---

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

Reconciliation review helps humans think.  
Sessions help humans commit.  
The engine records what actually happened.

Nothing else.