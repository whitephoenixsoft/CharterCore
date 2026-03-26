# ENG-DECISION — Decision Evaluation & Acceptance Orchestration

Status: REFACTORED (v13 – Voting Finality, Candidate Validity & Reference Alignment)  
Applies to: Engine Core (V1/V2+)  

Authority: Behavioral orchestration layer for decision evaluation and acceptance eligibility.

Subordinate to:

- ENG-DOMAIN
- ENG-SESSION
- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-RECEIPT
- ENG-INTEGRITY
- ENG-PERSISTENCE
- ENG-ERROR
- ENG-CANON
- ENG-SPECVERIFY
- ENG-COMPILATION

---

# 1. Purpose

ENG-DECISION defines how the Engine determines whether a session is eligible for acceptance.

It is the authoritative specification for:

- governance precondition evaluation ordering
- authority rule evaluation
- constraint evaluation
- voting outcome determination
- blocking determination at the decision layer
- acceptance eligibility determination
- orchestration of validation prior to atomic commit

ENG-DECISION does not define:

- domain object schemas
- session lifecycle mechanics
- supersession graph rules or ACTIVE derivation
- UNDER_REVIEW / RETIRED semantics
- receipt structure or hashing
- atomic commit implementation
- structural halt or degraded-mode policy
- EvaluationReport schema or ordering

Those belong to their respective specifications.

ENG-DECISION coordinates them.

---

# 2. Core Principle

## ENG-DECISION-01 — Acceptance Is Eligibility + Atomic Commit

Acceptance is a two-stage process:

1. **Eligibility determination (this specification)**
2. **Atomic commit (ENG-PERSISTENCE)**

ENG-DECISION is responsible only for determining eligibility.

If eligibility fails:

- no mutation occurs
- no legitimacy is created

If eligibility succeeds:

- control transfers to ENG-PERSISTENCE for atomic commit

---

# 3. Scope of Responsibility

ENG-DECISION is the behavioral decision layer.

It is responsible for:

- determining whether a session can accept
- evaluating authority rule satisfaction
- evaluating constraints
- determining voting outcome
- determining blocking conditions
- enforcing evaluation ordering

It is not responsible for:

- mutating session state → ENG-SESSION
- computing ACTIVE sets → ENG-SUPERSESSION
- determining usability → ENG-REVIEW-RETIRED
- validating structural integrity → ENG-INTEGRITY
- producing EvaluationReports → ENG-ERROR
- committing state → ENG-PERSISTENCE

---

# 4. Decision Evaluation Inputs

Decision evaluation operates over the current session round defined in ENG-SESSION.

Inputs include:

- participant set
- candidate set
- constraint set
- vote set
- authority_snapshot_id
- scope_snapshot_id
- session state and phase

All structure and mutability rules are defined externally.

ENG-DECISION consumes them without redefining them.

---

# 5. Evaluation Ordering

## ENG-DECISION-02 — Deterministic Validation Sequence

Decision evaluation must execute in a deterministic sequence.

At minimum:

1. governance preconditions
2. session state eligibility
3. governance immutability (freeze boundary)
4. candidate set validity
5. vote set validity
6. referenced artifact usability
7. constraint evaluation
8. authority rule evaluation
9. voting outcome determination
10. blocking determination
11. acceptance eligibility result

Error accumulation and ordering are governed by ENG-ERROR.

ENG-DECISION defines only the logical ordering.

---

# 6. Governance Preconditions

## ENG-DECISION-03 — Preconditions Before Rule Evaluation

Governance preconditions must be satisfied before authority rule evaluation executes.

These include:

- required governance slots exist
- referenced Authority is usable
- referenced Scope is usable (when required)

Definitions of:

- slot validity
- ACTIVE derivation
- usability

are external and provided by:

- ENG-SUPERSESSION
- ENG-REVIEW-RETIRED
- ENG-INTEGRITY

If governance preconditions fail:

- authority rule evaluation must not execute
- acceptance eligibility fails deterministically

---

## 6.1 Session-Type Preconditions

Session-type requirements are defined structurally in ENG-DOMAIN and behaviorally in ENG-SESSION.

ENG-DECISION enforces:

