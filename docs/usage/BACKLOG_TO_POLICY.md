# Usage Pattern: From Backlog Items to Long-Term Business Rules

This document describes an optional usage pattern where outcomes of executed work
are promoted into long-lived resolutions within Charter.

This pattern is not required.
It exists to preserve durable business decisions beyond execution tooling.


---

## Context

Execution tools such as Jira or Azure DevOps manage work items:

- Stories
- Epics
- Tasks

These tools are optimized for:

- Planning
- Tracking
- Delivery

They are not optimized for:

- Long-term memory
- Policy preservation
- Governance

As a result, durable business rules are often lost once work is completed.

---

## The Pattern

Some teams choose to:

1. Complete backlog items as usual in execution tooling
2. Identify items that encode durable business rules
3. Capture those rules as resolutions in Charter
4. Reference the Charter resolution from code, documentation, or future work

Examples include:

- Security requirements
- Compliance rules
- Architectural constraints
- Business invariants

---

## What Charter Becomes in This Pattern

- A ledger of what must remain true
- A memory of why certain behaviors exist
- A reference point when future changes are proposed

Charter does not replace:

- Backlog management
- Sprint planning
- Delivery workflows

It complements them by preserving intent.

---

## Governance Implications

When promoted into Charter:

- Business rules become explicit
- Authority and scope are recorded
- Changes require supersession, not deletion

This reduces accidental drift.

---

## Important Boundaries

Charter does not:

- Ingest entire backlogs
- Track completion status
- Enforce implementation

Only decisions with long-term relevance belong here.

---

## Summary

Execution systems answer:

> “What did we do?”

Charter answers:

> “What must continue to be true, and why?”
