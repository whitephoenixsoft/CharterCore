# Formal Change Spec

## Identifier
A stable, unique identifier for this decision.

Example:
- ENG-DEC-012
- CLI-DESIGN-004
- GOV-INVAR-001

This identifier must never be reused.

---

## Decision Statement
A clear, normative statement of what is now true.

This should be written as a rule, not an explanation.

Examples:
- “Charter Core MUST reject any resolution without explicit authority.”
- “The CLI SHALL refuse to import records with mismatched engine identity.”
- “This team owns the boundary between X and Y.”

Avoid justification here.
State the outcome.

---

## Scope
Where this decision applies.

Examples:
- Charter Core engine
- CLI behavior only
- Documentation semantics
- Governance process
- Specific subsystem or boundary

Be precise.
Ambiguous scope is a defect.

---

## Authority
Who had the authority to accept this decision.

Examples:
- Core maintainers
- Invariant owners
- CLI maintainers
- Explicit consensus (list participants)

If authority is unclear, legitimacy is unclear.

---

## Acceptance Moment
When this decision became real.

This may be:
- a date
- a release
- a recorded meeting
- a merged change with explicit acceptance

This is the “decision timestamp.”

---

## Supersession
What this decision replaces, if anything.

- Supersedes: <ID> (if applicable)
- Superseded by: (leave blank until it happens)

If nothing is superseded, state “None”.

---

## Invariant Interaction
State explicitly whether this decision:

- Introduces a new invariant
- Modifies an existing invariant
- Clarifies an invariant
- Does not interact with invariants

If there is tension, it must be named.

---

## Non-Goals
What this decision explicitly does NOT do.

This prevents overreach and reinterpretation.

Examples:
- “This does not define UI behavior.”
- “This does not introduce enforcement.”
- “This does not apply retroactively.”

---

## Notes (Optional)
Optional clarifications for future readers.

This is not for debate history.
This is for interpretation safety.

---

## Status
One of:
- ACCEPTED
- DEPRECATED
- SUPERSEDED

Status changes require a new Formal Change Spec.
This document itself is immutable once accepted.