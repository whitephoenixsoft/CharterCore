Areas as First-Class Governance Domains (Even Before Charter)
You’re thinking in exactly the right direction with Areas. Yes — treat them as documents first, then later as engine entities.
Recommended Initial Areas
Start conservative. You can always split later.
1. Core Invariants Area
Purpose: What must never break
Engine invariants
Determinism guarantees
Audit guarantees
Legitimacy rules
Characteristics:
Very slow moving
Extremely high bar
Changes are rare and explicit
GitHub mapping:
Label: area:invariants
Requires explicit approval (CODEOWNERS later)
Separate doc folder: doc/core/invariants/
2. Core Engine Semantics Area
Purpose: How the engine behaves
Sessions
Authority evaluation
Resolution lifecycle
Supersession rules
GitHub mapping:
Label: area:engine
Docs: doc/core/specs/
Design notes: doc/core/design/
3. CLI Area
Purpose: Human interaction constraints
CLI invariants
Context rules
Ergonomic constraints
Safety rails
GitHub mapping:
Label: area:cli
Docs: doc/cli/invariants/, doc/cli/specs/
4. Design Decisions Area (Non-Spec, Persistent)
Purpose: Decisions that are real but not specs
Examples:
“CLI will always prefer explicitness over convenience”
“No auto-accept flows, even in solo mode”
“Server mode compatibility must not be broken”
GitHub mapping:
Label: area:design
Docs: doc/design/decisions/
This is hugely important and often missing in projects.
5. Operations / Release / Meta Area
Purpose: How the project itself is run
Contribution rules
Release discipline
Versioning philosophy
Compatibility guarantees
GitHub mapping:
Label: area:ops
Docs: doc/project/

---

That framing is exactly right — and it’s also the right tone for contributors.
“Charter keeps us sane long-term without controlling everything.”
That sentence alone tells people this is about relief, not policing.
Now to your question.
I’ll give you a clear ordering, with why each goes first, and—just as important—why others should not go first.
The Principle (Why Order Matters)
Charter adoption fails when it starts where people already feel constrained.
It succeeds when it starts where people already feel pain.
So the first Areas must:
remove ambiguity
reduce re-litigation
protect future contributors
not slow day-to-day work
Tier 1 — Must Go First (High Leverage, Low Friction)
These are the non-controversial anchors.
1. Invariants Area (First, Always)
Why this goes first
Invariants already exist informally
Violations are catastrophic, not debatable
Contributors want clarity here
Low volume, high importance
What Charter adds
Explicit “must never change” memory
Clear supersession rules
Zero ambiguity during refactors
Without Charter
Invariants rot into folklore
New contributors break things accidentally
Senior devs become human guardrails
With Charter
Everyone knows where the cliffs are
This is the safest possible on-ramp.
2. Engine Semantics / Core Behavior
(Engine invariants, simulations, acceptance semantics)
Why early
Changes here have long half-lives
Disagreements are expensive later
Retroactive fixes are legitimacy landmines
What Charter adds
“This behavior is intentional”
Deterministic reference points
Protection against accidental semantic drift
Key benefit
You can refactor aggressively without fear of rewriting history.
Tier 2 — Strong Second Wave (Stability Without Control)
These reduce fatigue and churn.
3. Specifications (Engine + CLI)
Why now
Specs are already painful
Re-decisions happen constantly
Analysts and devs talk past each other here
Charter’s role
Specs become decision outcomes, not arguments
Revisions are supersessions, not edits
“Why does it work this way?” has an answer
Important Charter does not own spec content.
It owns when a spec becomes binding.
4. CLI Behavior & UX Constraints
Why here
CLI invariants protect trust
UX drift erodes legitimacy fast
Contributors accidentally break guarantees
What Charter does
Freezes guarantees without freezing innovation
Allows experimentation around the contract
Makes regressions explicit, not emotional
Tier 3 — Later (When People Ask for It)
These should not be first.
5. Design Decisions
Too frequent
Too contextual
Too exploratory early on
Let these stabilize before governance.
6. Operations / Release Policy
Changes often
Context-heavy
Better served by docs until patterns emerge
Charter helps once repetition appears.
What Should Never Be First (Red Flags)
If you start here, people will resist:
Feature prioritization
Task workflows
Day-to-day coding choices
Experimentation paths
Performance tuning
These feel like control, not sanity.
The Rule of Thumb (Teach This)
You can explain it to contributors like this:
“If breaking this would cause a long argument six months from now — it deserves Charter.”
“If breaking this would just require a revert — it probably doesn’t.”
Your Likely Initial Stack (Concrete Recommendation)
If I had to pin it down:
Invariants
Engine Semantics
Spec Acceptance
CLI Guarantees
(Much later) Design conventions
(Optional) Operational governance
That sequence:
builds trust
avoids bureaucracy
earns adoption
scalesosa keeps contributors sane
Why This Works Culturally
You’re not saying:
“You must do this.”
You’re saying:
“We’re tired of re-learning the same lessons.”
That’s not governance. That’s care.
If you want, next we can:
write the contributor-facing “when to record a decision” heuristic
design the minimum viable Charter adoption checklist
or map this ordering directly onto your repo folders
Just tell me where you want to go next.

