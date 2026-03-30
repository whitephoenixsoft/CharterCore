# Core Invariants

This folder contains **non-negotiable truths** about Charter Core.

These are not preferences.
They are not design ideas.
They are not implementation details.

If an invariant in this folder is violated:
- the engine is incorrect
- the behavior is invalid
- development must stop until resolved

## What Belongs Here

- Determinism guarantees
- Audit integrity rules
- Legitimacy constraints
- Identity and immutability requirements

If something here changes:
It is a *governance event*, not a refactor.

## What Does NOT Belong Here

- CLI behavior
- UX ergonomics
- Performance optimizations
- Convenience features
- Opinionated workflows

Those live elsewhere.

## Change Discipline

Changes to invariants:
- are rare
- require explicit discussion
- must include rationale
- must consider downstream impact

“Invariants > Features” is not a slogan.
It is the product.

If you are unsure whether something is an invariant:
It probably isn’t.