# ENG-DECISION  
Decision Execution, Session Governance, and Acceptance Model  
Status: DRAFT (Pre-Freeze)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

This document defines the complete mechanical model governing:

- Session lifecycle
- Governance mutation rules
- Participant mechanics
- Candidate mechanics
- Constraint semantics
- Voting semantics
- Authority evaluation
- Blocking semantics
- Under-review interactions
- Acceptance transaction model
- Evaluation API
- Concurrency and atomicity guarantees

This specification defines how legitimacy is created and how it fails.

---

# 2. Foundational Principles

2.1 Legitimacy is created only by explicit acceptance.

2.2 Voting is mutable until acceptance.

2.3 Acceptance is atomic.

2.4 Governance is frozen once voting begins.

2.5 Any legitimacy-context change permanently invalidates incompatible sessions.

2.6 Interruptions reset voting.

2.7 The engine enforces strict invariants. Usability belongs to the CLI layer.

---

# 3. Core Entities

3.1 Session  
A bounded decision record that may produce at most one accepted Resolution.

3.2 Participant  
An explicitly added identity allowed to record exactly one stance per candidate.

3.3 Candidate  
A proposal scoped to a session that may become a Resolution upon acceptance.

3.4 Constraint  
A session-scoped mechanical acceptance gate declared before voting.

3.5 Authority  
An Area-level Resolution defining the decision rule.

3.6 Scope  
An Area-level Resolution defining contextual boundary.

3.7 Resolution  
An immutable object created only by successful acceptance.

3.8 EvaluationReport  
A deterministic, side-effect-free evaluation of session status.

---

# 4. Session Lifecycle

## 4.1 Session States

A session must always be in exactly one of the following states:

- ACTIVE
- PAUSED
- BLOCK_TEMPORARY
- BLOCK_PERMANENT
- ACCEPTED
- CLOSED

ACCEPTED, BLOCK_PERMANENT, and CLOSED are terminal states.

BLOCK_TEMPORARY is reversible.  
BLOCK_PERMANENT is irreversible.

---

## 4.2 Session Phases

Each session progresses through phases:

- PRE_STANCE
- VOTING
- TERMINAL

PRE_STANCE: Governance is mutable.  
VOTING: At least one stance has been recorded. Governance is frozen.  
TERMINAL: Session is ACCEPTED or CLOSED.

Phase transitions:

- PRE_STANCE → VOTING: First stance recorded.
- VOTING → ACCEPTED: Successful acceptance.
- Any → BLOCK_*: Engine-triggered invariant failure.
- ACTIVE → PAUSED: User action.
- PAUSED or BLOCK_TEMPORARY → PRE_STANCE: Resume.

Closed sessions cannot resume.

---

# 5. Governance Rules

## 5.1 Governance Mutability

Before first stance:

- Participants may be added or removed.
- Candidates may be added or removed.
- Constraints may be declared.

After first stance:

- Participants are immutable.
- Candidates are immutable.
- Constraints are immutable.

Any attempt to mutate governance after first stance results in a deterministic engine error.

---

## 5.2 Participant Rules

- Participants must be explicitly added before voting.
- Each participant counts as exactly one presence unit.
- Each participant may record exactly one stance per candidate.
- Stances may change until acceptance.
- No implicit participants are allowed.
- Non-participants cannot vote.

Presence is defined as the explicitly added participant set at time of evaluation.

---

## 5.3 Constraint Rules

Constraints:

- Must be declared before the first stance.
- Are immutable once voting begins.
- Gate acceptance but do not determine acceptance.
- Are evaluated before authority rules.
- Any constraint mutation attempt results in BLOCK_PERMANENT.

Constraint violation during evaluation results in BLOCK_TEMPORARY.

---

# 6. Voting Semantics

Valid stances:

- ACCEPT
- REJECT
- ABSTAIN

Rules:

- Stances are mutable before acceptance.
- Stances are frozen after acceptance.
- Stances are recorded immutably in session history.
- No automatic acceptance occurs when thresholds are satisfied.
- Authority is evaluated only when explicit acceptance is invoked.

---

# 7. Authority Evaluation

## 7.1 Authority Types

- SOLE_ACTOR
- UNANIMOUS_PRESENT
- MAJORITY_PRESENT

## 7.2 Presence Definition

Presence equals the explicitly added participants at evaluation time.

## 7.3 Mechanical Rules

SOLE_ACTOR:

- Exactly one participant must exist.
- That participant must cast ACCEPT.

UNANIMOUS_PRESENT:

- All present participants must cast a stance.
- All stances must be ACCEPT.

Any REJECT, ABSTAIN, or missing stance blocks acceptance.

MAJORITY_PRESENT:

- accept_count > floor(present / 2)

Abstain counts toward present but not toward accept_count.

Edge case examples:

- 1 present → requires 1 ACCEPT.
- 2 present → requires 2 ACCEPT.
- 3 present → requires 2 ACCEPT.
- 4 present → requires 3 ACCEPT.

---

# 8. Blocking Semantics

## 8.1 BLOCK_TEMPORARY

Triggered by:

