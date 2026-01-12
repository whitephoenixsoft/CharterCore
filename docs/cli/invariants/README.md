# CLI Invariants

This folder defines **behavioral invariants for the Charter CLI**.

These invariants protect:
- user trust
- legitimacy boundaries
- cognitive safety
- audit correctness

Violations here indicate:
A CLI bug — not an engine flaw.

## What Belongs Here

- Context explicitness rules
- Session and baseline safety constraints
- Import and export guarantees
- Audit visibility requirements

## What These Invariants Are For

The CLI is the most dangerous layer:
It is where convenience can silently corrupt legitimacy.

These invariants exist to ensure:
- nothing implicit becomes authoritative
- no action feels accepted without being accepted
- no history disappears quietly

## Relationship to Engine

The CLI must:
- respect engine semantics
- never invent legitimacy
- never “help” by skipping steps

Ergonomics are allowed.
Shortcuts are not.