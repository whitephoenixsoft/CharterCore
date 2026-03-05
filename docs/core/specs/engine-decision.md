# ENG-DECISION — Decision Execution, Session Governance, Acceptance, and Receipt Verification
Status: FROZEN (v8 – Round Epoch Identity & Receipt Determinism Alignment)
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines the mechanical model for:

- Session lifecycle
- Governance mutation rules
- Governance bootstrap preconditions
- Participant mechanics (epoch-based)
- Candidate mechanics (round-epoch-based)
- Constraint semantics
- Voting semantics
- Authority evaluation
- Blocking and closure semantics
- Round segmentation and reconfirmation
- Under-review and retired interactions
- Acceptance transaction model
- Receipt generation, emission, and verification
- Deterministic EvaluationReport API
- Concurrency and race handling
- Degraded/read-only mode on receipt or structural failure

Legitimacy is created, recorded, and verifiable deterministically.

---

# 2. Foundational Principles

1. Legitimacy is created only via explicit ACCEPTED session.
2. Voting is mutable until acceptance.
3. Acceptance is atomic.
4. Governance is frozen after first stance within a round.
5. Permanent legitimacy-context changes produce permanent blocking.
6. BLOCK_PERMANENT requires explicit user closure.
7. Interruptions terminate participation epochs and begin a new round.
8. Runtime structural integrity governed by ENG-INTEGRITY.
9. Engine enforces invariants; CLI handles usability.
10. Receipts formalize closure; they do not create legitimacy.
11. Governance must be structurally valid before evaluation.
12. Degraded/read-only mode allows DAG export if receipts or structural validation fail.
13. Participant identity is session-scoped participation epoch.
14. Candidate identity is round-scoped proposal epoch.
15. Sessions may contain multiple deterministic rounds.

---

# 3. Core Entities

Session  
Bounded decision container that may produce one accepted Resolution.

Round  
Deterministic participation segment created after resume.

Participant  
Session-scoped participation epoch belonging to a specific round.

Candidate  
Round-scoped proposal epoch belonging to a specific round.

Constraint  
Mechanical acceptance gate.

Authority  
Area-level Resolution defining decision rules.

Scope  
Area-level Resolution defining contextual boundaries.

Resolution  
Immutable object created only by acceptance.

EvaluationReport  
Deterministic, side-effect-free session evaluation.

Session Receipt  
Immutable closure artifact emitted at ACCEPTED or CLOSED.

---

# 4. Governance Preconditions

Evaluated before session evaluation or acceptance. Failure prevents authority evaluation.

## 4.1 Session Type

Immutable:

AUTHORITY  
SCOPE  
REGULAR

---

## 4.2 Session Type Rules

### AUTHORITY

Allowed if:

- No ACTIVE Authority exists  
OR  
- Acceptance supersedes the current ACTIVE Authority.

Multiple ACTIVE Authorities cause StructuralIntegrityFailure (ENG-INTEGRITY).

Attempting parallel Authority creation must be rejected with GOVERNANCE_SLOT_VIOLATION.

---

### SCOPE

Requires exactly one ACTIVE Authority.

Failure returns AUTHORITY_REQUIRED.

---

### REGULAR

Requires:

- Exactly one ACTIVE Authority
- Exactly one ACTIVE Scope

Failure returns one or more of:

AUTHORITY_REQUIRED  
SCOPE_REQUIRED  
GOVERNANCE_NOT_INITIALIZED

---

## 4.3 Deterministic Rejection

If governance preconditions fail:

can_accept = false

blocking_reasons must contain one or more of:

GOVERNANCE_NOT_INITIALIZED  
AUTHORITY_REQUIRED  
SCOPE_REQUIRED  
GOVERNANCE_SLOT_VIOLATION

Authority evaluation must not execute.

---

# 5. Session Lifecycle

## 5.1 States

ACTIVE  
PAUSED  
BLOCK_TEMPORARY  
BLOCK_PERMANENT  
ACCEPTED  
CLOSED

ACCEPTED and CLOSED are terminal.

BLOCK_PERMANENT cannot resume and requires explicit closure.

---

## 5.2 Phases

PRE_STANCE  
VOTING  
TERMINAL

Transitions:

