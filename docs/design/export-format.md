# Charter Core — Export Format Specification (Engine Canonical)

## Purpose

The Charter Core export format provides a **complete, immutable, portable record**
of governance history for one or more Areas.

It is designed to support:

- Auditability
- Source control (Git)
- Backup and restore
- Cross-system transfer
- Offline / disconnected collaboration
- Deterministic consolidation and review

The export format is **engine-owned**.  
Clients (CLI, UI, integrations) MUST NOT redefine or extend its structure.

---

## Design Principles

1. **Single Canonical Format**  
   There is exactly one export format.  
   Import behavior (RESTORE vs CONSOLIDATE) is a function of the importer, not the file.

2. **Full Legitimacy Preservation**  
   Sessions, candidates, stances, lifecycle transitions, and references are preserved
   even if they are not required to legitimize current decisions.

3. **No Semantic Interpretation**  
   The engine does not infer meaning from content.  
   All relationships are explicit and structural.

4. **Append-Only Semantics**  
   Nothing in an export implies deletion, mutation, or rewriting of history.

5. **Deterministic Integrity**  
   Every exported object includes a deterministic `object_hash` derived from its
   canonical serialized form.

---

## Top-Level Structure

```json
{
  "charter_version": "1.0",
  "exported_at": "2025-12-24T18:42:00Z",
  "engine_metadata": {
    "generator": "charter-core",
    "generator_version": "0.1.0"
  },
  "object_hash": "hash-of-entire-export",
  "areas": [ ... ]
}
```

Notes
object_hash at the top level covers all nested content
Hash algorithm is engine-defined but MUST be deterministic and documented
Importers may verify hashes but must not reinterpret content
Area Object
Each Area is exported independently but may reference other Areas by ID.
```
{
  "area_id": "A-ENG-001",
  "object_hash": "hash-of-area-object",
  "metadata": {
    "created_at": "2025-01-10T09:00:00Z",
    "notes": "Engineering decision area"
  },
  "authority": {
    "active_resolution_id": "R-AUTH-002"
  },
  "scope": {
    "active_resolution_id": "R-SCOPE-003"
  },
  "resolutions": [ ... ],
  "sessions": [ ... ]
}
```
Notes
Area identity is stable and immutable
Area names, labels, or descriptions must be represented as resolutions
Area deletion or replacement is handled outside the export (import semantics)
Resolution Object
Resolutions are immutable legitimacy artifacts.
```
{
  "resolution_id": "R-ARCH-004",
  "object_hash": "hash-of-resolution",
  "type": "GENERAL",
  "state": "ACTIVE",
  "content": {
    "text": "Adopt PostgreSQL as the primary database"
  },
  "accepted_at": "2025-02-01T14:32:00Z",
  "accepted_in_session": "S-ARCH-002",
  "context": {
    "authority_resolution_id": "R-AUTH-002",
    "scope_resolution_id": "R-SCOPE-003",
    "referenced_area_ids": [],
    "referenced_resolution_ids": []
  },
  "supersedes": [],
  "superseded_by": null
}
```
Resolution Types
GENERAL
AUTHORITY
SCOPE
Types are informational.
Legitimacy rules are enforced by engine invariants, not by type.
Resolution Lifecycle States
```
ACTIVE
UNDER_REVIEW
SUPERSEDED
RETIRED
```
Rules:
Lifecycle transitions are explicit and auditable
No resolution is ever removed
UNDER_REVIEW resolutions may not govern Authority or Scope
Supersession is directional and permanent
Session Object
Sessions capture how legitimacy was created.
```
{
  "session_id": "S-ARCH-002",
  "object_hash": "hash-of-session",
  "area_id": "A-ENG-001",
  "problem_statement": "Choose database technology",
  "started_at": "2025-02-01T13:00:00Z",
  "closed_at": "2025-02-01T14:32:00Z",
  "state": "CLOSED",

  "authority_resolution_id": "R-AUTH-002",
  "scope_resolution_id": "R-SCOPE-003",

  "constraints": {
    "authority_rule": "MAJORITY_PRESENT",
    "required_participants": [],
    "approval_constraints": []
  },

  "references": {
    "area_ids": [],
    "resolution_ids": []
  },

  "candidates": [ ... ],
  "stances": [ ... ],

  "outcome": {
    "accepted_candidate_ids": ["C-DB-POSTGRES"]
  }
}
```
Notes
Authority and constraints are fixed at session start
References are informational and immutable
Candidate set freezes on first stance
Outcome may include zero, one, or many accepted candidates
Candidate Object
Candidates are neutral options scoped to a session.
```
{
  "candidate_id": "C-DB-POSTGRES",
  "object_hash": "hash-of-candidate",
  "content": {
    "text": "Use PostgreSQL for all production workloads"
  },
  "rationale": "Strong ACID guarantees and team experience",
  "created_at": "2025-02-01T13:10:00Z"
}
```
Notes
rationale is optional
Candidates have no meaning outside their session
Candidate content is never interpreted by the engine
Stance Object (Vote / Acknowledgement)
Stances are purely mechanical records of action.
```
{
  "actor_id": "alice",
  "candidate_id": "C-DB-POSTGRES",
  "type": "ACCEPT",
  "recorded_at": "2025-02-01T14:00:00Z",
  "object_hash": "hash-of-stance"
}
```
Stance Types
ACCEPT
REJECT
ABSTAIN
Presence is derived from recorded stances, not from participant lists.
Import Semantics (Informative)
The export file does not encode intent.
Intent is provided at import time.
Typical modes:
RESTORE — recreate full history verbatim
CONSOLIDATE — import objects as UNDER_REVIEW
The same export file may be used for both.
Integrity & Hashing Rules
Every object includes object_hash
Hashes are computed from a canonical, ordered serialization
Parent hashes do not replace child hashes
Importers MAY verify hashes
Importers MUST NOT auto-correct hash mismatches
Guarantees
This format guarantees:
Full reconstruction of governance history
Deterministic rehydration
No loss of legitimacy context
Git-friendly diffs
Long-term forward compatibility
Non-Goals
The export format does NOT:
Merge conflicting histories
Infer semantic equivalence
Enforce permissions
Resolve authority disputes
Invent governance
Those actions require explicit sessions after import.
Why This Matters
This export format is what makes Charter Core:
A governance ledger
A source-controlled decision system
A portable legitimacy engine
A foundation for CLI, UI, AI, and integrations
Without this format, Charter Core is a database.
With it, Charter Core becomes institutional memory with rules.