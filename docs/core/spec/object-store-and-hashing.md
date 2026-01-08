# Charter Core — Object Store & Hashing Specification (LOCKED)

**Status**: FROZEN (v1)
Changes require a new hash version and explicit migration logic.

## 1. Object Identity Model (LOCKED)

### Rule OS-01 — Object Identity Is Hash-Based

Every persisted object in Charter Core is identified solely by a deterministic cryptographic hash.

Object identity does not depend on:
- Filesystem path
- Filename
- Storage root
- Import source

The same object content MUST always produce the same object hash.

### Rule OS-02 — Hash Is Not Part of the Domain Model

Domain models (Area, Resolution, Session, Candidate, Stance, Audit, etc.):
- MUST NOT contain their own hash
- MUST NOT reference themselves by hash
- MUST NOT compute hashes internally

The hash is:
- A storage concern
- A verification concern
- An export/import concern

This prevents circularity, mutation ambiguity, and version lock-in.

## 2. Hash Versioning (LOCKED)

### Rule OS-03 — Hash Version Is Explicit

Every object hash is computed under a named Charter hash version.

- Initial version: v1
- Future changes require:
    - A new version (v2, v3, …)
    - Explicit rehash-on-import logic
    - Parallel coexistence of versions

Hash versions MUST NOT be inferred implicitly.

## 3. Canonical Hash Input (Digest) — v1 (LOCKED)

### Rule OS-04 — Hash Input Is Canonical and Deterministic

The **hash digest** is computed from the following canonical byte sequence:
```text
charter:<hash_version>\n
type:<object_type>\n
len:<byte_length>\n
<canonical_json_bytes>
```
Where:
- `hash_version = "v1"`
- `object_type` ∈ { area, session, resolution, candidate, stance, audit, … }
- `byte_length` = length in bytes of `<canonical_json_bytes>`
- `canonical_json_bytes`:
    - UTF-8 encoded
    - Canonically serialized
    - Deterministically ordered
    - No insignificant whitespace

#### Canonical JSON Rules (v1)
- Object keys are sorted lexicographically
- No trailing whitespace
- No pretty-print formatting
- Arrays preserve order
- Numbers are serialized without ambiguity

> These rules MUST be implemented once and reused everywhere.

### Rule OS-05 — Hash Algorithm (v1)

- Algorithm: `sha256`
- Output: lowercase hex string

## 4. Stored Object Envelope (LOCKED)

### Rule OS-06 — Objects Are Stored with an Explicit Envelope

Objects are persisted as **self-describing envelopes**:
```Json
{
  "charter_hash_version": "v1",
  "hash_algorithm": "sha256",
  "object_type": "area",
  "object_hash": "<hex-digest>",
  "object": { ... canonical domain object ... }
}
```
Properties
- The envelope is **not** part of the hash input
- The object_hash MUST equal the recomputed digest
- Envelope fields MUST match the digest header
- Mismatch is a **fsck error**

## 5. Object Store Semantics (LOCKED)

### Rule OS-07 — Object Store Is Append-Only

- Objects are never mutated
- Objects are never deleted implicitly
- Duplicate inserts with identical hashes are no-ops

### Rule OS-08 — Supersession Is Logical, Not Physica
l
Supersession:
- Creates new objects
- Adds references
- Does NOT remove or overwrite old objects

## 6. Export & Import Interaction (LOCKED)

### Rule OS-09 — Export Uses Stored Object Hashes

Exports:
- MUST include `object_hash`
- MUST NOT recompute or reinterpret object identity
- MUST preserve envelope metadata

### Rule OS-10 — Import May Rehash Only Across Versions

Rehashing is permitted **only** when:
- Importing from an older hash version
- Explicitly requested or required
- Recorded in audit metadata

Imported objects MUST retain:
- Original hash (as historical reference)
- New local hash (as authoritative identity)

## 7. Fsck & Validation (LOCKED)

### Rule OS-11 — Validation Is Mechanical

Object validation consists of:
1. Recomputing the digest
2. Verifying envelope metadata
3. Verifying referential integrity
4. Reporting:
    - Corruption
    - Mismatch
    - Orphans
    - Unknown types

No semantic interpretation is permitted.