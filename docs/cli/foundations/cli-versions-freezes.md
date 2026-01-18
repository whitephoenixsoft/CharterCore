# Charter CLI — Version Freezes & V3 Command Additions

Status: FROZEN  
Scope: Charter CLI surface (not engine semantics)


## I. Version Freezes

### V1 — Deterministic Solo Governance
Status: FROZEN

V1 is intentionally minimal and linear.

Characteristics:
- Single-user (solo mode)
- One active session at a time
- Explicit authority and scope
- Deterministic acceptance
- No orchestration primitives
- No exploratory workflows

V1 knows about:
- Contexts
- Areas
- Sessions
- Candidates
- Participants (single implicit user)
- Authority sets
- Scope sets
- Baseline review (import + acceptance)
- Audit (linear, factual)

V1 explicitly does NOT know about:
- Deliberate
- Breakouts
- Synthesis
- Draft candidates
- Participant groups
- Bundled discussions
- Multi-session orchestration

V1 mental model:
> “I make decisions, one session at a time, and I can review foreign input safely.”


### V2 — Multi-Participant Governance & Review
Status: FROZEN

V2 expands *who* participates and *how acceptance is evaluated*,
but not *how exploration is structured*.

Characteristics:
- Multiple participants
- Voting semantics
- Baseline review with per-resolution votes
- Participant auditing
- Still fundamentally session-driven

V2 adds:
- Explicit participant add/remove
- Voting commands
- Authority-aware acceptance rules
- Baseline votes per resolution
- Richer audit queries

V2 explicitly does NOT know about:
- Deliberate
- Breakouts
- Synthesis
- Workshop-style workflows
- Session bundling

V2 mental model:
> “We decide together, and the rules of agreement matter.”


### V3 — Deliberative Orchestration & Human Workflow Modeling
Status: FROZEN

V3 is **not** “more commands”.
It is a **new layer above sessions**.

Characteristics:
- Exploration before legitimacy
- Human meeting modeling
- Orchestration of many sessions
- Explicit separation of:
  - thinking
  - converging
  - deciding
- All legitimacy still flows through sessions and baseline review

V3 introduces:
- Deliberate (epic-scoped orchestration)
- Breakouts (write-only exploratory moments)
- Synthesis (structured convergence)
- Draft candidates (non-authoritative)
- Participant groups
- Explicit consolidation boundaries

V3 mental model:
> “We explore freely, converge explicitly, and only then decide.”


## II. Why V3 Is Significantly Different

V1 / V2:
- Treat sessions as the *primary* unit of work
- Assume decisions are the center of activity

V3:
- Treats **exploration as first-class**
- Treats **decisions as a terminal act**
- Models real human workflows:
  - long meetings
  - workshops
  - breakout rooms
  - deferred agreement
  - partial convergence

Key shift:
- Sessions stop being the place where thinking happens
- Sessions become the place where legitimacy is finalized

## III. Explicit Non-Goals (Frozen)

V3 still does NOT include:
- AI facilitation
- Implicit consensus
- Auto-generated acceptance
- Hidden authority inference
- Engine-side awareness of deliberation

These remain future (V4+) concerns.


## IV. Freeze Statement

V1, V2, and V3 CLI semantics are now frozen.

Future versions may:
- add new orchestration layers
- add automation or plugins
- add server-native workflows

They may NOT:
- weaken legitimacy guarantees
- bypass sessions
- collapse audit boundaries
- infer authority implicitly