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
CLI-EXP-01 — Export Ignores Non-Closed Sessions With Warning
When exporting:
CLOSED sessions are exported normally
ACTIVE, PAUSED, or BLOCKED sessions are:
Excluded from the export
Explicitly reported to the user as ignored
The export must still succeed.
The CLI must warn:
Session IDs ignored
Session states
That ignored sessions are not part of the exported history
Fail if:
The CLI silently drops sessions
The CLI exports active sessions as if legitimate
CLI-EXP-02 — No Implicit Session Closure
The CLI must never:
Auto-close sessions
Convert paused or blocked sessions into closed ones
Fabricate legitimacy for convenience
Fail if:
Export causes a state transition
Closure occurs without a decision session
CLI-IMP-01 — Consolidation Is a Singular, Long-Lived Process
When charter import consolidate is invoked:
A baseline review process is created
Other sessions must be at minimum PAUSED
No second consolidation may begin concurrently
Fail if:
Multiple baselines overlap
Consolidation can be interleaved with normal decision sessions
CLI-IMP-02 — Baseline Review Is Persistent
Baseline review state must persist across time:
Accepted resolutions may be reviewed one at a time
Each acceptance may create a local session (solo or multi-user)
Review progress is resumable
Fail if:
Baseline state is ephemeral
Progress is lost between commands
CLI-AUD-01 — Baseline Actions Are Auditable
Baseline review must produce an auditable trail:
Imports attempted
Resolutions accepted or rejected
Review opened and closed
Consolidation completion
Fail if:
Baseline outcomes cannot be reconstructed later


Final Note
These invariants define the behavioral contract of the Charter CLI.
They exist to ensure:
Transparency over speed
Legitimacy over convenience
Determinism over cleverness
Anything not covered here is intentionally left to higher layers.

---
New

CLI-INV-01: Context Is Explicit
The CLI must never guess:
Area
Session
Baseline (review)
Context must be:
explicitly selected, or
explicitly qualified
CLI-INV-02: Context Switching Is Visible
Changing:
area
active session
active baseline review
Must require an explicit command.
CLI-INV-03: Single Active Session (Solo Mode)
In solo mode:
only one active session may exist at a time
new sessions require pause or close
CLI-INV-04: Single Active Baseline Review
Only one baseline (import review) may be active at a time.
CLI-INV-05: Baseline Requires Session Pause
If:
a session is active Then:
baseline start or import must pause it
CLI-INV-06: Baseline Is a Mutable Workspace
Until baseline close:
accept/reject is reversible
no imported resolution is final
On close:
remaining UNDER_REVIEW → ABANDONED
CLI-INV-07: Review Does Not Create Legitimacy
Baseline review:
never evaluates authority
never votes
never blocks sessions directly
All acceptance still occurs via sessions (explicit or hidden).
CLI-INV-08: Batch Operations Are Explicit
Batch accept/reject must:
operate only on the active baseline
require force flags for superseding behavior
CLI-INV-09: Next Actions Are Always Shown
After every state-changing command, the CLI must list:
valid next actions
no illegal or blocked actions
CLI-INV-10: Labels Are Ergonomic Only
Labels:
never replace engine IDs
never encode meaning
are area-scoped by default
CLI-INV-11: Audits Are Read-Only
Audit commands:
never mutate state
never infer correctness
report facts only
CLI-INV-12: Export Ignores Active Sessions
On export:
active or paused sessions are ignored
warning is shown
only closed sessions are exported
CLI-INV-13: Import RESTORE Is Destructive and Explicit
RESTORE:
replaces the Area entirely
closes all sessions and baselines
requires explicit confirmation
CLI-INV-14: Baseline Preview Is Non-Mutating
Preview commands:
perform validation only
create no objects
write no audit entries

