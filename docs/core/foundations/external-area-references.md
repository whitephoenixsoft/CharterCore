# Charter Core — External Area References (Foundation)

Status: FROZEN (Conceptual)  
Applies to: Charter Core, Charter CLI, future interfaces  
Does NOT define: storage schema, wire formats, or command syntax

---

## Purpose

This document defines the *conceptual model* for referencing external or non-local Areas.

Its purpose is to preserve:
- historical intelligibility
- legitimacy boundaries
- audit clarity

while allowing humans to reference ideas that evolve over time.

---

## Core Principle

> References identify *what was pointed to*, not *what it is called now*.

A reference captures **intent at the moment of creation**, not live meaning.

---

## What an Area Reference Is

An Area reference is:

- a **pointer to an Area identity**
- accompanied by **human-readable context**
- immutable once created
- non-legitimate by design

A reference exists to support:
- understanding
- traceability
- audit reconstruction

It does **not** participate in authority, scope, or acceptance.

---

## What an Area Reference Is Not

An Area reference is NOT:

- a legitimacy claim
- an authority transfer
- a dependency declaration
- a live alias
- a semantic guarantee

References never:
- confer meaning
- update automatically
- rewrite history
- participate in governance

---

## Identity vs Meaning

Charter strictly separates:

- **Identity** — immutable, authoritative
- **Meaning** — human, contextual, time-bound

For Areas:
- Identity is the Area ID
- Meaning is conveyed through labels and descriptions

References bind to **identity**, not meaning.

---

## Reference Contents (Conceptual)

A reference MUST conceptually contain:

- **Target Area ID**  
  The immutable identity being referenced.

- **Label Snapshot**  
  The human-readable name known at the time of reference.

Optional (non-authoritative):
- long name
- short description

These fields are:
- informational
- historical
- never normative

---

## Renaming Semantics

Renaming an Area:

- modifies metadata
- emits an audit event
- does NOT alter existing references

Existing references remain historically correct:
> “This is what the author called it at the time.”

Current names are discoverable through:
- Area audit
- metadata history
- query layers

No back-patching occurs.

---

## Audit Philosophy

Area references exist to support the question:

> “What did the author believe they were pointing to at that moment?”

Audit must be able to show:
- the referenced Area identity
- the name known at the time
- the evolution of the Area’s metadata afterward

Audit never infers intent beyond what was recorded.

---

## Legitimacy Boundary

Area references:

- do not create authority
- do not create scope
- do not activate Areas
- do not imply agreement

They are informational artifacts only.

Any legitimacy involving Areas must occur through:
- sessions
- resolutions
- baseline review

---

## Engine Responsibility

The engine:

- stores references immutably
- treats reference content as facts, not truth
- never resolves or interprets meaning

The engine does NOT:
- validate label correctness
- enforce naming consistency
- infer relationships from references

---

## Interface Responsibility

Interfaces (CLI, UI, reports) MAY:

- display both historical and current labels
- annotate differences for humans
- assist navigation and discovery

Interfaces MUST NOT:
- mutate references
- infer authority
- collapse identity and meaning

---

## Design Intent (Frozen)

This model exists to ensure that:

- history never rewrites itself
- names remain human, not authoritative
- references stay safe across time
- audit remains intelligible decades later

If references ever become authoritative,
then Charter has violated its core legitimacy discipline.