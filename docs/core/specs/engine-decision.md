# ENG-DECISION — Decision Evaluation, Governance Validation & Acceptance Orchestration

Status: REFACTORED (v11 – Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)  

Authority: Subordinate to ENG-DOMAIN, ENG-SESSION, ENG-SUPERSESSION, ENG-REVIEW-RETIRED, ENG-RECEIPT, ENG-INTEGRITY, ENG-PERSISTENCE, ENG-ERROR, ENG-CANON, ENG-SPECVERIFY, ENG-COMPILATION

---

# 1. Purpose

ENG-DECISION defines the orchestration rules for deterministic decision evaluation and acceptance.

It governs:

- Governance precondition evaluation
- Authority rule evaluation
- Constraint evaluation
- Blocking and closure semantics
- Acceptance orchestration
- Deterministic evaluation reporting
- Incremental compilation participation at the decision layer

ENG-DECISION does not redefine:

- Structural schemas
- Supersession graph derivation
- UNDER_REVIEW / RETIRED state semantics
- Receipt structure
- Atomic persistence guarantees
- Canonical serialization rules
- Rule identity verification semantics

Those are defined respectively in:

- ENG-DOMAIN
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-RECEIPT
- ENG-PERSISTENCE
- ENG-CANON
- ENG-SPECVERIFY

Legitimacy must be created, recorded, and verifiable deterministically.

---

# 2. Foundational Principles

1. Legitimacy is created only through explicit ACCEPTED session transition.
2. Voting remains mutable until acceptance occurs.
3. Acceptance orchestration must be deterministic.
4. Governance mutability ends at the first stance within a round.
5. Structural legitimacy-context changes may permanently invalidate a session.
6. BLOCK_PERMANENT requires explicit closure.
7. Interruptions terminate participation epochs and create a new round.
8. Structural integrity is governed by ENG-INTEGRITY.
9. Receipts formalize closure but do not create legitimacy.
10. Governance must be structurally valid before authority rule evaluation executes.
11. Historical round state exists only in receipts.
12. Incremental compilation must not alter runtime legitimacy unless fully validated.

---

# 3. Scope of This Specification

ENG-DECISION is the behavioral decision layer.

It is responsible for:

- determining whether a session can accept
- evaluating authority rule satisfaction
- evaluating constraints
- applying blocking semantics
- orchestrating the acceptance decision path

ENG-DECISION is not the authority for:

- session lifecycle structure → ENG-SESSION
- object structure → ENG-DOMAIN
- ACTIVE derivation and graph conflict → ENG-SUPERSESSION
- usability suspension and deprecation → ENG-REVIEW-RETIRED
- receipt content and canonical form → ENG-RECEIPT
- atomic commit boundaries → ENG-PERSISTENCE
- structural halt conditions → ENG-INTEGRITY
- EvaluationReport schema and error ordering → ENG-ERROR

---

# 4. Governance Preconditions

## ENG-DECISION-01 — Preconditions Before Authority Evaluation

Governance preconditions must be validated before authority rule evaluation.

Failure prevents acceptance evaluation.

Governance slot validity and ACTIVE derivation are not defined here. They are determined through:

- ENG-INTEGRITY
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED

---

## 4.1 Session Type

Session type is defined by ENG-DOMAIN and governed operationally by ENG-SESSION.

Recognized session categories for decision evaluation are:

- AUTHORITY
- SCOPE
- REGULAR

Incremental compilation behavior is defined in ENG-COMPILATION and may reuse decision validation rules when replaying historical sessions.

---

## 4.2 Session Type Preconditions

### AUTHORITY

Allowed if:

- No ACTIVE Authority exists
- OR valid supersession of the current ACTIVE Authority is being attempted

ACTIVE derivation and slot exclusivity are defined in:

- ENG-SUPERSESSION
- ENG-INTEGRITY

### SCOPE

Requires exactly one usable Authority.

### REGULAR

Requires exactly one usable Authority and exactly one usable Scope.

Usability is not defined here. It is determined through:

- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION
- ENG-INTEGRITY

---

## 4.3 Deterministic Rejection

If governance preconditions fail:

- can_accept = false
- blocking_reasons must include the applicable governance failure

Authority rule evaluation must not execute.

Error classification and reporting are defined in ENG-ERROR.

---

# 5. Decision Evaluation Inputs

Decision evaluation operates over the current session round defined in ENG-SESSION.

