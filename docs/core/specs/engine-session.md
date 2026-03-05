# ENG-SESSION — Session Lifecycle, Governance Enforcement & Receipt Emission Specification
Status: FROZEN (v9 – Round-Scoped Identity & Deterministic Constraint Semantics Integrated)
Applies to: Engine Core (V1/V2+)
Scope: Session lifecycle, session classification, governance enforcement, legitimacy boundaries, deterministic acceptance, round segmentation, and receipt emission

Authority: Subordinate to ENG-DOMAIN, ENG-DECISION, ENG-RECEIPT, ENG-INTEGRITY

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

Receipts formalize session closure but do not create legitimacy.

Fail if:

- A Resolution is created outside a session
- Legitimacy emerges without session acceptance
- Supersession occurs without a session

---

# 2. Session Classification & Bootstrap Constraints

## ENG-SESSION-02 — Session Type Is Explicit and Immutable

Each session is classified at creation as exactly one of:

- AUTHORITY
- SCOPE
- REGULAR

Session type:

- Required at creation
- Immutable for the lifetime of the session
- Determines governance preconditions

Fail if:

- Session type changes
- Session type undefined

---

## ENG-SESSION-03 — Creation Preconditions

### AUTHORITY Session

Allowed only if:

- No ACTIVE Authority exists  
  OR  
- A current ACTIVE Authority exists and supersession rules apply

Fail if:

- Multiple ACTIVE Authority exist
- Parallel Authority attempted without supersession

---

### SCOPE Session

Allowed only if:

- Exactly one ACTIVE Authority exists

Fail if:

- No ACTIVE Authority
- Multiple ACTIVE Authority

---

### REGULAR Session

Allowed only if:

- Exactly one ACTIVE Authority
- Exactly one ACTIVE Scope

Fail if:

- Authority missing
- Scope missing
- Governance structurally invalid

---

# 3. Identity & Participation Epochs

## ENG-SESSION-04 — Engine-Assigned Identity

- Every session has a UUIDv7 `session_id`
- Engine generates all session IDs
- Identity immutable

Fail if:

- Caller-generated identity
- Identity mutation

---

## ENG-SESSION-04A — Participant Identity as Participation Epoch

- Each `participant_id` represents a single participation epoch
- Engine-generated UUIDv7
- Never reused within a session lifecycle
- Terminated explicitly (removal or resume reset)

Fail if:

- Reuse occurs
- Terminated epoch reactivated
- Vote references non-active epoch

---

## ENG-SESSION-04B — Candidate Identity as Round Epoch

Candidates represent proposal epochs scoped to a specific round.

- Each `candidate_id` is engine-generated (UUIDv7)
- Candidate IDs are unique within the round in which they are created
- Candidate identity must not persist across rounds

On RESUME:

- The candidate set is cleared
- New candidates receive new candidate_id values

Fail if:

- candidate_id reused across rounds
- vote references a candidate_id not present in the current round

---

# 4. State & Phase Model

## 4.1 SessionState

Exactly one of:

- ACTIVE
- PAUSED
- BLOCK_TEMPORARY
- BLOCK_PERMANENT
- ACCEPTED
- CLOSED

---

## 4.2 SessionPhase

Exactly one of:

- PRE_STANCE
- VOTING
- TERMINAL

Rules:

- PRE_STANCE → no votes
- VOTING → at least one vote or implicit Solo vote
- TERMINAL → state ∈ {ACCEPTED, CLOSED}

Fail if inconsistent.

---

# 5. Round Model

## ENG-SESSION-05 — Round Index

Sessions are segmented into deterministic rounds.

A session begins with:

- `round_index = 1`

Round indices must:

- Increase monotonically
- Be contiguous
- Never be renumbered

Fail if round ordering becomes inconsistent.

---

## ENG-SESSION-05A — Round Creation Rule

A new round is created **only** when a RESUME transition occurs.

RESUME is permitted only from:

- PAUSED
- BLOCK_TEMPORARY

On RESUME:

- `round_index` increments by 1
- Participant set cleared
- Candidate set cleared
- Constraint set cleared
- Vote set cleared
- Phase resets to PRE_STANCE
- Participation epochs reset

No other transition may create a round.

Fail if:

- Round created outside RESUME
- RESUME occurs with non-empty vote set

---

# 6. State Semantics

State semantics remain deterministic and explicitly enforced by the engine.

---

# 7. Valid State Transitions

Only listed transitions permitted.

Blocking transitions are engine-detected only.

No implicit transitions permitted.

---

# 8. Governance Context Freeze

## ENG-SESSION-06 — Governance Snapshot

At session creation:

- ACTIVE Authority Resolution snapshotted
- ACTIVE Scope Resolution snapshotted

Immutable for session lifetime.

Fail if governance renegotiated mid-session.

---

# 9. Freeze Boundary

## ENG-SESSION-07 — PRE_STANCE Is Sole Mutable Boundary

While PRE_STANCE:

- Participants mutable
- Candidates mutable
- Constraints mutable

On transition to VOTING:

- Participant set frozen
- Candidate set frozen
- Constraints frozen

Triggered by:

- First recorded vote
- OR implicit Solo vote

