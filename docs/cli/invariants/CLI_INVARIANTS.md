# Charter CLI — Invariants
Status: FROZEN  
Applies to: Charter CLI (all versions)  
Does NOT apply to: Charter Core engine semantics

Violations indicate a CLI correctness failure, not an engine bug.

---

## Purpose

These invariants define the behavioral contract of the Charter CLI.

They ensure that human-facing ergonomics never compromise:

- legitimacy
- determinism
- auditability

If an implementation violates an invariant,
the implementation is wrong — not the invariant.

---

## I. Identity & Naming

### CLI-ID-01 — Engine Identity Is Canonical

Every Area, Session, Resolution, Candidate, Scope, Authority, Participant,
and Deliberate artifact has a canonical engine ID.

Rules:

- Engine IDs are content-addressed hashes when possible, UUIDs otherwise
- CLI-visible labels never replace engine IDs
- All CLI operations ultimately resolve to engine IDs

Fail if:

- any CLI command treats a label as authoritative identity

---

### CLI-ID-02 — Labels Are Ergonomic Only

Labels exist solely to aid human reference.

Rules:

- labels never encode meaning
- labels do not affect authority, scope, or legitimacy
- labels do not influence engine evaluation
- labels are Area-scoped by default

Fail if:

- CLI logic interprets label structure or naming conventions

---

### CLI-ID-03 — Area Context Is Required for Unqualified Labels

If a label is used without Area qualification:

- an active Area context must exist, or
- the command must fail with a disambiguation error

Fail if:

- CLI guesses the Area implicitly

---

### CLI-ID-04 — Global Disambiguation Is Always Explicit

When listing across Areas:

- labels must render as `area/label`

Fail if:

- bare labels appear in global output

---

## II. Context & Mode Safety

### CLI-CTX-01 — Context Is Always Explicit

The CLI must never guess:

- Area
- Session
- Baseline
- Deliberate

Context must be explicitly selected or explicitly qualified.

Fail if:

- any command executes under inferred context

---

### CLI-CTX-02 — Context Switching Is Always Visible

Changing:

- Area
- active session
- active baseline
- active deliberate

requires an explicit command.

Fail if:

- context changes silently

---

### CLI-CTX-03 — One Active Context per Invocation

A CLI invocation operates against exactly one Charter context.

Fail if:

- commands act on multiple contexts implicitly
- context ambiguity exists without explicit user intent

---

### CLI-CTX-04 — Context Switching Never Moves Data

Context switching must not:

- move Charter data
- copy Charter data
- reinitialize storage

Fail if:

- history mutates due to context switching

---

## III. Governance Preconditions

### CLI-AREA-01 — Governance Precondition

No action that can create, affect, or validate legitimacy may occur unless the Area has:

- exactly one active Authority resolution
- exactly one active Scope resolution

Enforced at:

- baseline creation or opening
- session acceptance (explicit or ergonomic)
- import consolidation or restore
- deliberate → baseline handoff

If violated:

- CLI/library must reject the action
- emit an explicit error indicating missing governance

This applies Area-wide.

---

## IV. Session Handling (CLI Layer)

### CLI-SES-01 — Single Active Session (Solo Mode)

In solo mode:

- only one active session may exist
- new sessions require pause or close

Fail if:

- multiple active sessions exist

---

### CLI-SES-02 — Candidate Editing Is Pre-Stance Only

Candidates may be freely edited until the first stance is recorded.

After first stance:

- no add
- no remove
- no edit

Fail if:

- candidate mutation occurs after stances begin

---

### CLI-SES-03 — Restart-From Is Terminal

`restart-from`:

- closes the source session
- creates a new session with no stances
- records lineage for audit only

Fail if:

- votes or acceptance carry forward

---

### CLI-SES-04 — Participant State Is Explicit

Participants are explicit session state.

Rules:

- CLI never infers participants
- participant changes require explicit commands
- acceptance snapshots participants immutably

Fail if:

- participants are implied or inferred

---

### CLI-SES-05 — Resume Requires Participant Revalidation

On resume, CLI must:

- display prior participants
- display current participants
- require explicit confirmation

Until confirmed:

- no stances
- no acceptance
- no candidate changes

Fail if:

- resume proceeds silently

---

## V. Authority & Constraints (CLI Layer)

### CLI-AUTH-01 — Constraints Are Authority-Equivalent