The decision layer consumes:

- participant set
- candidate set
- constraint set
- vote set
- governance snapshot references
- current session state
- current session phase

Round structure, epoch rules, and mutability boundaries are defined in ENG-SESSION and ENG-DOMAIN.

---

# 6. Governance Mutability Boundary

## ENG-DECISION-02 — Freeze Boundary Enforcement

Before first stance:

- participants may mutate
- candidates may mutate
- constraints may mutate

After first stance:

- participants frozen
- candidates frozen
- constraints frozen

Mutation after the freeze boundary must fail deterministically.

Freeze boundary mechanics are structurally defined in ENG-SESSION.  
Error semantics are defined in ENG-ERROR.

---

# 7. Authority Rule Evaluation

## ENG-DECISION-03 — Authority Rule Execution

Authority evaluation executes only after:

- governance preconditions succeed
- structural validation succeeds
- referenced governance artifacts are usable
- blocking conditions are absent

Authority mechanics consume the active Authority rule referenced by the session governance snapshot.

Authority artifact usability is determined externally through:

- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-INTEGRITY

---

## 7.1 Mechanical Rules

Supported authority rule forms include:

- SOLE_ACTOR
- UNANIMOUS_PRESENT
- MAJORITY_PRESENT

Behavior:

SOLE_ACTOR  
- exactly one participant must be present
- exactly one ACCEPT stance required

UNANIMOUS_PRESENT  
- all present participants must vote
- all counted votes must be ACCEPT

MAJORITY_PRESENT  
- accept_count > floor(present / 2)

Constraint semantics may further restrict eligibility.

The Authority artifact itself is governed by:

- ENG-DOMAIN
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED

---

## 7.2 Solo Mode

If session conditions permit solo evaluation, the deterministic implicit ACCEPT vote rule is applied as defined in ENG-SESSION.

ENG-DECISION consumes that behavior for rule evaluation.  
It does not define vote object structure.

---

# 8. Constraint Evaluation

## ENG-DECISION-04 — Constraints Are Mechanical Acceptance Gates

Constraint objects and their schema are defined in ENG-DOMAIN.

Constraint lifecycle and mutability are defined in ENG-SESSION.

ENG-DECISION is responsible for evaluating whether declared constraints permit acceptance.

Rules:

- constraints may tighten authority rules
- constraint failure prevents acceptance
- constraint failure must not mutate session state

If constraints fail, the session remains non-accepted and may only proceed according to lifecycle rules defined elsewhere.

Error reporting is defined in ENG-ERROR.

---

# 9. Blocking Semantics

## ENG-DECISION-05 — Decision Layer Blocking

The decision layer applies blocking outcomes to the session based on current usability and conflict conditions.

Decision-layer blocking does not define the underlying state semantics of referenced objects.  
It consumes them from:

- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION
- ENG-INTEGRITY

---

## 9.1 BLOCK_TEMPORARY

Triggered by reversible decision conditions such as:

- referenced Resolution unusable due to UNDER_REVIEW
- referenced Resolution unusable due to RETIRED
- referenced Scope unusable due to UNDER_REVIEW
- other reversible governance usability failures

BLOCK_TEMPORARY semantics, vote clearing, and resume behavior are defined in:

- ENG-SESSION
- ENG-REVIEW-RETIRED

ENG-DECISION only determines that acceptance cannot proceed while the blocking condition exists.

---

## 9.2 BLOCK_PERMANENT

Triggered by irreversible decision conditions such as:

- referenced Authority or Scope structurally invalid for continued acceptance
- referenced Resolution SUPERSEDED
- supersession race loss
- governance mutation after freeze boundary
- structural legitimacy-context invalidation
- irreversible supersession conflict

BLOCK_PERMANENT lifecycle semantics are defined in ENG-SESSION.  
Graph conflict authority belongs to ENG-SUPERSESSION.

ENG-DECISION applies the result to acceptance eligibility.

---

## 9.3 Area Blocking Invariant

If Area-level blocking conditions prohibit acceptance, no session may ACCEPT.

Area-wide structural safety and halt semantics are defined in ENG-INTEGRITY.  
Session-level closure rules are defined in ENG-SESSION.

---

# 10. Supersession & Conflict Participation

## ENG-DECISION-06 — Decision Layer Consumes Graph Outcomes

ENG-DECISION does not define supersession graph rules.

