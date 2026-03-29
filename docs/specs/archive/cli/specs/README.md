# CLI Specifications

This folder describes **what the CLI must do**, not how friendly it looks.

Specifications cover:
- command behavior
- error conditions
- mode transitions
- safety guarantees

They are written so that:
- different CLIs could exist
- server mode remains possible
- behavior remains predictable

## What Belongs Here

- Command semantics
- Import and export behavior
- Context switching rules
- Solo vs multi-participant constraints

## What Does NOT Belong Here

- Flag names
- Output formatting details
- Color schemes
- Shell completion

Those are implementation details.

The CLI’s job is not to be clever.
It is to be honest.