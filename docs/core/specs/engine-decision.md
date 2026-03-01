# ENG-DECISION — Decision Execution, Session Governance, Acceptance, and Receipt Verification (Rewritten v6)
Status: DRAFT (v6 – Participant Epoch Model Integrated)
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines the mechanical model for:

- Session lifecycle  
- Governance mutation rules  
- Governance bootstrap preconditions  
- Participant mechanics (epoch-based)  
- Candidate mechanics  
- Constraint semantics  
- Voting semantics  
- Authority evaluation  
- Blocking and closure semantics  
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
4. Governance is frozen after first stance.  
5. Permanent legitimacy-context changes → permanent blocking.  
6. BLOCK_PERMANENT requires explicit user closure.  
7. Interruptions reset voting and terminate participation epochs.  
8. Runtime structural integrity governed by ENG-INTEGRITY.  
9. Engine enforces invariants; CLI handles usability.  
10. Receipts formalize closure; they do not create legitimacy.  
11. Governance must be structurally valid before evaluation.  
12. Degraded/read-only mode allows DAG export if receipts or structural validation fail.  
13. Participant identity is session-scoped and epoch-based.

---

# 3. Core Entities

- **Session:** Bounded decision record; may produce one accepted Resolution.  
- **Participant:** Session-scoped participation epoch; one stance per candidate.  
- **Candidate:** Proposal scoped to a Session.  
- **Constraint:** Mechanical acceptance gate.  
- **Authority:** Area-level Resolution defining decision rules.  
- **Scope:** Area-level Resolution defining contextual boundaries.  
- **Resolution:** Immutable object created only by acceptance.  
- **EvaluationReport:** Deterministic, side-effect-free session evaluation.  
- **Session Receipt:** Immutable closure artifact emitted at ACCEPTED or CLOSED.

---

# 4. Governance Preconditions

Evaluated before session evaluation or acceptance. Failure prevents authority evaluation.

## 4.1 Session Type

Immutable:

- AUTHORITY  
- SCOPE  
- REGULAR  

## 4.2 Session Type Rules

### AUTHORITY

- Allowed if no ACTIVE Authority exists OR acceptance supersedes current ACTIVE Authority.  
- Multiple ACTIVE Authorities → StructuralIntegrityFailure (ENG-INTEGRITY).  
- Attempt to create parallel Authority → REJECTED with GOVERNANCE_SLOT_VIOLATION.

### SCOPE

- Requires exactly one ACTIVE Authority.  
- Failure → REJECTED with AUTHORITY_REQUIRED.

### REGULAR

- Requires one ACTIVE Authority AND one ACTIVE Scope.  
- Failure → REJECTED with AUTHORITY_REQUIRED, SCOPE_REQUIRED, or GOVERNANCE_NOT_INITIALIZED.

---

## 4.3 Deterministic Rejection

If governance preconditions fail:

- can_accept = false  
- blocking_reasons include one of:  
  - GOVERNANCE_NOT_INITIALIZED  
  - AUTHORITY_REQUIRED  
  - SCOPE_REQUIRED  
  - GOVERNANCE_SLOT_VIOLATION  
- Authority evaluation is skipped.

---

# 5. Session Lifecycle

## 5.1 States

- ACTIVE  
- PAUSED  
- BLOCK_TEMPORARY  
- BLOCK_PERMANENT  
- ACCEPTED  
- CLOSED  

ACCEPTED and CLOSED are terminal.  
BLOCK_PERMANENT is non-resumable until explicitly CLOSED.

## 5.2 Phases

- PRE_STANCE  
- VOTING  
- TERMINAL  

Transitions:

- PRE_STANCE → VOTING when first stance recorded  
- VOTING → ACCEPTED on successful acceptance  
- ACTIVE → PAUSED by user action  
- PAUSED → PRE_STANCE on resume  
- BLOCK_TEMPORARY → PRE_STANCE on resume  
- BLOCK_PERMANENT → CLOSED by explicit user action  

Resume always returns to PRE_STANCE and requires participant re-addition.

---

# 6. Governance Rules

## 6.1 Mutability

Before first stance:

- Participants mutable  
- Candidates mutable  
- Constraints mutable  

After first stance:

- Participants immutable  
- Candidates immutable  
- Constraints immutable  

Mutation after first stance → BLOCK_PERMANENT.

---

## 6.2 Participant Rules (Epoch-Based)

Participants are session-scoped participation epochs.

Rules:

- Participants must be explicitly added.  
- Each add operation generates a new participant_id.  
- participant_id must never be reused within the Session.  
- display_name must be unique among active participants.  
- One stance per participant per candidate.  
- Stances mutable until acceptance.  

### Removal

- Removing a participant terminates that participation epoch.  
- Removed participant_id must not be reused.  

### Resume / Reconfirmation

When a Session transitions from PAUSED or BLOCK_TEMPORARY to PRE_STANCE:

- All prior participation epochs are terminated.  
- Participant set is cleared.  
- All votes are cleared.  
- Reconfirmation requires explicit re-addition of participants.  
- Each re-addition generates a new participant_id.  
- No participant_id reuse is permitted.  
- Reconfirmation events are included in the receipt snapshot metadata.  

Resume represents a new participation epoch set within the same Session container.

The Engine does not infer continuity between pre-interruption and post-interruption participants.

---

## 6.3 Constraint Rules

- Declared before first stance.  
- Immutable once VOTING begins.  
- Constraint mutation during VOTING → BLOCK_PERMANENT.  
- Constraint violation → BLOCK_TEMPORARY.

---

# 7. Voting Semantics