- AUTHORITY sessions require valid authority slot conditions
- SCOPE sessions require usable Authority
- REGULAR sessions require usable Authority and Scope

---

# 7. Governance Mutability Boundary

## ENG-DECISION-04 — Freeze Boundary Enforcement

After the first stance within a round:

- participant set must be treated as frozen
- candidate set must be treated as frozen
- constraint set must be treated as frozen
- informational reference sets must be treated as frozen

Mutation rules are defined in ENG-SESSION.

ENG-DECISION requires that:

- violations of the freeze boundary invalidate acceptance eligibility

Error classification is defined in ENG-ERROR.

---

# 8. Candidate Validity

## ENG-DECISION-05 — Candidate Set Must Be Valid

Acceptance requires a valid candidate set.

Rules:

- candidate set must not be empty
- all candidates must belong to the current round
- candidate identities must be unique
- candidates must conform to ENG-DOMAIN schema

If candidate set is invalid:

- acceptance eligibility fails

---

## ENG-DECISION-05A — Accepted Candidate Must Be From Current Round

The candidate selected for acceptance must:

- exist in the current round candidate set
- not originate from prior rounds
- not be externally injected or inferred

Acceptance must fail if:

- the selected candidate does not exist in the current round

---

# 9. Vote Validity

## ENG-DECISION-06 — Vote Set Must Be Valid

Votes must satisfy:

- each vote references an existing participant in the current round
- each vote references a candidate in the current round
- no duplicate vote per participant per candidate
- all votes belong to the current round

Vote validity is structural and must be enforced before authority evaluation.

---

## ENG-DECISION-06A — Final Vote State Is Authoritative

Votes are mutable during VOTING (vacillation allowed).

At evaluation time:

- only the final vote state is authoritative
- prior vote states must not influence evaluation

ENG-DECISION must not consider historical vote transitions.

---

## ENG-DECISION-06B — Solo Mode Consumption

If solo mode conditions are met:

- ENG-SESSION must have materialized a valid vote prior to evaluation

ENG-DECISION:

- treats this vote as a standard vote
- must not create or infer votes

---

# 10. Referenced Artifact Usability

## ENG-DECISION-07 — Usability Is Consumed, Not Defined

Decision evaluation must respect usability constraints of referenced objects.

These include:

- UNDER_REVIEW
- RETIRED
- SUPERSEDED

Semantics are defined in:

- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION

ENG-DECISION must:

- treat unusable references as blocking conditions
- not reinterpret lifecycle states

---

## ENG-DECISION-07A — Informational References Are Non-Semantic

Informational references:

- must not affect acceptance eligibility
- must not influence voting outcome
- must not influence authority evaluation
- must not influence constraint evaluation

They are:

- passed through to accepted artifacts
- included in receipts
- immutable once accepted

They are not inputs to decision logic.

---

# 11. Constraint Evaluation

## ENG-DECISION-08 — Constraints Gate Acceptance

Constraints are evaluated as mechanical acceptance conditions.

ENG-DECISION must:

- evaluate all declared constraints
- treat any failing constraint as acceptance failure

Constraint structure and lifecycle are defined in:

- ENG-DOMAIN
- ENG-SESSION

Constraint failure:

- prevents acceptance
- does not mutate state

---

# 12. Authority Rule Evaluation

## ENG-DECISION-09 — Authority Determines Agreement

Authority evaluation executes only if prior stages succeed.

Authority rules are defined by the Authority Resolution referenced in the session.

ENG-DECISION must:

- apply the authority rule deterministically to the vote set
- respect participant eligibility and round boundaries

Examples include:

- SOLE_ACTOR
- UNANIMOUS_PRESENT
- MAJORITY_PRESENT

Exact authority definitions belong to governance artifacts, not this specification.

---

# 13. Voting Outcome Determination

## ENG-DECISION-10 — Winning Candidate Must Satisfy Authority

A candidate is eligible for acceptance only if:

- it satisfies the authority rule under the final vote state
- it complies with all constraints

ENG-DECISION must determine:

- whether a winning candidate exists under authority rules

If no candidate satisfies authority:

- acceptance eligibility fails

---

