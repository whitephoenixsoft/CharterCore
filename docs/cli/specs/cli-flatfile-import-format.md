# Charter CLI — Flat File Import Format (Official Specification)

Status: **FROZEN**  
Applies to: **Charter CLI flat file imports**  
Does NOT apply to: Charter Core engine semantics  
Violations indicate a CLI parsing or validation error.

This document defines the **authoritative flat file import format** used by the Charter CLI
for importing external decision artifacts into a baseline review process.

This format is intentionally strict.
Ambiguity is treated as an error, not a convenience opportunity.

---

## 1. Purpose

Flat file imports exist to support:

- onboarding from legacy text-based decision records
- external system exports
- human-authored policy documents
- review-first legitimacy workflows

Flat imports **never create legitimacy**.
They exist only to populate a **baseline review workspace**.

---

## 2. High-Level Design Principles

1. **Strict syntax over permissive inference**
   - Ambiguous formatting is rejected
   - No guessing based on layout or content

2. **CLI responsibility ends at parsing and provenance**
   - The engine never interprets flat file structure
   - The engine receives only opaque content + provenance

3. **Human ergonomics must not leak into legitimacy**
   - Titles aid review and navigation
   - Titles never influence authority, scope, or acceptance

4. **Audit clarity over convenience**
   - Cosmetic changes must not appear semantic
   - Content diffs must remain stable and meaningful

---

## 3. File Structure Overview

A flat file consists of **one or more entries**.

Each entry represents **exactly one imported resolution candidate**.

Entries are delimited by triple-dash (`---`) blocks.

Example:
```yml
---
title: Logging retention policy
---
Logs must be retained for a minimum of 180 days.

This includes:

- Application logs
- Audit logs
- Access logs
```
---

## 4. Entry Grammar (Strict)

### 4.1 Delimiters

- Each entry **MUST** begin with a delimiter line:
```
---
```
- Each entry **MUST** contain exactly **two delimiter lines**:
- opening delimiter
- delimiter separating metadata from content

Fail if:
- delimiters are missing
- extra delimiters appear
- delimiters are misordered

---

### 4.2 Metadata Block

Between the first and second delimiter is the **metadata block**.

#### Required Fields

- `title:` (required, exactly once)

Example:
```yaml
---
title: Security approvals required for production deploys
---
```
Rules:
- `title:` must be a single-line scalar
- No multiline titles
- No inferred titles
- No unnamed entries

Fail if:
- `title:` is missing
- `title:` appears more than once
- unknown metadata fields are present

---

### 4.3 Content Block

All text **after the second delimiter** is the content.

Rules:
- Content is treated as opaque text
- Content may be multi-line
- Content may contain markdown, lists, or prose
- Content is preserved verbatim (newline-stable)

Fail if:
- content is empty or whitespace-only

---

## 5. Semantic Boundaries (Critical)

### 5.1 Titles Are Non-Authoritative

Titles are **explicitly non-semantic**.

They:
- are not part of the resolution content
- are not evaluated by the engine
- do not affect authority, scope, or legitimacy
- do not participate in diffing or supersession logic

Rationale:
- Titles are unstable and frequently edited
- Including them in content would cause false semantic changes
- Cosmetic renames must not appear as governance changes
- Foreign systems may choose arbitrary or misleading titles

The engine **never sees titles**.

---

### 5.2 Titles Exist Solely for CLI Ergonomics

The CLI MAY use titles for:
- display in baseline review lists
- temporary labels
- user navigation and selection
- referencing malformed or rejected entries

The CLI MUST NOT:
- treat titles as problem statements
- infer intent from titles
- auto-accept or group entries by title meaning

---

## 6. Session Interaction (Import Review)

Flat file imports always enter **baseline review**.

Rules:
- No imported resolution is ACTIVE on import
- All imported resolutions begin as `UNDER_REVIEW`
- Acceptance always occurs via sessions
- Sessions may be explicit or CLI-hidden (solo ergonomics)

### 6.1 Session Descriptions (Optional, Non-Semantic)

The CLI MAY suggest a session description derived from the title, e.g.:

> "Imported item: Logging retention policy"

Constraints:
- The suggestion must be visible and editable
- The user must explicitly confirm or modify it
- The engine ignores session descriptions entirely

Sessions **do not require** a problem statement.

---

## 7. One-Liners vs Multi-Line Content

The format supports both:

### Valid one-liner:
```yaml
---
title: MFA required for admin access
---
All admin access must require multi-factor authentication.
```
### Valid multi-line:
```yaml
---
title: Logging retention policy
---
Logs must be retained for a minimum of 180 days.

This includes:
- Application logs
- Audit logs
- Access logs
```
No alternative compact formats are supported.

Rationale:
- A single grammar prevents future parser forks
- Ergonomic tooling can be layered later
- Minimalism must not create ambiguity

---

## 8. Explicit Non-Goals

This format intentionally does NOT support:

- implicit titles
- title inference from content
- embedded authority or scope metadata
- inline acceptance markers
- semantic validation
- auto-diff acceptance
- version headers or global metadata

All legitimacy remains session-bound.

---

## 9. Failure Philosophy

The CLI MUST fail fast and loudly when:

- formatting is ambiguous
- metadata is malformed
- required fields are missing
- content boundaries are unclear

Failing early is considered a feature, not a defect.

---

## 10. Lock Statement

This specification is **locked**.

Any future extension (API import, server mode, UI tooling) must:
- preserve this grammar
- extend it explicitly
- never reinterpret existing fields

If an implementation choice conflicts with this document,
the implementation is wrong — not the format.
