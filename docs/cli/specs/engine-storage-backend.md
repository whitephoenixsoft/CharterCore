# ENG-STO — Engine Storage Foundation Specification
Document ID: ENG-STO
Status: FROZEN (v1)
Audience: Charter Core engine implementers
Scope: Engine-internal only (storage abstraction, traits, backends)

---

## 1. Purpose
ENG-STO-01 — Storage Abstraction Exists to Decouple Engine from Backend
The engine must not depend on a particular storage implementation.  
All persistent and semi-persistent state is written through well-defined storage interfaces (traits).  

This guarantees:

- Portability to different backends (in-memory, file-based, SQLite, cloud)
- Preservation of engine invariants regardless of backend
- Safety of legitimacy, audit, and other critical data
- Optimizations without compromising correctness

Fail if:

- Engine assumes backend-specific behavior
- Engine correctness depends on optional features

---

## 2. Core Principle
ENG-STO-02 — Engine Invariants Always Dominate
Storage backends may advertise optional capabilities (supports), but the engine must never depend on them for correctness.  

Backends may report support for:

- Transactions
- Ordering guarantees
- Append-only behavior
- Snapshotting / compaction

These are informational hints; engine logic must enforce all invariants itself.

---

## 3. Storage Categories & Traits

### 3.1 Object Store
- Stores immutable, append-only objects with full history
- Each object has a self-contained envelope (hash, canonical content)
- Guarantees:
  - Objects are never mutated in-place
  - Duplicate inserts are idempotent
  - Supersession is logical, handled by model, not store
- Typical backend supports:
  - append-only writes
  - optional transaction batching
  - optional ordering
  - no compaction

### 3.2 Ref Store
- Stores mutable pointers to current states (e.g., active resolution, last authority, active session)
- Guarantees:
  - Only latest reference matters for queries
  - Updates are explicit
- Typical backend supports:
  - single-value upserts
  - optional atomic transactions
  - optional versioning

### 3.3 Audit Store
- Stores immutable, append-only audit events
- Guarantees:
  - Event ordering must be preserved
  - Events are never deleted or altered
  - Engine logic, not backend, enforces append-only
- Typical backend supports:
  - transactions (optional)
  - ordered writes
  - no compaction or snapshots
  - filtered iteration by scope

### 3.4 Metadata Store (mutable generic + config)
- Stores mutable configuration, counters, or other generic data
- Guarantees:
  - Mutable but append-only log of changes
  - Optional compaction allowed for storage efficiency
- Typical backend supports:
  - key-value updates
  - optional compaction
  - optional transactional writes

### 3.5 Session Store (mutable, in-progress sessions)
- Stores sessions that are currently open and not yet accepted
- Guarantees:
  - Mutable state for votes, stances, and other ephemeral session fields
  - Engine ensures immutability once session is closed
- Typical backend supports:
  - transactional updates
  - optional snapshotting
  - rollback for incomplete sessions

---

## 4. Backend Capability Declaration

All backends must implement the storage trait with capability metadata:

```rust
struct BackendCapabilities {
    supports_transactions: bool,
    supports_ordering: bool,
    supports_append_only: bool,
    supports_snapshots: bool,
}
```
- Engine may check supports_* for optimizations  
- Engine must not rely on supports_* for correctness

---

## 5. Engine Behavior with Backends

1. Invariant enforcement is always engine responsibility:
   - Object immutability
   - Audit append-only
   - Ref updates explicit
   - Session closure sealing

2. Supports guide optimization, not correctness:
   - Transactional writes may be batched
   - Ordering hints may be leveraged for index building
   - Snapshots may be used to speed reads
   - Compaction may be used for non-critical mutable metadata

3. Rebuildable state:
   - Indexes are reconstructed from objects and refs on boot
   - Audit store may be replayed for verification
   - Mutable stores must be persisted in a way that allows recovery to last consistent state

---

## 6. Design Guarantees

- Engine correctness is independent of backend
- All invariants enforced in code, never assumed from storage
- Backends may evolve or be replaced without changing engine logic
- Performance optimizations are optional and safely bypassable

---

## 7. Mental Model (Lock This In)

- Objects = facts  
- Refs = current pointers  
- Metadata = mutable config  
- Sessions = in-progress deliberation  
- Audit = immutable memory  
- Backends = storage engines with optional support  

Engine correctness is never delegated. Backends support, but never decide.