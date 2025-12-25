# Charter Core — Export Format Specification

## Purpose

The Charter Core export format provides a complete, immutable, portable record of governance history for one or more Areas.

It is designed to support:
- Auditability
- Source control (Git)
- Backup and restore
- Cross-system transfer
- Offline / disconnected collaboration
- Later consolidation or review

The export format is engine-owned.
Clients (CLI, UI, integrations) must not redefine its structure.

---

## Design Principles

1. Single canonical format
    There is exactly one export format. Import behavior (restore vs consolidate) is a function of the importer, not the file.
2. Full audit preservation
    Sessions, candidates, and lifecycle transitions are preserved even if not required to legitimize decisions.
3. No semantic interpretation
    The engine does not infer meaning from content. All relationships are explicit.
4. Append-only semantics
    Nothing in an export implies deletion or rewriting of history.

---
## Top-Level Structure

```Json
{
  "charter_version": "1.0",
  "exported_at": "2025-12-24T18:42:00Z",
  "engine_metadata": {
    "generator": "charter-core",
    "generator_version": "0.1.0"
  },
  "areas": [ ... ]
}
```
---
## Area Object

Each Area is exported independently but may reference other Areas by ID.

```Json
{
  "area_id": "A-ENG-001",
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
### Notes
- Area identity is stable.
- Area names or descriptions, if any, are represented as resolutions, not mutable fields.

---
## Resolution Object

Resolutions are immutable decision artifacts.


```Json
{
  "resolution_id": "R-ARCH-004",
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
    "referenced_scope_ids": []
  },
  "supersedes": [],
  "superseded_by": null
}
```
### Resolution Types

- `GENERAL`
- `AUTHORITY`
- `SCOPE`

(Types are informational; enforcement comes from engine rules.)

---

## Resolution Lifecycle States


```Text
ACTIVE
UNDER_REVIEW
SUPERSEDED
RETIRED
```

Rules:
- State transitions are explicit and auditable
- No resolution is ever removed
- `UNDER_REVIEW` resolutions may not be accepted as authority or scope

---

## Session Object

Sessions capture how a decision was made, not just what was decided.

```Json
{
  "session_id": "S-ARCH-002",
  "area_id": "A-ENG-001",
  "problem_statement": "Choose database technology",
  "started_at": "2025-02-01T13:00:00Z",
  "closed_at": "2025-02-01T14:32:00Z",
  "state": "CLOSED",
  "authority_resolution_id": "R-AUTH-002",
  "scope_resolution_id": "R-SCOPE-003",
  "referenced_scope_ids": [],
  "candidates": [ ... ],
  "votes": [ ... ],
  "outcome": {
    "accepted_candidate_id": "C-DB-POSTGRES"
  }
}
```

---

## Candidate Object

Candidates are neutral options proposed during a session.

```Json
{
  "candidate_id": "C-DB-POSTGRES",
  "content": {
    "text": "Use PostgreSQL for all production workloads"
  },
  "rationale": "Strong ACID guarantees and team experience",
  "created_at": "2025-02-01T13:10:00Z"
}
```
### Notes
- `rationale` is optional
- Candidates have no authority outside their session

---

### Vote / Acknowledgement Object

Votes are purely mechanical records.

```Json
{
  "actor_id": "alice",
  "candidate_id": "C-DB-POSTGRES",
  "type": "ACCEPT"
  "recorded_at": "2025-02-01T14:00:00Z"
}
```

Voting Types:
- `ACCEPT`
- `REJECT`
- `ABSTAIN`

The engine does not interpret actor identity beyond matching authority rules.

---

## Import Semantics (Informative)

The export file itself does not encode intent.
Intent is provided at import time.

Typical modes:
- RESTORE — recreate full history verbatim
- CONSOLIDATE — import resolutions as `UNDER_REVIEW`

The same export file may be used for both.

---

## Guarantees

This format guarantees:
- Full reconstruction of governance history
- Deterministic rehydration
- No loss of legitimacy context
- Git-friendly diffs
- Long-term forward compatibility

---

## Non-Goals

The export format does not:
- Merge conflicting histories
- Infer semantic equivalence
- Enforce permissions
- Resolve authority disputes

Those actions require explicit sessions after import.

---

## Why This Matters

This export format is what makes Charter Core:
- A governance ledger
- A source-controlled decision system
- A portable engine
- A foundation for CLI, UI, AI, and integrations

Without this format, Charter Core would just be a database.
With it, Charter Core becomes institutional memory.