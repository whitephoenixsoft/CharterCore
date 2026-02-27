# ENG-DECISION  
Decision Execution, Session Governance, Acceptance Model, and Receipt Verification  
Status: DRAFT (Pre-Freeze – Governance Preconditions Integrated)  
Applies to: Engine Core (V1/V2+)  

This document must be interpreted in conjunction with:

- ENG-DOMAIN  
- ENG-SUPERSESSION  
- ENG-REVIEW-RETIRED  
- ENG-INTEGRITY  
- ENG-RECEIPT  
- ENG-API  

If conflict exists, ENG-INTEGRITY runtime guarantees take precedence.

---

# 1. Purpose

This document defines the complete mechanical model governing:

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
- Receipt generation and verification  
- Evaluation API  
- Concurrency guarantees  

This specification defines how legitimacy is created, recorded, and verified.

---

# 2. Foundational Principles

2.1 Legitimacy is created only by explicit acceptance.  
2.2 Voting is mutable until acceptance.  
2.3 Acceptance is atomic.  
2.4 Governance is frozen once voting begins.  
2.5 Permanent legitimacy-context changes force permanent blocking.  
2.6 Permanent blocks require explicit user closure.  
2.7 Interruptions reset voting.  
2.8 Runtime structural integrity is governed by ENG-INTEGRITY.  
2.9 The engine enforces invariants. Usability belongs to the CLI layer.  
2.10 Receipts formalize closure but do not create legitimacy.  
2.11 Governance must be structurally valid before evaluation may proceed.

---

# 3. Core Entities

3.1 Session — A bounded decision record that may produce at most one accepted Resolution.  
3.2 Participant — Explicitly added identity allowed one stance per candidate.  
3.3 Candidate — Proposal scoped to a session.  
3.4 Constraint — Mechanical acceptance gate declared before voting.  
3.5 Authority — Area-level Resolution defining decision rules.  
3.6 Scope — Area-level Resolution defining contextual boundaries.  
3.7 Resolution — Immutable object created only by successful acceptance.  
3.8 EvaluationReport — Deterministic, side-effect-free session evaluation.  
3.9 Session Receipt — Immutable closure artifact emitted at ACCEPTED or CLOSED.  

---

# 4. Governance Preconditions

Governance preconditions are evaluated before session evaluation or acceptance logic.

If governance preconditions fail, authority evaluation must not execute.

---

## 4.1 Session Type Classification

Each session is exactly one of:

- AUTHORITY  
- SCOPE  
- REGULAR  

Session type is immutable.

---

## 4.2 Governance Rules by Session Type

### AUTHORITY Session

Allowed only if:

- No ACTIVE Authority exists in the Area  
  OR  
- Acceptance would supersede the currently ACTIVE Authority under valid supersession rules.

Failure cases:

- Multiple ACTIVE Authority detected → StructuralIntegrityFailure  
- Attempt to create parallel Authority without supersession → REJECTED with GOVERNANCE_SLOT_VIOLATION

---

### SCOPE Session

Requires:

- Exactly one ACTIVE Authority.

Failure → REJECTED with AUTHORITY_REQUIRED

Authority usability must be ACTIVE at evaluation time.

---

### REGULAR Session

Requires:

- Exactly one ACTIVE Authority  
- Exactly one ACTIVE Scope  

Failure cases:

- No ACTIVE Authority → AUTHORITY_REQUIRED  
- No ACTIVE Scope → SCOPE_REQUIRED  
- Both missing → GOVERNANCE_NOT_INITIALIZED  

Evaluation must not proceed if these prerequisites fail.

---

## 4.3 Deterministic Rejection Codes

If governance preconditions fail, evaluation result must:

- Set can_accept = false  
- Include deterministic blocking reason:
  - GOVERNANCE_NOT_INITIALIZED  
  - AUTHORITY_REQUIRED  
  - SCOPE_REQUIRED  
  - GOVERNANCE_SLOT_VIOLATION  

No authority rule evaluation occurs.

---

# 5. Session Lifecycle

## 5.1 Session States

A session must be exactly one of:

- ACTIVE  
- PAUSED  
- BLOCK_TEMPORARY  
- BLOCK_PERMANENT  
- ACCEPTED  
- CLOSED  

ACCEPTED and CLOSED are terminal.  
BLOCK_PERMANENT is non-resumable but not terminal until explicitly CLOSED.

---

## 5.2 Session Phases

- PRE_STANCE — Governance mutable.  
- VOTING — At least one stance recorded; governance frozen.  
- TERMINAL — ACCEPTED or CLOSED.  

Transitions:

- PRE_STANCE → VOTING: first stance recorded.  
- VOTING → ACCEPTED: successful acceptance.  
- ACTIVE → PAUSED: user action.  
- PAUSED or BLOCK_TEMPORARY → PRE_STANCE: resume (with reconfirmation).  
- BLOCK_PERMANENT → CLOSED: explicit user action only.  
- Any non-terminal → CLOSED: explicit user action.

BLOCK_PERMANENT cannot resume.

---

# 6. Governance Rules

## 6.1 Governance Mutability

Before first stance:

- Participants may be added/removed.  
- Candidates may be added/removed.  
- Constraints may be declared.  

After first stance:

- Participants immutable.  
- Candidates immutable.  
- Constraints immutable.  

Mutation after first stance → BLOCK_PERMANENT.

---

## 6.2 Participant Rules

- Participants must be explicitly added before voting.  
- Each participant counts as one presence unit.  
- One stance per participant per candidate.  
- Stances mutable until acceptance.  
- Non-participants cannot vote.  

Presence equals explicitly added participant set at evaluation time.

On resume:

- Votes cleared.  
- Participant set must be explicitly reconfirmed.  
- Reconfirmation event must be recorded for receipt derivation.

