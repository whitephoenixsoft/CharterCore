# Charter Core — Engine Boundary Specification
Status: FROZEN (V1/V2 Structural Boundary)
Layer: Charter Core Engine
Audience: Engine implementers, CLI maintainers, future contributors
Purpose: Define the permanent structural boundary of the Charter Core engine to prevent architectural drift.

---

## 1. Intent

This document defines the immutable boundary of the Charter Core Engine.

It specifies:

- What responsibilities belong to the Engine
- What responsibilities are explicitly excluded
- The abstract storage contract
- The event emission contract
- The minimal query surface
- Embedding invariants for external consumers

This boundary ensures that:

- The Engine remains lean and stable
- Storage and deployment concerns remain application-level
- Legitimacy mechanics remain mechanically correct
- Future versions (V4+) do not expand the Engine improperly

This document freezes the structural scope of V1/V2.

---

## 2. Core Design Principle

The Engine is a deterministic legitimacy processor operating on a single Area namespace at a time.

The Engine:

- Enforces legitimacy rules
- Validates invariants
- Transitions state
- Emits audit events

The Engine does NOT:

- Own persistence
- Manage deployments
- Implement federation
- Perform guidance
- Manage commit relays
- Interpret meaning

The Engine is a legitimacy kernel — not an application framework.

---

## 3. Legitimacy Area Model

### 3.1 Area as Legitimacy Namespace

An Area is a legitimacy boundary.

An Area defines:

- Authority context
- Scope context
- Session concurrency domain
- Resolution namespace
- Ref namespace

The Engine operates on exactly one Area context per invocation.

The Engine does not:

- Enumerate Areas
- Coordinate multiple Areas
- Perform cross-Area operations

Area selection and multi-Area orchestration are application responsibilities.

---

## 4. What Lives in the Engine

### 4.1 Domain Primitives

The Engine defines and owns:

- Resolution
- Session
- Authority enums
- Scope semantics
- Session constraints
- Status enums (ACTIVE, SUPERSEDED, etc.)
- Reversibility declarations
- Supersession semantics

These define mechanical legitimacy.

---

### 4.2 State Transition Logic

The Engine must enforce:

- Authority evaluation rules
- Explicit acceptance requirement
- Session phase separation (evaluation vs commitment)
- Single active session per Area
- UNDER_REVIEW blocking rules
- Supersession directionality
- Irreversibility invariants
- Explicit transition requirements
- Immutable object guarantees

All legitimacy-creating transitions must be validated here.

---

### 4.3 Deterministic Query Surface

The Engine exposes only queries required to:

- Evaluate legitimacy
- Confirm invariant compliance
- Retrieve canonical state

Engine queries must:

- Be pure
- Be deterministic
- Never mutate state
- Never infer intent
- Never create legitimacy

Engine queries answer only:

"What is mechanically true?"

They do not summarize, interpret, or advise.

---

### 4.4 Event Emission Contract

For every state transition, the Engine must emit structured audit events.

Events must be:

- Deterministic
- Complete
- Immutable
- Sufficient for full reconstruction

The Engine guarantees:

- Events correspond exactly to accepted transitions
- No silent state mutations occur
- All legitimacy actions are auditable

The Engine does not persist events.

Event persistence is application responsibility.

---

## 5. Abstract Storage Interface

The Engine operates over an abstract storage adapter supplied by the embedding application.

The storage interface must support:

- Retrieving immutable objects by ID
- Storing immutable objects
- Reading refs (named pointers)
- Updating refs atomically
- Listing objects by type (if required)

Storage must guarantee:

- Atomicity per engine invocation
- Deterministic reads
- No hidden mutation
- Append-only object semantics

The Engine must not:

- Open files
- Connect to databases
- Assume filesystem layout
- Assume network topology

Persistence lifecycle belongs entirely to the application layer.

---

## 6. Indexing

Indexes are performance optimizations.

Indexes:

- May be used by the Engine internally
- Must be derivable from objects + refs
- Must never be a source of truth

Loss of indexes must not alter legitimacy.

Index persistence format is not part of Engine stability guarantees.

---

## 7. What Must NOT Live in the Engine

The Engine must never include:

- Commit abstraction (Commit is higher-layer concept)
- Relay or synchronization logic
- Federation aggregation
- Guidance or interpretation
- Cross-Area coordination
- UX-level queries
- Deployment topology awareness
- External authority inference
- Meaning interpretation

If a feature does not affect mechanical legitimacy within a single Area, it does not belong in the Engine.

---

## 8. Embedding Invariants

Any application embedding the Engine must guarantee:

- Only one Area context is provided per invocation
- Storage adapter enforces atomic updates
- Events are persisted immutably
- No partial state is exposed to queries
- Engine calls are treated as transactions

Applications may:

- Maintain multiple Areas
- Coordinate multiple Engine instances
- Implement relay or federation
- Wrap resolutions into higher-level commit objects
- Expose guidance systems

But these must remain outside the Engine boundary.

---

## 9. Stability Guarantee

The Engine boundary defined in this document is frozen for V1/V2.

Future versions may:

- Add new domain primitives if required for legitimacy
- Add new transitions consistent with invariants
- Add new queries that observe canonical state

Future versions must not:

- Introduce storage coupling
- Introduce federation logic
- Introduce commit transport concerns
- Introduce guidance logic
- Blur Area boundaries
- Blur legitimacy vs application concerns

If a change affects application behavior but not mechanical legitimacy, it must occur outside the Engine.

---

## 10. Mental Model

The Engine is a deterministic legitimacy CPU.

It:

- Operates inside one Area namespace
- Enforces legitimacy rules
- Transitions state
- Emits events

Everything else — storage, transport, federation, guidance, user experience — belongs to higher layers.

This separation preserves:

- Long-term freeze stability
- Library independence
- Mechanical clarity
- Architectural integrity

Nothing beyond mechanical legitimacy lives here.