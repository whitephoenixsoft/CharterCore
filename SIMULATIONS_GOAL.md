# Charter — Canonical Simulations (Goal State)

This document records key simulations used to reason about the Charter engine.

These are not tests or user stories.
They are **design validation scenarios** that answer the question:

> “Does this system preserve legitimacy and integrity under realistic pressure?”

These simulations justify the long-term invariants.

---

## Simulation 1 — Collaborative Student Project (No Hierarchy)

**Context**
- Two online college students
- No formal authority hierarchy
- Need to agree on a project direction

**Flow**
1. Create an Area for the course project
2. Define broad scope (project topic, tools, deadlines)
3. Start a session to decide the project theme
4. Propose multiple options
5. Accept one resolution by mutual agreement
6. Later, supersede it with more detailed sub-resolutions

**Validated Invariants**
- Explicit acceptance without hierarchy
- Scope still applies without authority
- Supersession preserves evolution
- Decision memory builds progressively

**Why It Matters**
Charter must work even when authority is flat or informal.

---

## Simulation 2 — Policy Import into an Existing Organization

**Context**
- Company already has policies
- Wants to adopt Charter without rewriting history

**Flow**
1. Import existing policies as resolutions
2. Mark them as “imported”
3. Run a confirmation session to acknowledge them
4. Later, supersede policies via normal sessions

**Validated Invariants**
- Non-retroactive authority
- Decision memory permanence
- Separation between import and acceptance

**Why It Matters**
Charter must integrate into reality, not require a clean slate.

---

## Simulation 3 — Sprint Goal Creation with Partial Acceptance

**Context**
- Product team planning a sprint
- Several proposed sprint goals
- Only some are accepted

**Flow**
1. Start a sprint planning session
2. Propose multiple sprint goal candidates
3. Accept only those within scope and capacity
4. Leave others unaccepted or explicitly rejected
5. Accepted goals become resolutions
6. Rejected goals remain session artifacts

**Validated Invariants**
- Explicit acceptance
- No implicit decisions
- Resolution creation only on acceptance

**Why It Matters**
The system must tolerate ambiguity and partial agreement.

---

## Simulation 4 — Authority Overreach Detection

**Context**
- User proposes a decision outside their authority

**Flow**
1. Start session
2. Propose resolution
3. Attempt acceptance
4. Authority check fails
5. Session blocks with clear reason

**Validated Invariants**
- Authority evaluated at acceptance
- No silent failure
- Overreach is visible

**Why It Matters**
Legitimacy depends on stopping plausible but invalid decisions.

---

## Simulation 5 — Scope Conflict Mid-Session

**Context**
- Session underway
- Another resolution changes scope

**Flow**
1. Session A proposes a resolution
2. Session B changes scope
3. Session A becomes blocked
4. Session A is paused
5. After review, Session A is resumed or abandoned

**Validated Invariants**
- Session blocking
- No continuation under invalid assumptions
- Pause/resume instead of restart

**Why It Matters**
Real decision-making is non-linear.

---

## Simulation 6 — Single-User Governance (Solo Founder)

**Context**
- One-person company
- Wants decision memory and reasoning aid

**Flow**
1. User creates Area
2. Acts as sole authority
3. Uses sessions to reason with AI assistance
4. Accepts resolutions deliberately
5. Revisits and supersedes decisions later

**Validated Invariants**
- Scale independence
- Explicit acceptance even without oversight
- AI as facilitator only

**Why It Matters**
Charter must be useful even with one human.

---

## Simulation 7 — AI Overconfidence

**Context**
- AI strongly recommends a decision
- Human is tempted to accept blindly

**Flow**
1. AI drafts resolution
2. Human reviews
3. Acceptance still requires explicit action
4. Authority and scope still checked

**Validated Invariants**
- AI has no authority
- Legitimacy remains human-owned

**Why It Matters**
This protects against subtle automation bias.


---

Simulation 8 — Manual Governance, AI Disabled (Baseline)

Context

AI orchestration is completely disabled

One or more humans

Charter is used purely as a governance engine


Flow

1. User creates an Area

Defines scope in plain language

Defines authority (self or group)

Selects a resolution template



2. User starts a session

Session is ACTIVE

No AI facilitation is present



3. Users manually add candidates

Resolution text is written directly

Supporting rationale may be pasted from external tools



4. Discussion happens externally or inline

Slack, meetings, documents, or notes

Charter does not orchestrate conversation



5. User explicitly accepts a candidate

Authority is checked

Scope is checked



6. Resolution is created

Immutable

Stored in decision memory




Validated Invariants

Explicit acceptance

Immutability

Decision memory

Scale independence

AI optionality


Why It Matters Charter must provide value as a ledger of legitimacy, not as a thinking engine.


---

Simulation 9 — Slack as the Thinking Space, Charter as the Record

Context

Team collaborates primarily in Slack

Charter is used for long-term memory and legitimacy


Flow

1. Team discusses options in a Slack thread


2. One user starts a Charter session


3. User manually summarizes key options into candidates


4. Team confirms agreement in Slack


5. Authorized user accepts a resolution in Charter


6. Resolution links back to the Slack thread



Validated Invariants

Separation of conversation from commitment

No implicit decisions

Integrations as context only


Why It Matters Charter must fit naturally into existing collaboration habits.


---

Simulation 10 — Single-User Reasoning Without AI

Context

Solo founder or individual contributor

Uses Charter as a decision journal


Flow

1. User creates Area with self-authority


2. User starts a session


3. User writes multiple candidate options manually


4. User reflects and revises text


5. User explicitly accepts one option


6. Months later, user supersedes it



Validated Invariants

Explicit acceptance even without oversight

Supersession preserves evolution

Decision memory over time


Why It Matters Governance is still meaningful without multiple participants.


---

Simulation 11 — UI-Imposed Turns (Engine-Agnostic)

Context

A frontend chooses to enforce turn-taking

Engine itself does not know about “rounds”


Flow

1. UI restricts who can add candidates at a time


2. Engine receives proposals normally


3. Acceptance still happens explicitly


4. Engine enforces scope and authority



Validated Invariants

Engine neutrality

UI freedom

No coupling of behavior to core semantics


Why It Matters Different cultures need different interaction styles.

---

## Summary

These simulations collectively validate that:
- Charter preserves legitimacy under ambiguity
- Authority and scope are never implicit
- Decisions evolve without rewriting history

Even without AI, without rounds, and without enforced UX patterns:

> Charter still preserves legitimacy, intent, and institutional memory.

If these simulations fail, Charter has lost its core value. If a future change breaks one of these simulations, it likely violates a core invariant.
