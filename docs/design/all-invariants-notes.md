This is the right next move.
You’re correct: we’ve been letting invariants blur across the boundary, and now that both products are real, we need a clean contract.
Below is a hard separation of invariants into:
Engine Invariants — enforceable, canonical, must hold regardless of client
CLI Invariants — process, ergonomics, safety, and honesty guarantees
I’ll also call out where something looks like an engine concern but is intentionally CLI-only.
Charter Core — Engine Invariants
These invariants define what the engine guarantees and what no client may violate.
The engine is blind to UX, mode, and ergonomics.
ENG-01: Explicit Decisions Only
No resolution may be created, superseded, or retired without:
a Session
explicit acceptance
Authority evaluation
Fail if:
a resolution appears without a session
ENG-02: Sessions Are the Unit of Legitimacy
All legitimacy-bearing actions occur only through Sessions.
Sessions:
define Authority
snapshot Scope
record participants
evaluate acceptance
Fail if:
acceptance occurs outside a session
ENG-03: Authority Is Mechanical and Immutable Per Session
Authority defines:
who has standing
how agreement is evaluated
Rules:
exactly one Authority governs a session
Authority cannot change mid-session
Authority cannot change on resume
Fail if:
evaluation rules change without a new session
ENG-04: Scope Is Informational and Immutable Per Session
Scope:
is recorded
is never interpreted
never enforces acceptance
Scope is snapshotted at session start.
Fail if:
scope affects evaluation logic
ENG-05: Constraints Are Authority-Equivalent
Any rule that changes:
who must agree
how agreement is evaluated
Is an Authority change.
Consequences:
requires its own session
governed by current Authority
cannot change mid-session or on resume
Fail if:
constraints mutate session legitimacy rules
ENG-06: Resume Cannot Introduce New Legitimacy Conditions
On resume:
participants may change
votes may be added
But:
Authority may not change
Constraints may not change
Fail if:
legitimacy rules differ from session start
ENG-07: Candidate Set Freezes on First Stance
Once any stance exists:
no candidate may be added
no candidate may be removed
no candidate may be edited
Fail if:
candidate mutation occurs after voting begins
ENG-08: Immutable History
Accepted resolutions:
are immutable
remain queryable forever
may only be superseded or retired via a new session
Fail if:
history is altered or erased
ENG-09: Non-Retroactivity
Changes to Authority or Scope:
never invalidate
never reinterpret
never downgrade prior resolutions
Fail if:
legitimacy is recalculated retroactively
ENG-10: Imported Resolutions Are Not Local History Until Accepted
Imported resolutions:
may exist
may be audited
do not affect local state
Until accepted via a session.
Fail if:
imports change active legitimacy without acceptance
ENG-11: Restore Is a Full History Replacement
An import restore:
replaces all local history
defines a new canonical root
Fail if:
restore partially merges history
(Engine supports this; CLI controls access.)
ENG-12: Audit Scope Supremacy
All auditable actions must be recorded in an audit scope that outlives the action.
Rules:
deletion never erases audit
global audit scope always exists
Fail if:
an action loses its audit trail


Charter CLI — Invariants
These invariants govern how the CLI behaves, not what the engine allows.
Violations are CLI bugs, not engine bugs.
CLI-01: Context Is Explicit
Area, Session, and Review context must be explicit.
Fail if:
CLI guesses context
state changes silently
CLI-02: Single Active Session (Solo Mode)
In solo mode:
at most one ACTIVE session may exist
Starting another requires:
pause
close
Fail if:
multiple active sessions exist
CLI-03: Session Pause Required Before Review or Import
If a session is active:
review start is blocked
import is blocked
CLI must guide the user.
Fail if:
CLI allows overlapping contexts
CLI-04: Review Is a Distinct Mode
Review:
is not a session
cannot vote
cannot evaluate authority
Fail if:
review mutates legitimacy directly
CLI-05: Single Active Review (Per Area)
Only one review may be active per Area.
Fail if:
multiple reviews overlap
CLI-06: Review Must Close Before Normal Sessions
While a review is active:
only review-related commands are allowed
Fail if:
unrelated sessions start
CLI-07: Import Consolidation Is Non-Destructive
Consolidation:
introduces UNDER_REVIEW artifacts
does not affect active resolutions
Fail if:
consolidation changes legitimacy
CLI-08: Import Restore Is Destructive and Explicit
Restore:
closes sessions
closes reviews
deletes local history
CLI must:
warn
require confirmation
Fail if:
restore is implicit or partial
CLI-09: Restore Invalidates Prior Reviews and Imports
After restore:
all prior reviews are invalid
all consolidation artifacts are discarded
Fail if:
stale review artifacts persist
CLI-10: Provenance Is Always Visible
The CLI must always be able to show:
origin import
original area
original session problem statement (read-only)
Fail if:
user cannot identify source
CLI-11: Ergonomic Collapse Must Preserve Engine History
CLI may collapse steps (e.g. solo accept), But engine must still record:
votes
session
acceptance
Fail if:
mechanical history is skipped
CLI-12: No Silent Legitimacy Change
Any action affecting:
authority
constraints
acceptance conditions
Must be explicit and confirmed.
Fail if:
legitimacy changes implicitly
CLI-13: Next Actions Are Always Shown
After any command, CLI must show:
legal next actions
blocked actions with reasons
Fail if:
user is left without guidance
CLI-14: Review Decisions Are Mutable Until Close
Within review:
accept/reject is reversible
close is final
Fail if:
review decisions finalize early
CLI-15: CLI Never Creates Consensus
The CLI:
records decisions
does not infer agreement
does not assume attendance
Fail if:
silence is treated as consent
Boundary Summary (This Is the Contract)
Engine guarantees legitimacy.
CLI guarantees honesty, safety, and usability.
If a rule affects:
legitimacy → engine
process, flow, safety → CLI
This separation is now clean enough that:
another CLI could be written
a server could replace it
the engine docs stand alone
If you want, next we can:
refactor the README to reflect this split, or
generate acceptance tests grouped by engine vs CLI, or
lock a CLI v1 invariant set and freeze it.