PRE_STANCE → VOTING when the first stance is recorded  
VOTING → ACCEPTED on successful acceptance  
ACTIVE → PAUSED by user action  
PAUSED → PRE_STANCE on resume  
BLOCK_TEMPORARY → PRE_STANCE on resume  
BLOCK_PERMANENT → CLOSED by explicit user action

Resume always returns to PRE_STANCE.

---

# 6. Round Model

## 6.1 Round Definition

Sessions are segmented into deterministic rounds.

A session begins with:

round_index = 1

Rounds increase only when a resume occurs.

Round indices must be:

- Monotonically increasing
- Contiguous
- Deterministic

---

## 6.2 Round Creation

A new round is created when:

PAUSED → PRE_STANCE  
BLOCK_TEMPORARY → PRE_STANCE

When a new round begins:

- All prior participant epochs terminate
- All prior candidate epochs terminate
- Participant set cleared
- Candidate set cleared
- Constraint set cleared
- Vote set cleared
- Phase reset to PRE_STANCE

Rounds must never be renumbered.

---

# 7. Governance Rules

## 7.1 Mutability Boundary

Before first stance within the current round:

Mutable:

Participants  
Candidates  
Constraints

After the first stance:

Participants frozen  
Candidates frozen  
Constraints frozen

Mutation attempt causes BLOCK_PERMANENT.

---

# 8. Participant Rules (Epoch-Based)

Participants represent participation epochs.

Rules:

- Participants must be explicitly added
- Each addition generates a new participant_id
- participant_id must never be reused within the session
- display_name must be unique among active participants
- One stance per participant per candidate

Stances remain mutable until acceptance.

---

## 8.1 Participant Removal

Removing a participant:

- Terminates that participation epoch
- Does not affect prior rounds

---

## 8.2 Resume / Reconfirmation

When a session resumes:

- All participation epochs from the prior round terminate
- Participant set cleared
- Votes cleared
- Participants must be explicitly re-added
- Each re-addition generates a new participant_id

The Engine must not infer identity continuity between rounds.

---

# 9. Candidate Rules (Round Epoch-Based)

Candidates represent proposal epochs within a round.

Rules:

- Candidates must be explicitly created
- Each creation generates a new candidate_id
- candidate_id must be unique within its round
- candidate_id must not persist across rounds
- Candidate content immutable after creation

Votes must reference candidate_id values from the same round.

---

# 10. Constraint Rules

Constraints act as mechanical acceptance gates.

Rules:

- Declared before first stance
- Immutable once VOTING begins
- Mutation after VOTING causes BLOCK_PERMANENT

Constraint violations during evaluation cause acceptance failure.

A session that cannot satisfy its constraints may only transition to CLOSED.

Constraint violations never appear in emitted LEGITIMACY receipts because acceptance cannot occur until all constraints are satisfied.

---

# 11. Voting Semantics

Valid stances:

ACCEPT  
REJECT  
ABSTAIN

Rules:

- One stance per participant per candidate
- Stances mutable before acceptance
- Stances frozen after acceptance
- Authority evaluated only on explicit acceptance

ABSTAIN counts toward presence but not acceptance.

Votes exist only within the round where they were cast.

---

## 11.1 Solo Mode

If exactly one participant exists:

Acceptance attempt inserts an implicit ACCEPT stance if none exists.

The implicit vote becomes part of the frozen round snapshot.

Evaluation itself must remain side-effect-free.

---

# 12. Authority Evaluation

## 12.1 Authority Types

SOLE_ACTOR  
UNANIMOUS_PRESENT  
MAJORITY_PRESENT

---

## 12.2 ACTIVE Definition

Authority or Scope usable only if:

- Not SUPERSEDED
- Not UNDER_REVIEW
- Not RETIRED
- Receipt integrity valid

Usability determined at evaluation time.

---

## 12.3 Mechanical Rules

SOLE_ACTOR  
Exactly one participant must exist and vote ACCEPT.

UNANIMOUS_PRESENT  
All present participants must vote and all votes must be ACCEPT.

MAJORITY_PRESENT  
accept_count > floor(present / 2)

Constraints may further restrict acceptance eligibility.

---

# 13. Blocking Semantics