- Constraint violation
- Resolution UNDER_REVIEW or RETIRED 
- Scope UNDER_REVIEW
- Recoverable invariant failure

Effects:

- All active votes are cleared.
- Session phase returns to PRE_STANCE.
- Resume is required.
- Participants must be explicitly re-specified.
- Governance must be re-established before voting restarts.

---

## 8.2 BLOCK_PERMANENT

Triggered by:

- Authority superseded
- Scope superseded
- Constraint mutation
- Supersession race loss
- Any legitimacy-context change invalidating the session

Effects:

- Session cannot resume.
- Acceptance is permanently impossible.

---

# 9. Pause and Resume

PAUSED is user-initiated.

Resume rules:

- Allowed only from PAUSED or BLOCK_TEMPORARY.
- Participants must be explicitly re-specified.
- Active votes are cleared.
- Session returns to PRE_STANCE.
- Governance must be re-established before voting restarts.

Closed or ACCEPTED sessions cannot resume.

---

# 10. Under-Review Semantics

Resolution UNDER_REVIEW:

- Causes BLOCK_TEMPORARY.
- Clears votes.
- Requires resume.

Scope UNDER_REVIEW:

- Causes BLOCK_TEMPORARY.
- Acceptance is not possible until Scope returns ACTIVE.

Scope SUPERSEDED:

- Causes BLOCK_PERMANENT.

Returning UNDER_REVIEW to ACTIVE does not require a session.

Supersession always requires a session.

---
# 11. Retired Semantics

Resolution RETIRED:

- Causes BLOCK_TEMPORARY.
- Clears votes.
- Requires resume or close.

ACTIVE to RETIRED requires a session.

Returning RETIRED to ACTIVE requires a session. 

---

# 12. Supersession and Concurrency

Acceptance must verify:

- Referenced Resolution is ACTIVE.
- Authority is ACTIVE.
- Scope is ACTIVE.

Race condition rule:

- The first successful acceptance wins.
- Competing sessions referencing superseded resolutions transition to BLOCK_PERMANENT.

---

# 13. Acceptance Transaction Model

Acceptance(session_id, candidate_id) executes:

1. Acquire legitimacy lock covering all affected legitimacy context.
2. Validate session state is ACTIVE and in VOTING phase.
3. Validate governance immutability.
4. Validate Authority is ACTIVE.
5. Validate Scope is ACTIVE.
6. Validate no supersession conflict exists.
7. Validate constraints.
8. Evaluate Authority rule.
9. If any validation fails:
   - Return structured failure.
   - Perform no state mutation.
10. Snapshot session state.
11. Create Resolution from candidate.
12. Mark session ACCEPTED.
13. Release lock.

Acceptance is atomic relative to:

- Session state
- Referenced Resolution state
- Authority Resolution
- Scope Resolution

No partial acceptance is possible.

---

# 14. Evaluation API

evaluate(session_id) returns EvaluationReport.

Evaluation:

- Has no side effects.
- Is deterministic.
- Does not mutate session.

EvaluationReport includes:

- session_state
- session_phase
- can_accept (boolean)
- blocking_reasons (structured list)
- warnings (optional)

Evaluation behavior by phase:

PRE_STANCE:

- Validate governance prerequisites (e.g., participant existence).

VOTING:

- Evaluate constraints.
- Evaluate authority math.
- Evaluate supersession conflicts.
- Evaluate under-review state.

TERMINAL:

- Return state only.

Blocking reasons must be stable, machine-readable codes.

---

# 15. Candidate Semantics

- Candidates exist only within a session.
- Acceptance requires explicit candidate_id.
- Only one candidate may be accepted.
- Non-accepted candidates remain historical session artifacts.
- They do not become standalone objects.

---

# 16. Legitimacy Receipt Derivation

Upon ACCEPTED:

- The session and audit history deterministically derive:
  - Participant snapshot
  - Candidate snapshot
  - Stance snapshot
  - Authority ID
  - Scope ID
  - Acceptance result
  - Annotations

The legitimacy receipt is derived from the session record.

It is not an independent legitimacy source.

Only ACCEPTED sessions produce legitimacy receipts.

---

# 17. Atomicity Guarantees

Acceptance is atomic relative to all referenced legitimacy context.

If any validation fails, no mutation occurs.

If a crash occurs before commit, no legitimacy is created.

All implementations must produce identical outcomes given identical inputs.

---

# 18. Engine Invariants

- Only ACCEPTED sessions create Resolutions.
- No automatic acceptance.
- Governance cannot mutate after first stance.
- Any legitimacy-context change permanently invalidates incompatible sessions.
- Blocking resets voting.
- Every acceptance must be reproducible deterministically from audit.
- Evaluation must be deterministic across independent implementations.

---

# 19. Heavy Engine Doctrine

The engine enforces strict legitimacy mechanics.

The CLI layer may provide usability, summaries, defaults, and workflow assistance.

The engine must never:

- Auto-accept
- Infer consensus
- Relax invariants
- Mask legitimacy failures

Legitimacy is created only when explicitly commanded and mechanically validated.