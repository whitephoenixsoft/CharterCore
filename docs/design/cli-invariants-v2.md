Charter CLI — Invariants  (Frozen)
Status: FROZEN
Applies to: Charter CLI (all versions)
Does NOT apply to: Charter Core engine semantics
Violations indicate a CLI correctness failure, not an engine bug.
1. Identity & Naming Invariants
CLI-INV-01: Engine Identity Is Canonical
Every Area, Session, Resolution, Candidate, Scope, and Authority has a canonical engine ID.
Rules:
Engine IDs are content-addressed hashes when possible, UUIDs otherwise
CLI-visible labels never replace engine IDs
All CLI operations ultimately resolve to engine IDs
Fail if:
Any CLI command treats a label as authoritative identity
CLI-INV-02: Labels Are Ergonomic, Not Semantic
Labels exist only for human usability.
Rules:
Labels do not encode meaning
Labels do not affect authority, scope, or legitimacy
Labels do not influence engine evaluation
Fail if:
CLI logic interprets label structure or naming conventions
CLI-INV-03: Labels Are Area-Scoped by Default
User-assigned labels must be unique only within an Area.
Rules:
CLI must never assume global label uniqueness
Cross-area operations require explicit qualification
Fail if:
Label collisions across Areas cause ambiguity
CLI-INV-04: Area Context Is Required for Unqualified Labels
If a label is used without area qualification:
An active Area context must exist, or
The command must fail with a disambiguation error
Fail if:
CLI guesses the Area implicitly
CLI-INV-05: Global Disambiguation Is Always Explicit
When listing across Areas, labels must render as: area/label
Fail if:
Bare labels appear in global output
2. Context & Mode Safety
CLI-CTX-01: Context Switching Is Explicit
Area switches, session pauses, baseline entry, and exits must be explicit commands.
Fail if:
CLI silently changes context
CLI-CTX-02: Import, Baseline, and Sessions Are Distinct Mental Modes
The CLI must structurally distinguish:
Normal decision-making
Baseline review / consolidation
Import preview
Fail if:
Imported resolutions feel indistinguishable from local ones
3. Session Invariants
CLI-INV-SESSION-01: Single Active Session (Solo Mode)
In solo mode, only one active session may exist at a time.
Rules:
Starting a new session requires pausing or closing the current one
CLI must guide the user explicitly
Fail if:
Multiple active sessions exist in solo mode
CLI-INV-SESSION-02: Candidate Editing Is Pre-Vote Only
Candidates may be added or removed freely until the first stance is recorded.
Once any stance exists:
Candidates cannot be added
Candidates cannot be removed
Candidates cannot be edited
Fail if:
Candidate mutation occurs after voting begins
CLI-INV-SESSION-03: Restart-From Is Terminal
restart-from:
Automatically closes the source session
Creates a new session with no votes
Records lineage for audit only
Fail if:
Votes or acceptance state carry forward
CLI-INV-SESSION-04: Participant State Is Explicit
Participants are explicit session state.
Rules:
CLI never infers participants
Participant changes require explicit commands
Acceptance snapshots participants immutably
Fail if:
Participants are implied or inferred
CLI-INV-SESSION-05: Resume Requires Participant Revalidation
On session resume, CLI must:
Display prior participants
Display current participants
Require explicit confirmation
Until confirmed:
No votes
No acceptance
No candidate changes
Fail if:
Resume proceeds silently
4. Authority & Constraint Invariants (CLI Layer)
CLI-INV-AUTH-01: Constraints Are Authority-Equivalent
Any rule that changes:
Who must agree
How agreement is evaluated
Is authority-equivalent.
Rules:
Cannot change mid-session
Cannot change on resume
Requires its own session
Fail if:
Constraints mutate without authority governance
CLI-INV-AUTH-02: Constraints Must Be Declared at Session Start
All constraints must be:
Declared before any stance
Visible in session metadata
Immutable for session lifetime
Fail if:
Constraints are added after voting starts
Constraints are implied implicitly
CLI-INV-AUTH-03: CLI Never Creates Consensus
The CLI records decisions; it does not create them.
Fail if:
Running a command implies agreement
Silence is treated as approval
5. Audit Invariants (CLI)
CLI-AUD-01: Audits Are Read-Only
Audit commands must never mutate state.
CLI-AUD-02: Audits Do Not Infer Meaning
Audit output must not:
Judge correctness
Flag violations
Infer intent
CLI-AUD-03: Audits Are Deterministic
Same input → same output.
Ordering, timestamps, or aggregation must be explicit.
CLI-AUD-04: Grep-Friendliness Is Mandatory
Audit output must:
One event per line
Stable field order
Explicit keywords (AREA, SESSION, RESOLUTION, AUTH, SCOPE, BASELINE)
Always include engine IDs
Non-negotiable.
CLI-AUD-05: Participant Audits Are First-Class
CLI must support:
Audit by participant
Participant timelines
Resolution participation views
Fail if:
Participant involvement cannot be reconstructed
6. Baseline (formerly Review) Invariants
CLI-INV-BL-01: Single Active Baseline
At most one active baseline per Area.
Fail if:
Multiple baselines exist concurrently
CLI-INV-BL-02: Consolidation Is Persistent
Baseline progress must persist across restarts.
Fail if:
Progress is lost implicitly
CLI-INV-BL-03: Preview Is Read-Only
baseline preview must:
Create no objects
Emit no audits
Change no state
Fail if:
Preview mutates anything
CLI-INV-BL-04: Baseline Pauses Sessions
Starting a baseline:
Pauses all active sessions
Blocks new sessions
Requires baseline close before resuming
Fail if:
Sessions and baseline interleave
CLI-INV-BL-05: Baseline Accept Always Creates Sessions
Every accepted resolution:
Corresponds to a session
Is auditable
Has explicit acceptance context
Fail if:
Resolution appears without a session
CLI-INV-BL-06: Baseline Is Not a Session
Baseline:
Has no voting
Does not evaluate authority
Does not mutate sessions directly
Fail if:
Baseline behaves like a session
CLI-INV-BL-07: Baseline Close Is Terminal
On baseline close:
All remaining UNDER_REVIEW items → ABANDONED
Baseline becomes immutable
Fail if:
Under-review items remain
CLI-INV-BL-08: Restore Import Is Destructive and Exclusive
RESTORE:
Requires explicit confirmation
Terminates all sessions and baselines
Emits global audit
Fail if:
Restore partially mutates state
CLI-INV-BL-09: Baseline Is Fully Auditable
Baseline lifecycle events must be auditable:
Start
Accept
Reject
Close
Fail if:
Progress cannot be reconstructed
CLI-INV-BL-10: No Implicit Carryover Between Baselines
Each baseline is independent.
Fail if:
Prior outcomes influence new baselines implicitly
7. Ergonomics & Transparency
CLI-ERG-01: Explicitness Beats Convenience
CLI may reduce keystrokes but must never:
Skip legitimacy checks
Infer intent
Create decisions by default
CLI-ERG-02: Ergonomic Collapse Is Allowed
Vote + accept may collapse in solo authority, but full engine history must still exist.
Fail if:
Mechanical history is elided
CLI-ERG-03: Next Actions Are Always Shown
After any command, CLI must show valid next actions.
Fail if:
User is left without guidance

Final Note
These invariants define the behavioral contract of the Charter CLI.
They exist to ensure:
Transparency over speed
Legitimacy over convenience
Determinism over cleverness
Anything not covered here is intentionally left to higher layers.
