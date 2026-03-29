# Charter CLI Library — Object Store & Hashing Specification

Status: LOCKED (v1)  
Scope: CLI Library (V4) — Persistence Layer  
Change Policy: Hash changes require new version + migration  

---

## 1. Purpose

This document defines how the CLI Library:

- Persists domain objects
- Computes object identity hashes
- Stores object envelopes
- Maintains append-only guarantees
- Validates integrity
- Supports import/export and migration

The CLI Library owns all persistence concerns.

---

## 2. Object Identity Model

### CLI-OS-01 — Object Identity Is Hash-Based

Every persisted object is identified by a deterministic cryptographic hash.

Identity MUST NOT depend on:

- Filesystem path
- Filename
- Storage root
- Import source
- Runtime context

Identical canonical content MUST produce identical hashes.

Fail if:
- Identity changes without explicit migration
- Identity depends on storage location

---

## 3. Hash Versioning

### CLI-OS-02 — Hash Version Is Explicit

Each object hash includes a Charter hash version.

Initial version: v1

Future changes require:

- New version identifier
- Explicit migration command
- Audit record of migration

The CLI MUST support coexistence of multiple hash versions.

The CLI MUST NOT auto-upgrade hashes.

---

## 4. Canonical Digest Input (v1)

For hash version v1, the digest input byte sequence MUST be:

charter:<hash_version>\n  
type:<object_type>\n  
len:<byte_length>\n  
<canonical_json_bytes>

Where:

- hash_version = "v1"
- object_type is explicit
- byte_length is length of canonical JSON bytes
- canonical JSON is UTF-8 encoded

---

## 5. Canonical JSON Rules (v1)

Canonical serialization MUST:

- Use UTF-8 encoding
- Sort object keys lexicographically
- Remove insignificant whitespace
- Preserve array order
- Serialize numbers deterministically

Canonicalization MUST be implemented once and reused everywhere.

Fail if:
- Different serializations produce different hashes

---

## 6. Hash Algorithm (v1)

Algorithm: sha256  
Output: lowercase hexadecimal string  

---

## 7. Object Envelope

Persisted objects MUST use the following envelope structure:

{
  "charter_hash_version": "v1",
  "hash_algorithm": "sha256",
  "object_type": "<type>",
  "object_hash": "<hex>",
  "object": { ... domain object ... }
}

The envelope itself is NOT part of the hash input.

The CLI MUST verify:

- object_hash matches recomputed digest
- Metadata matches digest headers

Mismatch is a validation failure.

---

## 8. Append-Only Object Store

The object store MUST be append-only.

- Objects are never modified in place
- Objects are never deleted implicitly
- Duplicate identical hashes are no-ops

Supersession creates new objects.
Historical objects remain accessible indefinitely.

---

## 9. Import & Export

Exports MUST:

- Preserve object_hash
- Preserve envelope metadata
- Avoid recomputation of hashes

Imports MUST:

- Preserve original hashes
- Avoid implicit rehashing

Rehashing is allowed only:

- During explicit migration
- When importing across hash versions
- With audit record

---

## 10. Hash Migration

Hash migration MUST:

- Be explicitly initiated
- Write new objects
- Rebind logical refs
- Emit audit records
- Never rewrite objects in place

---

## 11. Liveness & Ref Traversal

The CLI determines object reachability by:

- Traversing refs
- Supplying active logical refs to the Engine

Unreferenced objects:

- May exist indefinitely
- Must not affect legitimacy
- May be reported by validation tools

---

## 12. Validation & fsck

Validation includes:

- Recomputing digests
- Verifying envelope metadata
- Checking referential integrity

fsck MUST be read-only.
fsck MUST NOT mutate storage.
fsck MUST NOT rewrite history.

---

## 13. Boundary with Engine

The CLI:

- Computes hashes
- Validates envelopes
- Stores objects
- Manages refs
- Supplies logical objects to the Engine

The Engine:

- Evaluates legitimacy
- Enforces lifecycle rules
- Does not compute hashes
- Does not perform storage validation

---

## Mental Model

- Hash defines storage identity.
- Envelope ensures integrity.
- Store is append-only.
- Engine evaluates meaning.
- CLI owns persistence.