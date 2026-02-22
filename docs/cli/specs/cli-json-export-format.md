# Charter Canonical Export (CCE) — Specification

Status: FROZEN (Canonical Ledger Format)  
Applies to: Charter CLI storage layer, Charter Core legitimacy engine  
Replaces: “Engine Canonical Export Format” terminology  
Does NOT define: Import intent (RESTORE vs CONSOLIDATE), workflow semantics, or CLI governance discipline  

---

## 1. Purpose

The Charter Canonical Export (CCE) format provides a **complete, immutable, portable ledger snapshot** of governance history for one or more Areas.

CCE is the canonical serialization of:

- Resolutions (legitimacy artifacts)
- Sessions (legitimacy creation records)
- Authority and Scope state
- Structural references and supersession chains

It exists to support:

- Deterministic restore
- Cryptographic verification
- Git-based storage
- Cross-system transfer
- Long-term archival
- Replay-free legitimacy preservation

CCE is a **ledger format**, not a workflow format.

---

## 2. Architectural Position

Charter is layered:

- **Engine** → legitimacy primitive (session math + resolution creation)
- **CLI runtime** → governance runtime (storage, invariants, discipline)
- **CCE** → canonical ledger serialization

CCE preserves engine invariants and structural integrity,  
but does not encode CLI workflow constructs such as:

- Baseline review
- Deliberate workspaces
- Context switching
- Session blocking discipline

CCE records what became legitimate — not how workflow behaved.

---

## 3. Design Principles

### 3.1 Single Canonical Format

There is exactly one CCE format.

Import behavior (RESTORE vs CONSOLIDATE) is determined by the importer, not by the file.

---

### 3.2 Full Legitimacy Preservation

CCE preserves:

- Closed sessions
- All resolutions
- Supersession chains
- Authority and Scope lineage
- Lifecycle transitions
- Informational references

Nothing required to reconstruct legitimacy is omitted.

---

### 3.3 No Semantic Interpretation

CCE encodes structure only.

- Content is opaque.
- Meaning is not inferred.
- Relationships are explicit.
- Authority math is never recomputed during import.

---

### 3.4 Append-Only Ledger Semantics

CCE never implies:

- Deletion
- Mutation
- Rewriting
- History compression

Every resolution and session is preserved as recorded.

---

### 3.5 Deterministic Integrity

Every exported object includes an `object_hash`.

Canonical serialization MUST use:

- UTF-8 encoding
- Stable field ordering
- Explicit nulls where applicable
- No insignificant whitespace

Hashes MUST be deterministic and documented.

---

## 4. Version & Trust Model

CCE includes explicit version domains.

### 4.1 Required Version Fields

Top-level MUST include:

- `export_format_version`
- `charter_version`
- `hash_algorithm_version`
- `spec_set_hash`

### 4.2 Trust Evaluation

An importer MUST evaluate:

1. Export format compatibility  
2. Hash algorithm compatibility  
3. Top-level object hash validity  
4. All nested object hashes  
5. Referential integrity  
6. Spec set compatibility  

If any validation fails:

- RESTORE MUST be refused.
- File MUST be treated as foreign.
- Import MAY proceed only via CONSOLIDATE (baseline review).

Spec mismatch MUST NOT silently reinterpret legitimacy.

History is evaluated under the rules it was created with.

---

## 5. Top-Level Structure

```json
{
  "export_format_version": "1.0",
  "charter_version": "1.2.0",
  "hash_algorithm_version": "sha256-v1",
  "spec_set_hash": "abc123...",
  "exported_at": "2025-12-24T18:42:00Z",
  "object_hash": "hash-of-entire-export",
  "areas": [ ... ]
}
```
Notes:

- `object_hash` covers all nested content.
- `spec_set_hash` is computed over the ordered list of spec IDs + spec hashes.
- Spec text itself is not hashed directly.
- Deterministic ordering is required.

---

## 6. Area Object


```json
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
### Area Rules

- Area identity is immutable.
- Exactly one ACTIVE Authority resolution MUST exist.
- Exactly one ACTIVE Scope resolution MUST exist.
- Authority MUST NOT be in UNDER_REVIEW.
- Scope MAY be in UNDER_REVIEW.
- Area deletion/replacement is governed by import semantics, not export.

---

## 7. Resolution Object

```json
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
### Resolution Types

- GENERAL
- AUTHORITY
- SCOPE

Type does not affect content interpretation.  
Type determines which engine invariants apply.

---

### Resolution Lifecycle States

ACTIVE  
UNDER_REVIEW  
SUPERSEDED  
RETIRED

#### Lifecycle Rules

- No resolution is ever removed.
- Supersession is directional and permanent.
- Authority MUST NOT enter UNDER_REVIEW.
- Scope MAY enter UNDER_REVIEW.
- UNDER_REVIEW does not alter historical legitimacy.
- UNDER_REVIEW suspends governance effect.

---

### UNDER_REVIEW Semantics

UNDER_REVIEW is a governance suspension state.

It:

- Does not revoke legitimacy.
- Does not alter accepted history.
- Suspends operational effect.
- Signals structural movement.

If Scope is UNDER_REVIEW:

- New sessions SHOULD NOT be opened (CLI governance discipline).
- Engine legitimacy math remains unchanged.

If a GENERAL resolution is UNDER_REVIEW:

- Sessions attempting to supersede it may be governance-blocked by the CLI.
- Engine supersession integrity still applies.

CCE records state only.  
Blocking discipline belongs to the runtime.

---

## 8. Session Object

```json
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

### Session Rules

- Only CLOSED sessions may appear in CCE.
- Sessions are never replayed or re-evaluated during import.
- Authority and Scope are fixed at session start.
- Candidate set freezes on first stance.
- Outcome may contain zero or more accepted candidates per authority rule.
- Supersession integrity MUST be preserved.

If a session attempts to supersede a resolution that is no longer ACTIVE,  
restore MUST reject structural inconsistency.

---

## 9. Candidate Object

```json
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
- Candidates are scoped to a session.
- They have no meaning outside that session.
- Content is opaque.

---

## 10. Stance Object

```json
{
  "actor_id": "alice",
  "candidate_id": "C-DB-POSTGRES",
  "type": "ACCEPT",
  "recorded_at": "2025-02-01T14:00:00Z",
  "object_hash": "hash-of-stance"
}
```
Stance types:

- ACCEPT
- REJECT
- ABSTAIN

Presence is derived from recorded stances.

---

## 11. Import Modes (Informative)

CCE does not encode intent.

Importer decides:

- RESTORE → Rehydrate full ledger if trust checks pass.
- CONSOLIDATE → Treat as foreign proposals (Authority and Scope MUST NOT be imported).

Authority and Scope MUST NOT be accepted through consolidation.

---

## 12. Guarantees

CCE guarantees:

- Deterministic rehydration
- Preservation of legitimacy lineage
- Structural integrity verification
- Spec-aware compatibility validation
- Git-friendly diffs
- Long-term archival safety

---

## 13. Non-Goals

CCE does not:

- Merge histories
- Infer semantic equivalence
- Enforce workflow discipline
- Evaluate authority
- Resolve disputes
- Invent governance

All governance requires explicit sessions.

---

## 14. Final Statement

CCE is a governance ledger snapshot.

It records:

- What became legitimate
- Under which authority
- Within which scope
- At what time
- Through which session

It does not decide.  
It does not interpret.  
It does not enforce workflow.

Without CCE, Charter is transient state.  
With CCE, Charter is institutional memory with rules.