---
CLI-STOR-01 — Charter Data Is Never Stored in Working Directories
The CLI must never store authoritative Charter data inside user working directories.
Working directories may contain only:
Non-authoritative context pointers
Metadata required to locate a context
Fail if:
Deleting a project directory deletes Charter history
Charter data is colocated with source files
CLI-STOR-02 — Authoritative Data Lives in a Durable User Scope
All authoritative Charter data must be stored in a durable, user-scoped location (e.g., user profile).
This location must:
Outlive individual projects
Be independent of repository lifecycle
Be suitable for long-term audit retention
Fail if:
Data durability depends on project folders
Moving or deleting folders breaks legitimacy history
CLI-CTX-01 — Context Binding Is Explicit and Observable
A working directory may bind to a Charter context only via an explicit pointer.
The binding must be:
Inspectable (charter context show)
Changeable only via explicit command
Never inferred implicitly
Fail if:
Context is guessed from directory structure
Multiple contexts are implicitly merged
CLI-CTX-02 — One Active Context per Invocation
At any time, a CLI invocation operates against exactly one active Charter context.
Fail if:
Commands act on multiple contexts implicitly
Context ambiguity exists without explicit user intent
CLI-CTX-03 — Context Switching Never Moves or Duplicates Data
Switching contexts must not:
Move Charter data
Copy Charter data
Reinitialize storage
Context switching only changes which storage root the CLI targets.
Fail if:
context use mutates stored history
Data duplication occurs silently
CLI-IMP-01 — Consolidation Is a Long-Lived, Exclusive Process
When a CONSOLIDATE import is initiated:
Exactly one consolidation baseline exists
All normal decision sessions must be paused or blocked
Consolidation state is persisted until closed or aborted
Fail if:
Multiple consolidations run concurrently
Consolidation can be silently abandoned
CLI-IMP-02 — Preview Does Not Mutate State
Any preview or inspection command (e.g., baseline preview) must:
Perform zero mutations
Create no audit records
Leave engine state unchanged
Fail if:
Preview alters history
Preview leaves persistent artifacts
CLI-IMP-03 — Closed Sessions Only Are Imported
On export and import:
Only CLOSED sessions may be included
ACTIVE or PAUSED sessions are ignored with a warning
Rationale:
Prevents private forks of legitimacy
Preserves session integrity
Fail if:
Active sessions are imported
Exporting active sessions enables legitimacy bypass
CLI-UX-01 — Folder Deletion Must Be Safe
Deleting a working directory must never delete Charter history.
At worst, deletion may remove:
A local context pointer
Fail if:
Folder deletion destroys authoritative data
User can accidentally erase governance history
CLI-FWD-01 — Server Mode Compatibility Is Preserved
CLI storage and context behavior must be forward-compatible with a future server or daemon mode.
Fail if:
CLI-only assumptions block shared or remote storage
Context semantics cannot map to multi-user environments

CLI-INV-FLAT-01: Flat Imports Never Create Legitimacy
Flat file imports must never result in accepted or active resolutions without explicit review and acceptance.
Fail if:
Any imported resolution becomes ACTIVE automatically
CLI-INV-FLAT-02: All Flat Imports Enter Baseline Review
Every flat file import must create a baseline review context.
Fail if:
Imported resolutions appear outside a review
Review context is implicit or hidden
CLI-INV-FLAT-03: Foreign Provenance Is Preserved
Imported resolutions must retain clear provenance indicating they originated externally.
Fail if:
Imported resolutions are indistinguishable from local ones
CLI-INV-FLAT-04: Content Matching Is Ergonomic Only
The CLI may detect identical content between imported and local resolutions, but this must not alter legitimacy behavior.
Fail if:
Identical content is auto-accepted
Sessions are skipped due to matching
CLI-INV-FLAT-05: Acceptance Always Produces Sessions
Accepting an imported resolution must correspond to a session (explicit or hidden).
Fail if:
A resolution appears accepted without a session in audit
CLI-INV-FLAT-06: Batch Acceptance Is Explicit
Batch accept or reject commands must:
Operate only within the active baseline review
Require explicit flags (e.g. --all, --unchanged)
Fail if:
Batch operations act implicitly or globally
CLI-INV-FLAT-07: Baseline Review Is Persistent
Baseline reviews may remain open across CLI invocations.
Fail if:
Review state is lost without explicit close
Review is implicitly abandoned

Lock Statement
These invariants are frozen.
Any future feature (server mode, UI, multi-user collaboration, auditing enhancements) must:
Build on these invariants
Extend them explicitly
Never weaken them implicitly
If an implementation choice violates an invariant, the implementation is wrong — not the invariant.