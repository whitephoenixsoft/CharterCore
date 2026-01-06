Charter Core — Canonical Simulations
(Re-run with Updated Engine + CLI Semantics)
Purpose
These simulations validate that Charter Core preserves:
legitimacy
determinism
audit integrity
cognitive safety
under realistic and stressful conditions.
They are not tests, not UX flows, and not guarantees of ergonomics.
If any simulation fails, an invariant has been violated.
Simulation 1 — Area Initialization (Mandatory Bootstrap)
Context
A new Area cannot function until governance exists.
Flow
charter init
Area A-1 created
Area status: UNINITIALIZED
No sessions except Authority/Scope allowed
Authority definition:
Session started: “Define Authority”
Participants explicitly declared
Authority candidate proposed and accepted
Scope definition:
Session started: “Define Scope”
Participants explicitly declared
Scope candidate proposed and accepted
Outcome
Area becomes INITIALIZED
Exactly one active Authority resolution
Exactly one active Scope resolution
Validated Invariants
Areas require explicit governance
Authority and Scope are first-class resolutions
No implicit defaults
Participants are never inferred
Simulation 2 — Flat Authority Collaboration (Peers)
Context
Two peers collaborate without hierarchy.
Authority
UNANIMOUS_PRESENT
Flow
Session started with explicit participants (Alice, Bob)
Multiple candidates proposed
Both participants explicitly accept one candidate
Session accepted
Outcome
Resolution accepted legitimately
Participant set frozen at first stance
Validated Invariants
Explicit unanimity only
Participant presence is mechanical, not assumed
No silent acceptance
Simulation 3 — Single-User Governance (Solo Journal)
Context
A solo founder uses Charter as a decision journal.
Flow
Session started
Candidate proposed
session accept used without explicit voting
Engine Reality
Participant list = [founder]
Stance = ACCEPT
Authority evaluated normally
Outcome
Full audit trail
No shortcuts at the engine layer
Validated Invariants
Scale independence
Explicit acceptance even in solo mode
Ergonomics do not weaken legitimacy
Simulation 4 — Normal Team Decision (Majority)
Context
A team chooses an architecture.
Authority
MAJORITY_PRESENT
Flow
Session started
Participants declared (Alice, Bob, Carol)
Votes:
Alice: ACCEPT
Bob: ACCEPT
Carol: ABSTAIN
Outcome
Majority satisfied
Resolution accepted
Abstention recorded explicitly
Validated Invariants
Abstention is first-class
Majority is computed mechanically
No semantic inference
Simulation 5 — Deadlock Without Coercion
Context
Unanimous authority with disagreement.
Flow
Participants: Alice, Bob, Carol
Votes:
Alice: ACCEPT
Bob: REJECT
Carol: ACCEPT
Attempt to accept fails
Outcome
Session enters BLOCKED
No forced outcome
No auto-closure
Validated Invariants
Deterministic blocking
No coercion
No implicit failure
Simulation 6 — Participant Leaves Mid-Session
Context
Deadlocked unanimous session reflects reality.
Initial State
Alice ACCEPT
Bob REJECT
Carol ACCEPT
Session BLOCKED
Change
Bob is removed from participant list
Engine Response
Authority re-evaluated
Remaining participants: Alice, Carol
Outcome
Resolution accepted legitimately
Validated Invariants
Authority evaluates present participants only
No vote reinterpretation
Participant changes are explicit and auditable
Simulation 7 — Authority Change Requires Its Own Session
Context
Team wants faster decisions.
Flow
New session started under existing Authority
New Authority candidate proposed
Accepted under old Authority
Outcome
Old Authority superseded
New Authority becomes active
No retroactive impact
Validated Invariants
Authority is mutable only via sessions
Non-retroactivity
Governance evolves explicitly
Simulation 8 — Concurrent Sessions (CLI Constraint)
Context
Solo mode limits cognitive load.
Flow
Session A is active
User attempts to start Session B
Outcome
CLI rejects action
Engine unchanged
Validated Invariants
CLI enforces ergonomic limits
Engine remains neutral
One-review / one-session rule respected
Simulation 9 — Baseline (Formerly Review) Initialization
Context
External history is imported for consolidation.
Flow
charter import consolidate legacy.json
Baseline is created
All imported resolutions marked UNDER_REVIEW
All active sessions are paused
Outcome
No local state mutated
No authority inferred
Baseline becomes the sole active review process
Validated Invariants
Consolidation is non-destructive
Import does not legitimize decisions
One baseline at a time
Simulation 10 — Baseline Acceptance (Solo)
Context
Solo user reviews imported history.
Flow
Baseline opened
User accepts resolutions one at a time
Each acceptance creates a hidden session
Authority evaluated locally
Outcome
Accepted resolutions become ACTIVE
Rejected ones remain REJECTED or ABANDONED
Baseline persists until explicitly closed
Validated Invariants
Acceptance always occurs via sessions
Ergonomic batching does not weaken legitimacy
Baseline is a controlled process
Simulation 11 — Baseline Acceptance (Multi-User, Long-Running)
Context
Departments review external decisions over time.
Flow
Baseline opened
Team deliberates resolution-by-resolution
Baseline is paused and resumed
Sessions occur across days or weeks
Outcome
Baseline persists
Partial progress preserved
Audit trail shows deliberation timeline
Validated Invariants
Baselines are durable
Legitimacy is never time-dependent
Audit integrity across pauses
Simulation 12 — Baseline Close
Context
Review process is complete.
Flow
baseline close
Outcome
Remaining UNDER_REVIEW resolutions become ABANDONED
Baseline is immutable
Normal sessions may resume
Validated Invariants
Explicit closure
No silent resolution drops
Clear audit boundary
Simulation 13 — Import Restore (Hard Reset)
Context
User restores a verified export.
Flow
charter import restore backup.json
Engine Behavior
Existing Area replaced
All sessions (active or not) discarded
Global audit records replacement
Outcome
Restored history is authoritative
No merging
No reinterpretation
Validated Invariants
Restore is destructive and explicit
Audit scope supremacy
Deterministic replacement
Simulation 14 — Export with Active Sessions
Context
User exports while sessions are active.
Flow
Export requested
Engine Behavior
Closed sessions included
Active / paused sessions ignored
Warning emitted (CLI)
Outcome
No legitimacy fork
Export remains safe to share
Validated Invariants
Sessions cannot be cloned
Closed sessions are immutable history
Engine protects legitimacy
Simulation 15 — Divergent History Preservation (Audit Edge Case)
Context
Imported history contains deep supersession chains.
Flow
Imported in CONSOLIDATE mode
Original imported resolutions preserved internally
Local accepted resolutions reference imported lineage
Outcome
Local legitimacy is clean
Imported history remains queryable
No history erased
Validated Invariants
Audit completeness over convenience
Imported timelines are preserved, not replayed
Future V2/V3 audit tooling enabled