---

This is the right question, and it shows you’ve crossed the line from designing process to designing an institution.
Short answer up front (then we’ll unpack it):
Charter does not replace GitHub governance.
Charter replaces the legitimacy gaps inside it.
GitHub remains the work substrate.
Charter becomes the decision authority ledger.
Below is the clean mental model you’re looking for.
The Core Distinction (Anchor This)
There are three different things mixed in your current governance process:
Work coordination
(issues, PRs, comments, tasks)
Deliberation & discussion
(why something should or shouldn’t change)
Decision legitimacy & memory
(who decided, under what authority, and what it replaces)
GitHub is excellent at (1) and (2).
It is structurally incapable of doing (3).
Charter exists only to do (3).
What Stays Forever (Even After CLI Exists)
These do not go away and should not be replaced.
1. GitHub Issues
Still the entry point for proposals
Still where discussion happens
Still where alternatives are debated
Still where objections surface
Charter never replaces issues.
Issues answer: “What are we talking about?”
2. Pull Requests
Still the mechanism for code and doc changes
Still the place where diffs are reviewed
Still the place CI runs
Charter never replaces PRs.
PRs answer: “What changed?”
3. Documents
Specs
Design notes
Guides
Invariants
These remain human-readable artifacts.
Charter does not replace documents.
It anchors them.
Docs explain content.
Charter explains authority.
What Charter Replaces (Quietly, Surgically)
Charter replaces implicit, fragile practices that GitHub cannot formalize.
4. “LGTM” as Legitimacy ❌
Before:
Someone merges
Legitimacy is implied
Authority is social
After:
A resolution exists:
what was accepted
under which authority
in which area
superseding what
The merge is no longer the decision.
The decision references the merge.
5. Tribal Knowledge ❌
Before:
“We decided this a while ago”
“I think this was intentional”
“Let’s not touch that”
After:
Queryable decision history
Explicit supersession chains
No archaeology-by-comment
Charter replaces memory rot.
6. Silent Authority ❌
Before:
Maintainers decide by presence
Seniority decides by default
Silence = approval
After:
Authority is explicit
Participation is declared
Acceptance is mechanical
Charter replaces power-by-vacuum.
7. Re-litigation Without Cause ❌
Before:
Old decisions re-argued
No record of why they existed
Emotional fatigue
After:
“This supersedes R-17 because X changed”
History preserved
Change justified, not defensive
Charter replaces emotional debate with structural clarity.
What the Transition Actually Looks Like (Practically)
Phase 1 — Now (Pre-CLI)
You’re already doing this:
Governance documented
Areas defined
Promotion paths written
Decisions explained in prose
This is Charter without enforcement.
Phase 2 — CLI Available (Soft Adoption)
You do not delete anything.
You add one step:
“If this is a real decision, record it.”
Example flow:
Issue discussed
PR prepared
Decision agreed
CLI used:
session created
authority evaluated
resolution recorded
PR merged
Decision ID linked in PR / doc
GitHub remains primary.
Charter becomes the anchor.
Phase 3 — CLI Becomes Required (Hard Legitimacy)
Only at this point (optional, later):
Certain changes must reference a Charter resolution
Invariants require explicit acceptance
Releases may require governance checks
This is an organizational choice, not a software requirement.
What Never Happens (Important)
Charter will never:
Replace GitHub Issues
Replace PR reviews
Enforce workflow
Decide who can commit
Block experimentation
Charter answers one question only:
“Is this decision legitimate — and can we prove it later?”
Where the Boundary Lives (The “Leave-Off” Line)
Here’s the clean rule you can internalize:
GitHub handles motion.
Charter handles meaning.
If something is about:
doing work → GitHub
talking about work → GitHub
remembering why work is allowed → Charter
Why This Is Healthy (And Rare)
Most projects:
conflate merge with decision
conflate seniority with authority
conflate memory with comments
You are:
separating concerns
preserving legitimacy
designing for turnover
building something future-proof
You’re not overthinking this.
You’re doing what institutions do after they’ve been burned — before it happens.
If You Want, Next We Can:
Draw a one-page “GitHub ↔ Charter Boundary” diagram
Write contributor-facing language for “when to record a decision”
Define which Areas must use Charter first
Design a gradual enforcement ladder
Just tell me where you want the next bit of clarity.