## ENG-DECISION-10A — No Implicit Candidate Selection

ENG-DECISION must not:

- infer a winner without authority rule satisfaction
- select a candidate outside the defined candidate set
- break ties using non-deterministic methods

Candidate selection must be fully determined by:

- vote set
- authority rule

---

# 14. Blocking Determination

## ENG-DECISION-11 — Decision Layer Blocking

Decision evaluation must determine whether acceptance is:

- allowed
- temporarily blocked
- permanently blocked

Blocking causes are derived from:

- usability constraints
- supersession outcomes
- governance invalidation
- session state

---

## 14.1 BLOCK_TEMPORARY

Triggered by reversible conditions such as:

- UNDER_REVIEW references
- RETIRED references
- Scope under review

ENG-DECISION determines the block.

State transitions and resume behavior are defined in ENG-SESSION.

---

## 14.2 BLOCK_PERMANENT

Triggered by irreversible conditions such as:

- SUPERSEDED references
- supersession race loss
- structural invalidation of session context

ENG-DECISION determines that acceptance is impossible.

Lifecycle handling is external.

---

# 15. Supersession & Conflict Integration

## ENG-DECISION-12 — Graph Outcomes Are Consumed

ENG-DECISION must consume supersession outcomes from ENG-SUPERSESSION.

This includes:

- whether referenced resolutions are structurally ACTIVE
- whether conflicts exist
- whether a supersession race has been lost

ENG-DECISION must not:

- compute graph structure
- resolve conflicts independently

---

# 16. Acceptance Eligibility Result

## ENG-DECISION-13 — Eligibility Is Binary

After evaluation:

- session is either eligible for acceptance
- or not eligible

If not eligible:

- no mutation occurs

If eligible:

- control transfers to ENG-PERSISTENCE

---

# 17. Receipt Dependency

## ENG-DECISION-14 — Acceptance Requires Receipt Emission

Acceptance requires that:

- a LEGITIMACY receipt will be emitted during atomic commit

Closure requires:

- an EXPLORATION receipt

ENG-DECISION does not define receipt structure.

It requires their existence.

---

# 18. Evaluation API Semantics

## ENG-DECISION-15 — Evaluation Is Pure

Evaluation must be:

- deterministic
- side-effect free
- non-mutating

It must not:

- emit receipts
- emit audit
- alter session state

EvaluationReport behavior is defined in ENG-ERROR.

---

# 19. Atomicity Boundary

## ENG-DECISION-16 — No Partial Mutation

ENG-DECISION requires:

- failure → no mutation
- success → atomic commit delegated to ENG-PERSISTENCE

ENG-DECISION does not define durability mechanics.

---

# 20. Determinism Guarantee

## ENG-DECISION-17 — Deterministic Behavior

Given identical:

- session state
- governance references
- votes
- constraints
- supersession outcomes
- usability outcomes

ENG-DECISION must produce identical results.

It must not depend on:

- timestamps
- UUID ordering
- storage order
- environment

---

# 21. Engine Invariants (Decision Layer)

- governance preconditions evaluated before authority rules
- freeze boundary enforced before acceptance
- candidate set must be valid and non-empty
- votes must be valid and current-round scoped
- final vote state determines outcome
- vote vacillation allowed prior to evaluation
- unusable references block acceptance
- informational references are non-semantic
- constraints must pass before authority evaluation succeeds
- authority determines winning candidate
- supersession outcomes must be respected
- eligibility determination is deterministic
- acceptance requires atomic commit (external)

---

# 22. Prohibited Behaviors

ENG-DECISION must never:

- create legitimacy directly
- bypass atomic commit
- override supersession outcomes
- reinterpret lifecycle states
- infer consensus
- repair invalid structures
- mutate state during evaluation
- treat informational references as structural inputs

---

# 23. Mental Model

ENG-DECISION answers:

“Can this session be accepted right now, and is there a valid winning candidate?”

It does not:

- mutate the world
- define the graph
- define structure
- define receipts

It evaluates:

- whether governance is valid
- whether votes satisfy authority
- whether constraints pass
- whether a valid candidate wins

If the answer is “yes,”  
another layer commits that truth. 