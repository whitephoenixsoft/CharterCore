# Charter CLI → Engine Command Map (V3)
*A canonical map of human actions to engine effects*

Status: FROZEN
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

## Legend

- **CLI-only**: Ergonomic or navigational; no engine mutation
- **Engine-affecting**: Creates, mutates, or evaluates engine state
- **Hidden session**: CLI may create a session implicitly, but never skip one
- **Version tags**:
  - V1 — Minimum viable governance
  - V2 — Multi-participant and voting mechanics
  - V3 — Advanced workflows, nested deliberation, breakouts

---

## Context & Workspace Commands

### charter init
**Purpose**  
Bootstrap a new Charter workspace.

**Engine Interaction**  
- Creates empty engine store
- No Areas
- No governance

**Invariants Touched**
- CLI-CTX-01, CLI-STOR-02

**Introduced**  
V1

**Notes & Constraints**
- Must not create default Areas
- Must not infer Authority or Scope
- Workspace intentionally unusable until governance exists

---

### charter context use / switch <name>
**Purpose**  
Switch the active workspace context.

**Engine Interaction**  
CLI-only

**Invariants Touched**
- CLI-CTX-02, CLI-CTX-04

**Introduced**  
V1 (switch shortcut in V3)

**Notes & Constraints**
- Must never copy or migrate data
- Must never auto-create missing contexts

---

### charter context clone <name>
**Purpose**  
Copy context for experimentation

**Engine Interaction**  
CLI-only

**Invariants Touched**
- CLI-CTX-01, CLI-STOR-02

**Introduced**  
V3

**Notes & Constraints**
- Creates new context, no legitimacy
- Useful for sandboxed deliberation

---

## Area Commands

### charter area create
**Purpose**  
Create a new governance boundary.

**Engine Interaction**  
- Creates a new Area entity (UNINITIALIZED)

**Invariants Touched**
- CLI-ID-01, Engine invariant: Areas require explicit Authority/Scope

**Introduced**  
V1

**Notes & Constraints**
- CLI must guide user toward Authority/Scope definition

---

### charter area reference <area>/<label>
**Purpose**  
Reference a resolution from another area.

**Engine Interaction**  
CLI-only (reference mapping for sessions/baselines)

**Invariants Touched**
- CLI-ID-03, CLI-FLAT-03

**Introduced**  
V3

---

## Session Lifecycle Commands

### charter session start
**Purpose**  
Begin deliberation toward a decision.

**Engine Interaction**  
- Creates session
- Declares participants
- Declares authority context

**Invariants Touched**
- CLI-SES-01, CLI-SES-04, Engine session invariants

**Introduced**  
V1

**Notes & Constraints**
- Current user auto-added (default-participant)
- No implied candidates

---

### charter session restart-from <session_id>
**Purpose**  
Restart session while preserving lineage.

**Engine Interaction**  
- Closes source session
- Creates new session

**Invariants Touched**
- CLI-SES-03

**Introduced**  
V1

---

## Authority & Scope Commands

### charter authority set / scope set
**Purpose**  
Define Area governance and decision boundaries.

**Engine Interaction**  
- Creates Authority or Scope resolutions

**Invariants Touched**
- CLI-AUTH-01, Engine invariants

**Introduced**  
V1

---

## Voting & Participation Commands

### charter participant add / remove
**Purpose**  
Explicit participant management

**Engine Interaction**  
- Mutates session participant set

**Invariants Touched**
- CLI-SES-04, CLI-SES-05

**Introduced**  
V2

---

### charter vote <accept|reject|abstain>
**Purpose**  
Explicit voting

**Engine Interaction**  
- Records stance
- Triggers authority evaluation

**Invariants Touched**
- CLI-AUTH-03

**Introduced**  
V2

---

## Deliberate / Breakouts (V3)

### charter deliberate start <epic>
**Purpose**  
Start nested, exploratory multi-session workflow.

**Engine Interaction**  
- CLI-only
- Prepares structure for breakouts/options

**Invariants Touched**
- CLI-DEL-01 through CLI-DEL-10
- CLI-BRK-01 through CLI-BRK-05
- CLI-SYN-01 through CLI-SYN-05

**Notes**
- Pauses any active session or baseline
- Declares Epic for synthesis and option tracking

---

### charter deliberate add-option <description>
**Purpose**  
Add a new option/problem for the current Deliberate

**Engine Interaction**  
- CLI-only until baseline review

**Notes**
- Option initially IN_PROGRESS until finalized

---

### charter breakout start <deliberate-id> [--participants <group>]
**Purpose**  
Start nested exploration

**Engine Interaction**  
- CLI-only, artifacts stored for synthesis

**Notes**
- No authority or acceptance
- Multiple breakouts may exist sequentially (one active at a time)

---

### charter breakout complete <breakout-id>
**Purpose**  
Finish breakout, push artifacts to synthesis

**Engine Interaction**  
- CLI-only; prepares silent synthesis

---

### charter breakout close <breakout-id>
**Purpose**  
Abandon breakout, discard outputs

---

### charter breakout restart-from <breakout-id>
**Purpose**  
Restart breakout while preserving lineage

---

### charter deliberate declare-complete
**Purpose**  
Finalize Deliberate → triggers baseline review

**Engine Interaction**  
- CLI-only; creates foreign baseline-style review

**Notes**
- Only finalized options marked READY enter baseline review

---

### Draft Candidates & Participant Groups
**Purpose**  
Reusable support structures

**Engine Interaction**  
- CLI-only

Commands:
- `charter draft-candidate create|list|show|delete`
- `charter participant-group create|list|show|delete`

---

### Option States
**Purpose**  
Track lifecycle of Deliberate options

**Engine Interaction**  
- CLI-only until baseline review

Commands:
- `charter option list <deliberate-id>` — list options with states
- `charter option set-state <option-id> <state>` — adjust state (READY, IN_PROGRESS, OPEN_ISSUE, DEFERRED)

---

## Baseline / Import / Export

- `charter baseline preview` / `preview-unchanged` / `highlight-changes`
- `charter baseline open|accept|reject|close|purge`
- `charter import consolidate-auto|restore-auto`  
- `charter import` / `charter export`
- `charter deliberate export|import` — share Deliberate artifacts (foreign treatment)

**Engine Interaction**
- Hidden sessions created via accept
- Baseline review enforces authority

---

## Audit & Spec Verification

- `charter audit timeline|participants|by-participant|by-area|by-resolution`
- `charter spec list|show|verify|diff`

**Engine Interaction**
- Read-only
- Deterministic, auditable

---

## Notes / Constraints

- Nested workflows (Deliberate → Breakouts → Options → Baseline Review) are **CLI-level only** until baseline acceptance
- One active Deliberate / Breakout per workspace
- Sessions / Baseline must be paused when Deliberate is active
- Silent synthesis categorizes options automatically; state can be manually adjusted
- Draft candidates and participant groups are global, reusable, CLI-only supports