# ENG-SESSION — Session Lifecycle & Governance Enforcement Specification  
Status: FROZEN (v3 – Freeze Boundary & Solo Mode Clarified)  
Applies to: Engine Core (V1/V2+)  
Scope: Session lifecycle, transitions, governance hygiene, and legitimacy boundaries  

---

# 1. Purpose

## ENG-SESSION-01 — Sessions Are the Sole Legitimacy Mechanism

Sessions are the only structure through which legitimacy may be created.

All:

- Resolution acceptance  
- Supersession  
- Authority evolution  
- Scope evolution  

must occur through a valid session.

Fail if:

- A Resolution is created outside a session
- Legitimacy emerges without session acceptance
- Supersession occurs without a session

---

# 2. Identity & Purity

## ENG-SESSION-02 — Engine-Assigned Identity

- Every session has a UUIDv7 `session_id`
- The Engine generates all session IDs
- Identity is stable for the lifetime of the session

Identity must not depend on:

- Storage
- Hashing
- File location
- Import/export mechanics

Fail if:

- Session identity is caller-generated
- Identity changes after creation

---

# 3. State & Phase Model

Sessions have two independent lifecycle dimensions:

## 3.1 SessionState (Governance Lifecycle)

Exactly one of:

- ACTIVE  
- PAUSED  
- BLOCK_TEMPORARY  
- BLOCK_PERMANENT  
- ACCEPTED  
- CLOSED  

No additional states are permitted.

---

## 3.2 SessionPhase (Voting Lifecycle)

Exactly one of:

- PRE_STANCE  
- VOTING  
- TERMINAL  

Phase is distinct from state.

Rules:

- PRE_STANCE → no votes recorded  
- VOTING → at least one vote recorded OR implicit vote inserted during acceptance (Solo Mode)  
- TERMINAL → state is ACCEPTED or CLOSED  

Fail if:

- Phase and state conflict
- TERMINAL exists without ACCEPTED or CLOSED

---

# 4. State Semantics

## 4.1 ACTIVE

- Session is live
- Participants and candidates may be modified only if phase = PRE_STANCE
- Votes may be recorded
- Acceptance may be evaluated

---

## 4.2 PAUSED

- Explicit user pause
- No votes may be recorded
- No acceptance allowed
- Session may resume if valid
- Participants and candidates may be modified only if phase = PRE_STANCE

---

## 4.3 BLOCK_TEMPORARY

- Reversible interruption
- Votes are cleared upon entry
- Session may resume explicitly
- Does not area-block acceptance

Purpose:
Used when legitimacy context becomes uncertain but recoverable.

Participants and candidates remain subject to phase rules.

---

## 4.4 BLOCK_PERMANENT

- Irreversible governance conflict
- Votes are not cleared
- Session progression is frozen
- Acceptance is permanently disallowed

Area Hygiene Rule:

If any session in an Area is BLOCK_PERMANENT:

- No session in that Area may transition to ACCEPTED

The Engine must reject acceptance attempts until all BLOCK_PERMANENT sessions are explicitly CLOSED.

The Engine must not:

- Auto-close sessions
- Auto-resume sessions
- Auto-consolidate sessions

Explicit operator action is required.

---

## 4.5 ACCEPTED

- One or more candidates satisfied deterministic acceptance rules
- Resolution(s) created
- Phase becomes TERMINAL
- Session becomes immutable

Produces legitimacy.

---

## 4.6 CLOSED

- Explicit termination without acceptance
- No Resolution created
- Phase becomes TERMINAL
- Session becomes immutable

Produces no legitimacy.

---

# 5. Valid State Transitions

Only the following transitions are permitted:

| From            | To                | Trigger                          |
|-----------------|-------------------|----------------------------------|
| ACTIVE          | PAUSED            | explicit pause                   |
| PAUSED          | ACTIVE            | explicit resume (valid)          |
| ACTIVE          | BLOCK_TEMPORARY   | invariant-triggered interruption |
| PAUSED          | BLOCK_TEMPORARY   | invariant-triggered interruption |
| BLOCK_TEMPORARY | ACTIVE            | explicit resume                  |
| ACTIVE          | BLOCK_PERMANENT   | invariant-triggered conflict     |
| PAUSED          | BLOCK_PERMANENT   | invariant-triggered conflict     |
| BLOCK_PERMANENT | CLOSED            | explicit closure                 |
| ACTIVE          | ACCEPTED          | deterministic acceptance         |
| ACTIVE          | CLOSED            | explicit closure                 |
| PAUSED          | CLOSED            | explicit closure                 |
| BLOCK_TEMPORARY | CLOSED            | explicit closure                 |

No other transitions are permitted.

Fail if:

- Transition not listed occurs
- Transition happens implicitly
- BLOCK_PERMANENT transitions to ACTIVE
- ACCEPTED transitions to any other state
- CLOSED transitions to any other state

Blocking transitions are engine-detected, never caller-declared.

---

# 6. Authority & Scope Snapshot

## ENG-SESSION-03 — Context Freeze

At session creation:

- Active Authority Resolution is snapshotted
- Active Scope Resolution is snapshotted

These references are immutable for the lifetime of the session.

Fail if:

- Authority changes mid-session
- Scope changes mid-session
- Resume renegotiates governance

---

# 7. Participant & Candidate Freeze Boundary

## ENG-SESSION-04 — PRE_STANCE Is the Only Mutable Boundary

While phase = PRE_STANCE:

- Participants may be added or removed
- Candidates may be added or removed
- Constraints may be modified (until VOTING)

When phase transitions to VOTING:

- Participant set becomes frozen
- Candidate set becomes frozen
- Constraints become frozen

Freeze Trigger:

- The first recorded vote
- OR an implicit vote inserted during acceptance (Solo Mode)

Fail if:

- Participant modified after VOTING begins
- Candidate modified after VOTING begins
- Constraints modified after VOTING begins

Freeze is phase-driven, not state-driven.

---

# 8. Candidate Rules

## ENG-SESSION-05 — Candidate Set Is Structurally Frozen at VOTING

- Candidates may be added or removed only in PRE_STANCE
- No candidate edits permitted after VOTING begins

Fail if:

- Candidate added after VOTING
- Candidate removed after VOTING
- Candidate content mutated at any time

---

# 9. Vote Rules

## ENG-SESSION-06 — Votes Are Mechanical & Immutable

- One vote per participant per candidate
- Votes immutable once recorded
- Silence is not interpreted as consent
- Acceptance is purely mechanical

Votes are cleared when entering BLOCK_TEMPORARY.

Votes are preserved in BLOCK_PERMANENT.

Fail if:

- Engine infers intent
- Engine interprets silence (except Solo Mode implicit vote rule)
- Vote mutated after recording

---

## ENG-SESSION-06A — Solo Mode Implicit Vote Rule

In Solo Governance Mode (V1):

- Exactly one participant exists in the session.

If acceptance is attempted and:

- No vote exists for the candidate

Then:

- The Engine must insert an implicit ACCEPT vote by the sole participant.
- This insertion:
  - Occurs before acceptance evaluation
  - Triggers phase transition PRE_STANCE → VOTING
  - Freezes participants, candidates, and constraints
  - Becomes part of the immutable session record

Solo Mode does not bypass voting semantics.

It is a degenerate case of unanimous governance.

Fail if:

- Acceptance occurs without a vote object existing
- Implicit vote is not persisted
- Solo Mode creates a separate acceptance path

---

# 10. Deterministic Acceptance

## ENG-SESSION-07 — Acceptance Is Pure Evaluation

Acceptance is evaluated solely based on:

- Authority rule
- Constraints
- Recorded votes (explicit or implicit)

Given identical inputs, outcome must be identical.

Fail if:

- Acceptance depends on storage order
- Acceptance depends on runtime timing
- Acceptance depends on external context

---

# 11. Governance Hygiene Enforcement

## ENG-SESSION-08 — Area Blocking Invariant

If any session in an Area has:

state = BLOCK_PERMANENT

Then:

- No session in that Area may transition to ACCEPTED
- Engine must reject acceptance attempts

This enforces explicit cleanup.

The Engine does not:

- Auto-close sessions
- Auto-resolve conflicts
- Downgrade blocks

Manual closure is required.

---

# 12. Immutability Guarantees

## ENG-SESSION-09 — Terminal States Are Immutable

If state ∈ {ACCEPTED, CLOSED}:

- Session is immutable
- No further transitions allowed
- No participants may be modified
- No candidates may be modified
- No votes may be added

Fail if:

- Any mutation occurs after TERMINAL phase

---

# 13. Audit Requirements

## ENG-SESSION-10 — Lifecycle Is Auditable

The Engine must emit audit events for:

- Session creation
- State transitions
- Vote recording (explicit or implicit)
- Acceptance
- Closure
- Block entry

Audit emission must not:

- Alter legitimacy
- Mutate session structure

Audit is descriptive, not generative.

---

# 14. Failure Semantics

## ENG-SESSION-11 — Fail Explicitly

Violations must:

- Fail immediately
- Leave session unchanged
- Produce structured EvaluationReport output

The Engine must not:

- Attempt silent correction
- Attempt automatic recovery
- Modify state implicitly

---

# 15. Summary Guarantees

- Legitimacy is session-scoped  
- Authority and Scope are frozen per session  
- Participant and candidate freeze boundary is phase-driven  
- Solo Mode uses implicit vote insertion, not bypass  
- Acceptance is deterministic  
- Governance hygiene is enforced  
- BLOCK_PERMANENT prevents silent drift  
- No implicit transitions exist  
- Terminal states are immutable  

---

# Mental Model

- Sessions freeze governance context.
- PRE_STANCE is the only mutable boundary.
- The first vote freezes structure.
- Solo Mode inserts the vote instead of skipping it.
- Acceptance is deterministic.
- BLOCK_PERMANENT enforces hygiene.
- Legitimacy is created only at ACCEPTED.
- Nothing changes unless explicitly commanded.