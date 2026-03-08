# ENG-SESSION — Session Lifecycle, Governance Enforcement & Receipt Emission Specification

Status: FROZEN (v11 – Rule Identity Binding & Candidate Epoch Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Session lifecycle, governance enforcement, deterministic acceptance, round segmentation, and receipt emission

Authority: Subordinate to ENG-DOMAIN, ENG-DECISION, ENG-RECEIPT, ENG-INTEGRITY, ENG-CANON, ENG-SPECVERIFY

---

# 1. Purpose

## ENG-SESSION-01 — Sessions Are the Sole Legitimacy Mechanism

Sessions are the only mechanism through which legitimacy may be created.

All:

- Resolution acceptance
- Resolution supersession
- Authority evolution
- Scope evolution

must occur through a valid session acceptance.

Receipts formalize terminal closure but do not create legitimacy.

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
- An ACTIVE Authority exists and supersession rules apply

Fail if:

- Multiple ACTIVE Authorities exist
- Parallel Authority creation attempted without supersession

---

### SCOPE Session

Allowed only if:

- Exactly one ACTIVE Authority exists

Fail if:

- No ACTIVE Authority
- Multiple ACTIVE Authorities

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

## ENG-SESSION-04 — Session Identity

Every session has a UUIDv7 `session_id`.

Rules:

- Engine generates all session IDs
- Identity immutable
- Caller-provided IDs prohibited

Fail if identity is externally supplied or mutated.

---

## ENG-SESSION-04A — Participant Identity as Participation Epoch

Each `participant_id` represents a single participation epoch.

Rules:

- Engine-generated UUIDv7
- Never reused within a session
- Belongs to exactly one round
- Terminated by removal or round reset

Fail if:

- Reuse occurs
- Terminated epoch reactivated
- Vote references a participant from a different round

---

## ENG-SESSION-04B — Candidate Identity as Proposal Epoch

Candidates represent proposals introduced during a specific round.

Rules:

- Each `candidate_id` is engine-generated UUIDv7
- candidate_id must be unique within the round
- candidate belongs to exactly one round
- candidate identity never reused across rounds

On RESUME:

- Candidate set is cleared
- New candidates receive new candidate_id values

Fail if:

- candidate_id reused
- vote references candidate from another round

---

# 4. Session State & Phase Model

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

PRE_STANCE

- No votes recorded

VOTING

- At least one vote exists

TERMINAL

- session_state ∈ {ACCEPTED, CLOSED}

Fail if phase contradicts state or vote presence.

---

# 5. Round Model

## ENG-SESSION-05 — Round Index

Sessions are segmented into deterministic rounds.

A session begins with:

round_index = 1

Round indices must:

- Increase monotonically
- Remain contiguous
- Never be renumbered

Fail if ordering becomes inconsistent.

---

## ENG-SESSION-05A — Round Creation Rule

A new round is created only by a RESUME transition.

RESUME permitted only from:

- PAUSED
- BLOCK_TEMPORARY

On RESUME:

- round_index increments
- participant set cleared
- candidate set cleared
- constraint set cleared
- vote set cleared
- phase resets to PRE_STANCE

Session collections always represent the current active round only.

Historical rounds exist only in the terminal receipt.

Fail if:

- Round created outside RESUME
- RESUME occurs with non-empty vote set

---

# 6. Governance Context Freeze

## ENG-SESSION-06 — Governance Snapshot

At session creation:

- ACTIVE Authority Resolution is snapshotted
- ACTIVE Scope Resolution is snapshotted

These references remain immutable for the lifetime of the session.

Fail if governance context changes mid-session.

---

# 7. Freeze Boundary

## ENG-SESSION-07 — PRE_STANCE Is Sole Mutable Boundary

While phase = PRE_STANCE:

Mutable:

- Participants
- Candidates
- Constraints

When transitioning to VOTING:

- Participant set freezes
- Candidate set freezes
- Constraint set freezes

Triggered by:

- First vote
- OR implicit Solo vote

Fail if mutation occurs after freeze.

---

# 8. Vote Rules

## ENG-SESSION-08 — Votes Are Mechanical

Rules:

- One vote per participant epoch per candidate
- Vote references participant and candidate from same round
- Votes immutable once recorded
- Silence never counts as consent

Votes cleared when a new round begins.

Votes preserved only if the session remains in the same round.

Fail if the engine infers intent or mutates existing votes.

---

## ENG-SESSION-08A — Solo Mode Implicit Vote

If exactly one participant exists:

- Acceptance attempt inserts implicit ACCEPT vote if absent
- PRE_STANCE transitions to VOTING
- Governance freezes

The inserted vote must exist as a normal vote object.

---

# 9. Resume & Reconfirmation

## ENG-SESSION-09 — Epoch Reset on Resume

Resume allowed only from:

- PAUSED
- BLOCK_TEMPORARY

On resume:

- All participant epochs terminate
- Participant set cleared
- Candidate set cleared
- Constraint set cleared
- Vote set cleared
- Phase resets to PRE_STANCE
- round_index increments

Fail if:

- participant_id reused
- votes persist across rounds

---

# 10. Deterministic Acceptance

## ENG-SESSION-10 — Full Validation Pass

Acceptance evaluation must:

- Execute a full deterministic validation pass
- Evaluate governance, constraints, votes, and blocking rules
- Accumulate all violations
- Never short-circuit

Acceptance outcome determined only after evaluation completes.

Acceptance must not depend on:

- storage order
- timing
- host behavior

Identical inputs must produce identical outcomes.

---

## ENG-SESSION-10A — Constraint Semantics

Session constraints act as mechanical acceptance gates.

Constraints:

- May tighten authority decision rules
- May require specific participants to vote ACCEPT

If constraints are not satisfied:

- Acceptance must fail deterministically
- Session may only transition to CLOSED

Constraint snapshots must still appear in the final receipt to preserve full round reconstruction.

---

## ENG-SESSION-10B — Acceptance Finalizes Current Round

Acceptance:

- Finalizes the current round
- Transitions session state to ACCEPTED
- Transitions phase to TERMINAL

Acceptance occurs atomically with:

- Resolution creation
- Supersession application
- Receipt emission

Acceptance never creates a new round.

---

# 11. Receipt Emission

## ENG-SESSION-11 — Mandatory Receipt Emission

Exactly one receipt must be emitted for each session.

Terminal outcomes:

ACCEPTED → LEGITIMACY receipt  
CLOSED → EXPLORATION receipt

Receipt emission must be:

- Atomic with terminal transition
- Deterministic
- Immutable

Receipt snapshot derived from:

- Current session state
- Current round structures
- Governance snapshot

Receipt must include:

- Ordered round history
- Final round index
- Participant snapshots
- Candidate snapshots
- Constraint snapshots
- Vote snapshots
- Governance references
- Rule identity fields (`engine_version`, `spec_set_hash`)
- Deterministic content_hash

Canonical serialization defined in ENG-CANON.

The receipt hash therefore binds:

- the decision outcome
- the structural state
- the rule system that evaluated the session

Fail if receipt diverges from session state.

---

# 12. Governance Hygiene

## ENG-SESSION-12 — Area Blocking Invariant

If any session in an Area is BLOCK_PERMANENT:

- No session may transition to ACCEPTED

Acceptance must fail after deterministic validation.

---

# 13. Immutability

## ENG-SESSION-13 — Terminal States Immutable

If state ∈ {ACCEPTED, CLOSED}:

- No transitions permitted
- No participant mutation
- No candidate mutation
- No vote mutation
- No receipt mutation

---

# 14. Audit

## ENG-SESSION-14 — Lifecycle Auditable

The engine must emit audit events for lifecycle actions.

Audit events:

- Descriptive only
- Do not create legitimacy
- Do not replace receipts

Receipts remain the canonical legitimacy artifact.

---

# 15. Failure Semantics

## ENG-SESSION-15 — Explicit Failure Only

Violations must:

- Fail deterministically
- Leave session unchanged
- Produce structured EvaluationReport

The engine must never:

- Short-circuit evaluation
- Attempt silent correction
- Attempt automatic recovery
- Mutate state implicitly

---

# 16. Summary Guarantees

- Sessions are the sole legitimacy mechanism
- Governance bootstrap enforced
- Governance context frozen at creation
- PRE_STANCE sole mutable boundary
- First vote freezes structure
- Resume creates a new round
- Resume resets participation epochs
- Candidates are round-scoped
- Votes never cross round boundaries
- participant_id never reused
- candidate_id never reused within a round
- Acceptance uses full deterministic validation
- Acceptance finalizes current round
- Terminal states emit exactly one receipt
- Governance hygiene enforced
- No implicit transitions

---

# 17. Mental Model

Sessions freeze governance context at creation.

PRE_STANCE is the only mutable boundary.

First vote freezes structure.

Resume resets participation epochs and begins a new round.

Session collections always represent the current round only.

Historical rounds exist exclusively in the terminal receipt.

Acceptance deterministically finalizes the current round.

Terminal transition emits exactly one immutable receipt.

Nothing changes unless explicitly commanded.