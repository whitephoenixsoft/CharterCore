# Core Engine Specifications

This folder describes **what the Charter engine does**, not how it is implemented.

Specifications define:
- observable behavior
- state transitions
- guarantees made to callers
- boundaries of responsibility

They are written to be:
- precise
- testable
- implementation-agnostic

## Relationship to Invariants

Specifications must **respect invariants**.
They may clarify them, but never weaken them.

If a spec conflicts with an invariant:
The spec is wrong.

## What Belongs Here

- Session lifecycle behavior
- Authority evaluation semantics
- Resolution creation and supersession
- Baseline and consolidation behavior

## What Does NOT Belong Here

- CLI commands
- Storage layout
- Internal data structures
- Performance tuning

Think of these as:
“What must always be true from the outside.”

Not:
“How we happen to do it today.”