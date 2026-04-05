# Charter Persistence Layer — Foundational Specification (Revised)

Status: FOUNDATIONAL  
Intent: Define the storage abstractions and persistence guarantees used by Charter local runtime systems  
Scope: Runtime persistence contracts, object storage, refs, audit, metadata, workflow stores, and indexing boundaries  
Does NOT Define: legitimacy semantics, commit structure (CCS), commit storage lifecycle, alignment (CAS), structure (CSG), identity (CIS), guidance, or relay transport  

---

# 1. Purpose

The Charter Persistence Layer defines how the Runtime Layer persists and recovers local operational state.

It exists to:

- decouple Runtime and Engine behavior from specific storage backends  
- preserve local object integrity and mutability boundaries  
- support deterministic recovery and rebuild  
- enforce separation between:
  - immutable runtime objects  
  - mutable workflow state  
  - append-only audit memory  
  - mutable local metadata  
  - derived indexes  

The Persistence Layer is a **local runtime substrate**.

---

# 2. Core Principle

> Engine and Runtime invariants dominate storage implementation.

Correctness must never depend on backend-specific behavior.

---

# 3. Architectural Position

The Persistence Layer exists below the Runtime Layer and beside the Legitimacy Engine.

It provides storage contracts for:

- runtime objects  
- workflow state  
- refs  
- audit  
- metadata  
- indexes  

It does not create legitimacy or define artifact meaning.

---

# 4. Separation from Other Storage Systems

## 4.1 Persistence Layer vs Commit Store

- Commit Store → immutable artifact truth (CCS)  
- Persistence Layer → runtime operational state  

They must remain strictly separate.

---

## 4.2 Persistence Layer vs Relay Namespace Stores (CRS)

Relay Namespace Stores:

- store foreign commit artifacts  
- are append-only  
- are non-trusting  

The Persistence Layer:

- must not store relay data as runtime objects  
- must not assume relay semantics  
- must not implicitly trust relay artifacts  

---

## 4.3 Persistence Layer vs Derived Substrates

The Persistence Layer does not define storage for:

- CSG (graph materialization)  
- CIS (identity projection)  
- CAS (alignment state)  

These are separate derived systems.

---

## 4.4 No Cross-Domain Persistence

Storage domains must remain isolated.

The Persistence Layer must not:

- write to Commit Store implicitly  
- write to Relay Namespace Stores  
- ingest foreign artifacts without explicit runtime workflows  

---

# 5. Storage Categories

---

## 5.1 Object Store

Immutable runtime-domain objects.

Examples:

- session outputs  
- accepted runtime artifacts  
- baseline review artifacts  
- finalized structures  

### Guarantees

- immutable  
- idempotent inserts  
- integrity-preserving  

### Boundary

Not:
- Commit Store  
- Relay storage  

---

## 5.2 Session Store

Mutable in-progress legitimacy sessions.

Examples:

- participants  
- votes / stances  
- candidate state  
- closure readiness  

### Guarantees

- mutable until closure  
- sealed on completion  
- recoverable  

### Boundary

Represents **pre-closure legitimacy state**, not final truth.

---

## 5.3 Review Workspace Store

Mutable integration review workspace.

Examples:

- foreign artifacts under review  
- provisional references  
- review decisions  
- review annotations  

### Guarantees

- isolated  
- mutable until review closure  
- non-legitimizing  

---

## 5.4 Deliberate Workspace Store

Mutable pre-governance exploration.

Examples:

- epics  
- breakouts  
- options  
- synthesis drafts  
- workflow state  

### Guarantees

- mutable  
- supports branching and parallel work  
- non-legitimizing  

---

## 5.5 Deliberate Artifact Store

Immutable deliberate artifacts.

Examples:

- frozen options  
- frozen syntheses  
- breakout outputs  
- deliberate closure artifacts  

### Guarantees

- immutable  
- preservational  
- may later be used in review  

### Boundary

Does not create legitimacy.

---

## 5.6 Ref Store

Mutable pointers to current runtime state.

Examples:

- current authority  
- active session  
- active area  

### Guarantees

- mutable  
- rebuildable  
- non-authoritative  

---

## 5.7 Audit Store

Append-only record of runtime events.

### Guarantees

- immutable  
- ordered  
- never rewritten  

### Boundary

Audit is local runtime memory, not artifact history.

---

## 5.8 Metadata Store

Mutable local metadata.

Examples:

- labels  
- configuration  
- local area state  
- ergonomic settings  

### Guarantees

- mutable  
- non-legitimizing  

---

## 5.9 Index Layer

Derived acceleration structures.

### Guarantees

- rebuildable  
- non-authoritative  

---

# 6. Backend Abstraction

Backends may include:

- in-memory  
- file-based  
- SQLite  
- key-value  

Optional capabilities:

- transactions  
- ordered writes  
- snapshotting  

Must not affect correctness.

---

# 7. Invariant Ownership

## Engine / Runtime

Responsible for:

- immutability  
- session rules  
- workflow transitions  
- audit correctness  
- rebuild logic  

## Backend

Responsible for:

- storing bytes correctly  
- honoring contracts  

---

# 8. Object Identity and Integrity

- stable identity  
- deterministic hashing  
- canonical serialization  
- explicit hash versioning  

---

# 9. Ref Semantics

Refs:

- mutable pointers  
- must resolve deterministically  
- must not encode hidden history  

---

# 10. Audit Semantics

Audit:

- append-only  
- replayable  
- non-authoritative  

---

# 11. Index Semantics

Indexes:

- derived  
- rebuildable  
- non-authoritative  

---

# 12. Recovery and Rebuild

On startup:

1. load stores  
2. verify integrity  
3. rebuild refs  
4. rebuild indexes  

---

# 13. Foreign Artifact Boundary

Foreign artifacts (e.g., from CRS):

- must remain outside runtime object storage  
- must pass through explicit workflows  
- must not be implicitly trusted  

---

# 14. Import / Export Boundary

Imports may originate from:

- files  
- restore operations  
- relay fetch  

The Persistence Layer must not:

- auto-admit foreign data  
- override local truth  

---

# 15. Validation (fsck)

May:

- verify integrity  
- validate refs  
- rebuild indexes  

Must not:

- mutate data  
- rewrite history  

---

# 16. Design Guarantees

- backend-independent correctness  
- immutable objects remain immutable  
- workflows remain explicit  
- audit remains append-only  
- metadata remains non-authoritative  
- indexes remain derived  

---

# 17. Mental Model

- Objects → immutable facts  
- Sessions → active decisions  
- Review → controlled admission  
- Deliberate → exploration  
- Metadata → local description  
- Audit → memory  
- Indexes → acceleration  

---

# 18. Final Principle

The Persistence Layer preserves runtime truth and workflow state  
without allowing storage mechanics to redefine system behavior.