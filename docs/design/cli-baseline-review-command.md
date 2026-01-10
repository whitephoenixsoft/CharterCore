This is a good tightening. You’re converging on a baseline = controlled legitimacy intake model, which is exactly what CONSOLIDATE imports actually are in practice. Let’s review and normalize this so it’s internally consistent, ergonomic, and invariant-safe.
I’ll do this in four passes:
Clarify the baseline mental model (engine-true)
Lock the process invariants
Review and refine the command set
Call out any remaining holes / edge cases
1. Baseline Mental Model (Lock This In)
A baseline is:
A temporary, exclusive review state in which imported history is evaluated for legitimacy under the current local authority.
Key consequences:
A CONSOLIDATE import starts a baseline
There can be only one active baseline
While a baseline is active:
No new sessions may begin
Existing sessions must be PAUSED
No authority or scope changes may occur
A baseline is not history
It is a gate before history is allowed to mutate
This is not just UX — it maps directly to legitimacy invariants.
You are right to treat CONSOLIDATE as the start of a process, not a repeatable action.
2. Process Invariants (Implicit → Explicit)
Let’s make these rules crisp and CLI-enforceable.
Baseline Invariants
Single Active Baseline
At most one baseline may exist at a time
Starting a new baseline fails if one is active
Baseline Exclusivity While a baseline is ACTIVE:
No new sessions may be started
All active sessions must be PAUSED
No resumes may occur
No authority or scope sessions may be created
Baseline Is Not History
Imported objects under baseline:
Do not affect authority
Do not affect scope
Do not supersede anything
Until explicitly accepted
Baseline Closure Is Mandatory
A baseline must be explicitly:
CLOSED (completed), or
ABANDONED (rejected / reset)
No silent exit
Multiple Imports Are Not Persisted
CONSOLIDATE import is a single intake
Subsequent imports require baseline closure
These align perfectly with your earlier import / audit acceptance tests.
3. Command Review & Refinement
Let’s go through your list and tighten semantics.
Import Commands (Entry Points)
✅ charter import consolidate <file>
Behavior
Verifies file integrity
Creates a new BASELINE
Imports all objects as UNDER_REVIEW
Pauses all active sessions
Fails if a baseline is already active
Notes
This is a state transition, not just a file load
Should emit a clear warning:
“Baseline review started. All sessions are paused.”
This is correct as-is.
✅ charter import restore <file>
Behavior
Destroys all current Areas, Sessions, Baselines
Rehydrates history verbatim
Emits global audit record
Leaves system with no active baseline
Notes
This bypasses baseline entirely
Explicitly destructive (you already documented this well)
No changes needed.
Baseline Inspection (Non-State-Changing)
✅ charter baseline preview <file> (New — Good Call)
This is the right abstraction.
Behavior
Parses and verifies the file
Displays:
Areas
Resolutions
Authority & Scope timelines
Divergences vs local state (if any)
Does not create a baseline
Does not pause sessions
Does not persist anything
Important
This is read-only
No audit mutation beyond “previewed” (optional global audit)
This command solves:
Safety
Trust
Cognitive load
“What am I about to let in?”
You should definitely keep it.
Baseline Management Commands
✅ charter baseline status
Behavior
Shows:
Baseline state (ACTIVE / CLOSED)
Imported file fingerprint
Count of UNDER_REVIEW items
Paused sessions (IDs)
Authority governing the review
Solid and sufficient.
✅ charter baseline show
Behavior
Displays imported items in review order
Shows for each:
Imported context (authority, scope)
Local evaluation authority
Current review state
Optional flags you might add later:
--only pending
--only authority
--diff
But V2 doesn’t need them.
✅ charter baseline accept [--all] [--force]
This one needs precision.
Base Behavior
Accepts the current item or all items
Acceptance:
Creates new local resolutions
Applies local authority
Preserves imported context as metadata
Flags
--all
Accept everything in order
Still evaluated one-by-one mechanically
--force
Bypasses warnings
Does not bypass authority
Useful for batch flows, not legitimacy shortcuts
This aligns with your “authority supremacy” invariant.
✅ charter baseline reject [--all]
Behavior
Marks item(s) as REJECTED
Does not affect subsequent items
Does not abort the baseline
This preserves non-cascading rejection, which you’ve locked in.
✅ charter baseline close
Behavior
Ends the baseline
Remaining UNDER_REVIEW items become:
REJECTED or ABANDONED (explicitly recorded)
Paused sessions remain paused (user must resume intentionally)
Baseline cannot be reopened
This is correct and important.
4. Remaining Holes / Decisions to Lock (Small)
These are not blockers, but worth deciding explicitly.
5. What Happens to Paused Sessions After Baseline Close?
Current model:
Sessions remain PAUSED
User must explicitly resume or restart-from
👍 This is consistent with:
No silent legitimacy continuation
Human revalidation
I recommend keeping it.
6. Can Users Abandon a Baseline Without Reviewing Everything?
You implicitly allow this via baseline close.
That’s fine as long as:
All remaining items are explicitly marked
Audit records the abandonment
No hole here — just document it.
7. Preview vs Status Naming
Your naming is good:
preview → file-based, read-only
status/show → baseline-based, stateful
No conflict.
Summary (What You’ve Achieved)
You’ve now cleanly separated:
Import (file intake)
Baseline (legitimacy review gate)
History (accepted resolutions)
Sessions (decision-making)
The CLI now:
Prevents silent authority changes
Prevents legitimacy interleaving
Encourages intentional review
Matches engine invariants cleanly
This is a very strong V2 foundation.
If you want, next we can:
Turn this into a CLI invariants section
Or write a baseline simulation doc like the others
Or freeze the V2 command list formally

