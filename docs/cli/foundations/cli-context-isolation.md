# Charter CLI — Context Isolation Specification
Status: FOUNDATION (V1+)
Applies to: Charter CLI, V6 Commit Store, V7 Relay
Does NOT apply to: Engine legitimacy semantics

---

## Purpose

This document defines the structural and isolation rules for **Contexts** within Charter CLI.

A Context is a hard storage and identity isolation boundary.

Its purpose is to:

- Physically separate groups of Areas
- Partition object stores
- Partition commit stores (V6+)
- Enable multi-identity and multi-tenant usage
- Provide strong operational isolation

A Context is **not** a legitimacy boundary.

Legitimacy boundaries are Areas.

---

## Core Principle

A Context is an isolated universe of storage.

No state, metadata, index, or object inside one Context is visible to another unless explicitly exported or routed through approved mechanisms (e.g., V7 relay).

Isolation is absolute by default.

---

## Definitions

### Context

A Context is:

- A physical storage root
- A container of Areas
- A container of commit stores (V6+)
- A container of indexes
- A container of audit history
- Identified by a globally unique Context ID

A Context is not:

- A governance structure
- An authority boundary
- A trust domain
- A semantic modifier of Areas
- A federation primitive

---

### Context ID

Each Context MUST have:

- A globally unique immutable ID (UUID recommended)
- A mutable human-readable label
- Optional annotations (non-normative metadata)

Context IDs:

- Never change
- Are not derived from filesystem paths
- Are not influenced by Areas
- May appear in export metadata for provenance

---

## Isolation Guarantees

### 1. Storage Isolation

Each Context MUST have:

- Its own object store root
- Its own ref store
- Its own indexes
- Its own metadata store
- Its own audit log

No shared storage is permitted.

---

### 2. Runtime Isolation

At runtime:

- Only one Context may be active at a time.
- The CLI MUST fully load the selected Context.
- The CLI MUST fully unload the previous Context before switching.
- No shared in-memory caches across Contexts are allowed.
- No global singletons referencing multiple Contexts are allowed.

Context switching is explicit.

---

### 3. Visibility Rules

While operating inside a Context:

- The CLI may know other Contexts exist.
- The CLI MUST NOT expose their Areas.
- The CLI MUST NOT expose their commits.
- The CLI MUST NOT expose their metadata.
- The CLI MUST NOT resolve references across Contexts.

Contexts are opaque to each other.

---

### 4. Area Containment

Areas exist within a single Context.

Area IDs:

- Are globally unique
- Remain valid if exported
- Do not encode Context identity

An Area MUST NOT exist in two Contexts simultaneously unless explicitly restored from backup.

Context membership does not affect Area legitimacy semantics.

---

## Relationship to the Engine

The Engine:

- Operates on a single Area at a time
- Is unaware of Contexts
- Receives deterministic input from the CLI
- Does not validate Context rules

The CLI:

- Enforces Context boundaries
- Injects Area state into the Engine
- Prevents cross-context contamination

Context is a CLI concern, not an Engine concern.

---

## Relationship to V6 Commit Store

From V6 onward:

Each Context contains:

- Its own commit store
- Its own commit indexes
- Its own baseline review workflows

Commits MUST NOT cross Context boundaries except through:

- Explicit export/import
- V7 relay routing

A commit received into a Context becomes part of that Context's immutable history.

Contexts do not share commit stores.

---

## Relationship to V7 Relay

The V7 relay may:

- Receive commits
- Route commits to specific Contexts
- Store commits for archival

The relay MUST NOT:

- Merge Contexts
- Infer legitimacy relationships
- Modify commit semantics
- Collapse Area boundaries

Relay routing is explicit and deterministic.

Context remains a hard boundary.

---

## Export and Import

### Restore (Backup)

Restore recreates a prior state into a Context.

Restore:

- Preserves Area IDs
- Preserves commit history
- Preserves spec versions
- Does not merge with other Contexts

Restore is not consolidation.

---

### Consolidation (Rebase-like)

Consolidation:

- Occurs at the Area level
- Uses baseline review
- Is explicit and auditable
- Does not operate at the Context level

Context boundaries remain intact.

---

## Context Switching

Context switching:

- Is explicit
- Is user-invoked
- Fully replaces runtime state
- May emit a CLI-level audit event
- Does not affect legitimacy history

Switching Context does not:

- Modify Areas
- Modify commits
- Modify engine state

It only changes which isolated universe is active.

---

## Security & Multi-Tenancy

Contexts serve as:

- Identity partitions
- Storage partitions
- Multi-tenant boundaries
- Privacy boundaries

Contexts must not:

- Imply trust
- Imply shared authority
- Imply governance relationships

Legitimacy remains scoped to Areas only.

---

## Invariants

1. Context isolation is absolute by default.
2. Context does not influence legitimacy semantics.
3. Engine is unaware of Context.
4. Context IDs are immutable.
5. No cross-context read access is allowed.
6. No cross-context write access is allowed.
7. All cross-boundary movement is explicit and auditable.
8. Contexts cannot be nested.
9. Context identity is independent of filesystem path.
10. Deleting a Context deletes its entire storage universe.

---

## Advanced Feature Classification

Contexts are:

- Available from V1
- Advanced usage
- Not required for basic operation
- Default Context name: "default"

The default Context behaves identically to any other Context.

It has no special authority or semantics.

---

## Design Intent

Contexts exist to:

- Provide strong compartmentalization
- Enable multi-identity systems
- Enable federation isolation
- Reduce accidental cross-contamination
- Support long-lived institutional usage

If Context ever:

- Influences legitimacy
- Implicitly transfers authority
- Enables hidden state sharing
- Weakens isolation

Then the system has violated this specification.

---

## Closing Principle

Areas define legitimacy.

Contexts define isolation.

These boundaries must never collapse into each other.