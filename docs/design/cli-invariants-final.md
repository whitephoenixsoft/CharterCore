# Charter CLI — Invariants (Frozen)

Status: **FROZEN**  
Applies to: **Charter CLI (all versions)**  
Does NOT apply to: **Charter Core engine semantics**

Violations indicate a **CLI correctness failure**, not an engine bug.

These invariants define the **behavioral contract of the Charter CLI**.  
They ensure that human-facing ergonomics never compromise legitimacy, determinism, or auditability.

---

## I. Identity & Naming

### CLI-ID-01 — Engine Identity Is Canonical
Every Area, Session, Resolution, Candidate, Scope, and Authority has a canonical **engine ID**.

Rules:
- Engine IDs are content-addressed hashes when possible, UUIDs otherwise
- CLI-visible labels never replace engine IDs
- All CLI operations ultimately resolve to engine IDs

Fail if:
- Any CLI command treats a label as authoritative identity

---

### CLI-ID-02 — Labels Are Ergonomic Only
Labels exist solely for human usability.

Rules:
- Labels never encode meaning
- Labels do not affect authority, scope, or legitimacy
- Labels do not influence engine evaluation
- Labels are Area-scoped by default

Fail if:
- CLI logic interprets label structure or naming conventions

---

### CLI-ID-03 — Area Context Is Required for Unqualified Labels
If a label is used without Area qualification:
- An active Area context must exist, or
- The command must fail with a disambiguation error

Fail if:
- CLI guesses the Area implicitly

---

### CLI-ID-04 — Global Disambiguation Is Always Explicit
When listing across Areas:
- Labels must render as `area/label`

Fail if:
- Bare labels appear in global output

---

## II. Context & Mode Safety

### CLI-CTX-01 — Context Is Explicit
The CLI must never guess:
- Area
- Session
- Baseline (review)

Context must be:
- Explicitly selected, or
- Explicitly qualified

Fail if:
- Any command executes under an inferred context

---

### CLI-CTX-02 — Context Switching Is Visible
Changing:
- Area
- Active session
- Active baseline review

Requires an explicit command.

Fail if:
- Context changes silently

---

### CLI-CTX-03 — One Active Context per Invocation
At any time, a CLI invocation operates against **exactly one** Charter context.

Fail if:
- Commands act on multiple contexts implicitly
- Context ambiguity exists without explicit user intent

---

### CLI-CTX-04 — Context Switching Never Moves or Duplicates Data
Switching contexts must not:
- Move Charter data
- Copy Charter data
- Reinitialize storage

Fail if:
- Context switching mutates stored history
- Data duplication occurs silently

---

## III. Session Handling (CLI Layer)

### CLI-SES-01 — Single Active Session (Solo Mode)
In solo mode:
- Only one active session may exist
- New sessions require pause or close

Fail if:
- Multiple active sessions exist in solo mode

---

### CLI-SES-02 — Candidate Editing Is Pre-Vote Only
Candidates may be edited freely **until the first stance is recorded**.

After first stance:
- No add
- No remove
- No edit

Fail if:
- Candidate mutation occurs after voting begins

---

### CLI-SES-03 — Restart-From Is Terminal
`restart-from`:
- Closes the source session
- Creates a new session with no votes
- Records lineage for audit only

Fail if:
- Votes or acceptance state carry forward

---

### CLI-SES-04 — Participant State Is Explicit
Participants are explicit session state.

Rules:
- CLI never infers participants
- Participant changes require explicit commands
- Acceptance snapshots participants immutably

Fail if:
- Participants are implied or inferred

---

### CLI-SES-05 — Resume Requires Participant Revalidation
On resume, CLI must:
- Display prior participants
- Display current participants
- Require explicit confirmation

Until confirmed:
- No votes
- No acceptance
- No candidate changes

Fail if:
- Resume proceeds silently

---

## IV. Authority & Constraints (CLI Layer)

### CLI-AUTH-01 — Constraints Are Authority-Equivalent
Any rule that changes:
- Who must agree
- How agreement is evaluated

Is authority-equivalent.

Rules:
- Cannot change mid-session
- Cannot change on resume
- Requires its own decision session

Fail if:
- Constraints mutate without authority governance

---

### CLI-AUTH-02 — Constraints Must Be Declared at Session Start
All constraints must be:
- Declared before any stance
- Visible in session metadata
- Immutable for the session lifetime

Fail if:
- Constraints are added after voting begins
- Constraints are implied implicitly

---

### CLI-AUTH-03 — CLI Never Creates Consensus
The CLI records decisions; it never creates them.

Fail if:
- Running a command implies agreement
- Silence is treated as approval

---

## V. Baseline (Import Review / Consolidation)

### CLI-BL-01 — Single Active Baseline
At most one baseline may be active per Area.

Fail if:
- Multiple baselines exist concurrently

---

### CLI-BL-02 — Baseline Requires Session Pause
If a session is active:
- Baseline start or import must pause it

Fail if:
- Sessions and baseline interleave

---

### CLI-BL-03 — Baseline Is a Mutable Workspace
Until baseline close:
- Accept/reject is reversible
- No imported resolution is final