---

Great questions. You’re now reasoning at the temporal + human layer of the system, not just the mechanical one — and your instincts are right. I’ll answer directly, then give a clear V1 recommendation.
1. Baseline Acceptance Is a Long-Running Workflow (and Must Be Persisted)
You’ve already uncovered the key insight:
Baseline review is not an atomic action — it is a decision program.
What’s actually happening
For each imported resolution:
A new local session must exist (explicit or implicit)
Authority is evaluated
Participants may deliberate
Outcome becomes a local resolution or rejection
This means:
Baseline review cannot be ephemeral
It must survive:
Pauses
Process restarts
Multi-day or multi-week deliberation
Therefore (engine-true implications)
Baseline must have:
A stable ID
A lifecycle:
ACTIVE
PAUSED (optional but useful)
CLOSED
A persisted cursor:
“Which imported item is currently under review?”
A record of:
Which imported items have been:
ACCEPTED
REJECTED
SKIPPED / DEFERRED (optional)
This is not workflow bloat — it’s legitimacy continuity.
You are correct: baseline accept creates sessions over time.
Closing the baseline simply seals the intake boundary.
2. Audit Receipts Are Not Optional — They’re the Payoff
Your instinct here is strong and long-term correct.
Why receipts matter
For inter-team or inter-department handoffs:
“What did you accept?”
“Under whose authority?”
“What was rejected and why?”
“How long did this take?”
“Were there reversals?”
Baseline review is a governance event, not just data entry.
What a “receipt” really is
Not a PDF. Not a report.
It’s a derived audit view, for example:
Baseline ID
Source import fingerprint
Time opened / closed
Authority governing review
For each imported resolution:
Imported ID
Local resolution ID (if accepted)
Final state (ACCEPTED / REJECTED / ABANDONED)
Timestamp
Participants involved (if non-solo)
This can be exposed as:
Copy code
Text
charter baseline audit
charter baseline export-receipt
This is read-only, derived from audit logs — no new engine state.
This is the moment where Charter becomes institutionally legible.
You don’t need it in V1, but designing so it’s possible is important.
3. What Belongs in V1 (Solo-Only) vs Later
Here’s the clean cut I recommend.
✅ V1 (Solo Mode Only — Minimal, Correct, Powerful)
Baseline core behavior
Single active baseline
Baseline is persisted
Baseline accept creates implicit sessions
One imported resolution reviewed at a time
Baseline close seals intake
Commands
charter baseline preview <file>
charter import consolidate <file>
charter baseline status
charter baseline show
charter baseline accept
charter baseline reject
charter baseline close
Constraints
Authority = SOLO only
Implicit background sessions allowed
No participant modeling
No pause/resume command (process pause = just stop working)
Why this is enough
You personally validate:
Import semantics
Review ordering
Authority non-retroactivity
Audit integrity
You prove:
Baseline ≠ history
Acceptance ≠ import
Closure ≠ evaluation
This already delivers massive value.
🚫 Explicitly Out of V1
Do not include yet:
Multi-user baseline deliberation
Explicit baseline pause/resume commands
Progress receipts or reports
Cross-department sharing semantics
Exporting consolidated baselines
These introduce:
Coordination guarantees
Identity semantics
Notification expectations
All of that belongs after the engine proves itself solo.
🟡 V2 / V3 (When You Add Non-Solo Authority)
This is where you add:
Baseline pause / resume
Participant declaration for baseline review
Explicit review sessions (not hidden)
Baseline audit receipts
CLI ergonomics for long-running governance
At that point, baseline becomes a first-class governance object.
The Big Picture (Why This Is the Right Cut)
V1 answers:
“Can a single human safely and honestly integrate external governance history?”
V2 answers:
“Can multiple humans coordinate legitimacy over time?”
Trying to do both at once would blur authority, which you’ve been rigorously avoiding.
You’re sequencing this correctly.
If you want next:
I can write a Baseline Lifecycle Simulation doc
Or a V1 vs V2 feature boundary table
Or help you freeze the V1 scope contract so you stop second-guessing

---

