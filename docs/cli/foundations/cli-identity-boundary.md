# Charter CLI — Area Identity & Boundary Foundation
Status: FOUNDATION (V1+)  
Layer: CLI / Infrastructure  
Applies to: Charter CLI, Engine Integration, Import/Export, External References  
Excludes: Engine legitimacy semantics, storage engine implementation details

---

## 1. Purpose

This document defines the identity model and lifecycle rules for Areas within Charter.

It establishes:

- What an Area is at the CLI layer
- How Area identity is created and preserved
- How Areas interact with the Engine
- How Areas behave during export, restore, and consolidation
- How Area identity supports external references
- How identity remains stable across versions and installations

This document prevents identity drift and preserves long-term legitimacy boundaries.

---

## 2. Definition: Area

### 2.1 Conceptual Definition

An Area is:

- A legitimacy boundary
- A namespace for sessions and resolutions
- A governance isolation domain
- The root context provided to the Engine

All legitimacy operations occur within exactly one Area at a time.

---

### 2.2 Structural Definition

An Area consists of:

- Area ID (immutable)
- Object store (append-only, content-addressed)
- Metadata store (mutable, non-legitimate fields)
- Audit history
- Optional reference/index structures for performance

The Engine operates only within a single Area context provided by the CLI.

---

## 3. Area ID

### 3.1 Identity Model

- Area ID is a UUID version 7.
- It is generated at Area creation by the CLI.
- It is immutable for the lifetime of the Area.
- It is globally unique with high probability.
- It is time-ordered for efficient indexing and storage.

Area ID is opaque. It encodes no meaning.

---

### 3.2 Identity Invariants

Area ID:

- Never changes
- Is never reused
- Must survive export and import
- Must survive relay transport
- Must survive backup and restore
- Must be included in export metadata
- Must be preserved during restore

Identity is permanent.

---

### 3.3 Non-Goals

Area ID does not:

- Encode organization name
- Encode ownership
- Encode hierarchy
- Encode semantics
- Encode version information

Identity and meaning are strictly separated.

---

## 4. Area Metadata (Mutable)

### 4.1 Labels

- Human-readable name
- May change over time
- Changes emit audit events
- Do not affect Area ID
- Do not alter stored references

Renaming does not rewrite history.

---

### 4.2 Annotations

- Optional descriptive metadata
- May change
- Stored outside the object store
- Not part of legitimacy evaluation

Annotations support understanding, not authority.

---

### 4.3 Storage Separation

- Object store: immutable resolutions, sessions, and related artifacts
- Metadata store: mutable labels and annotations
- Area ID binds both stores together

Immutable legitimacy and mutable description are separated by design.

---

## 5. Area Lifecycle

### 5.1 Creation

When creating a new Area, the CLI:

1. Generates a UUIDv7 Area ID
2. Initializes a new object store
3. Initializes a metadata store
4. Registers the Area locally

The Engine is not responsible for Area creation.

---

### 5.2 Operation

During normal operation:

- CLI selects an Area
- CLI provides Area context to the Engine
- Engine operates deterministically within that Area
- Engine remains unaware of other Areas

Areas are isolated legitimacy domains.

---

### 5.3 Renaming

Renaming:

- Updates metadata only
- Emits an audit event
- Does not modify Area ID
- Does not modify historical references

Historical references preserve original label snapshots.

---

## 6. Clone vs Consolidation

### 6.1 Restore (Clone)

Restore exists for backup and replication purposes.

Restore:

- Copies the entire object store
- Preserves Area ID
- Preserves metadata
- Produces an identical legitimacy boundary

Multiple installations may host the same Area ID if created via restore.

Restore does not require baseline review.

Restore does not alter legitimacy.

---

### 6.2 Consolidation (Import Rebase)

Consolidation is used to merge external data.

Consolidation:

- Does not preserve Area ID of the source
- Treats imported data as foreign
- Requires baseline review
- Produces new resolutions in the receiving Area
- Preserves provenance in audit

Consolidation creates new legitimacy events.
It does not merge identity silently.

---

## 7. Interaction with Engine

### 7.1 Engine Contract

The Engine:

- Receives Area ID as context
- Operates within a single Area at a time
- Treats Area ID as an opaque token
- Does not generate Area IDs
- Does not validate Area uniqueness
- Does not manage Area registry

Area coordination is strictly a CLI concern.

---

## 8. Interaction with External Area References

### 8.1 Reference Target

External Area References bind to:

- Target Area ID
- Snapshot label at time of reference

References are immutable.

References are informational only.

---

### 8.2 CLI Responsibilities

The CLI may:

- Display current and historical labels
- Detect missing referenced Areas
- Provide navigation assistance
- Build reverse reference indexes

The CLI must not:

- Modify stored references
- Infer authority
- Treat references as dependency declarations
- Enforce cross-Area governance

Legitimacy never flows through references.

---

## 9. Collision Handling

UUIDv7 provides high entropy and time ordering.

If an Area ID collision is detected:

- The CLI must fail immediately
- No automatic reassignment is permitted
- No silent mutation is allowed

Identity corruption is fatal and must be prevented.

---

## 10. Storage and Indexing Implications

The CLI must maintain:

- Local Area registry (Area ID to storage location)
- Optional reverse reference index (Area ID to resolution IDs)
- Metadata store separate from object store

Indexes are performance aids only.
Identity truth resides in stored objects.

---

## 11. Versioning Implications

Area ID is independent of:

- Engine version
- Embedded spec set
- CLI version
- Export file version
- Hash algorithm version

Upgrading software must never mutate Area ID.

History remains bound to its original identity.

---

## 12. Design Constraints

- Identity must remain stable across decades.
- Identity must not drift due to rename.
- Identity must not depend on content.
- Restore preserves identity.
- Consolidation never merges identity implicitly.
- References bind to identity, not meaning.

If Area identity becomes mutable,
legitimacy boundaries collapse.

---

## 13. Mental Model

Area ID is comparable to a repository identity.

It is not a branch.
It is not a label.
It is not a role.
It is not a semantic name.

Labels may change.
Identity does not.

External references point to the Area itself,
not to its current description.

Identity is permanent.
Meaning evolves.