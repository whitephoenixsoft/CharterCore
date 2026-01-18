# Charter CLI — Invariants
Status: FROZEN  
Applies to: Charter CLI (all versions)  
Does NOT apply to: Charter Core engine semantics

Violations indicate a CLI correctness failure, not an engine bug.

These invariants define the behavioral contract of the Charter CLI.
They ensure that human-facing ergonomics never compromise legitimacy,
determinism, or auditability.

---

## I. Identity & Naming

### CLI-ID-01 — Engine Identity Is Canonical
Every Area, Session, Resolution, Candidate, Scope, Authority, and Participant
has a canonical engine ID.

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
- Labels must render as area/label

Fail if:
- Bare labels appear in global output

---

## II. Context & Mode Safety

### CLI-CTX-01 — Context Is Explicit
The CLI must never guess:
- Area
- Session
- Baseline (review)

Context must be explicitly selected or explicitly qualified.

Fail if:
- Any command executes under inferred context

---

### CLI-CTX-02 — Context Switching Is Visible
Changing:
- Area
- Active session
- Active baseline

Requires an explicit command.

Fail if:
- Context changes silently

---

### CLI-CTX-03 — One Active Context per Invocation
A CLI invocation operates against exactly one Charter context.

Fail if:
- Commands act on multiple contexts implicitly
- Context ambiguity exists without explicit user intent

---

### CLI-CTX-04 — Context Switching Never Moves Data
Context switching must not:
- Move Charter data
- Copy Charter data
- Reinitialize storage

Fail if:
- History mutates due to context switching

---

## III. Session Handling (CLI Layer)

### CLI-SES-01 — Single Active Session (Solo Mode)
In solo mode:
- Only one active session may exist
- New sessions require pause or close

Fail if:
- Multiple active sessions exist

---

### CLI-SES-02 — Candidate Editing Is Pre-Vote Only
Candidates may be edited freely until the first stance is recorded.

After first stance:
- No add
- No remove
- No edit

Fail if:
- Candidate mutation occurs after voting begins

---

### CLI-SES-03 — Restart-From Is Terminal
restart-from:
- Closes the source session
- Creates a new session with no votes
- Records lineage for audit only

Fail if:
- Votes or acceptance carry forward

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

---

### CLI-AUTH-03 — CLI Never Creates Consensus
The CLI records decisions; it never creates them.

Fail if:
- Running a command implies agreement
- Silence is treated as approval

---

### CLI-AUTH-04 — Ergonomic Collapse Preserves History
The CLI may collapse steps (e.g., vote + accept in solo mode),
but votes, acceptance, and authority context must still be recorded.

Fail if:
- Mechanical history is skipped

---

### CLI-AUTH-05 — CLI Is Honest About Its Limits
The CLI must not claim to know:
- Who attended
- Who agreed
- What was discussed

Annotations may exist. Inferences may not.

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

On close:
- Remaining UNDER_REVIEW → ABANDONED
- Baseline becomes immutable

Fail if:
- Outcomes finalize implicitly

---

### CLI-BL-04 — Baseline Does Not Create Legitimacy
Baseline review:
- Never evaluates authority
- Never votes
- Never creates legitimacy directly

All acceptance still occurs via sessions.

Fail if:
- Baseline behaves like a session

---

### CLI-BL-05 — Baseline Accept Always Creates Sessions
Every accepted resolution:
- Corresponds to a session
- Is auditable
- Has explicit acceptance context

Fail if:
- Resolution appears without a session

---

### CLI-BL-06 — Preview Is Read-Only
Preview commands must:
- Perform validation only
- Create no objects
- Emit no audits
- Mutate no state

Fail if:
- Preview leaves artifacts

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
- Prior baseline affects new baseline implicitly

---

## VI. Flat File Import Rules

### CLI-FLAT-01 — Flat Imports Never Create Legitimacy
Flat imports must never result in accepted decisions
without explicit review and acceptance.

Fail if:
- Imported resolutions become ACTIVE automatically

---

### CLI-FLAT-02 — All Flat Imports Enter Baseline Review
Every flat file import must create a baseline.

Fail if:
- Imported resolutions bypass review

---

### CLI-FLAT-03 — Foreign Provenance Is Preserved
Imported resolutions must retain clear external provenance.

Fail if:
- Imported and local resolutions are indistinguishable

---

### CLI-FLAT-04 — Content Matching Is Ergonomic Only
Identical content detection:
- Must not alter legitimacy behavior
- Must not skip sessions

