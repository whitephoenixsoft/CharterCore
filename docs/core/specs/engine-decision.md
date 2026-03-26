# ENG-DECISION — Decision Evaluation & Acceptance Orchestration

Status: REFACTORED (v14 – Candidate-Scoped Blocking & Winning-Candidate Eligibility Alignment)  
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
- candidate validity and candidate-level blocking
- session-level blocking determination
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
- determining candidate validity and candidate-level blocking
- determining session-level blocking conditions
- enforcing evaluation ordering

It is not responsible for:

- mutating session state → ENG-SESSION
- computing ACTIVE sets → ENG-SUPERSESSION
- determining usability semantics → ENG-REVIEW-RETIRED
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
- current usability and supersession outcomes supplied by authoritative specifications

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
7. candidate action validity and candidate-level blocking
8. constraint evaluation
9. authority rule evaluation
10. voting outcome determination over eligible candidates
11. session-level blocking determination
12. acceptance eligibility result

Error accumulation and ordering are governed by ENG-ERROR.

ENG-DECISION defines only the logical ordering.

---

# 6. Governance Preconditions

## ENG-DECISION-03 — Preconditions Before Rule Evaluation

Governance preconditions must be satisfied before authority rule evaluation executes.

These include:

- required governance slots exist
- referenced Authority is usable
- referenced Scope is usable when required

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

## ENG-DECISION-05B — Candidate Action Structure Must Be Valid

Each candidate must have a structurally valid action definition as defined by ENG-DOMAIN.

This includes:

- valid candidate_action_type
- payload shape matching candidate_action_type
- locally valid structural action targets where required
- no inferred action targets
- no action semantics derived from informational references

Candidate action validity is required before a candidate may participate in winning determination.

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

Votes are mutable during VOTING.

Vacillation is allowed.

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

- treat unusable references as blocking or invalidating conditions according to their scope
- not reinterpret lifecycle states

---

## ENG-DECISION-07A — Informational References Are Non-Semantic

Informational references:

- must not affect acceptance eligibility
- must not influence voting outcome
- must not influence authority evaluation
- must not influence constraint evaluation
- must not be interpreted as structural action targets

They may be:

- preserved on accepted artifacts according to governing schema
- preserved in receipts according to governing artifact specifications
- immutable once accepted

They are not inputs to decision logic.

---

# 11. Candidate-Level Blocking and Viability

## ENG-DECISION-08 — Candidate-Level Blocking Is Distinct From Session-Level Blocking

Decision evaluation must distinguish:

- candidate-level blocking
- session-level blocking

Candidate-level blocking removes a candidate from current acceptance eligibility without necessarily invalidating the session as a whole.

Candidate-level blocking or invalidity must be evaluated before voting outcome determination.

---

## ENG-DECISION-08A — Candidate-Level Temporary Blocking

A candidate is temporarily blocked when its action depends on a reversibly unusable artifact.

Examples include:

- a candidate action target is UNDER_REVIEW
- a candidate depends on a reversible governance usability suspension that applies to that candidate specifically

A temporarily blocked candidate:

- must not be eligible to win
- remains part of the current session history
- may become eligible again if the blocking condition is removed

---

## ENG-DECISION-08B — Candidate-Level Permanent Blocking

A candidate is permanently blocked when its action depends on an irreversibly unusable or invalidated artifact.

Examples include:

- a supersession target is already SUPERSEDED
- a retirement target is already RETIRED
- a candidate has lost a supersession race
- a candidate action target has become permanently unusable for the action proposed

A permanently blocked candidate:

- must not be eligible to win
- remains part of the current session history
- may not regain eligibility within the same session merely through reevaluation

---

## ENG-DECISION-08C — Invalid Candidates Do Not Participate in Winning Determination

A candidate is invalid when its action definition is structurally or semantically unusable independent of reversible governance conditions.

Examples include:

- malformed payload
- unresolved required local action target
- wrong-area action target
- illegal action-type/payload combination

Invalid candidates:

- must not be eligible to win
- must not be treated as merely informational
- do not create legitimacy paths

---

# 12. Constraint Evaluation

## ENG-DECISION-09 — Constraints Gate Acceptance

Constraints are evaluated as mechanical acceptance conditions.

ENG-DECISION must:

- evaluate all declared constraints
- treat any failing constraint as acceptance failure for the affected acceptance path

Constraint structure and lifecycle are defined in:

- ENG-DOMAIN
- ENG-SESSION

Constraint failure:

- prevents acceptance
- does not mutate state

---

# 13. Authority Rule Evaluation

## ENG-DECISION-10 — Authority Determines Agreement

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

# 14. Voting Outcome Determination

## ENG-DECISION-11 — Winning Candidate Must Be Eligible and Satisfy Authority