Simulation 1 — First-Time Bootstrap from Flat File
Context
A team adopts Charter with existing business rules in text form.
Flow
User runs flat file import
Baseline review opens
All resolutions marked UNDER_REVIEW
User accepts all via batch accept
Sessions created behind the scenes
Review closed
Outcome
Area initialized with legitimate resolutions
No inferred authority
Full audit trail exists
Validated
Explicit legitimacy
Flat imports are non-authoritative
Review-first workflow
Simulation 2 — Repeated Consultation with Unchanged Rules
Context
External team sends the same rules again for verification.
Flow
Flat file imported again
CLI detects identical content
CLI labels items as “UNCHANGED”
User batch-accepts unchanged items
Sessions recorded
No content changes occur
Outcome
Legitimacy reaffirmed explicitly
No silent trust of foreign data
Minimal cognitive load
Validated
Ergonomic matching without semantic shortcuts
Explicit reaffirmation over inference
Simulation 3 — Mixed Changes and Stability
Context
Some rules unchanged, some updated.
Flow
Flat file imported
CLI highlights:
Unchanged items
Modified items
User accepts unchanged
User deliberates on modified ones individually
Review remains open across days
Review closed explicitly
Outcome
Only deliberate changes affect governance
Stable rules remain stable
Audit shows exactly what changed and when
Validated
Persistence of review
Granular acceptance
Historical clarity
Simulation 4 — Misuse Attempt (Auto-Accept Expectation)
Context
User expects unchanged imports to auto-activate.
Flow
Flat file imported
User skips review and checks status
CLI Response
All imported resolutions still UNDER_REVIEW
Next actions displayed:
baseline show
baseline accept
baseline close
Outcome
No silent legitimacy
User guided explicitly
Validated
CLI honesty
No convenience over correctness

Closing Note
These simulations intentionally avoid:
UI promises
Permission systems
Role semantics
Semantic validation
They define what must never break, even as CLI ergonomics evolve.