Perfect timing to freeze this. What you’re asking for is not documentation fluff — it’s a self-protection boundary so future insight doesn’t destabilize execution. I’ll give you two frozen artifacts:
Baseline Lifecycle Simulation (Frozen)
CLI Feature Boundary: V1 vs V2 (Frozen Contract)
Both are engine-accurate, CLI-aware, and written so you can point back to them later and say “we already decided this.”
Charter Core — Baseline Lifecycle Simulation (Frozen)
Status: FROZEN
Scope: Engine truth + CLI v1/v2 interaction
Purpose: Sanity-check baseline behavior under realistic human timelines
Simulation — Baseline Review as a Long-Running Legitimacy Program
Context
A user imports external governance history and must decide which decisions to legitimize locally.
This is not a single decision.
It is a sequence of legitimacy acts under one authority context.
Preconditions
Area A-CORE is initialized
Active Authority exists
No active session
No active baseline
Step 1 — Preview (Non-Mutating)
Copy code
Bash
charter baseline preview history.json
Engine behavior
Parses file
Verifies integrity
Reports:
Areas found
Resolution count
Authority / Scope entries
Timeline ordering
No state change
No audit mutation
Validated invariant
Read-only inspection is safe
Import ≠ consolidation
Step 2 — Consolidation Start
Copy code
Bash
charter import consolidate history.json
Engine behavior
Global audit records import attempt
Imported resolutions materialized
All imported resolutions marked UNDER_REVIEW
Baseline review object created
Baseline enters ACTIVE
All sessions are paused (if any existed)
Validated invariants
Single active baseline
Consolidation is stateful
Import does not create legitimacy
Step 3 — Review One Resolution (Solo)
Copy code
Bash
charter baseline show
charter baseline accept
Engine behavior
Implicit session created (hidden)
Authority evaluated (SOLO)
New local resolution created
Imported resolution marked CONSOLIDATED
Baseline remains ACTIVE
Validated invariants
Each acceptance is a session
Authority is not bypassed
Review ≠ session
Step 4 — Partial Progress Over Time
User stops working for the day.
Engine behavior
Baseline remains ACTIVE
Progress is persisted
No timeout
No auto-close
Validated invariants
Governance may be slow
Legitimacy cannot expire
Step 5 — Reject an Imported Resolution
Copy code
Bash
charter baseline reject
Engine behavior
Imported resolution marked REJECTED
No local resolution created
No side effects
Validated invariants
Rejection is explicit
Rejection does not cascade
Step 6 — Close Baseline
Copy code
Bash
charter baseline close
Engine behavior
All remaining UNDER_REVIEW resolutions → ABANDONED
Baseline enters CLOSED
Global audit records closure
Normal sessions may resume
Validated invariants
Closure is terminal
Silence is not acceptance
Audit completeness preserved
Observations (Why This Matters)
Baseline is not a session
Baseline is not history
Baseline is a controlled intake boundary
Closure defines the end of legitimacy opportunity
Charter CLI — Feature Boundary Contract (V1 vs V2)
Status: FROZEN
Audience: You, future contributors, future-you
Purpose: Prevent scope bleed and decision churn
CLI V1 — Solo Legitimacy Tool (Frozen)
Target User
Single human integrating decisions safely.
Authority
SOLO only
Baseline
Exactly one active baseline
Baseline persisted across time
No explicit pause command
No multi-baseline support
Sessions
Single active session (solo invariant)
Implicit sessions allowed for:
baseline accept
No participant modeling
Commands Included
Baseline
Copy code
Bash
charter baseline preview <file>
charter import consolidate <file>
charter baseline status
charter baseline show
charter baseline accept
charter baseline reject
charter baseline close
Import
Copy code
Bash
charter import restore <file>
Explicitly Excluded
Multi-user authority
Participants
Session constraints beyond SOLO
Baseline pause / resume
Audit receipts / reports
Exporting baselines
Cross-team workflows
Design Intent
V1 proves:
Engine correctness
Import legitimacy separation
Audit fidelity
Human-scale usability
If V1 feels “slow” or “manual,” it is correct.
CLI V2 — Multi-Actor Governance (Planned, Not Implemented)
Authority
UNANIMOUS_PRESENT
MAJORITY_PRESENT
Required participants
Constraint declarations
Baseline Enhancements
Explicit baseline pause / resume
Participant declaration for baseline
Baseline-mediated deliberation sessions
Progress visibility for others
Baseline audit receipt generation
Sessions
Explicit participants
Resume revalidation
Constraint enforcement
Restart-from ergonomics
Audit
Participant timelines
Authority changes over time
Governance post-mortems
Non-Goals
Server hosting
Presence detection
Real-time collaboration
Hard Boundary Rule (Frozen)
If a feature requires knowing or coordinating more than one human, it is not V1.
If future-you wants to violate this, it must be:
Explicit
Versioned
Documented as a break
Why This Freeze Matters (Human Note)
You’re not indecisive — you’re discovering invariants faster than implementation.
This freeze lets you keep discovering without stalling execution.
When you feel tempted to “just add one more thing”:
Check which side of the boundary it’s on
Write it down
Keep coding
Charter will grow because of this discipline, not in spite of it.
If you want next:
I can generate a V1 CLI help output
Or a contributor onboarding doc
Or help you define “done” for V1