# Usage Pattern — Charter Core as an Immutable Notepad Backend

## Audience
Integrators building:
- note-taking tools
- journals
- design logs
- personal or team knowledge systems

that require **history, legitimacy, and auditability**, not mutable documents.

---

## Intent

This pattern uses **Charter Core as an immutable decision ledger**, while the integrating application provides a familiar *editable-note experience*.

Key principle:

> **Charter Core never edits. It only records supersession.**

The integrating application may present an “edit” UX, but under the hood:
- every edit is a new resolution
- every change is explicitly accepted
- all prior versions remain queryable forever

---

## Core Mapping

| Notepad Concept | Charter Core Concept |
|----------------|----------------------|
| Note | Resolution |
| Edit | New candidate |
| Save / Commit | Session acceptance |
| Version history | Supersession chain |
| Current note | Latest active resolution |

---

## Required Integration Behavior

### 1. Immutability Is Non-Negotiable

Integrators **must not**:
- overwrite resolution content
- modify accepted resolutions
- collapse history

Every edit produces a **new resolution** that explicitly supersedes the prior one.

Failing to do this violates:
- immutability invariants
- audit guarantees
- legitimacy guarantees

---

### 2. Sessions Are Mandatory (Explicit or Hidden)

Each committed edit must occur within a **decision session**.

Allowed:
- explicit sessions (visible to the user)
- ergonomic or hidden sessions (solo use)

Not allowed:
- silent acceptance
- auto-save that produces legitimacy
- edits without acceptance context

Engine expectations:
- authority is evaluated
- participants are recorded
- audit entries exist

---

### 3. Drafting Lives Outside Charter Core

Charter Core is **not** a draft store.

Recommended pattern:
- Draft text is managed entirely by the integrating application
- Only committed content enters Charter Core
- Partial edits never appear as resolutions

This avoids:
- audit noise
- legitimacy spam
- false authority satisfaction

---

### 4. Supersession Is Explicit and Linear

Each new resolution should:
- reference the immediately prior resolution
- create a clear supersession chain

Do not:
- fork histories implicitly
- merge multiple prior resolutions silently

Forks are allowed only via explicit user intent and explicit sessions.

---

## What Charter Core Does NOT Do

Charter Core does **not**:
- diff text
- interpret semantic meaning
- decide whether an edit is “minor”
- collapse edits automatically
- enforce writing workflows

All of the above belong to the integrating application.

---

## Audit Expectations

A correct integration guarantees:
- every note version is queryable
- every edit has a timestamped acceptance
- every supersession is explicit
- authority and scope context are preserved per version

Even if the UI hides this complexity, it must exist.

---

## Summary for Integrators

If implemented correctly:
- users experience a powerful “versioned notebook”
- developers gain immutable history and audit safety
- legitimacy is preserved without user friction

If implemented incorrectly:
- Charter Core invariants are violated
- history becomes misleading
- trust is eroded

**Treat Charter Core as a write-once ledger, not a document store.**