A candidate is eligible for acceptance only if all of the following hold:

- it is structurally valid
- it is not blocked
- it satisfies the authority rule under the final vote state
- it complies with all applicable constraints

ENG-DECISION must determine:

- the set of currently eligible candidates
- whether a winning candidate exists among that eligible set

If no eligible candidate satisfies authority:

- acceptance eligibility fails

---

## ENG-DECISION-11A — No Implicit Candidate Selection

ENG-DECISION must not:

- infer a winner without authority rule satisfaction
- select a candidate outside the defined current-round candidate set
- select a blocked or invalid candidate
- break ties using non-deterministic methods

Candidate selection must be fully determined by:

- final vote set
- authority rule
- candidate validity
- candidate blocking status
- applicable constraints

---

# 15. Session-Level Blocking Determination

## ENG-DECISION-12 — Session-Level Blocking Is Reserved for Session-Global Conditions

Decision evaluation must determine whether the session as a whole is:

- allowed
- temporarily blocked
- permanently blocked

Session-level blocking applies only to conditions that invalidate the session’s acceptance context as a whole.

These are distinct from candidate-level blocking conditions.

---

## 15.1 BLOCK_TEMPORARY

Triggered by reversible session-global conditions such as:

- Scope UNDER_REVIEW
- another reversible governance usability condition that affects the session’s governing context as a whole

ENG-DECISION determines the block.

State transitions and resume behavior are defined in ENG-SESSION.

---

## 15.2 BLOCK_PERMANENT

Triggered by irreversible session-global conditions such as:

- Authority invalidation
- Scope supersession or other permanent governance context invalidation
- structural invalidation of the session’s governing context

ENG-DECISION determines that acceptance is impossible for the session in its current identity.

Lifecycle handling is external.

---

# 16. Supersession & Conflict Integration

## ENG-DECISION-13 — Graph Outcomes Are Consumed

ENG-DECISION must consume supersession outcomes from ENG-SUPERSESSION.

This includes:

- whether referenced Resolutions are structurally ACTIVE where required
- whether conflicts exist
- whether a supersession race has been lost

ENG-DECISION must not:

- compute graph structure
- resolve conflicts independently

---

# 17. Acceptance Eligibility Result

## ENG-DECISION-14 — Eligibility Is Binary

After evaluation:

- session is either eligible for acceptance
- or not eligible

If not eligible:

- no mutation occurs

If eligible:

- control transfers to ENG-PERSISTENCE

---

# 18. Receipt Dependency

## ENG-DECISION-15 — Acceptance Requires Receipt Emission

Acceptance requires that:

- a LEGITIMACY receipt will be emitted during atomic commit

Closure requires:

- an EXPLORATION receipt

ENG-DECISION does not define receipt structure.

It requires their existence.

---

# 19. Evaluation API Semantics

## ENG-DECISION-16 — Evaluation Is Pure

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

# 20. Atomicity Boundary

## ENG-DECISION-17 — No Partial Mutation

ENG-DECISION requires:

- failure → no mutation
- success → atomic commit delegated to ENG-PERSISTENCE

ENG-DECISION does not define durability mechanics.

---

# 21. Determinism Guarantee

## ENG-DECISION-18 — Deterministic Behavior

Given identical:

- session state
- governance references
- votes
- constraints
- supersession outcomes
- usability outcomes
- candidate action definitions

ENG-DECISION must produce identical results.

It must not depend on:

- timestamps
- UUID ordering
- storage order
- environment

---

# 22. Engine Invariants (Decision Layer)

- governance preconditions are evaluated before authority rules
- freeze boundary is enforced before acceptance
- candidate set must be valid and non-empty
- candidate action structure must be valid before candidate participation in winning determination
- votes must be valid and current-round scoped
- final vote state determines outcome
- vote vacillation is allowed prior to evaluation
- informational references are non-semantic
- candidate-level blocking is distinct from session-level blocking
- blocked or invalid candidates do not participate in winning determination
- constraints must pass for a candidate to remain acceptance-eligible
- authority rule applies only to eligible candidates
- supersession outcomes must be respected
- eligibility determination is deterministic
- acceptance requires atomic commit defined elsewhere

---

# 23. Prohibited Behaviors

ENG-DECISION must never:

- create legitimacy directly
- bypass atomic commit
- override supersession outcomes
- reinterpret lifecycle states
- infer consensus
- repair invalid structures
- mutate state during evaluation
- treat informational references as structural inputs
- infer structural action targets from informational references
- allow blocked or invalid candidates to win

---

# 24. Mental Model

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
- whether candidates are valid and currently eligible
- whether a valid candidate wins

If the answer is “yes,”  
another layer commits that truth.