# Session Constraints 

## 1. What Session Constraints Are (Clean Definition)

Session constraints are temporary, explicit acceptance prerequisites scoped to a single session.

They answer this question:

> “Before the Authority rule is even evaluated, what explicit conditions must be satisfied for acceptance to be allowed?”

Key properties:
- Declared at session creation
- Apply only to that session
- Are purely mechanical
- May block or pause acceptance
- Do not grant acceptance power

This is important:
**Constraints gate acceptance; they do not decide acceptance.**

---

## 2. Authority vs Session Constraints (The Crucial Distinction)

This distinction must stay sharp, or the engine will blur.

### Authority

- Decides how acceptance is computed
- Is an Area-level, first-class resolution
- Is persistent and contextual
- Defines standing and decision rule

Examples:
- SOLE_ACTOR
- UNANIMOUS_PRESENT
- MAJORITY_PRESENT

---

### Session Constraints

Decide when acceptance is allowed to be attempted
- Are session-local and temporary
- Do not compute acceptance
- Do not override authority

Examples of constraints:
- “Alice and Bob must explicitly acknowledge”
- “Security review must be present”
- “This candidate requires external confirmation”

Notice what’s missing:
- No voting weights
- No veto semantics
- No role inheritance
- No permanent power

That’s intentional.

--- 

## What Session Constraints Are Not

This is important to state explicitly (you may want this in docs later).

Session constraints are not:
- Roles
- Veto powers
- Delegation
- Approval workflows
- Escalation paths
- Governance inheritance

They are simply explicit gating conditions.