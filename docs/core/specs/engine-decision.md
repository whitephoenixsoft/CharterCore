# ENG-DECISION — Decision Execution, Session Governance, Acceptance, and Receipt Verification

Status: FROZEN (v9 – Canonical Model Alignment & Runtime/Receipt Separation)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines the mechanical model for:

- Governance bootstrap enforcement
- Participant participation epochs
- Candidate proposal epochs
- Constraint evaluation
- Voting semantics
- Authority rule evaluation
- Blocking and closure semantics
- Deterministic round segmentation
- Acceptance transaction execution
- Supersession race handling
- Receipt derivation and verification
- Deterministic evaluation reporting

Legitimacy must be created, recorded, and verifiable deterministically across independent implementations.

Behavioral execution defined here operates over the structural model defined in ENG-DOMAIN.

Canonical serialization defined in ENG-CANON.

---

# 2. Foundational Principles

1. Legitimacy is created only through explicit ACCEPTED session transition.
2. Voting remains mutable until acceptance occurs.
3. Acceptance is atomic across session, resolution, and receipt.
4. Governance mutability ends at the first stance within a round.
5. Structural legitimacy-context changes may permanently invalidate a session.
6. BLOCK_PERMANENT requires explicit user closure.
7. Interruptions terminate participation epochs and create a new round.
8. Structural integrity is governed by ENG-INTEGRITY.
9. Engine enforces invariants; external tools handle usability.
10. Receipts formalize closure but do not create legitimacy.
11. Governance must be structurally valid before evaluation executes.
12. Degraded/read-only mode allows graph export when structural validation fails.
13. Participant identity represents a session-scoped participation epoch.
14. Candidate identity represents a proposal epoch bound to a specific round.
15. Session runtime state contains only the current round structures.

Historical round state exists only in emitted receipts.

---

# 3. Core Entities

Session  
Bounded decision container that may produce a single accepted Resolution.

Round  
Deterministic participation segment beginning at session start or after resume.

Participant  
Session-scoped participation epoch belonging to a specific round.

Candidate  
Proposal epoch created within a specific round.

Constraint  
Mechanical acceptance gate.

Authority  
Area-level Resolution defining the decision rule.

Scope  
Area-level Resolution defining contextual boundaries.

Resolution  
Immutable artifact created only by successful acceptance.

EvaluationReport  
Deterministic evaluation output with no side effects.

Session Receipt  
Immutable closure artifact emitted at ACCEPTED or CLOSED.

---

# 4. Governance Preconditions

Governance preconditions must be validated before authority rule evaluation.

Failure prevents acceptance evaluation.

---

## 4.1 Session Type

Session type is immutable:

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

Multiple ACTIVE Authorities constitute StructuralIntegrityFailure.

Attempting parallel Authority creation must return:

GOVERNANCE_SLOT_VIOLATION

---

### SCOPE

Requires exactly one ACTIVE Authority.

Failure returns:

AUTHORITY_REQUIRED

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

blocking_reasons must include the applicable governance failure.

Authority rule evaluation must not execute.

---

# 5. Session Lifecycle

## 5.1 Session States

ACTIVE  
PAUSED  
BLOCK_TEMPORARY  
BLOCK_PERMANENT  
ACCEPTED  
CLOSED

ACCEPTED and CLOSED are terminal.

BLOCK_PERMANENT cannot resume.

---

## 5.2 Session Phases

PRE_STANCE  
VOTING  
TERMINAL

Transitions:

PRE_STANCE → VOTING  
Triggered by first vote or implicit Solo vote.

VOTING → ACCEPTED  
Occurs on successful acceptance.

ACTIVE → PAUSED  
User action.

PAUSED → PRE_STANCE  
Resume.

BLOCK_TEMPORARY → PRE_STANCE  
Resume.

BLOCK_PERMANENT → CLOSED  
Explicit closure.

---

# 6. Round Model

## 6.1 Round Definition

Sessions are segmented into deterministic rounds.

Initial state:

round_index = 1

Round indices must:

- Increase monotonically
- Remain contiguous
- Never be renumbered

---

## 6.2 Round Creation

A new round is created only when RESUME occurs.

Allowed transitions:

PAUSED → PRE_STANCE  
BLOCK_TEMPORARY → PRE_STANCE

When a new round begins:

- Participant set cleared
- Candidate set cleared
- Constraint set cleared
- Vote set cleared
- Phase reset to PRE_STANCE

Session runtime collections always represent only the active round.

Historical rounds exist only in the terminal receipt.

---

# 7. Governance Mutability Rules

## 7.1 Mutability Boundary

Before first stance:

Mutable:

Participants  
Candidates  
Constraints

After first stance:

Participants frozen  
Candidates frozen  
Constraints frozen

Mutation attempt triggers:

BLOCK_PERMANENT

---

# 8. Participant Mechanics

Participants represent participation epochs.

Rules:

- Participants explicitly added
- Engine generates participant_id
- participant_id never reused within session
- display_name unique among active participants

Each participant may cast:

One stance per candidate.

Stances mutable until acceptance.

---

## 8.1 Participant Removal

Removing a participant:

- Terminates that epoch
- Does not alter historical receipt state

---

## 8.2 Resume / Reconfirmation

On RESUME:

- All prior participation epochs terminate
- Participant set cleared
- Votes cleared
- Participants must be re-added

Engine must not infer identity continuity across rounds.

---

# 9. Candidate Mechanics

Candidates represent proposal epochs.

Rules:

- Created explicitly
- Engine generates candidate_id
- candidate_id must be unique within the session
- Candidate belongs to the round where it was created
- Candidate content immutable after creation

Candidates never persist across rounds.

Votes must reference candidates from the current round.

---

# 10. Constraint Semantics

Constraints act as deterministic acceptance gates.

Rules:

- Declared before first stance
- Immutable once VOTING begins
- Mutation after VOTING causes BLOCK_PERMANENT

Constraint failure during evaluation:

- Prevents acceptance
- Does not mutate session state

Sessions that cannot satisfy constraints may only transition to CLOSED.

Constraints never appear in successful LEGITIMACY receipts.

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

ABSTAIN counts toward presence but not acceptance.

Votes belong exclusively to the round in which they were created.

Votes are cleared when a new round begins.

---

## 11.1 Solo Mode

If exactly one participant exists:

Acceptance attempt inserts an implicit ACCEPT vote if none exists.

This vote becomes part of the final round snapshot.

Evaluation itself remains side-effect-free.

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

All present participants must vote.  
All votes must be ACCEPT.

MAJORITY_PRESENT

accept_count > floor(present / 2)

Constraints may further restrict acceptance eligibility.

---

# 13. Blocking Semantics

## 13.1 BLOCK_TEMPORARY

Triggered by reversible conditions such as:

- External dependency under review
- Referenced resolution under review
- Scope under review

Effects:

- Session cannot accept
- Resume required to continue participation

Votes cleared on new round.

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

Area invariant:

If any session is BLOCK_PERMANENT, no session may ACCEPT.

---

# 14. Supersession & Concurrency

Acceptance validates:

Authority ACTIVE  
Scope ACTIVE  
Referenced Resolution ACTIVE

Acceptance requires acquiring the legitimacy lock.

First successful acceptance wins.

Competing sessions transition to BLOCK_PERMANENT.

Supersession graph semantics defined in ENG-SUPERSESSION.

---

# 15. Acceptance Transaction

Atomic transaction:

1. Acquire legitimacy lock
2. Validate governance preconditions
3. Validate session state
4. Validate governance immutability
5. Validate Authority usability
6. Validate Scope usability
7. Validate referenced Resolution usability
8. Validate constraints
9. Evaluate authority rule
10. Freeze participant snapshot
11. Freeze candidate snapshot
12. Freeze constraint snapshot
13. Freeze vote snapshot
14. Finalize round
15. Create Resolution
16. Mark session ACCEPTED
17. Emit LEGITIMACY receipt
18. Release lock

Atomic across:

Session  
Resolution  
Receipt

Crash before commit results in no legitimacy creation.

---

# 16. Receipt Derivation & Verification

Receipts emitted only on terminal transition.

ACCEPTED → LEGITIMACY receipt  
CLOSED → EXPLORATION receipt

Receipts must include:

- Ordered round history
- Participant snapshots
- Candidate snapshots
- Constraint snapshots
- Vote snapshots
- Governance references
- Deterministic content_hash

Canonical serialization defined in ENG-CANON.

Receipts immutable.

---

## Receipt Verification

Verification requires:

1. Session state is ACCEPTED
2. Receipt exists
3. Snapshots match canonical session state
4. content_hash validates

Mismatch indicates structural corruption.

Receipts prove legitimacy events but do not create legitimacy.

---

# 17. Evaluation API

evaluate(session_id) returns EvaluationReport.

Evaluation includes:

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

Evaluation must not:

- Emit receipts
- Insert votes
- Alter session state

Acceptance must not depend on prior evaluation.

---

# 18. Atomicity Guarantees

Acceptance is atomic.

Receipt emission is atomic with acceptance.

Validation failure produces no mutation.

Crash before commit produces:

No Resolution  
No Receipt

Identical input must produce identical result.

---

# 19. Engine Invariants

Governance preconditions enforced before evaluation.

Only ACCEPTED sessions create Resolutions.

Only ACCEPTED sessions emit LEGITIMACY receipts.

Governance immutable after first stance.

Participant identity epoch-based.

Candidate identity proposal-epoch-based.

Resume creates new round.

Votes never cross round boundaries.

participant_id never reused.

candidate_id never reused.

BLOCK_TEMPORARY requires resume.

BLOCK_PERMANENT requires closure.

Receipt snapshot must match frozen session state.

Determinism required across implementations.

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
Alter emitted receipts

Legitimacy is created only through explicit, mechanically validated acceptance.

Receipts preserve legitimacy events for deterministic reconstruction and admissible verification.