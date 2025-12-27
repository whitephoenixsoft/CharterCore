
# Charter Core — Canonical Simulations

## Purpose

This document records design-validation simulations for Charter Core.

These are:
- not tests
- not UI flows
- not product promises

They answer one question:

> Does Charter Core preserve legitimacy, determinism, and historical integrity under real pressure?

If any simulation fails, the engine has violated a core invariant.

--- 

Simulation 1 — Area Initialization (Mandatory Bootstrap)
Context
A new Area is created. No decisions are allowed until governance exists.
Actors
Alice
Bob
Flow
Area A-Design-Project is created
System requires an initialization session
Authority candidates are proposed
Scope candidates are proposed
Authority and Scope are accepted
Area becomes initialized
Outcome
One active Authority resolution
One active Scope resolution
All future sessions reference these
Validated Invariants
Areas require explicit governance
Authority and Scope are first-class resolutions
No silent defaults
Simulation 2 — Flat Authority Collaboration (Students)
Context
Two students collaborate with no hierarchy.
Flow
Session opened in initialized Area
Multiple candidates proposed
Both students agree on one candidate
Resolution is accepted
Later sessions supersede it with more detail
Outcome
Legitimate decisions without hierarchy
Clear decision lineage
Validated Invariants
Explicit acceptance
Supersession preserves history
Authority need not be hierarchical
Simulation 3 — Single-User Governance
Context
A solo founder uses Charter Core as a decision journal.
Flow
Founder defines self-authority and scope
Writes multiple candidates
Explicitly accepts one
Months later supersedes it
Outcome
Full audit trail
No shortcuts
Validated Invariants
Scale independence
Explicit acceptance even without oversight
Immutable decision memory
Simulation 4 — Normal Decision Session (Baseline)
Context
A team chooses an architecture.
Flow
Session opened
Candidates proposed
Positions recorded
Authority rule satisfied
Resolution created
Outcome
Resolution references:
Area
Authority
Scope
Validated Invariants
Sessions are the unit of legitimacy
Authority evaluated mechanically
Scope recorded, not enforced
Simulation 5 — Partial Acceptance (Sprint Goals)
Context
Multiple sprint goals are proposed.
Flow
Candidates proposed
Some accepted
Others rejected or left undecided
Outcome
Only accepted candidates become resolutions
No implied decisions
Validated Invariants
Explicit acceptance only
Rejection ≠ resolution
Undecided ≠ failure
Simulation 6 — Deadlock Without Abuse
Context
Unanimous authority, three participants disagree.
Flow
Votes split
Authority rule not satisfied
Session becomes blocked
Outcome
No decision created
No forced closure
Validated Invariants
Deterministic evaluation
No coercion
Blocking is explicit
Simulation 7 — Participant Leaves Mid-Session
Context
A deadlocked session changes composition.
Flow
Alice, Bob, Charlie present
Charlie leaves the session
Authority rule re-evaluated
Remaining votes satisfy rule
Outcome
Resolution accepted legitimately
Departure recorded
Validated Invariants
Authority evaluated on present participants
No reinterpretation of past votes
Simulation 8 — Authority Change Requires a Session
Context
A team wants faster decisions.
Flow
New session opened
New Authority candidate proposed
Old Authority governs the change
New Authority accepted
Old Authority superseded
Outcome
Authority evolves explicitly
History preserved
Validated Invariants
Authority mutable only via resolutions
Non-retroactivity
Simulation 9 — Scope Awareness Without Enforcement
Context
A candidate is clearly outside scope.
Flow
Candidate proposed
Participants recognize mismatch
Candidate rejected manually
Outcome
No mechanical enforcement
Human judgment preserved
Validated Invariants
Scope is informational
Engine does not interpret semantics
Simulation 10 — Referencing Multiple Areas (Awareness Only)
Context
A decision touches multiple domains.
Flow
Session opened in primary Area
Additional Areas’ Scopes explicitly referenced
Acceptance governed only by primary Area Authority
Outcome
Participants are informed
Authority remains singular
Validated Invariants
Primary vs referenced Areas
No authority leakage
Simulation 11 — Late Discovery of Overlap (Hindsight)
Context
Months later, a decision affects another domain.
Flow
Session paused
New Authority or Scope created elsewhere
Original session resumed or closed
Outcome
No retroactive invalidation
Explicit correction
Validated Invariants
Context preservation
Pause vs close distinction
Simulation 12 — Legitimate Pause vs Required Closure
Legitimate Pause
Same problem
Missing clarification
Resume later
Required Closure
Problem definition changes
New session required
Validated Invariants
Sessions represent one problem
Integrity over convenience
Simulation 13 — Anti-Abuse Guardrail
Context
User attempts to bypass legitimacy.
Flow
Session paused
Authority or Scope changed elsewhere
Session resumed blindly
System Response
Context change detected
User must revalidate or close
Validated Invariants
No pause abuse
Determinism preserved
Simulation 14 — Closed Sessions Are Historical Only
Context
A closed session is referenced later.
Flow
Notes reference prior session
No mechanical linkage
Outcome
History informs
Does not govern
Validated Invariants
Sessions are not reusable
Resolutions carry legitimacy
Simulation 15 — Separate Areas, Separate Authorities
Context
Engineering and Finance coexist.
Flow
Independent Areas
Independent decisions
Outcome
No inferred overlap
Validated Invariants
Areas are hard boundaries
No semantic inference
Simulation 16 — Blanket Authority Statement (Ignored Unless Referenced)
Context
High-level authority exists elsewhere.
Flow
Decision proceeds without reference
No conflict raised
Outcome
No guessing
No inference
Validated Invariants
Authority is opt-in
Silence has no meaning
Simulation 17 — Explicit Authority Conflict Resolution
Context
User explicitly references multiple Authorities.
Flow
Engine detects conflict
User must resolve
New Authority supersedes prior ones
Outcome
Conflict resolved explicitly
Validated Invariants
No silent precedence
Intent must be declared
Simulation 18 — Why Streams Are Not Required
Observation
Policy streams would:
Still require human assignment
Still require conflict resolution
Outcome
Charter defers meaning instead of encoding it
Validated Invariants
Minimalism
Explicitness over taxonomy
Simulation 19 — Area Names Are Non-Semantic
Context
Terminology changes, governance does not.
Flow
Area renamed without a session
No Authority or Scope change
Outcome
No re-evaluation
No retroactive impact
Validated Invariants
Names carry no authority
Only resolutions define meaning
Simulation 20 — Fresh Import from Flat Resolution List
Context
An organization adopts Charter for the first time.
They have:
A flat list of existing decisions
No sessions
No candidates
No recorded Authority or Scope
Flow
Import executed in CONSOLIDATE mode
Flat list contains only resolutions
System Behavior
All imported resolutions are created
All are marked UNDER_REVIEW
No Authority or Scope is inferred
No resolution becomes ACTIVE
Outcome
History preserved
Legitimacy explicitly unresolved
Users must validate via sessions
Validated Invariants
Legitimacy is session-bound
Import never fabricates governance
Under-review is explicit, not punitive
Simulation 21 — Concurrent Sessions with Late Supersession
Context
Area: Product Architecture
Active Resolution: R-ARCH-1 (“Monolith”)
Flow
Session S-1 references R-ARCH-1
Session S-2 proposes R-ARCH-2 (“Microservices”)
S-2 accepts R-ARCH-2
System Response
R-ARCH-1 superseded
S-1 flagged for revalidation
Outcome
No silent continuation
No retroactive invalidation
Validated Invariants
Concurrency isolation
Supersession requires revalidation
Legitimacy preserved
Summary
Taken together, these simulations demonstrate that Charter Core:
Preserves legitimacy under disagreement, ambiguity, and change
Requires Authority and Scope to be explicit, never inferred
Allows decisions to evolve without rewriting history
Handles imports conservatively and transparently
Crucially, these properties hold without reliance on:
AI
Turn-taking rules
Mandatory UX flows
Organizational assumptions
These simulations define the behavioral contract of the engine.
If a future change causes one of these scenarios to fail,
it is not a feature regression — it is an invariant violation.
If you want next:
a mapping from simulations → acceptance tests, or
a CLI walkthrough derived from simulations, or
the EVE worst-case simulation rewritten with rejection + participants