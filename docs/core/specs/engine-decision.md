# ENG-DECISION — Decision Evaluation & Acceptance Orchestration

Status: REFACTORED (v12 – Fully Reference-Aligned Orchestration Model)  
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
4. referenced artifact usability
5. constraint evaluation
6. authority rule evaluation
7. blocking determination
8. acceptance eligibility result

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

Mutation rules are defined in ENG-SESSION.

ENG-DECISION requires that:

- violations of the freeze boundary invalidate acceptance eligibility

Error classification is defined in ENG-ERROR.

---

# 8. Referenced Artifact Usability

## ENG-DECISION-05 — Usability Is Consumed, Not Defined

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

# 9. Constraint Evaluation

## ENG-DECISION-06 — Constraints Gate Acceptance

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

# 10. Authority Rule Evaluation

## ENG-DECISION-07 — Authority Determines Agreement

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

## 10.1 Solo Mode

If session conditions allow implicit voting:

- ENG-DECISION consumes solo behavior defined in ENG-SESSION

It does not define vote creation semantics.

---

# 11. Blocking Determination

## ENG-DECISION-08 — Decision Layer Blocking

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

## 11.1 BLOCK_TEMPORARY

Triggered by reversible conditions such as:

- UNDER_REVIEW references
- RETIRED references
- Scope under review

ENG-DECISION determines the block.

State transitions and resume behavior are defined in ENG-SESSION.

---

## 11.2 BLOCK_PERMANENT

Triggered by irreversible conditions such as:

- SUPERSEDED references
- supersession race loss
- structural invalidation of session context

ENG-DECISION determines that acceptance is impossible.

Lifecycle handling is external.

---

# 12. Supersession & Conflict Integration

## ENG-DECISION-09 — Graph Outcomes Are Consumed

ENG-DECISION must consume supersession outcomes from ENG-SUPERSESSION.

This includes:

- whether referenced resolutions are structurally ACTIVE
- whether conflicts exist
- whether a supersession race has been lost

ENG-DECISION must not:

- compute graph structure
- resolve conflicts independently

---

# 13. Acceptance Eligibility Result

## ENG-DECISION-10 — Eligibility Is Binary

After evaluation:

- session is either eligible for acceptance
- or not eligible

If not eligible:

- no mutation occurs

If eligible:

- control transfers to ENG-PERSISTENCE

---

# 14. Receipt Dependency

## ENG-DECISION-11 — Acceptance Requires Receipt Emission

Acceptance requires that:

- a LEGITIMACY receipt will be emitted during atomic commit

Closure requires:

- an EXPLORATION receipt

ENG-DECISION does not define receipt structure.

It requires their existence.

---

# 15. Evaluation API Semantics

## ENG-DECISION-12 — Evaluation Is Pure

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

# 16. Atomicity Boundary

## ENG-DECISION-13 — No Partial Mutation

ENG-DECISION requires:

- failure → no mutation
- success → atomic commit delegated to ENG-PERSISTENCE

ENG-DECISION does not define durability mechanics.

---

# 17. Determinism Guarantee

## ENG-DECISION-14 — Deterministic Behavior

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

# 18. Engine Invariants (Decision Layer)

- governance preconditions evaluated before authority rules
- freeze boundary enforced before acceptance
- unusable references block acceptance
- constraints must pass before authority evaluation succeeds
- supersession outcomes must be respected
- eligibility determination is deterministic
- acceptance requires atomic commit (external)

---

# 19. Prohibited Behaviors

ENG-DECISION must never:

- create legitimacy directly
- bypass atomic commit
- override supersession outcomes
- reinterpret lifecycle states
- infer consensus
- repair invalid structures
- mutate state during evaluation

---

# 20. Mental Model

ENG-DECISION answers:

“Can this session be accepted right now?”

It does not:

- mutate the world
- define the graph
- define structure
- define receipts

It evaluates.

If the answer is “yes,”  
another layer commits that truth.