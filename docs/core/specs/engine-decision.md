# ENG-DECISION — Decision Execution, Session Governance, Acceptance, and Receipt Verification

Status: DRAFT (Adjusted for V3 Governance & Incremental Compilation)  
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines the mechanical model for:

- Governance bootstrap enforcement, including initial Authority solo mode  
- Participant participation epochs  
- Candidate proposal epochs  
- Constraint evaluation  
- Voting semantics  
- Authority and Scope evaluation with UNDER_REVIEW/RETIRED semantics  
- Blocking and closure semantics  
- Deterministic round segmentation  
- Acceptance transaction execution  
- Supersession race handling  
- Incremental compilation integration  
- Receipt derivation and verification  
- Deterministic evaluation reporting

Legitimacy must be created, recorded, and verifiable deterministically.

Behavioral execution defined here operates over the structural model in ENG-DOMAIN.

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
16. Incremental compilation sessions operate on historical Resolutions only and do not alter runtime session state unless fully validated.

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
Area-level Resolution defining the decision rule. Must always be ACTIVE for legitimacy.

Scope  
Area-level Resolution defining contextual boundaries. May enter UNDER_REVIEW; REQUIRED for regular sessions.

Resolution  
Immutable artifact created only by successful acceptance. Supports states: ACTIVE, UNDER_REVIEW, RETIRED, SUPERSEDED.

EvaluationReport  
Deterministic evaluation output with no side effects.

Session Receipt  
Immutable closure artifact emitted at ACCEPTED or CLOSED.

Incremental Compilation Index  
Engine-managed list of Resolution IDs with acceptance timestamps to support replay and conflict detection.

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
INCREMENTAL_COMPILATION

---

## 4.2 Session Type Rules

### AUTHORITY

Allowed if:

- No ACTIVE Authority exists  
OR  
- Acceptance supersedes the current ACTIVE Authority.

Multiple ACTIVE Authorities constitute StructuralIntegrityFailure.

Attempting parallel Authority creation returns:

GOVERNANCE_SLOT_VIOLATION

---

### SCOPE

Requires exactly one ACTIVE Authority.  
Failure returns: AUTHORITY_REQUIRED

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

### INCREMENTAL_COMPILATION

- Operates on historical Resolutions only  
- Requires Engine-managed index of Resolution IDs and acceptance timestamps  
- Conflicts resolved deterministically based on index (earlier acceptance wins)  
- May replay sessions without full runtime state  
- Cannot create new governance objects without standard session acceptance  

---

## 4.3 Deterministic Rejection

If governance preconditions fail:

can_accept = false

blocking_reasons must include the applicable governance failure.

Authority or Scope evaluation must not execute.

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
Resume; triggers new round creation.

BLOCK_TEMPORARY → PRE_STANCE  
Resume; triggers new round creation.

BLOCK_PERMANENT → CLOSED  
Explicit closure.

---

# 6. Round Model

## 6.1 Round Definition

Sessions are segmented into deterministic rounds.  
Initial state: round_index = 1

Round indices:

- Increase monotonically  
- Remain contiguous  
- Never be renumbered

---

## 6.2 Round Creation

A new round is created **on RESUME**:

- Participant set cleared  
- Candidate set cleared  
- Constraint set cleared  
- Vote set cleared  
- Phase reset to PRE_STANCE  

Session runtime collections always represent only the active round.

Historical rounds exist only in terminal receipts.

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
- candidate_id must be unique within the round  
- Candidate belongs to the round where it was created  
- Candidate content mutable during PRE_STANCE  
- Candidate content immutable once VOTING begins  

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

Constraint snapshots remain present in receipts for full round reconstruction.

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

## 11.1 Solo Mode Bootstrap

If no Authority exists during first session creation:

- Engine assumes SOLE_ACTOR mode  
- Must announce the bootstrap in audit  
- Solo mode only applies to initial session  
- Subsequent sessions require standard Authority  

---

# 12. Authority & Scope Evaluation

## 12.1 ACTIVE Definition

Authority or Scope usable only if:

- Not SUPERSEDED  
- Not UNDER_REVIEW  
- Not RETIRED  
- Receipt integrity valid  

Usability determined at evaluation time.

## 12.2 Mechanical Rules

SOLE_ACTOR: exactly one participant votes ACCEPT  
UNANIMOUS_PRESENT: all present participants vote ACCEPT  
MAJORITY_PRESENT: accept_count > floor(present / 2)

Constraints may further restrict eligibility.

---

# 13. Blocking Semantics

## 13.1 BLOCK_TEMPORARY

Triggered by reversible conditions such as:

- External dependency under review  
- Referenced Resolution UNDER_REVIEW or RETIRED  
- Scope UNDER_REVIEW  

Effects:

- Session cannot accept  
- Resume required to continue participation  

Votes cleared on new round.

---

## 13.2 BLOCK_PERMANENT

Triggered by:

- Authority superseded  
- Scope superseded  
- Referenced Resolution SUPERSEDED  
- Governance mutation after VOTING  
- Supersession race loss  
- Structural legitimacy-context invalidation  

Effects:

- Session cannot resume  
- Acceptance permanently impossible  
- Explicit closure required  

Area invariant: if any session is BLOCK_PERMANENT, no session may ACCEPT.

---

# 14. Supersession, Incremental Compilation & Concurrency

- Acceptance validates Authority ACTIVE, Scope ACTIVE, Referenced Resolution ACTIVE  
- Incremental compilation sessions consult Engine-managed index of accepted Resolutions  
- Conflicts resolved deterministically by earliest acceptance timestamp in index  
- First successful session wins; competing sessions blocked  
- Supersession graph semantics defined in ENG-SUPERSESSION  

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

Crash before commit: no legitimacy created.

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
- engine_version  
- spec_set_hash  
- hash_algorithm  
- Deterministic content_hash  

Canonical serialization defined in ENG-CANON.

Receipts immutable.

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

Acceptance must not depend on prior evaluation.

---

# 18. Atomicity Guarantees

Acceptance is atomic.

Receipt emission atomic with acceptance.

Validation failure produces no mutation.

Crash before commit produces:

No Resolution  
No Receipt

Identical input produces identical result.

---

# 19. Engine Invariants

- Governance preconditions enforced before evaluation  
- Only ACCEPTED sessions create Resolutions  
- Only ACCEPTED sessions emit LEGITIMACY receipts  
- Governance immutable after first stance  
- Participant identity epoch-based  
- Candidate identity proposal-epoch-based  
- Resume creates new round  
- Votes never cross round boundaries  
- participant_id never reused  
- candidate_id never reused within a round  
- BLOCK_TEMPORARY requires resume  
- BLOCK_PERMANENT requires closure  
- Receipt snapshot must match frozen session state  
- RETIRED/UNDER_REVIEW semantics enforced  
- Incremental compilation consults Resolution index for deterministic conflict resolution  

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