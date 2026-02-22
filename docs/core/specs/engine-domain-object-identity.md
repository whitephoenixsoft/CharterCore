# Charter Core — Engine Domain Object & Identity Semantics

Status: LOCKED (v1)  
Scope: Engine (V1/V2) — No Persistence  
Change Policy: Changes require invariant review  

---

## 1. Purpose

This document defines how the Engine understands object identity, immutability, and legitimacy relationships without any knowledge of storage, hashing, or persistence mechanisms.

The Engine operates purely on domain objects and deterministic state transitions.

The Engine does not:
- Compute hashes
- Serialize JSON
- Read or write files
- Validate envelopes
- Perform storage integrity checks

---

## 2. Domain Identity Model

### ENG-OBJ-01 — Domain Identity Is Logical, Not Hash-Based

Each domain object has a stable logical identifier:

- area_id
- session_id
- resolution_id
- candidate_id
- stance_id
- audit_event_id

These identifiers:

- Are provided by the caller (CLI layer)
- Must be stable for the lifetime of the object
- Must be unique within their namespace

The Engine does not define how identifiers are generated.

Fail if:
- Identity depends on storage location
- Identity depends on hash algorithm
- Identity changes during engine execution

---

## 3. Object Immutability

### ENG-OBJ-02 — Domain Objects Are Immutable

Once created, domain objects:

- MUST NOT be mutated
- MUST NOT be rewritten
- MAY only be superseded via new objects

All lifecycle transitions are expressed through:

- New objects
- Explicit state transitions
- Supersession links

Fail if:
- An object is modified in place
- History is rewritten

---

## 4. Supersession Graph Semantics

### ENG-OBJ-03 — Supersession Is Logical

Supersession relationships:

- Are expressed via resolution_id references
- Form a directed acyclic graph
- Are immutable once created

The Engine evaluates supersession based on logical identifiers only.

The Engine does not depend on storage ordering, timestamps, or hash ordering.

Fail if:
- Replacement order is inferred implicitly
- Graph structure is mutated after creation

---

## 5. Liveness Semantics

### ENG-OBJ-04 — Liveness Is Derived from Active References

The Engine evaluates which objects are considered active based on:

- Logical ref inputs provided by the CLI
- Lifecycle state
- Supersession rules

The Engine does not traverse storage.
The Engine does not compute reachability from physical storage.

Fail if:
- Mere existence of an object affects legitimacy
- Unreferenced objects alter session behavior

---

## 6. Engine Assumptions About Persistence

The Engine assumes:

- Domain objects provided to it are complete and valid
- Identifiers are stable
- Historical objects are immutable
- Supersession links are structurally valid

Validation of storage integrity is outside engine scope.

---

## 7. Determinism Guarantee

Given identical:

- Domain objects
- Logical refs
- Session state
- Authority rules
- Scope rules

The Engine MUST produce identical outcomes.

No behavior may depend on:

- Hash
- Serialization
- Storage order
- Import source

---

## Mental Model

- The Engine evaluates legitimacy.
- The Engine does not manage storage.
- The Engine operates on logical identity only.
- Persistence is a caller responsibility.