Fail if mutation after freeze.

---

# 10. Vote Rules

## ENG-SESSION-08 — Votes Are Mechanical

- One vote per participant epoch per candidate
- Immutable once recorded
- Silence is not consent
- Acceptance purely mechanical

Votes cleared on:

- RESUME
- BLOCK_TEMPORARY

Votes preserved on:

- BLOCK_PERMANENT

Fail if engine infers intent or mutates votes.

---

## ENG-SESSION-08A — Solo Mode Implicit Vote

If exactly one participant epoch exists:

- Acceptance attempt inserts implicit ACCEPT vote if absent
- PRE_STANCE → VOTING
- Governance freezes

Acceptance still requires a vote object to exist in state.

---

# 11. Resume & Reconfirmation

## ENG-SESSION-09 — Epoch Reset on Resume

Resume allowed only from:

- PAUSED
- BLOCK_TEMPORARY

On resume:

- All prior participation epochs terminated
- Participant set cleared
- Candidate set cleared
- Votes cleared
- Phase resets to PRE_STANCE
- Round index increments

Fail if:

- Prior participant_id reused
- Votes persist across rounds

---

# 12. Deterministic Acceptance

## ENG-SESSION-10 — Full Validation, No Short-Circuit

Acceptance evaluation must:

- Execute a full deterministic validation pass
- Evaluate all applicable governance, constraint, vote, and block rules
- Accumulate all detected violations
- Never short-circuit on first violation
- Derive outcome only after evaluation completes

Acceptance depends solely on:

- Governance preconditions
- Authority rule
- Constraints
- Recorded votes
- Block state

Identical inputs → identical:

- Violation set
- Violation ordering
- Outcome

Fail if:

- Acceptance depends on timing
- Acceptance depends on storage order
- Acceptance depends on external context

---

## ENG-SESSION-10A — Constraint Semantics

Session constraints act as mechanical acceptance gates.

Constraints:

- May tighten the authority decision rule
- May designate participants with mandatory voting rights
- Are evaluated only during acceptance

If constraints are not satisfied:

- Acceptance must fail deterministically
- Session may only transition to CLOSED

Constraints never appear as violations within emitted receipts because a LEGITIMACY receipt can only occur when all constraints are satisfied.

---

## ENG-SESSION-10B — Acceptance Finalizes Current Round

Acceptance:

- Finalizes the current round
- Transitions session state to ACCEPTED
- Transitions phase to TERMINAL

Acceptance must occur atomically with:

- Resolution creation
- Supersession application (if applicable)
- Receipt emission

No new round is created during acceptance.

---

# 13. Receipt Emission

## ENG-SESSION-11 — Mandatory Receipt Emission

Exactly one receipt must be emitted per session upon TERMINAL transition.

If:

- ACCEPTED → LEGITIMACY receipt
- CLOSED → EXPLORATION receipt

Receipt emission is:

- Atomic with terminal transition
- Deterministic
- Immutable

Receipt must contain:

- Ordered round history
- Final round index
- Participant epoch snapshots per round
- Candidate snapshots per round
- Constraint snapshots per round
- Vote snapshots per round
- Governance snapshot references
- Deterministic `content_hash`

Fail if receipt snapshot diverges from session state.

---

# 14. Governance Hygiene

## ENG-SESSION-12 — Area Blocking Invariant

If any session in Area is BLOCK_PERMANENT:

- No session may transition to ACCEPTED

Engine must reject acceptance attempts after full validation pass.

---

# 15. Immutability

## ENG-SESSION-13 — Terminal States Immutable

If state ∈ {ACCEPTED, CLOSED}:

- No transitions permitted
- No participant mutation
- No candidate mutation
- No vote mutation
- No receipt mutation

---

# 16. Audit

## ENG-SESSION-14 — Lifecycle Auditable

Engine must emit audit events for lifecycle actions.

Audit:

- Descriptive only
- Does not create legitimacy
- Does not replace receipts

---

# 17. Failure Semantics

## ENG-SESSION-15 — Explicit Failure, No Implicit Repair

Violations must:

- Fail deterministically
- Leave session unchanged
- Produce structured EvaluationReport

Engine must not:

- Short-circuit evaluation
- Attempt silent correction
- Attempt automatic recovery
- Mutate state implicitly

---

# 18. Summary Guarantees

- Sessions are the sole legitimacy mechanism
- Governance bootstrap enforced
- Governance context frozen at creation
- PRE_STANCE sole mutable boundary
- First vote freezes structure
- Resume creates new round
- Resume resets participation epochs
- Candidates are round-scoped
- Votes never cross round boundaries
- No participant_id reuse
- No candidate_id reuse across rounds
- Acceptance uses full deterministic validation
- Acceptance finalizes current round
- Terminal states emit exactly one receipt
- Governance hygiene enforced
- No implicit transitions

---

# Mental Model

Sessions freeze governance context at creation.

PRE_STANCE is the only mutable boundary.

First vote freezes structure.

Resume resets participation epochs and begins a new round.

Each round captures a complete structural snapshot.

Acceptance deterministically finalizes the current round.

Terminal transition emits exactly one receipt.

Nothing changes unless explicitly commanded.