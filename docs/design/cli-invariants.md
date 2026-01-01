# CLI Invariants — Naming, Identity, Audit, and Ergonomics

These are CLI-layer invariants only.
Violations do not imply engine bugs, but CLI correctness failures.

## Identity & Naming

### CLI-INV-01: Engine Identity Is Canonical

Every Area, Session, Resolution, and Scope has a canonical **engine ID***.

Engine IDs are:
- Content-addressed hashes when possible
- UUIDs otherwise
Rules:
- CLI-visible labels NEVER replace engine IDs
- All engine operations ultimately resolve to engine IDs
#### Fail if
- Any CLI operation relies on a label as authoritative identity

### CLI-INV-02: Labels Are Ergonomic, Not Semantic

Labels exist only for human usability.

Rules:
- Labels do not encode meaning
- Labels do not affect legitimacy
- Labels do not affect authority or scope
#### Fail if
- CLI logic interprets label structure or naming patterns

### CLI-INV-03: Labels Are Area-Scoped by Default

User-assigned labels (e.g. R-1, Auth-Core, Scope-Infra) must be unique **within an Area**.

Rules:
- CLI must never assume global label uniqueness
- Cross-area operations require explicit qualification
#### Fail if
- A label collision across Areas causes ambiguity or hidden coupling

#### CLI-INV-04: Area Context Is Required for Unqualified Labels

If a command references a label without an Area qualifier:
- The CLI must have an active Area context, OR
- The command must fail with a disambiguation prompt

#### Pass examples
```Bash
charter area use platform
charter resolution show R-2
```
#### Fail example
```Bash
charter resolution show R-2
error: label ambiguous without area context
```

### CLI-INV-05: Global Disambiguation Is Always Explicit

When listing or searching across Areas, labels MUST be rendered as:

`<area>/<label>`

#### Example
```
platform/R-3
finance/R-1
security/R-7
```
#### Fail if
- Bare labels appear in a global view

### CLI-INV-06: Global Label Uniqueness Is Optional and Explicit

The CLI MAY support an optional configuration for globally unique labels.

Rules:
- Must be opt-in
- Must be reversible
- Must be documented as increasing merge/import friction
#### Fail if
- Global uniqueness is silently enforced

---

## Audit Invariants

### CLI-AUD-01: Audits Are Read-Only

No audit command may mutate state.

### CLI-AUD-02: Audits Do Not Infer Meaning

Audit commands must not:
- Judge correctness
- Flag violations
- Infer scope breaches

They may only report **recorded facts**.

### CLI-AUD-03: Audits Are Deterministic

Same input → same output.

No timestamps, randomness, or reordering unless explicitly requested.

### CLI-AUD-04: Grep-Friendliness Is a First-Class Constraint

- Audit output must obey:
    - One event per line
    - Stable field order
- Explicit keywords:
    - AREA
    - SESSION
    - RESOLUTION
    - AUTH
    - SCOPE
    - REVIEW 
- Never wrap lines
- Engine IDs always included

This is **non-negotiable**.

---
## Ergonomics & Authority Awareness

### CLI-ERG-01: Authority-Aware Command Collapsing

When the active Authority rule makes individual voting redundant (e.g. SOLO):
- The CLI MAY collapse vote + accept into a single user action
- The engine must still record:
    - Vote(s)
    - Acceptance
    - Authority context
#### Fail if
- Mechanical history is skipped or elided

### CLI-ERG-02: Explicitness Beats Convenience

The CLI may reduce keystrokes, but must never:
- Skip legitimacy checks
- Infer intent
- Apply defaults that create decisions
#### Fail if
- A resolution can be accepted without a conscious user action

---
## Context & Safety

### CLI-CTX-01: Context Switching Is Explicit

Area switches, session pauses, or review mode entry must be explicit commands.
#### Fail if
- The CLI silently changes context
### CLI-CTX-02: Import and Review Are Separate Mental Modes

The CLI must visually and structurally distinguish:
- Normal decision-making
- Import review / consolidation
#### Fail if
- Imported resolutions feel indistinguishable from locally created ones

CLI-INV-SOLO-01: Single Active Session in Solo Mode
In solo mode, the CLI must allow at most one ACTIVE session at a time.
If a session is active:
Starting a new session must fail unless the current session is paused or closed.
The CLI must guide the user to pause or close explicitly.
Fail condition
CLI allows multiple active sessions in solo mode.

CLI-ERG-02: Status Commands Must Suggest Legal Next Actions
Every status output must:
List zero or more valid next actions
Never suggest illegal or blocked actions
Never auto-execute them

CLI-INV-REVIEW-01: Review Actions Require Explicit Import Context
A review accept or reject command must either:
be executed within an active review context, or
explicitly qualify the import ID.
Fail if provenance is ambiguous.

CLI-IO-02: Forced Imports Must Explicitly Mark All Resolutions UNDER_REVIEW

CLI-OUT-01: Human-Readable Output Is Default; Machine Output Requires Flags

CLI-INV-06: Review Is a Mutable Workspace Until Closed
Accepting or rejecting during review is reversible
Closing a review is irreversible
Fail if:
Review decisions are final before review close

CLI-INV-07: Batch Operations Must Be Explicit
Batch accept/reject must operate only on:
current import
current review
Force flags required for superseding behavior
Fail if:
CLI silently supersedes local resolutions

