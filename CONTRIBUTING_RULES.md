# Contributing to Charter — Rules of Integrity

Thank you for your interest in Charter.

Charter is a governance engine.
Correctness, legitimacy, and long-term trust matter more than features, speed, or convenience.

This project is intentionally conservative by design.

---

## Core Rule

**Invariants > Features**

If a contribution violates a long-term invariant, it will not be accepted —
even if it improves usability, performance, or ergonomics.

Charter exists to preserve legitimacy, not optimize workflows.

---

## Before You Contribute

Before proposing changes, please read:

### Conceptual Foundation (Required)
These documents explain *why* Charter exists and the problems it is designed to solve:

- `README.md`
- `doc/guide/` (start here)
  - Why Group Projects Fall Apart
  - Self-Management Isn’t Freedom — It’s Responsibility Without a Map
  - Not All Decisions Are the Same
  - Good Decisions Can Still Lead to Bad Outcomes
  - Where Decisions Live (Decision Memory Models)
  - Charter (Meta)

These guides are not marketing material.
They define the mental model Charter enforces.

### Core Engine Contract
These documents define what must never break:

- `doc/core/INVARIANTS.md`
- `doc/core/SIMULATIONS.md`
- `doc/core/MENTAL_MODEL.md`

If your change affects:
- decision-making
- authority
- scope
- legitimacy
- auditability

it **must** align with these documents.

If there is tension, the documents win.

---

## What We Welcome

- Documentation improvements
- Clarifying or strengthening acceptance tests
- Simplifying implementations without changing semantics
- Making invariants more explicit
- Bug fixes that improve determinism or audit integrity

Improvements that reduce ambiguity are especially valuable.

---

## What Requires Discussion First

Please open an issue or discussion before working on:

- New authority models
- Changes to session lifecycle semantics
- Automation or enforcement mechanisms
- Semantic interpretation of scope
- Cross-area behavior
- Anything that “feels convenient” but weakens explicitness

If a change makes decisions easier but less explicit, it is likely wrong.

---

## What Charter Will Not Become

Charter will not become:

- a workflow engine
- a task executor
- a rules engine
- a configuration manager
- an AI decision-maker
- a collaboration or chat platform

If your idea pushes Charter in these directions,
it almost certainly belongs in an integration layer or external tool —
not the core engine.

---

## Final Note

Charter is intentionally boring, explicit, and resistant to shortcuts.

That is not a limitation.
That *is* the product.

If you find yourself wanting Charter to “just decide,”
“auto-accept,”
or “infer intent” —

pause.

That discomfort is usually the point.