# Label Taxonomy

This document defines the canonical labels used in the Charter repository.

Labels are not cosmetic.
They encode intent, scope, and review expectations.

Mislabeling work causes:
- incorrect reviews
- misplaced debate
- silent scope creep

## Area Labels (Exactly One Required)

Every issue or PR must have exactly one Area label.

- area:invariants
- area:core-engine
- area:cli
- area:guides
- area:design
- area:operations
- area:project-governance

If you are unsure which Area applies:
That uncertainty is part of the discussion.

## Type Labels (At Least One)

These describe *what kind of change* is proposed.

- type:proposal
- type:clarification
- type:bug
- type:refactor
- type:documentation
- type:discussion

## Impact Labels (Optional but Encouraged)

These describe *risk and reach*.

- impact:breaking
- impact:non-breaking
- impact:behavioral
- impact:ergonomic
- impact:audit-critical
- impact:long-term

## Status Labels (Mutually Exclusive)

These describe lifecycle state.

- status:draft
- status:blocked
- status:under-review
- status:accepted
- status:rejected
- status:superseded

## Special Discipline Labels

Used sparingly.

- requires-invariant-review
- governance-required
- needs-simulation
- audit-sensitive
- forward-compatibility

## Labeling Rules

- Area labels are mandatory.
- Status labels must reflect reality.
- Labels do not replace explanation.
- Labels guide reviewers; they do not decide outcomes.

If a label feels political:
The issue probably needs clarification, not pressure.

Labels exist to reduce friction — not hide disagreement.