It consumes graph validity and conflict outcomes from ENG-SUPERSESSION, including:

- whether referenced Resolutions are structurally ACTIVE
- whether a supersession target is valid
- whether a conflict exists
- whether first-successful acceptance has already occurred
- whether replay or compilation conflicts invalidate acceptance

For incremental compilation, canonical historical ordering and replay behavior are defined in ENG-COMPILATION.

---

# 11. Acceptance Orchestration

## ENG-DECISION-07 — Acceptance Is a Coordinated Validation Procedure

Acceptance is not defined here as a persistence operation.

ENG-DECISION defines the ordered behavioral validation that must succeed before atomic commit can occur.

Acceptance orchestration must:

1. validate governance preconditions
2. validate current session state
3. validate governance immutability
4. validate referenced artifact usability
5. validate constraints
6. evaluate authority rule
7. determine blocking outcome or acceptance eligibility

If validation fails:

- no legitimacy is created
- no receipt is emitted
- no structural mutation may occur

If validation succeeds:

- control passes to the atomic persistence boundary defined in ENG-PERSISTENCE

---

## 11.1 Acceptance Dependencies

Successful acceptance requires coordinated success from:

- ENG-SESSION → lifecycle and round state
- ENG-INTEGRITY → structural validity
- ENG-SUPERSESSION → ACTIVE derivation and graph conflict checks
- ENG-REVIEW-RETIRED → usability suspension checks
- ENG-PERSISTENCE → atomic commit
- ENG-RECEIPT → terminal receipt structure
- ENG-SPECVERIFY → rule provenance binding

ENG-DECISION does not duplicate those rules.

---

# 12. Receipt Relationship

## ENG-DECISION-08 — Decision Layer Requires, But Does Not Define, Receipt Emission

Receipts are emitted only on terminal transition.

Decision success at ACCEPTED requires:

- LEGITIMACY receipt emission during atomic commit

Decision success at explicit closure requires:

- EXPLORATION receipt emission during atomic commit

Receipt structure, canonical serialization, and integrity rules are defined in:

- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

ENG-DECISION only depends on their existence and deterministic availability.

---

# 13. Evaluation API Semantics

## ENG-DECISION-09 — Evaluation Is Pure

evaluate(session_id) must return a deterministic, side-effect-free decision result.

Evaluation:

- must not mutate session state
- must not emit receipts
- must not emit audit
- must not alter votes
- must not alter session lifecycle

EvaluationReport schema, ordering, and error classification are defined in ENG-ERROR.

Acceptance must not depend on prior evaluation.

---

# 14. Atomicity Boundary

## ENG-DECISION-10 — Atomicity Is Required but Defined Elsewhere

Acceptance must be atomic, but ENG-DECISION is not the authority for atomic durability rules.

Atomic commit semantics are defined in ENG-PERSISTENCE.

ENG-DECISION requires that:

- validation failure produces no mutation
- success hands off to atomic commit
- crash before commit results in no legitimacy creation

---

# 15. Determinism Guarantee

## ENG-DECISION-11 — Deterministic Behavioral Execution

Given identical:

- session runtime structures
- governance references
- vote state
- constraint state
- supersession outcomes
- usability outcomes

ENG-DECISION must produce identical behavioral results.

Behavior must not depend on:

- storage order
- runtime timing
- external host behavior
- timestamp ordering
- UUID timestamp ordering

Canonical output formatting belongs to ENG-ERROR and ENG-CANON.

---

# 16. Engine Invariants at the Decision Layer

- Governance preconditions enforced before authority rule evaluation
- Only ACCEPTED sessions may create Resolutions
- Governance immutable after first stance
- Votes never cross round boundaries
- BLOCK_TEMPORARY requires external resolution plus resume semantics defined elsewhere
- BLOCK_PERMANENT requires explicit closure semantics defined elsewhere
- RETIRED / UNDER_REVIEW usability must be respected
- Supersession conflict outcomes must be respected
- Incremental compilation may reuse decision validation but does not bypass runtime legitimacy rules

---

# 17. Heavy Engine Doctrine

The decision layer must never:

- auto-accept
- infer consensus
- repair structural violations
- override supersession outcomes
- reinterpret historical legitimacy under different rules
- alter emitted receipts
- bypass atomic persistence requirements
- bypass usability suspension rules

Legitimacy is created only through explicit, mechanically validated acceptance coordinated across the authoritative specifications.