Fail if:
- Identical content auto-accepts

---

### CLI-FLAT-05 — Batch Operations Are Explicit
Batch accept/reject must:
- Operate only within the active baseline
- Require explicit flags

Fail if:
- Batch operations act implicitly

---

## VII. Export & Import Safety

### CLI-EXP-01 — Export Ignores Non-Closed Sessions With Warning
On export:
- CLOSED sessions are exported
- ACTIVE / PAUSED sessions are ignored with warnings

Fail if:
- Sessions are silently dropped
- Active sessions are exported as legitimate

---

### CLI-EXP-02 — No Implicit Session Closure
Export must never:
- Auto-close sessions
- Convert paused or blocked sessions

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

---

### CLI-AUD-02 — Audits Are Deterministic
Same input → same output.

Ordering and aggregation must be explicit.

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
- Metadata

Fail if:
- Deleting a folder deletes Charter history

---

### CLI-STOR-02 — Authoritative Data Lives in Durable User Scope
Authoritative data must:
- Live in durable, user-scoped storage
- Outlive project directories

Fail if:
- Durability depends on project folders

---

## X. Forward Compatibility

### CLI-FWD-01 — Server Compatibility Is Preserved
CLI behavior must remain compatible with:
- Server mode
- Shared or remote storage

Fail if:
- CLI assumptions block multi-user futures

---
## Deliberate (CLI Layer) — Invariants

These invariants apply to the Deliberate feature and all related CLI affordances
(workshops, breakouts, drafts, synthesis).

Violations indicate a CLI correctness failure, not an engine bug.

### CLI-DEL-01 — Deliberate Creates No Legitimacy
A Deliberate must never:
- accept resolutions
- activate resolutions
- supersede existing resolutions
- evaluate authority

Fail if:
- any Deliberate action results in ACTIVE or ACCEPTED engine state
- legitimacy appears without a baseline review or session

---

### CLI-DEL-02 — Deliberate Artifacts Are Non-Authoritative
Artifacts created during Deliberate (drafts, notes, syntheses):
- have no authority
- have no scope
- have no acceptance state

They are inputs to review only.

Fail if:
- drafts behave like candidates
- syntheses behave like resolutions

---

### CLI-DEL-03 — Deliberate Always Terminates in a Baseline Review
Every Deliberate must end in exactly one of:
- an explicit synthesis that creates a baseline review, or
- explicit abandonment

Fail if:
- Deliberate artifacts can persist indefinitely without closure
- outputs bypass baseline review

---

### CLI-DEL-04 — Participation Does Not Imply Authority
Participation in:
- a Deliberate
- a breakout
- a synthesis

Does not imply:
- session participation
- voting rights
- acceptance eligibility

Fail if:
- deliberation participants are implicitly treated as decision-makers

---

### CLI-DEL-05 — Breakouts Are Isolated and Non-Legitimizing
Breakouts:
- may produce drafts
- may revise drafts
- may merge drafts

They must not:
- accept anything
- modify engine state
- affect sessions directly

Fail if:
- breakout activity mutates authoritative data

---

### CLI-DEL-06 — Synthesis Is Explicit and Auditable
Synthesis of Deliberate output must be:
- an explicit CLI action
- auditable
- reproducible

Fail if:
- synthesis happens implicitly
- synthesis outcome cannot be reconstructed from audit

---

### CLI-DEL-07 — Deliberate Is Time-Independent
Deliberate:
- may pause and resume
- may span arbitrary time

Legitimacy must never depend on:
- duration
- inactivity
- implied consensus over time

Fail if:
- time passage changes outcomes

---

### CLI-DEL-08 — Deliberate Does Not Replace Sessions
Deliberate may:
- prepare
- cluster
- explore

It must not:
- replace sessions
- weaken session authority
- reduce acceptance explicitness

Fail if:
- Deliberate becomes a shortcut to avoid sessions

---

### CLI-DEL-09 — Receipts Exist for All Deliberate Actions
All Deliberate actions must emit audit records:
- open
- join
- breakout creation
- draft creation or revision
- synthesis
- close or abandon

Fail if:
- any deliberation step is unauditable

---

### CLI-DEL-10 — Deliberate Is a CLI-Level Construct Only
The engine:
- is unaware of Deliberate semantics
- stores only resulting imports and sessions

Fail if:
- engine behavior depends on Deliberate metadata

---

## Lock Statement
These invariants are frozen.

If an implementation violates an invariant,
the implementation is wrong — not the invariant.