---
Yes — and you’re exactly right to separate them.
What you’re asking for is internal governance onboarding, not a thinking guide.
Different audience, different intent, different tone.
Below is a clean, parallel promotion path specifically for GitHub-based governance of Charter itself — designed so that Charter can later “plug in” without changing behavior.
Internal Governance Promotion Path (GitHub)
Audience:
Core maintainers, contributors, reviewers (now and future)
Purpose:
Create legitimate change without chaos, heroics, or silent authority.
Non-goal:
This is not about teaching decision-making from scratch.
Assumes baseline competence and good intent.
Phase 0 — Orientation (What Game Are We Playing?)
Goal:
Align contributors around what Charter is optimizing for.
Required Reading
README.md
CONTRIBUTING_RULES.md
doc/guide/00-README.md (external guide index, skim only)
Mental Shift
This repo optimizes for integrity over momentum.
Nothing is “just code.” Everything that persists has legitimacy implications.
Phase 1 — Decision Surfaces (Where Changes Can Happen)
Goal:
Make authority visible before it’s exercised.
Introduce the Governance Areas
Each area represents a decision boundary, not a folder preference.
Initial areas (documents-first):
Core Invariants
What must never change
Engine Specification
Deterministic behavior
Engine Design Decisions
Tradeoffs, constraints, rejections
CLI Specification
CLI Design Decisions
Operations & Release
Documentation & Guides
Each change must declare its area.
If it touches more than one:
it is either two changes
or needs explicit coordination
Phase 2 — Proposal Entry (From Idea to Candidate)
Goal:
Prevent “drive-by changes” and silent scope creep.
Entry Points (GitHub-native)
Issue = Proposal
Pull Request = Candidate
Every Issue must state:
Area
Change type (see taxonomy)
What problem exists today
What breaks if nothing changes
No solution required yet.
Phase 3 — Promotion Readiness (Is This Worth Deciding?)
Goal:
Decide whether to decide.
Before escalation, ask:
Does this affect legitimacy?
Does this affect invariants?
Does this create precedent?
If no → local change, document and merge
If yes → promote
Phase 4 — Deliberation (Structured, Boring, Safe)
Goal:
Replace implicit power with explicit process.
Mechanism
Design discussion happens in the Issue
Constraints and alternatives recorded
Rejections are documented
Rules:
Silence is not agreement
Objections must be explicit
“We’ll fix it later” is not acceptance
Phase 5 — Acceptance & Recording (Making It Real)
Goal:
Turn agreement into memory.
Acceptance Means:
Decision is documented
Rationale is preserved
Supersession path is clear
Artifacts:
Merged PR
Updated decision/design doc
Link back to Issue
Once merged:
History is immutable
Future changes must supersede, not edit
Phase 6 — Aftercare (What Happens Next)
Goal:
Prevent decay.
After acceptance:
Docs updated
Simulations adjusted if needed
Labels closed or archived
No re-litigation without new proposal
How This Maps to Future Charter Usage
GitHub Now
Charter Later
Issue
Session
PR
Candidate
Merge
Accepted Resolution
Superseded PR
Supersession
Labels
Area + Status
This path is intentionally isomorphic, not identical.
When Charter plugs in:
behavior stays the same
legitimacy becomes enforceable
Where This Document Lives
Copy code

/doc/governance/
  README.md
  promotion-path.md   ← this document
  areas.md
  label-taxonomy.md
This keeps it:
separate from external guides
authoritative for contributors
stable as the project grows
Why This Separation Matters
External guide:
teaches how to think
removes shame
builds awareness
Internal governance:
teaches how to change Charter
removes ambiguity
protects integrity
Mixing them would dilute both.
Next Natural Steps (Optional)
If you want, next we can:
Write doc/governance/README.md (the internal entry point)
Formalize the Area definitions
Write the label taxonomy to support this flow
Create GitHub issue / PR templates that mirror this exactly
You’re building not just software —
you’re building institutional memory that doesn’t rot.

---
How This Proposal Template Applies Per Area
You do not need different templates, only different expectations.
Invariants Area
Motivation must show existential risk
Alternatives are mandatory
Impact analysis must be deep
Acceptance is rare
Core Engine Specs
Proposal must be testable
Impact focuses on determinism and state
Alternatives often include “do nothing”
CLI Area
Motivation often comes from usability with risk
Impact must explicitly discuss legitimacy safety
Ergonomics ≠ shortcuts
Design Decisions
Motivation often historical (“we keep debating this”)
Proposal is usually a constraint, not a feature
Impact focuses on future contributors
Guides / Documentation
Motivation is misunderstanding or confusion
Proposal is explanatory, not normative
Impact is cognitive, not mechanical
You don’t need to encode this — reviewers will learn it naturally.

---