## 13.1 BLOCK_TEMPORARY

Triggered by:

- Constraint violations
- Referenced Resolution UNDER_REVIEW
- Scope UNDER_REVIEW
- Other reversible interruptions

Effects:

- Votes cleared
- Participant epochs terminated
- Phase reset to PRE_STANCE

Resume creates a new round.

---

## 13.2 BLOCK_PERMANENT

Triggered by:

- Authority superseded
- Scope superseded
- Referenced Resolution superseded
- Governance mutation after VOTING
- Supersession race loss
- Structural legitimacy-context invalidation

Effects:

- Session cannot resume
- Acceptance permanently impossible
- Explicit closure required

If any session in an Area is BLOCK_PERMANENT, no session may ACCEPT.

---

# 14. Supersession & Concurrency

Acceptance verifies:

Authority ACTIVE  
Scope ACTIVE  
Referenced Resolution ACTIVE

First successful acceptance wins.

Competing sessions transition to BLOCK_PERMANENT.

Graph rules defined in ENG-SUPERSESSION.

---

# 15. Acceptance Transaction

1. Acquire legitimacy lock
2. Validate governance preconditions
3. Validate session state
4. Validate governance immutability
5. Validate Authority and Scope usability
6. Validate referenced Resolution usability
7. Validate constraints
8. Evaluate authority rule
9. On failure no mutation occurs
10. Freeze participant epoch snapshot
11. Freeze candidate epoch snapshot
12. Freeze constraint snapshot
13. Freeze stance snapshot
14. Finalize current round
15. Create Resolution
16. Mark session ACCEPTED
17. Emit LEGITIMACY receipt with deterministic content_hash
18. Release lock

Atomic across Session, Resolution, and Receipt.

Crash before commit produces no Resolution and no Receipt.

---

# 16. Receipt Derivation & Verification

Receipts are emitted only on terminal transition.

ACCEPTED produces LEGITIMACY receipt.  
CLOSED produces EXPLORATION receipt.

Receipts must include:

- Ordered round history
- Participant snapshots per round
- Candidate snapshots per round
- Constraint snapshots per round
- Stance snapshots per round
- Authority reference
- Scope reference
- Acceptance result
- Deterministic content_hash

Receipts are immutable.

---

## Receipt Verification

Verification requires:

1. Session state equals ACCEPTED
2. Receipt exists
3. Snapshots match canonical session state
4. content_hash validates

Mismatch indicates structural corruption under ENG-INTEGRITY.

Receipts prove legitimacy events but do not create legitimacy.

---

# 17. Evaluation API

evaluate(session_id) returns EvaluationReport.

Evaluation must include:

session_state  
session_phase  
round_index  
can_accept  
blocking_reasons  
warnings

Evaluation must be:

Non-mutating  
Deterministic  
Idempotent  
Side-effect-free

Evaluation must not emit receipts or insert implicit votes.

Acceptance must not depend on prior evaluation.

---

# 18. Atomicity Guarantees

Acceptance is atomic.

Receipt emission is atomic with acceptance.

Validation failure causes no mutation.

Crash before commit produces no legitimacy and no receipt.

Identical input must produce identical outcome and identical receipt hash.

---

# 19. Engine Invariants

Governance preconditions enforced before evaluation.

Only ACCEPTED sessions create Resolutions.

Only ACCEPTED sessions create LEGITIMACY receipts.

Governance immutable after first stance within a round.

Participant identity epoch-based.

Candidate identity round-epoch-based.

Resume creates a new round.

Votes never cross round boundaries.

participant_id never reused.

candidate_id never reused across rounds.

BLOCK_TEMPORARY resets the round.

BLOCK_PERMANENT requires closure.

Receipt snapshot must match frozen session state.

Deterministic across implementations.

Receipt integrity required for legitimacy verification.

---

# 20. Heavy Engine Doctrine

The Engine must never:

Auto-accept  
Infer consensus  
Merge participation epochs  
Reuse participant_id  
Reuse candidate_id  
Repair structural violations  
Mask legitimacy failures  
Alter receipts once emitted

Legitimacy is created only by explicit, mechanically validated acceptance.

Receipts preserve legitimacy events for deterministic reconstruction and admissible verification.