Valid stances:

- ACCEPT  
- REJECT  
- ABSTAIN  

Rules:

- Stances mutable before acceptance.  
- Frozen after acceptance.  
- Authority evaluated only on explicit acceptance.  

Abstain counts toward presence but not toward accept_count.

### Solo Mode

If no stance exists at acceptance attempt:

- An implicit ACCEPT stance is inserted.  
- That stance becomes part of the frozen snapshot.  

Evaluation must not mutate state; implicit insertion occurs only during acceptance.

---

# 8. Authority Evaluation

## 8.1 Authority Types

- SOLE_ACTOR  
- UNANIMOUS_PRESENT  
- MAJORITY_PRESENT  

## 8.2 ACTIVE Definition

Authority or Scope usable only if:

- Not SUPERSEDED  
- Not UNDER_REVIEW  
- Not RETIRED  
- Receipt integrity valid  

Usability determined at evaluation time.

## 8.3 Mechanical Rules

SOLE_ACTOR:
- Exactly one active participant  
- That participant casts ACCEPT  

UNANIMOUS_PRESENT:
- All present must vote  
- All must ACCEPT  

MAJORITY_PRESENT:
- accept_count > floor(present / 2)

---

# 9. Blocking Semantics

## 9.1 BLOCK_TEMPORARY

Triggered by:

- Constraint violation  
- Referenced Resolution UNDER_REVIEW or RETIRED  
- Scope UNDER_REVIEW  
- Other reversible interruption  

Effects:

- Votes cleared  
- Participation epochs terminated  
- Phase → PRE_STANCE  
- Resume requires explicit re-addition of participants  

## 9.2 BLOCK_PERMANENT

Triggered by:

- Authority superseded  
- Scope superseded  
- Referenced Resolution superseded  
- Supersession race loss  
- Governance mutation after VOTING began  
- Structural legitimacy-context invalidation  

Effects:

- Session cannot resume  
- Acceptance permanently impossible  
- Reported as governance_conflict_permanent  
- Explicit user closure required  

While any Session in an Area is BLOCK_PERMANENT, acceptance in that Area is prohibited (ENG-INTEGRITY).

---

# 10. Supersession & Concurrency

Acceptance verifies:

- Authority ACTIVE  
- Scope ACTIVE  
- Referenced Resolution ACTIVE  

First successful acceptance wins.  
Competing sessions → BLOCK_PERMANENT.

Graph rules defined in ENG-SUPERSESSION.

---

# 11. Acceptance Transaction

1. Acquire legitimacy lock  
2. Validate governance preconditions  
3. Validate session state (ACTIVE and VOTING, or PRE_STANCE in Solo Mode)  
4. Validate governance immutability  
5. Validate Authority and Scope usability  
6. Validate referenced Resolution usability  
7. Validate constraints  
8. Evaluate authority rule  
9. On failure → no mutation  
10. Freeze participant epoch snapshot  
11. Freeze candidate snapshot  
12. Freeze stance snapshot  
13. Create Resolution  
14. Mark session ACCEPTED  
15. Emit LEGITIMACY receipt (deterministic content_hash)  
16. Release lock  

Atomic across Session, Resolution, and Receipt.

Crash before commit → no Resolution and no Receipt.

---

# 12. Receipt Derivation & Verification

- ACCEPTED → LEGITIMACY receipt  
- CLOSED → EXPLORATION receipt  

Receipt must include:

- Participant epoch snapshot  
- Candidate snapshot  
- Stance snapshot  
- Authority reference  
- Scope reference  
- Acceptance result  
- Reconfirmation history  
- Deterministic content_hash  

Receipt is immutable.

Verification requires:

1. Session state = ACCEPTED  
2. Receipt exists  
3. Snapshots match canonical session state  
4. content_hash valid  

Receipt absence or mismatch → structural corruption (ENG-INTEGRITY).

Receipt proves legitimacy event; it does not create legitimacy.

---

# 13. Evaluation API

evaluate(session_id) → EvaluationReport

Must include:

- session_state  
- session_phase  
- can_accept  
- blocking_reasons  
- warnings  

Evaluation:

- Strictly non-mutating  
- Deterministic  
- Idempotent  
- Does not emit receipts  
- Does not insert implicit votes  
- Includes governance precondition failures in blocking_reasons  

Acceptance does not depend on prior evaluation.

---

# 14. Atomicity Guarantees

- Acceptance atomic  
- Receipt emission atomic with acceptance  
- Validation failure → no mutation  
- Crash before commit → no legitimacy, no receipt  
- Identical input → identical outcome and identical content_hash  

---

# 15. Engine Invariants

- Governance preconditions enforced before evaluation  
- Only ACCEPTED sessions create Resolutions  
- Only ACCEPTED sessions create LEGITIMACY receipts  
- Governance immutable after first stance  
- Participant identity is epoch-based and session-scoped  
- Resume terminates prior participation epochs  
- participant_id never reused within a Session  
- BLOCK_TEMPORARY resets voting and participation epochs  
- BLOCK_PERMANENT requires explicit closure  
- Receipt snapshot must match frozen session state  
- Deterministic across implementations  
- Receipt integrity mandatory for legitimacy verification  

---

# 16. Heavy Engine Doctrine

The Engine must never:

- Auto-accept  
- Infer consensus  
- Merge participation epochs  
- Reuse participant_id  
- Repair structural violations  
- Mask legitimacy failures  
- Alter receipts once emitted  

Legitimacy is created only by explicit, mechanically validated acceptance.

Receipts preserve the legitimacy event for deterministic reconstruction and admissible verification.