On close:
- Remaining UNDER_REVIEW → ABANDONED
- Baseline becomes immutable

Fail if:
- Baseline outcomes finalize implicitly

---

### CLI-BL-04 — Baseline Does Not Create Legitimacy
Baseline review:
- Never evaluates authority
- Never votes
- Never creates legitimacy directly

All acceptance still occurs via sessions (explicit or hidden).

Fail if:
- Baseline behaves like a session

---

### CLI-BL-05 — Baseline Accept Always Creates Sessions
Every accepted resolution:
- Corresponds to a session
- Is auditable
- Has explicit acceptance context

Fail if:
- A resolution appears without a session

---

### CLI-BL-06 — Preview Is Read-Only
Preview commands must:
- Perform validation only
- Create no objects
- Emit no audits
- Mutate no state

Fail if:
- Preview leaves persistent artifacts

---

### CLI-BL-07 — Baseline Is Fully Auditable
Baseline lifecycle must be auditable:
- Start
- Accept
- Reject
- Close

Fail if:
- Progress cannot be reconstructed

---

### CLI-BL-08 — No Implicit Carryover Between Baselines
Each baseline is independent.

Fail if:
- Prior baseline outcomes influence new baselines implicitly

---

## VI. Flat File Import Rules

### CLI-FLAT-01 — Flat Imports Never Create Legitimacy
Flat file imports must never result in:
- ACTIVE resolutions
- Accepted decisions

Without explicit review and acceptance.

Fail if:
- Any imported resolution becomes ACTIVE automatically

---

### CLI-FLAT-02 — All Flat Imports Enter Baseline Review
Every flat file import must:
- Create a baseline review context

Fail if:
- Imported resolutions appear outside a review

---

### CLI-FLAT-03 — Foreign Provenance Is Preserved
Imported resolutions must retain clear external provenance.

Fail if:
- Imported and local resolutions are indistinguishable

---

### CLI-FLAT-04 — Content Matching Is Ergonomic Only
The CLI may detect identical content, but:
- Must not alter legitimacy behavior
- Must not skip sessions

Fail if:
- Identical content is auto-accepted

---

### CLI-FLAT-05 — Batch Operations Are Explicit
Batch accept/reject must:
- Operate only within the active baseline
- Require explicit flags (e.g. `--all`, `--unchanged`)

Fail if:
- Batch operations act implicitly or globally

---

## VII. Export & Import Safety

### CLI-EXP-01 — Export Ignores Non-Closed Sessions With Warning
On export:
- CLOSED sessions are exported
- ACTIVE / PAUSED / BLOCKED sessions are ignored
- CLI must warn with IDs and states

Fail if:
- Sessions are silently dropped
- Active sessions are exported as legitimate

---

### CLI-EXP-02 — No Implicit Session Closure
The CLI must never:
- Auto-close sessions
- Convert paused or blocked sessions
- Fabricate legitimacy for convenience

Fail if:
- Export causes state transitions

---

### CLI-IMP-01 — RESTORE Is Destructive and Explicit
RESTORE:
- Replaces the Area entirely
- Closes all sessions and baselines
- Requires explicit confirmation
- Emits a global audit

Fail if:
- Restore partially mutates state

---

## VIII. Audit Guarantees (CLI)

### CLI-AUD-01 — Audits Are Read-Only
Audit commands:
- Never mutate state
- Never infer correctness
- Report facts only

---

### CLI-AUD-02 — Audits Are Deterministic
Same input → same output.

Rules:
- Ordering is explicit
- Aggregation is explicit

---

### CLI-AUD-03 — Grep-Friendliness Is Mandatory
Audit output must:
- One event per line
- Stable field order
- Explicit keywords:
  AREA, SESSION, RESOLUTION, AUTH, SCOPE, BASELINE
- Always include engine IDs

Non-negotiable.

---

### CLI-AUD-04 — Participant Audits Are First-Class
CLI must support:
- Audit by participant
- Participant timelines
- Resolution participation views

Fail if:
- Participant involvement cannot be reconstructed

---

## IX. Storage & Durability

### CLI-STOR-01 — No Authoritative Data in Working Directories
Working directories may contain only:
- Context pointers
- Metadata to locate a context

Fail if:
- Deleting a project directory deletes Charter history

---

### CLI-STOR-02 — Authoritative Data Lives in Durable User Scope
Authoritative Charter data must:
- Live in a durable, user-scoped location
- Outlive project directories
- Support long-term audit retention

Fail if:
- Data durability depends on project folders

---

### CLI-UX-01 — Folder Deletion Must Be Safe
Deleting a working directory must never delete Charter history.

Fail if:
- Governance history can be erased accidentally

---

## X. Forward Compatibility

### CLI-FWD-01 — Server Mode Compatibility Is Preserved
CLI storage and context behavior must be forward-compatible with:
- Server or daemon mode
- Shared or remote storage

Fail if:
- CLI-only assumptions block future multi-user or server deployments

---

## Lock Statement

These invariants are **frozen**.

Any future feature (server mode, UI, collaboration, auditing enhancements) must:
- Build on these invariants
- Extend them explicitly
- Never weaken them implicitly

If an implementation choice violates an invariant, **the implementation is wrong — not the invariant**.