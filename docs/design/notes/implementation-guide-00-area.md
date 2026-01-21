# Charter Core — Area Initialization & Storage Flow (Implementation Guide)

This document is a practical, step-by-step guide for implementing Area initialization and storage behavior in Charter Core.
It reflects locked invariants and storage rules and is intended for direct use during implementation and validation.

---

## Core Mental Model (Lock This In)

- The object store holds immutable facts (objects).
- Objects are never modified or deleted.
- All change over time is expressed by:
  - new objects
  - ref updates
  - metadata events
- “What is active” is determined only by refs and indexes.
- Runtime engine state is always derived and rebuildable.

---

## Stores and Their Responsibilities

### 1. Object Store (Immutable)

- Stores object envelopes:
  - domain object
  - object type
  - hash metadata
- Append-only
- Deduplicated by hash
- No updates
- No deletes
- Store is unaware of legitimacy, liveness, or supersession

---

### 2. Metadata Store (Non-Legitimizing Events)

- Records descriptive, non-legitimacy changes
- Examples:
  - Area name changes
  - Area description updates
- Append-only
- Auditable
- Must NOT:
  - affect refs
  - affect authority
  - affect scope
  - affect legitimacy
- Treat as events, not overwrites

---

### 3. Ref Store (Mutable, Authoritative)

- Stores “what is currently active”
- Examples:
  - Active Authority per Area
  - Active Scope per Area
- Mutable by design
- Ref changes MUST:
  - be explicit
  - emit audit events
- Ref store defines legitimacy context

---

### 4. Indexes (Derived, Rebuildable)

- Derived from object store + ref store + metadata
- Discardable and rebuildable at boot
- No authority of their own

Recommended V1 structure:
- One index per object type
- Mapping:
  - Logical ID → Object Hash

Example:
- AreaIndex: AreaId → ObjectHash

---

### 5. Audit Store (First-Class)

- Records all auditable actions
- Must outlive all other stores
- No auditable action may exist only in a destructible scope
- Import, ref changes, creation, retirement all emit audit events

---

## Area Initialization Flow (Step-by-Step)

### Step 1 — Area Creation

When creating a new Area:

1. Generate a new AreaId (UUID)
2. Create an Area domain object (immutable identity)
3. Persist Area object to the object store
4. Persist descriptive metadata (e.g. name, description) to metadata store
5. Add AreaId → ObjectHash entry to Area index
6. Emit audit event: “Area created”

Notes:
- Area exists but is NOT usable yet
- No authority or scope exists at this point

---

### Step 2 — Governance Preconditions

An Area is considered “initialized” only when:

- Exactly one active Authority resolution exists
- Exactly one active Scope resolution exists

Until both are present:
- Area may exist
- Sessions MUST be blocked
- Only sessions whose purpose is to establish Authority and/or Scope are permitted

This is enforced mechanically by the engine.

---

### Step 3 — Authority and Scope Establishment

For each of Authority and Scope:

1. Create resolution via a session
2. Persist resolution object to object store
3. Update ref store:
   - Area → Active Authority
   - Area → Active Scope
4. Emit audit events for acceptance and ref update
5. Rebuild or update relevant indexes

Only after both refs exist:
- Area becomes eligible for normal sessions

---

## Engine Boot / Rehydration Sequence (Relevant Parts)

On engine startup:

1. Load object store
2. Load metadata store
3. Load ref store
4. Replay audit events (for validation, not state)
5. Rebuild indexes from stores
6. Validate invariants:
   - One active authority per initialized area
   - One active scope per initialized area
7. Enter READY state

No runtime state is persisted.
Everything is derivable.

---

## Area Modification Rules (Lock These)

- Area core identity is immutable
- Descriptive changes:
  - go to metadata store
  - are auditable
  - do NOT affect legitimacy
- Authority or Scope changes:
  - require sessions
  - create new resolutions
  - update refs explicitly
- Supersession:
  - creates new objects
  - old objects remain forever

---

## Area Retirement (Not Deletion)

- Areas are never deleted
- Retirement is semantic:
  - emit AreaRetired event
  - clear or freeze refs
  - mark inactive in indexes
- Area remains:
  - addressable
  - auditable
  - referenceable

Physical deletion is forbidden.

---

## Runtime vs Persisted State (Critical Distinction)

Runtime engine structs (example):

- AreaRuntime
- SessionRuntime
- EvaluationContext

These:
- are never persisted
- are never hashed
- are reconstructed at boot

Persisted state consists only of:
- objects
- metadata events
- refs
- audit events

---

