# Mapping: Invariants → Acceptance Tests

This document maps long-term invariants to acceptance tests.

Acceptance tests are **validation mechanisms**, not definitions of correctness.
An invariant may exist without a test, but a test must never contradict an invariant.

---

## Decision & History

- **Immutability**  
  → Test: Resolution creation produces immutable records

- **Supersession**  
  → Test: New resolution supersedes existing one for same scope

- **Single Active Resolution**  
  → Test: Prevent two active resolutions in same scope

---

## Sessions

- **Decisions Require Sessions**  
  → Test: Resolution cannot be created outside a session

- **Explicit Acceptance**  
  → Test: Resolution only created after accept action

- **Session Blocking**  
  → Test: Scope or authority conflicts block session

- **Session Resume**  
  → Test: Blocked sessions require conflict resolution before resume

---

## Scope

- **Scope Defines Legitimacy**  
  → Test: Out-of-scope resolution blocks acceptance

- **Explicit Scope Violations**  
  → Test: Scope violations are visible and logged

---

## Authority

- **Authority Checked at Acceptance**  
  → Test: Acceptance fails without authority

- **Explicit Authority**  
  → Test: No default or implicit authority paths exist

---

## AI

- **AI Is Non-Decisive**  
  → Test: AI cannot accept or finalize resolutions

- **AI Optionality**  
  → Test: System functions with AI disabled

---

## Templates

- **Template Conformance**  
  → Test: Resolution must match template structure

- **No Semantic Enforcement**  
  → Test: Template does not auto-approve content

---

## Integration & Execution

- **No Execution**  
  → Test: No external side effects occur on acceptance

- **Integration as Context Only**  
  → Test: External links do not affect authority or scope

---

## Notes

- Not all invariants have tests in the MVP.
- Some invariants are enforced socially or structurally.
- Missing tests indicate **intentional deferral**, not omission.

If a test ever conflicts with an invariant, the test is wrong.