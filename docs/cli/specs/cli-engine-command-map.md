# Charter CLI → Engine Command Map
*A canonical map of human actions to engine effects*

Status: DRAFT (living, authoritative)
Audience: Core maintainers, CLI implementers, future collaborators

This document is the **single source of truth** for how Charter CLI commands interact with the Charter engine.

It exists to:
- Prevent accidental legitimacy creation
- Make engine impact explicit
- Preserve long-term auditability
- Allow staged delivery (V1 → V2 → V3) without confusion

This is **not**:
- A user guide
- A UX walkthrough
- An implementation checklist

It is a **map**, not a recipe.

---

## How to Read This Document

Each command is described across five dimensions:

1. **Purpose** — why the command exists
2. **Engine Interaction** — what it touches (if anything)
3. **Invariants Touched** — CLI or engine rules it must obey
4. **Version Introduction** — when it becomes available
5. **Notes & Constraints** — what it must never do

If a command is listed here, it is intentional.
If a behavior is not listed here, it is not guaranteed.

---

## Legend

- **CLI-only**: Ergonomic or navigational; no engine mutation
- **Engine-affecting**: Creates, mutates, or evaluates engine state
- **Hidden session**: CLI may create a session implicitly, but never skip one
- **Version tags**:
  - V1 — Minimum viable governance
  - V2 — Multi-participant and voting mechanics
  - V3 — Advanced workflows and integrations

---

## Context & Workspace Commands

### charter init
**Purpose**  
Bootstrap a new Charter workspace.

**Engine Interaction**  
- Creates an empty engine store
- Creates no Areas
- Creates no governance

**Invariants Touched**
- CLI-CTX-01 (Context is explicit)
- CLI-STOR-02 (Durable storage)

**Introduced**  
V1

**Notes & Constraints**
- Must not create default Areas
- Must not infer Authority or Scope
- Resulting state is intentionally unusable until governance exists

---

### charter context use <name>
**Purpose**  
Switch the active workspace context.

**Engine Interaction**  
CLI-only.

**Invariants Touched**
- CLI-CTX-02 (Context switching is visible)
- CLI-CTX-04 (No data movement)

**Introduced**  
V1

**Notes & Constraints**
- Must never copy or migrate data
- Must never auto-create missing contexts

---

## Area Commands

### charter area create
**Purpose**  
Create a new governance boundary.

**Engine Interaction**  
- Creates a new Area entity
- Area starts in UNINITIALIZED state

**Invariants Touched**
- Engine invariant: Areas require explicit Authority and Scope
- CLI-ID-01 (Canonical engine IDs)

**Introduced**  
V1

**Notes & Constraints**
- Area cannot host sessions until initialized
- CLI must guide user toward Authority/Scope definition

---

### charter area use <area>
**Purpose**  
Set active Area context.

**Engine Interaction**  
CLI-only.

**Invariants Touched**
- CLI-ID-03 (Area context required for labels)

**Introduced**  
V1

**Notes & Constraints**
- Must fail if area is ambiguous
- Must surface Area status (INITIALIZED / UNINITIALIZED)

---

## Session Lifecycle Commands

### charter session start
**Purpose**  
Begin deliberation toward a decision.

**Engine Interaction**  
- Creates a new session
- Declares participants
- Declares authority context

**Invariants Touched**
- CLI-SES-01 (Single active session in solo mode)
- CLI-SES-04 (Participants explicit)
- Engine session invariants

**Introduced**  
V1

**Notes & Constraints**
- Current user is auto-added as participant
- Additional participants optional (V2+)
- No candidates implied

---

### charter session restart-from <session_id>
**Purpose**  
Abandon a path and restart deliberation cleanly.

**Engine Interaction**  
- Closes source session
- Creates new session with lineage

**Invariants Touched**
- CLI-SES-03 (Restart is terminal)

**Introduced**  
V1

**Notes & Constraints**
- Votes and acceptance never carry forward
- Lineage is audit-only

---

## Authority & Scope Commands

### charter authority set
**Purpose**  
Define or change how decisions are evaluated.

**Engine Interaction**  
- Creates an Authority resolution
- Supersedes prior Authority

**Invariants Touched**
- Engine invariant: Authority is first-class
- CLI-AUTH-01 (Authority equivalence)

**Introduced**  
V1

**Notes & Constraints**
- Requires its own session
- Non-retroactive by definition

---

### charter scope set
**Purpose**  
Define what an Area governs.

**Engine Interaction**  
- Creates a Scope resolution

**Invariants Touched**
- Engine scope invariants

**Introduced**  
V1

**Notes & Constraints**
- Scope is descriptive, not permissive
- Does not grant authority

---

## Voting & Participation (Future)

### charter participant add / remove
**Purpose**  
Explicitly manage session participants.

**Engine Interaction**  
- Mutates session participant set

**Invariants Touched**
- CLI-SES-04 (Participants explicit)
- CLI-SES-05 (Resume revalidation)

**Introduced**  
V2

**Notes & Constraints**
- Removal may unblock authority
- Changes are auditable events

---

### charter vote <accept|reject|abstain>
**Purpose**  
Record explicit stance.

**Engine Interaction**  
- Records stance
- Triggers authority evaluation

**Invariants Touched**
- Engine authority invariants
- CLI-AUTH-03 (CLI never creates consensus)

**Introduced**  
V2

**Notes & Constraints**
- Abstention is first-class
- No implicit votes

---

## Baseline (Import / Consolidation)

### charter import consolidate <file>
**Purpose**  
Introduce external history for review.

**Engine Interaction**  
- Creates baseline
- Imports resolutions as UNDER_REVIEW

**Invariants Touched**
- CLI-BL-01 through CLI-BL-08
- Flat import invariants

**Introduced**  
V1

**Notes & Constraints**
- Must pause active sessions
- Creates no legitimacy

---

### charter baseline accept
**Purpose**  
Accept imported resolutions deliberately.

**Engine Interaction**  
- Creates hidden sessions
- Applies authority rules

**Invariants Touched**
- CLI-BL-05 (Acceptance always via sessions)

**Introduced**  
V1 (solo), V2 (multi-user)

**Notes & Constraints**
- Batch requires explicit flags
- No auto-accept

---

## Audit & Introspection

### charter audit timeline
**Purpose**  
Expose immutable history.

**Engine Interaction**  
Read-only.

**Invariants Touched**
- CLI-AUD-01 through CLI-AUD-04

**Introduced**  
V1

**Notes & Constraints**
- Deterministic output
- Grep-friendly by design

---

## Spec Verification

### charter spec verify
**Purpose**  
Verify engine self-consistency.

**Engine Interaction**  
- Evaluates embedded specs
- No state mutation

**Invariants Touched**
- Engine spec invariants
- CLI-AUD-01 (read-only)

**Introduced**  
V1

**Notes & Constraints**
- Failure blocks destructive operations
- Identity verification only, not trust enforcement

---

## Closing Notes

This map is deliberately conservative.

If:
- a command feels convenient but unclear
- a behavior feels implicit
- a shortcut feels tempting

Assume it is wrong until proven otherwise.

Charter optimizes for **legitimacy over speed**, **clarity over comfort**, and **memory over momentum**.