---

## 6.3 Constraint Rules

- Must be declared before first stance.  
- Immutable once VOTING begins.  
- Gate acceptance but do not determine authority outcome.  
- Evaluated before authority rules.  

Constraint mutation during VOTING → BLOCK_PERMANENT.  
Constraint violation → BLOCK_TEMPORARY.

---

# 7. Voting Semantics

Valid stances:

- ACCEPT  
- REJECT  
- ABSTAIN  

Rules:

- Stances mutable before acceptance.  
- Frozen after acceptance.  
- No automatic acceptance.  
- Authority evaluated only on explicit acceptance.

Solo Mode:

- If no stance recorded at acceptance attempt, implicit ACCEPT inserted.  
- Implicit stance becomes part of frozen stance snapshot.

---

# 8. Authority Evaluation

Authority evaluation only occurs after governance preconditions pass.

## 8.1 Authority Types

- SOLE_ACTOR  
- UNANIMOUS_PRESENT  
- MAJORITY_PRESENT  

## 8.2 ACTIVE Definition

A Resolution is usable for legitimacy evaluation only if:

- Not SUPERSEDED  
- Not UNDER_REVIEW  
- Not RETIRED  

ACTIVE usability determined at evaluation time.

---

## 8.3 Mechanical Rules

SOLE_ACTOR:
- Exactly one participant.  
- That participant casts ACCEPT.

UNANIMOUS_PRESENT:
- All present must vote.  
- All must ACCEPT.

MAJORITY_PRESENT:
- accept_count > floor(present / 2)

Abstain counts toward presence but not accept_count.

---

# 9. Blocking Semantics

## 9.1 BLOCK_TEMPORARY

Triggered by:

- Constraint violation  
- Referenced Resolution UNDER_REVIEW  
- Referenced Resolution RETIRED  
- Scope UNDER_REVIEW  
- Other reversible interruption  

Effects:

- Votes cleared  
- Phase → PRE_STANCE  
- Resume required  
- Participant reconfirmation required  

---

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
- Evaluation reports governance_conflict_permanent  
- User must explicitly close session  

While any session in an Area is BLOCK_PERMANENT,
acceptance in that Area is prohibited (see ENG-INTEGRITY).

---

# 10. Supersession and Concurrency

Acceptance must verify:

- Authority usable (ACTIVE)  
- Scope usable (ACTIVE)  
- Referenced Resolution usable (ACTIVE)  

Race:

- First successful acceptance wins  
- Competing sessions → BLOCK_PERMANENT  

Graph integrity rules defined in ENG-SUPERSESSION.

---

# 11. Acceptance Transaction Model

1. Acquire legitimacy lock.  
2. Validate governance preconditions.  
3. Validate session ACTIVE and VOTING (or PRE_STANCE in Solo Mode).  
4. Validate governance immutability.  
5. Validate Authority usability.  
6. Validate Scope usability.  
7. Validate referenced Resolution usability.  
8. Validate constraints.  
9. Evaluate authority rule.  
10. On failure → no mutation.  
11. Freeze participant snapshot.  
12. Freeze candidate snapshot.  
13. Freeze stance snapshot (including implicit vote if applicable).  
14. Create Resolution.  
15. Mark session ACCEPTED.  
16. Emit LEGITIMACY receipt.  
17. Release lock.

Atomic across session, resolution, and receipt.

Crash before commit → no resolution and no receipt.

---

# 12. Receipt Derivation and Verification

## 12.1 Receipt Emission

ACCEPTED → emit LEGITIMACY receipt.  
CLOSED (no acceptance) → emit EXPLORATION receipt.

Receipt must include:

- Participant snapshot  
- Candidate snapshot  
- Stance snapshot  
- Authority reference  
- Scope reference  
- Acceptance result  
- Participant reconfirmation history  
- content_hash  

Receipt is immutable.

---

## 12.2 Legitimacy Verification via Receipt

A Resolution's legitimacy may be verified by:

1. Confirming Resolution references originating session_id.  
2. Confirming session_state = ACCEPTED.  
3. Confirming corresponding LEGITIMACY receipt exists.  
4. Confirming receipt snapshots match Resolution snapshots.  
5. Confirming receipt content_hash valid.

Receipt absence or mismatch indicates structural corruption.

Receipt does not create legitimacy; it proves recorded legitimacy event.

---

# 13. Evaluation API

evaluate(session_id) → EvaluationReport

Must include:

- session_state  
- session_phase  
- can_accept  
- blocking_reasons  
- warnings  

No side effects.  
Must not generate receipts.  
Must include governance precondition failures in blocking_reasons.

---

# 14. Atomicity Guarantees

- Acceptance atomic.  
- Receipt emission atomic with acceptance.  
- Validation failure → no mutation.  
- Crash before commit → no legitimacy, no receipt.  
- Identical inputs → identical outcomes and identical receipt content_hash.

---

# 15. Engine Invariants

- Governance preconditions enforced before evaluation.  
- Only ACCEPTED sessions create Resolutions.  
- Only ACCEPTED sessions create LEGITIMACY receipts.  
- Governance immutable after first stance.  
- BLOCK_TEMPORARY resets voting and requires reconfirmation.  
- BLOCK_PERMANENT requires explicit closure.  
- Receipt snapshot must match frozen session state.  
- Deterministic across implementations.

---

# 16. Heavy Engine Doctrine

The engine enforces legitimacy mechanics.

It must never:

- Auto-accept  
- Infer consensus  
- Repair structural violations  
- Mask legitimacy failures  
- Alter receipts once emitted  
- Bypass governance preconditions  

Legitimacy is created only when explicitly commanded and mechanically validated.

Receipts preserve that legitimacy event for deterministic reconstruction and admissible verification.