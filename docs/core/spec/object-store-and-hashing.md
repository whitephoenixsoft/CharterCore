# Charter Core — Object Store & Hashing Specification

Status: LOCKED (v1)
Scope: Engine-internal
Change Policy: Any change requires a new hash version and explicit migration

---
## 1. Object Identity Model

### Rule OS-01 — Object Identity Is Hash-Based

Every persisted object in Charter Core is identified solely by a deterministic cryptographic hash.

Object identity MUST NOT depend on:
- Filesystem path
- Filename
- Storage root
- Import source
- Runtime context

The same canonical object content MUST always produce the same object hash.

Fail if:
- Object identity changes without explicit migration
- Identity depends on storage location or runtime state

### Rule OS-02 — Hash Is Not Part of the Domain Model

Domain models (Area, Session, Resolution, Candidate, Stance, Audit, etc.):
- MUST NOT contain their own hash
- MUST NOT compute hashes internally
- MUST NOT self-reference by hash

The object hash is strictly:
- A storage concern
- A verification concern
- An import/export concern

Rationale:
- Prevents circular dependencies
- Prevents mutation ambiguity
- Prevents hash-version lock-in

---
## 2. Hash Versioning

### Rule OS-03 — Hash Version Is Explicit

Every object hash MUST be computed under an explicit Charter hash version.

Initial version: v1

Future changes require:
- A new version identifier (v2, v3, …)
- Explicit rehash logic
- Parallel coexistence of versions

Fail if:
- Hash version is inferred implicitly
- Hash version changes without migration

### Rule OS-04 — Multiple Hash Versions May Coexist

The engine MUST:
- Accept objects with different hash versions
- Preserve original hashes during import
- Treat hash version as part of object identity

The engine MUST NOT:
- Assume a single global hash version
- Auto-upgrade hashes implicitly

---
## 3. Canonical Hash Input (Digest) — v1

### Rule OS-05 — Hash Input Is Canonical and Deterministic

For hash version v1, the digest input MUST be the following byte sequence:
```
charter:<hash_version>\n
type:<object_type>\n
len:<byte_length>\n
<canonical_json_bytes>
```
Where:
- `hash_version` = "v1"
- `object_type` ∈ { area, session, resolution, candidate, stance, audit, … }
- `byte_length` = length (in bytes) of `<canonical_json_bytes>`
- `<canonical_json_bytes>` is UTF-8 encoded

### Rule OS-06 — Canonical JSON Rules (v1)

Canonical JSON serialization MUST obey:
UTF-8 encoding
Lexicographically sorted object keys
No insignificant whitespace
No pretty-print formatting
Arrays preserve order
Numbers serialized unambiguously
These rules MUST be implemented once and reused everywhere.
Fail if:
Different serializations produce different hashes
Canonicalization logic diverges across components

Rule OS-07 — Hash Algorithm (v1)
Algorithm: sha256
Output: lowercase hexadecimal string

---
## 4. Stored Object Envelope

Rule OS-08 — Objects Are Stored with an Explicit Envelope
Objects MUST be persisted as self-describing envelopes:
```Json
{
  "charter_hash_version": "v1",
  "hash_algorithm": "sha256",
  "object_type": "area",
  "object_hash": "<hex-digest>",
  "object": { ... canonical domain object ... }
}
```

Rule OS-09 — Envelope Is Not Hashed
The envelope itself:
MUST NOT be part of the hash input
Exists solely for storage, validation, and tooling
The engine MUST verify:
object_hash matches recomputed digest
Envelope metadata matches digest headers
Mismatch is a fsck error.

---
## 5. Object Store Semantics

Rule OS-10 — Object Store Is Append-Only
The object store MUST be append-only.
Objects are never mutated
Objects are never deleted implicitly
Duplicate inserts with identical hashes are no-ops
Fail if:
Any engine operation deletes stored objects
History is rewritten implicitly

Rule OS-11 — Supersession Is Logical, Not Physical
Supersession:
Creates new objects
Updates refs
NEVER removes or overwrites old objects
Historical objects MUST remain accessible indefinitely.

---
## 6. Import & Export Interaction

Rule OS-12 — Export Preserves Object Identity
Exports:
MUST include object_hash
MUST preserve envelope metadata
MUST NOT recompute or reinterpret hashes
Fail if:
Export alters object identity

Rule OS-13 — Import Does Not Rewrite Hashes
During import:
Objects retain their original hash
Rehashing is permitted only when:
Importing across hash versions
Explicitly requested or required
Recorded in audit metadata
If rehashed:
Original hash MUST be preserved as historical reference
New hash becomes authoritative locally
Fail if:
Import implicitly rehashes objects

---
## 7. Hash Migration & Auditing

Rule OS-14 — Hash Migration Is Explicit and Auditable
Hash upgrades:
MUST be explicitly initiated
MUST NOT occur at startup
MUST NOT mutate existing objects
A migration:
Writes new objects
Rebinds refs explicitly
Emits global and area-level audit records
Fail if:
Objects are rewritten in place
Hashes change without audit events

---
## 8. Refs and Object Liveness

Rule OS-15 — Refs Define Liveness
An object is considered live only if reachable from a ref.
Unreferenced objects:
May exist indefinitely
MUST NOT affect legitimacy
MAY be reported by fsck
Fail if:
Mere object existence affects engine behavior

---
## 9. Validation & fsck

Rule OS-16 — Validation Is Mechanical
Validation consists of:
Recomputing object digests
Verifying envelope metadata
Verifying referential integrity
Validation MUST NOT:
Infer semantics
Modify objects
Rewrite storage

Rule OS-17 — fsck Is Read-Only
fsck:
MUST be read-only
MUST NOT repair, rewrite, or delete data
MAY emit diagnostics or audit warnings
Fail if:
fsck mutates engine state