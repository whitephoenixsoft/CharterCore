# ENG-DECISION — Decision Execution, Session Governance, Acceptance, and Receipt Verification (Rewritten v5)
Status: DRAFT (v5 – Governance, Receipt & Degraded Mode Integrated)
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Defines the mechanical model for:

- Session lifecycle  
- Governance mutation rules  
- Governance bootstrap preconditions  
- Participant mechanics  
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
7. Interruptions reset voting and require participant reconfirmation.  
8. Runtime structural integrity governed by ENG-INTEGRITY.  
9. Engine enforces invariants; CLI handles usability.  
10. Receipts formalize closure; they do not create legitimacy.  
11. Governance must be structurally valid before evaluation.  
12. Degraded/read-only mode allows DAG export if receipts or structural validation fail.

---

# 3. Core Entities

- **Session:** Bounded decision record; may produce one accepted Resolution.  
- **Participant:** Explicit identity; one stance per candidate.  
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

- `can_accept = false`  
- Include blocking reason (`GOVERNANCE_NOT_INITIALIZED`, `AUTHORITY_REQUIRED`, `SCOPE_REQUIRED`, `GOVERNANCE_SLOT_VIOLATION`)  
- Authority evaluation **skipped**.

---

# 5. Session Lifecycle

## 5.1 States

- ACTIVE, PAUSED, BLOCK_TEMPORARY, BLOCK_PERMANENT, ACCEPTED, CLOSED  
- ACCEPTED/CLOSED → terminal  
- BLOCK_PERMANENT → non-resumable until explicit CLOSED

## 5.2 Phases

- PRE_STANCE → VOTING → TERMINAL  
- PRE_STANCE → first stance recorded → VOTING  
- VOTING → successful acceptance → ACCEPTED  
- ACTIVE → PAUSED → PRE_STANCE on resume (participant reconfirmation required)  
- BLOCK_PERMANENT → CLOSED (user action)  

---

# 6. Governance Rules

## 6.1 Mutability

- Before first stance: Participants, Candidates, Constraints mutable.  
- After first stance: immutable.  
- Mutation after first stance → BLOCK_PERMANENT.  

## 6.2 Participant Rules

- Must be explicitly added.  
- One stance per participant per candidate.  
- Stances mutable until acceptance.  
- Resuming resets votes; participant set reconfirmation required.  

## 6.3 Constraint Rules

- Declared before first stance.  
- Immutable once VOTING begins.  
- Constraint mutation → BLOCK_PERMANENT  
- Constraint violation → BLOCK_TEMPORARY

---

# 7. Voting Semantics

- Stances: ACCEPT, REJECT, ABSTAIN  
- Mutable before acceptance, frozen after  
- Authority evaluated **only** on explicit acceptance  

**Solo Mode:** Implicit ACCEPT inserted if no stances exist.

---

# 8. Authority Evaluation

## 8.1 Types

- SOLE_ACTOR, UNANIMOUS_PRESENT, MAJORITY_PRESENT

## 8.2 ACTIVE Definition

- Resolution usable if: not SUPERSEDED, not UNDER_REVIEW, not RETIRED  
- Receipt must exist and match session snapshots for legitimacy verification  

## 8.3 Rules

- SOLE_ACTOR: single participant, ACCEPT stance  
- UNANIMOUS_PRESENT: all present must ACCEPT  
- MAJORITY_PRESENT: accept_count > floor(present/2)  
- Abstain counts toward presence, not accept_count

---

# 9. Blocking Semantics

## 9.1 BLOCK_TEMPORARY

Triggered by:

- Constraint violation  
- Referenced Resolution UNDER_REVIEW/RETIRED  
- Scope UNDER_REVIEW  
- Other reversible interruption  

Effects:

- Votes cleared  
- Phase → PRE_STANCE  
- Resume requires participant reconfirmation

## 9.2 BLOCK_PERMANENT

Triggered by:

- Authority superseded  
- Scope superseded  
- Referenced Resolution superseded  
- Supersession race loss  
- Governance mutation post-VOTING  
- Structural legitimacy-context invalidation  

Effects:

- Cannot resume  
- Acceptance impossible  
- Reported governance_conflict_permanent  
- User must explicitly close  

While any session in Area is BLOCK_PERMANENT, acceptance in Area prohibited (ENG-INTEGRITY).

---

# 10. Supersession & Concurrency

Acceptance verifies:

- Authority ACTIVE  
- Scope ACTIVE  
- Referenced Resolution ACTIVE  

First-accept wins; competing sessions → BLOCK_PERMANENT.  
Graph rules in ENG-SUPERSESSION.

---

# 11. Acceptance Transaction

1. Acquire legitimacy lock  
2. Validate governance preconditions  
3. Validate session state (ACTIVE/VOTING or PRE_STANCE in Solo Mode)  
4. Validate governance immutability  
5. Validate Authority & Scope usability  
6. Validate referenced Resolution usability  
7. Validate constraints  
8. Evaluate authority rule  
9. On failure → no mutation  
10. Freeze participant, candidate, stance snapshots  
11. Create Resolution  
12. Mark session ACCEPTED  
13. Emit LEGITIMACY receipt (content_hash deterministic)  
14. Release lock  

Atomic across session, Resolution, and receipt.  
Crash before commit → no legitimacy, no receipt.  
Degraded/read-only mode may allow DAG export if receipt verification fails.

---

# 12. Receipt Derivation & Verification

- ACCEPTED → LEGITIMACY receipt  
- CLOSED → EXPLORATION receipt  
- Must include participant, candidate, stance snapshots, Authority, Scope, result, reconfirmation history, content_hash  
- Immutable  
- Verification confirms: session ACCEPTED, receipt exists, snapshots match, content_hash valid  
- Missing/mismatched receipt → structural corruption (ENG-ERROR)  
- Receipt proves legitimacy event, does not create it

---

# 13. Evaluation API

`evaluate(session_id)` → EvaluationReport

- Includes session_state, session_phase, can_accept, blocking_reasons, warnings  
- No side effects  
- Does **not** generate receipts  
- Governance precondition failures included in blocking_reasons

---

# 14. Atomicity Guarantees

- Acceptance atomic  
- Receipt emission atomic with acceptance  
- Validation failure → no mutation  
- Crash before commit → no legitimacy, no receipt  
- Identical input → identical outcome and receipt content_hash

---

# 15. Engine Invariants

- Governance preconditions enforced before evaluation  
- Only ACCEPTED sessions create Resolutions and LEGITIMACY receipts  
- Governance immutable after first stance  
- BLOCK_TEMPORARY resets voting & requires reconfirmation  
- BLOCK_PERMANENT requires explicit closure  
- Receipt snapshots match frozen session state  
- Deterministic across implementations  
- Receipt integrity mandatory for legitimacy verification  
- Degraded mode allows read-only DAG export on receipt or structural failure

---

# 16. Heavy Engine Doctrine

- Must never auto-accept, infer consensus, repair violations, mask failures, or alter receipts  
- Legitimacy created **only** by explicit, mechanically validated acceptance  
- Receipts preserve legitimacy event for deterministic reconstruction and admissible verification