## Final Lock

If something cannot be reconstructed from:
- object store
- metadata store
- ref store
- audit store

It must not exist.

Charter Core optimizes for:
- determinism
- auditability
- long-term legitimacy
- future orchestration layers (V3+)

Use this guide as the implementation spine.

----
ALL BELOW IS INTERNAL ANALYSIS OF ABOVE 

---
# Charter Core — Initialization Spine  
## Spec vs Design Decision Extraction

Status: INTERNAL / WORKING  
Purpose: Separate **engine invariants (spec)** from **implementation choices (design decisions)**  
Scope: Engine initialization, persistence, and rehydration

This document exists to **reduce cognitive load while coding** and prevent premature over-specification.

---

## Framing Principle

- **Specs** define *what must always be true*
- **Design decisions** define *how we choose to make it true*

If an alternative implementation could satisfy the rule → it is **spec**  
If changing it would change the engine’s nature → it is **design**

---

## Step-by-Step Initialization Flow

### 1. Resolution Creation via Session

**Spec**
- Resolutions may only be created via sessions
- No resolution may exist without an authority context
- Legitimacy originates only from sessions

**Design Decisions**
- Whether resolution objects are instantiated before or after acceptance
- How session state is represented in memory
- When resolution IDs are generated

---

### 2. Persist Resolution Object

**Spec**
- Accepted resolutions must be persisted
- Persisted resolutions are immutable
- Persisted data must be sufficient for reconstruction

**Design Decisions**
- Use of an object store
- Serialization format
- Persistence ordering relative to audit emission

---

### 3. Update Area References (Authority / Scope)

**Spec**
- Each initialized area has exactly one active Authority
- Each initialized area has exactly one active Scope
- Authority and Scope changes require sessions
- Supersession creates new resolutions; old ones remain

**Design Decisions**
- Existence of a dedicated ref store
- Reference representation (pointer, ID, lookup key)
- Update timing relative to acceptance

---

### 4. Emit Audit Events

**Spec**
- Acceptance must be auditable
- Reference changes must be auditable
- Audit history must allow reconstruction of what happened

**Design Decisions**
- Append-only audit log
- Event granularity
- Event schema
- Storage backend
- Sync vs async emission

> Note: “Append-only” is an implementation strategy, not a spec requirement.  
> The spec requirement is *non-destructive, reconstructible history*.

---

### 5. Index Rebuild / Update

**Spec**
- None

**Design Decisions**
- Index structure
- Incremental vs full rebuild
- Storage location

Indexes are derived, discardable, and non-authoritative.

---

## Area Eligibility for Normal Sessions

**Spec**
- An area may not host normal sessions until:
  - Active Authority exists
  - Active Scope exists

**Design Decisions**
- Where eligibility checks live
- Eager vs lazy validation
- Error vs blocked state behavior

---

## Engine Boot / Rehydration

### Core Guarantees (Spec)

- Engine state must be reconstructible
- No runtime-only data may be authoritative
- All invariants must be validated at boot
- Exactly one active Authority and Scope per initialized area

> No runtime state is persisted.  
> Everything authoritative must be derivable.

### Implementation Choices (Design)

- Store separation (object / metadata / refs / audit)
- Load order
- Whether audit replay is validation-only or state-driving

---

## Area Modification Rules

### Identity and Description

**Spec**
- Area identity is immutable
- Descriptive changes do not affect legitimacy
- Descriptive changes are auditable

**Design Decisions**
- Metadata store existence
- Event structure for descriptive changes

---

### Authority and Scope Changes

**Spec**
- Require sessions
- Create new resolutions
- Explicitly update references
- No retroactive effect

**Design Decisions**
- Ref mutation mechanics

---

## Area Retirement (Not Deletion)

**Spec**
- Areas are never deleted
- Retirement is semantic, not physical
- Retired areas remain:
  - addressable
  - auditable
  - referenceable

**Design Decisions**
- Event naming (e.g. `AreaRetired`)
- Ref freezing vs clearing
- Index behavior

---

## Runtime vs Persisted State

**Spec**
- Runtime state is never authoritative
- Runtime state is never persisted
- Persisted state must be sufficient for full reconstruction

**Design Decisions**
- Runtime struct layout
- Caching strategies
- Naming conventions

---

## Final Lock Principle

**Spec**
If something cannot be reconstructed from:
- object store
- metadata events
- ref store
- audit store

Then it must not exist.

Charter Core optimizes for:
- determinism
- auditability
- long-term legitimacy
- future orchestration layers

This document is the implementation spine.