Any rule that changes:

- who must agree
- how agreement is evaluated

is authority-equivalent.

Rules:

- cannot change mid-session
- cannot change on resume
- requires its own decision session

Fail if:

- constraints mutate without authority governance

---

### CLI-AUTH-02 — Constraints Must Be Declared at Session Start

All constraints must be:

- declared before any stance
- visible in session metadata
- immutable for the session lifetime

Fail if:

- constraints are added after stances begin

---

### CLI-AUTH-03 — CLI Never Creates Consensus

The CLI records outcomes; it never creates agreement.

Fail if:

- running a command implies consensus
- silence is treated as approval

---

### CLI-AUTH-04 — Ergonomic Collapse Preserves History

The CLI may collapse steps for ergonomics,
but votes, acceptance, and authority context must still be recorded.

Fail if:

- mechanical history is skipped

---

### CLI-AUTH-05 — CLI Is Honest About Its Limits

The CLI must not claim to know:

- who attended
- who agreed
- what was discussed

Annotations may exist.
Inferences may not.

---

## VI. Baseline Review (Import / Consolidation)

### CLI-BL-01 — Single Active Baseline

At most one baseline may be active per Area.

Fail if:

- multiple baselines exist concurrently

---

### CLI-BL-02 — Baseline Requires Session Pause

If a session is active:

- baseline start or import must pause it

Fail if:

- sessions and baseline interleave

---

### CLI-BL-03 — Baseline Is a Mutable Review Workspace

Until baseline close:

- accept / reject decisions are reversible

On close:

- remaining UNDER_REVIEW → ABANDONED
- baseline becomes immutable history

Fail if:

- outcomes finalize implicitly

---

### CLI-BL-04 — Baseline Review Never Creates Legitimacy

Baseline review:

- does not evaluate authority
- does not vote
- does not legitimize outcomes directly

All acceptance still occurs via sessions.

Fail if:

- baseline behaves like a session

---

### CLI-BL-05 — Baseline Acceptance Always Produces Sessions

Every accepted proposal:

- corresponds to a session
- is auditable
- has explicit authority context

Fail if:

- a resolution appears without a session

---

### CLI-BL-06 — Preview Is Read-Only

Preview commands must:

- perform validation only
- create no objects
- emit no audits
- mutate no state

Fail if:

- preview leaves artifacts

---

### CLI-BL-07 — Baseline Lifecycle Is Fully Auditable

Audit must reconstruct:

- start
- accept
- reject
- close

Fail if:

- progress cannot be reconstructed

---

### CLI-BL-08 — No Implicit Carryover Between Baselines

Each baseline is independent.

Fail if:

- prior baseline affects a new baseline implicitly

---

## VII. Flat File Import Rules

### CLI-FLAT-01 — Flat Imports Never Create Legitimacy

Flat imports must never result in accepted resolutions
without explicit review and acceptance.

Fail if:

- imported resolutions become ACTIVE automatically

---

### CLI-FLAT-02 — All Flat Imports Enter Baseline Review

Every flat file import must create a baseline.

Fail if:

- imported resolutions bypass review

---

### CLI-FLAT-03 — Foreign Provenance Is Preserved

Imported resolutions must retain clear external provenance.

Fail if:

- imported and local resolutions are indistinguishable

---

### CLI-FLAT-04 — Content Matching Is Ergonomic Only

Identical content detection:

- must not alter legitimacy behavior
- must not skip sessions

Fail if:

- identical content auto-accepts

---

### CLI-FLAT-05 — Batch Operations Are Explicit

Batch accept / reject must:

- operate only within the active baseline
- require explicit flags

Fail if:

- batch operations act implicitly

---

## VIII. Export & Restore Safety

### CLI-EXP-01 — Export Ignores Non-Closed Sessions With Warning

On export:

- CLOSED sessions are exported
- ACTIVE / PAUSED sessions are ignored with warnings

Fail if:

- sessions are silently dropped
- active sessions are exported as legitimate

---

### CLI-EXP-02 — Export Never Causes State Transitions

Export must never:

- auto-close sessions
- convert paused or blocked state

Fail if:

- export causes mutation

---

### CLI-IMP-01 — RESTORE Is Destructive and Explicit

RESTORE:

- replaces the Area entirely
- closes all sessions and baselines
- requires explicit confirmation
- emits a global audit

Fail if:

- restore partially mutates state