CLI-INV-08: Provenance Is Always Visible in Review
For every imported resolution, the CLI must be able to show:
Import ID
Original area label (if any)
Original session problem statement (read-only)
Superseded local resolution (if applicable)
Fail if:
User cannot tell where a resolution came from

CLI-INV-09: Review Commands Never Mutate Engine Without Sessions
Every accepted resolution must:
correspond to a session (even if hidden)
have an acceptance context
Batch operations may hide sessions, not eliminate them
Fail if:
Resolution appears without a session in audit

CLI-INV-10: Review Is Not a Session
Review does not:
vote
evaluate authority
block or pause sessions
Review only prepares or applies resolutions
Fail if:
Review alters active sessions directly

CLI-INV-11: Next Actions Are Always Shown
After every review command, CLI must show:
Valid next steps (contextual) Examples:
review show
review accept
review reject
review close
Fail if:
User is left without guidance

CLI-INV-12: Single Active Review (Solo Mode)
In solo mode, the CLI must allow at most one active review at a time.
Starting a new review requires closing the current review explicitly.
Fail if
User can open review open I-2 while I-1 is still active

CLI-INV-13: Review Closure Semantics
When charter review close is executed, any imported resolution still in UNDER_REVIEW must be transitioned to ABANDONED.
Properties
ABANDONED resolutions:
are immutable
are queryable in audit
may be re-imported later
do not affect local resolutions
Fail if
UNDER_REVIEW resolutions remain after close
ABANDONED resolutions are treated as rejected or consolidated

CLI-REV-01: Review Acceptance Is Mode-Constrained
In solo mode, the CLI MAY allow direct acceptance or rejection of reviewed resolutions.
In non-solo modes, the CLI MUST require session-mediated acceptance, or forbid acceptance entirely.

---

CLI outside with no solo mode

CLI-INV-V2-01: CLI Never Creates Consensus
The CLI records decisions; it does not create them.
Fail if:
running a command implies agreement
absence of data is treated as approval
CLI-INV-V2-02: Non-Solo Authority Requires Explicit Participants
If Authority ≠ SOLO:
sessions must specify participants
acceptance must validate against them
Fail if:
acceptance occurs without participant grounding
CLI-INV-V2-03: Review Is a Governance Action
Accepting or rejecting imported resolutions:
is subject to Authority
must be auditable
must not bypass sessions
Fail if:
review behaves differently from normal decision-making
CLI-INV-V2-04: CLI Is Honest About Its Limits
The CLI must not pretend to know:
who attended
who agreed
what was discussed
Annotations may exist. Assertions may exist. Inferences may not.
CLI-INV-V2-01: CLI Never Creates Consensus
The CLI records decisions; it does not create them.
Fail if:
running a command implies agreement
absence of data is treated as approval
CLI-INV-V2-02: Non-Solo Authority Requires Explicit Participants
If Authority ≠ SOLO:
sessions must specify participants
acceptance must validate against them
Fail if:
acceptance occurs without participant grounding

CLI-INV-V2-03: Review Is a Governance Action
Accepting or rejecting imported resolutions:
is subject to Authority
must be auditable
must not bypass sessions
Fail if:
review behaves differently from normal decision-making
CLI-INV-V2-04: CLI Is Honest About Its Limits
The CLI must not pretend to know:
who attended
who agreed
what was discussed
Annotations may exist. Assertions may exist. Inferences may not.

CLI-INV-REVIEW-01: Review Singularity
Scope: CLI layer (process invariant, not engine invariant)
Statement
At most one Review may be active per Area at any given time in the CLI.
Definition
A Review is a bounded CLI construct representing the evaluation of an imported decision branch.
A Review is active from charter review start until charter review close.
Rules
The CLI must reject attempts to start a new Review in an Area that already has an active Review.
The CLI must surface the active Review clearly when commands are issued that would conflict with it.
The CLI must guide the user to either:
complete the current Review, or
explicitly close it.
Rationale
Reviews represent focused deliberation, not background tasks.
Allowing concurrent reviews creates implicit parallel legitimacy paths.
Coordination required for multiple reviews exceeds CLI guarantees.
Fail if
Multiple Reviews can be active in the same Area without explicit user awareness.
Imported resolutions from different Reviews are accepted without a clearly scoped evaluation context.
Notes
This invariant may be relaxed in hosted or server-based systems with participant coordination, ownership, and notification guarantees.
The engine remains capable of supporting multiple Reviews; enforcement is purely CLI-level.

CLI-INV-SESSION-RESUME-01: Participant Revalidation
Statement
On session resume, the CLI must explicitly revalidate participants and constraints before allowing votes or acceptance.
Rules
When charter session resume is issued:
CLI must display:
previous participant set
current participant set
CLI must require explicit confirmation:
that the current participants are correct
that the authority rule still applies
that required approvers (if any) are present or knowingly absent
Until confirmed:
no voting
no acceptance
no candidate changes

CLI-INV-PARTICIPANTS-01: Participants Are Explicit State
The CLI must never infer participants
Participant membership changes must be explicit commands
Authority evaluation always uses the declared participant set
Acceptance context must snapshot participants immutably
This applies in:
solo mode
recorded meetings
future multi-user systems

CLI-ERG-02: Session Derivation Is Explicit and Non-Legitimizing
The CLI may create a new session derived from a prior one
The new session:
has a new ID
has no votes
requires fresh acceptance
Lineage is recorded for audit only
Fail if:
Votes or acceptance state are carried forward
The new session auto-accepts anything