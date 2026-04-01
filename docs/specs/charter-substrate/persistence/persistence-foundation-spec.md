# Charter Persistence Layer — Foundational Specification

Status: FOUNDATIONAL  
Intent: Define the storage abstractions and persistence guarantees used by Charter local runtime systems  
Scope: Runtime persistence contracts, object storage, ref storage, audit storage, metadata storage, indexing boundaries  
Does NOT Define: legitimacy semantics, commit structure (CCS), commit storage lifecycle, alignment, guidance, or relay transport  

---

# 1. Purpose

The Charter Persistence Layer defines how the Runtime Layer persists and recovers local operational state.

It exists to:

- decouple Runtime and Engine behavior from specific storage backends
- preserve local object integrity and mutability boundaries
- support deterministic recovery and rebuild
- enforce the separation between:
  - immutable historical objects
  - mutable current pointers
  - append-only audit memory
  - mutable local metadata

The Persistence Layer is a **local runtime substrate**.

It is not:

- the Charter Commit System (CCS)
- the Commit Store
- the Relay
- an interpretation layer

---

# 2. Core Principle

> Engine and Runtime invariants dominate storage implementation.

Backends may differ in capability and performance, but correctness must never depend on backend-specific behavior.

The Persistence Layer must support:

- portability
- deterministic recovery
- explicit mutability boundaries
- rebuildable derived state

---

# 3. Architectural Position

The Persistence Layer exists below the Runtime Layer and beside the Legitimacy Engine as a supporting substrate.

It provides storage contracts for:

- local runtime object state
- mutable references
- append-only audit memory
- mutable runtime metadata
- rebuildable indexes

It does not create legitimacy, define commits, or preserve global artifact truth.

---

# 4. Separation from Other Storage Systems

## 4.1 Persistence Layer vs Commit Store

The Persistence Layer and Commit Store are distinct.

### Persistence Layer
Stores:

- runtime objects
- refs
- audit events
- mutable metadata
- rebuildable indexes

Characteristics:

- local
- partly mutable
- optimized for active workflows

### Commit Store
Stores:

- immutable commit artifacts

Characteristics:

- append-only
- preservation-oriented
- separate lifecycle model

These systems must never be conflated.

---

## 4.2 Persistence Layer vs CCS

CCS defines the structure of commits.

The Persistence Layer does not:

- define commit envelopes
- assign commit meaning
- transport commit artifacts

---

# 5. Storage Categories

The Persistence Layer is composed of distinct storage categories.

---

## 5.1 Object Store

The Object Store stores immutable local runtime objects.

Examples may include:

- accepted runtime-domain objects
- session records
- baseline review artifacts
- deliberate artifacts
- other persisted local domain objects

### Guarantees

- objects are immutable once written
- objects are never mutated in place
- duplicate inserts are idempotent
- identity and integrity are preserved
- supersession is logical, not physical

### Important Boundary

The Object Store is not the Commit Store.

It stores local runtime-domain objects, not commit artifacts.

---

## 5.2 Ref Store

The Ref Store stores mutable pointers to current local state.

Examples:

- current area authority
- current area scope
- active session pointer
- optional ergonomic pointers

### Guarantees

- refs are mutable
- refs represent current state only
- refs are rebuildable from object history and audit where applicable
- refs do not define legitimacy

### Important Boundary

Refs are pointers, not facts.

---

## 5.3 Audit Store

The Audit Store stores append-only audit events.

### Guarantees

- audit events are immutable
- audit ordering is preserved
- audit is never rewritten
- audit remains reconstructable

### Important Boundary

Audit records what occurred.  
It does not decide meaning.

---

## 5.4 Metadata Store

The Metadata Store stores mutable local metadata and configuration.

Examples:

- labels
- annotations
- local counters
- non-legitimizing configuration
- storage configuration

### Guarantees

- mutable by design
- not legitimacy-bearing
- may support compaction where explicitly allowed

---

## 5.5 Index Layer

Indexes are derived acceleration structures.

Examples:

- lifecycle indexes
- membership indexes
- supersession indexes
- lookup maps

### Guarantees

- indexes are never authoritative
- indexes are fully derivable
- loss of indexes is non-fatal
- indexes may be rebuilt at startup

---

# 6. Backend Abstraction

The Persistence Layer must be backend-agnostic.

Backends may include:

- in-memory
- file-based
- SQLite
- key-value
- cloud-backed implementations