---

## IX. Audit Guarantees (CLI)

### CLI-AUD-01 — Audit Is Read-Only

Audit commands:

- never mutate state
- never infer correctness

---

### CLI-AUD-02 — Audit Is Deterministic

Same input → same output.

Ordering and aggregation must be explicit.

---

### CLI-AUD-03 — Audit Is Grep-Friendly by Design

Audit output must:

- one event per line
- stable field order
- explicit keywords:
  `AREA, SESSION, RESOLUTION, AUTH, SCOPE, BASELINE, DELIBERATE, BREAKOUT`
- always include engine IDs

Non-negotiable.

---

### CLI-AUD-04 — Participant Audits Are First-Class

CLI must support:

- audit by participant
- participant timelines
- resolution participation views

Fail if:

- participant involvement cannot be reconstructed

---

### CLI-AUD-05 — Audit Is the System of Record

Audit is authoritative for:

- what happened
- when it happened
- under what authority
- with which participants

Fail if:

- outcomes are not reconstructible via audit

---

### CLI-AUD-06 — Provenance Is Preserved End-to-End

Audit must preserve lineage between:

- breakouts → synthesis
- synthesis → proposals
- proposals → sessions
- sessions → acceptance

Fail if:

- origin cannot be traced

---

### CLI-AUD-07 — Audit Never Interprets Intent

Audit records facts only.

Fail if:

- audit output implies agreement, consensus, or correctness

---

## X. Storage & Durability

### CLI-STOR-01 — No Authoritative Data in Working Directories

Working directories may contain only:

- context pointers
- metadata

Fail if:

- deleting a folder deletes Charter history

---

### CLI-STOR-02 — Authoritative Data Lives in Durable User Scope

Authoritative data must:

- live in durable, user-scoped storage
- outlive project directories

Fail if:

- durability depends on project folders

---

## XI. Forward Compatibility

### CLI-FWD-01 — Server Compatibility Is Preserved

CLI behavior must remain compatible with:

- server mode
- shared or remote storage

Fail if:

- CLI assumptions block multi-user futures

---

## XII. Deliberate, Breakout & Synthesis (Guided Recording)

### CLI-DEL-01 — Deliberate Records Thinking, Not Decisions

Deliberate exists to capture exploration.

It must never:

- accept
- reject
- activate
- supersede
- evaluate authority

Fail if:

- legitimacy appears without sessions or baseline review

---

### CLI-DEL-02 — Deliberate Artifacts Are Non-Authoritative

Drafts, options, notes, syntheses:

- have no authority
- have no scope
- have no acceptance state

Fail if:

- artifacts behave like candidates or resolutions

---

### CLI-DEL-03 — Deliberate Concludes Explicitly

Every deliberate must end in exactly one of:

- synthesis handed off to baseline review
- explicit abandonment

Fail if:

- artifacts persist indefinitely
- outputs bypass baseline review

---

### CLI-DEL-04 — Participation Never Implies Authority

Participation in deliberation does not imply:

- session participation
- voting rights
- acceptance eligibility

Fail if:

- authority is inferred from attendance

---

### CLI-BRK-01 — Breakouts Capture Exploration Only

Breakouts may:

- explore
- draft
- reframe
- question

They must not:

- vote
- accept
- mutate authoritative engine state

Fail if:

- breakout activity affects legitimacy

---

### CLI-BRK-02 — Breakouts Are Time-Bound Records

Closed breakouts are immutable records.

Further work requires restart-from or a new breakout.

Fail if:

- closed breakout content is edited

---

### CLI-SYN-01 — Synthesis Produces Framed Outcomes

Synthesis outputs:

- options
- questions
- proposals-in-waiting

Fail if:

- synthesis implies decision or authority

---

### CLI-SYN-02 — Synthesis State Is Explicit and Descriptive

Every option must be explicitly categorized:

- READY
- IN_PROGRESS
- OPEN_ISSUE
- DEFERRED

States describe attention, not priority or approval.

Fail if:

- state is inferred or omitted

---

### CLI-DEL-05 — Deliberate Is Fully Auditable

Audit must reconstruct:

- deliberate start
- breakouts
- synthesis evolution
- baseline handoff

Fail if:

- deliberate history cannot be followed end-to-end

---

## Lock Statement

These invariants are frozen.

Future features may extend them explicitly,
but must never weaken them implicitly.