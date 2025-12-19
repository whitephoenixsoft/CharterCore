# Charter — MVP Deviations from Goal Invariants

This document records **intentional deviations** between the Charter MVP
and the long-term invariants defined in `INVARIANTS_GOAL.md`.

These deviations are:
- Explicit
- Temporary
- Non-authoritative

They exist to enable early development without compromising long-term integrity.

---

## Guiding Rule

> The MVP may violate implementation details,  
> but it may not violate **conceptual invariants**.

If a shortcut undermines trust, legitimacy, or auditability, it is not allowed.

---

## Deviations by Category

### 1. Authority Management

**Goal Invariant**
- Authority changes require dedicated sessions.
- Authority is non-retroactive.

**MVP Deviation**
- Single-user authority is hard-coded.
- No authority-change sessions exist yet.

**Why**
- Simplifies implementation
- Preserves explicit acceptance
- Does not undermine legitimacy

**Risk**
- Low (single-user mode only)

---

### 2. Scope Semantics

**Goal Invariant**
- Scope is semantically evaluated and enforced.

**MVP Deviation**
- Scope is manually validated by users.
- No semantic automation or dependency tracking.

**Why**
- Avoids premature AI dependency
- Keeps scope visible and explicit

**Risk**
- Medium (human error)
- Mitigated by session blocking and audit trail

---

### 3. Session Dependency Tracking

**Goal Invariant**
- Blocked sessions resume only after blocking issues are resolved.

**MVP Deviation**
- Resume allowed manually.
- No automatic dependency resolution.

**Why**
- Reduces orchestration complexity
- Preserves human control

**Risk**
- Medium
- Acceptable for MVP with clear UI warnings

---

### 4. Multi-Area Federation

**Goal Invariant**
- Multiple independent Areas with no implicit authority sharing.

**MVP Deviation**
- Single Area only.

**Why**
- Keeps domain small
- Federation model preserved conceptually

**Risk**
- None (design assumption preserved)

---

### 5. Templates Enforcement

**Goal Invariant**
- Templates strictly enforce required structure.

**MVP Deviation**
- Templates are lightly validated.

**Why**
- Faster iteration on formats
- Focus on resolution lifecycle first

**Risk**
- Low
- Structure still visible and human-readable

---

## Explicit Non-Deviations

The following invariants are **never allowed to be violated**, even in MVP:

- Resolution immutability
- Supersession over editing
- Explicit acceptance
- No AI authority
- No execution or enforcement
- No silent state changes

---

## Exit Criteria

A deviation may be removed only when:
- The invariant is fully enforced
- Migration path is defined
- Historical integrity is preserved

Until then, this document remains authoritative.