Backends may advertise optional capabilities such as:

- transactions
- ordered writes
- append-only primitives
- snapshotting
- compaction

These capabilities may be used for optimization.

They must never be required for correctness.

---

# 7. Invariant Ownership

## 7.1 Engine / Runtime Responsibility

Correctness must be enforced by Engine and Runtime logic, not delegated to the backend.

Required invariant ownership includes:

- object immutability enforcement
- explicit ref mutation
- audit append-only guarantees
- session sealing rules
- rebuild correctness
- import / restore boundary handling

---

## 7.2 Backend Responsibility

Backends are responsible only for:

- faithfully implementing storage contracts
- preserving bytes as written
- exposing optional capabilities honestly

Backends must not silently redefine behavior.

---

# 8. Object Identity and Integrity

## 8.1 Object Identity

Persisted runtime objects must have stable identity.

Identity must not depend on:

- filesystem path
- storage root
- runtime context
- import source

Identity may be:

- hash-based
- versioned by hash scheme
- explicitly migrated when required

---

## 8.2 Canonical Integrity

Object hashing and integrity validation must be deterministic.

Required properties:

- canonical serialization
- explicit hash versioning
- explicit hash algorithm declaration
- integrity verification on read/import/fsck

Hash migration must be explicit and auditable.

---

## 8.3 Envelope Principle

Persisted objects may be wrapped in an integrity envelope containing:

- hash version
- hash algorithm
- object type
- object hash
- object body

Envelope metadata must be verifiable against object contents.

---

# 9. Ref Semantics

Refs are mutable local pointers.

They must:

- resolve deterministically
- fail explicitly when missing
- remain scoped
- never infer legitimacy
- never encode hidden history

Required refs may exist for current local context such as:

- authority
- scope

Optional refs may exist for ergonomics.

Ergonomic refs must never affect correctness.

---

# 10. Audit Semantics

Audit is append-only memory.

Audit must:

- preserve stable ordering
- remain immutable
- support replay and verification
- distinguish event categories clearly

Audit must never be used to silently repair local state.

---

# 11. Index Semantics

Indexes exist only for speed.

Indexes must:

- be fully derivable from authoritative stores
- never be exported as truth
- never change runtime meaning
- be rebuildable deterministically

Loss of indexes must be equivalent to a rebuild, not a data loss event.

---

# 12. Recovery and Rebuild

The Persistence Layer must support deterministic recovery.

On startup or rebuild:

1. load authoritative local stores
2. verify integrity where required
3. rebuild refs if needed
4. rebuild indexes
5. expose consistent runtime state

Partial or ambiguous recovery must fail explicitly.

---

# 13. Import / Export Boundary

The Persistence Layer participates in local import/export handling, but does not define legitimacy or commit semantics.

### It may support:

- local object persistence for restore workflows
- integrity validation
- local ref recreation
- local rebuild after import

### It must not:

- treat imported refs as authority by default
- allow imported mutable state to silently override legitimacy
- export indexes as authoritative data

---

# 14. fsck / Validation

Validation tools may:

- recompute hashes
- verify envelopes
- verify ref targets
- verify referential integrity
- compare rebuilt indexes to live indexes

Validation tools must not:

- mutate objects
- rewrite refs automatically
- rewrite indexes silently
- reinterpret history

fsck is read-only evidence, not repair.

---

# 15. What the Persistence Layer Does NOT Do

The Persistence Layer does not:

- create legitimacy
- define commit structure
- store commit artifacts as CCS truth
- transport artifacts
- compute alignment
- interpret meaning
- enforce guidance behavior

It is a storage substrate only.

---

# 16. Design Guarantees

The Persistence Layer guarantees that:

- runtime correctness is backend-independent
- immutable objects remain immutable
- mutable pointers remain explicit
- audit remains append-only
- metadata remains non-legitimizing
- indexes remain derived and rebuildable
- local recovery remains deterministic

---

# 17. Mental Model

- Objects are local facts
- Refs are current pointers
- Audit is immutable memory
- Metadata is mutable description
- Indexes are scratchpads for speed
- Backends support storage, but never decide correctness

---

# 18. Final Principle

The Persistence Layer exists so that local runtime truth can be stored safely without allowing storage mechanics to redefine system behavior.

It preserves structure, integrity, and recoverability —  
while leaving legitimacy, interpretation, and transport to the layers that actually own them.