# ENG-PERSISTENCE — Atomic Commit, Durability & Crash Recovery

Status: REFACTORED (v2 – Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)  
Scope: Atomic durability boundaries, crash recovery guarantees, and persistence safety requirements

Authority: Foundational authority for atomic persistence boundaries and crash recovery guarantees.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-INTEGRITY
- ENG-SPECVERIFY
- ENG-AUD

---

# 1. Purpose

ENG-PERSISTENCE defines the persistence guarantees required for legitimacy artifacts and terminal session closure.

It is the authoritative specification for:

- atomic durability boundaries
- crash recovery guarantees
- storage adapter persistence requirements
- separation of audit from legitimacy durability
- persistence-level non-reconstruction requirements
- persistence trust assumptions consumed by runtime integrity

ENG-PERSISTENCE does not define:

- session acceptance validity
- session lifecycle semantics
- supersession graph meaning
- receipt structure
- canonical serialization rules
- runtime halt policy
- UNDER_REVIEW / RETIRED usability semantics

Those are defined respectively in:

- ENG-DECISION
- ENG-SESSION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-CANON
- ENG-INTEGRITY
- ENG-REVIEW-RETIRED

ENG-PERSISTENCE defines how legitimacy artifacts must be durably committed once they are valid for creation.

---

# 2. Foundational Principle

## ENG-PERSISTENCE-01 — Legitimacy Events Are Indivisible

Legitimacy creation must be durably atomic.

A legitimacy event must either:

- fully persist
- or not exist at all

Partial persistence of legitimacy artifacts is structural corruption.

ENG-PERSISTENCE is the authority for this indivisibility guarantee.

---

# 3. Acceptance Atomic Boundary

## ENG-PERSISTENCE-02 — Legitimacy Atomic Commit Set

When a session transitions to ACCEPTED, the following structural results must commit in one durable atomic operation:

1. session state transition to ACCEPTED
2. Resolution creation
3. supersession graph mutation, if applicable
4. LEGITIMACY receipt creation

These four elements form the legitimacy atomic boundary.

All must commit or none must commit.

ENG-PERSISTENCE does not define whether acceptance is valid.  
It defines that once acceptance is valid, these artifacts must be durably committed together.

---

## ENG-PERSISTENCE-03 — Resolution Provenance Must Be Committed Atomically

The Resolution created during atomic acceptance must include the structural rule provenance fields required by ENG-DOMAIN and ENG-SPECVERIFY:

- engine_version
- spec_set_hash

These fields must be committed as part of the same atomic durability boundary as:

- session ACCEPTED state
- supersession mutation
- LEGITIMACY receipt

ENG-PERSISTENCE defines the durability requirement only.  
Meaning of those fields belongs to ENG-SPECVERIFY.

---

## ENG-PERSISTENCE-04 — Invalid Partial Acceptance States

The following persisted states are forbidden:

- Resolution exists without ACCEPTED session
- ACCEPTED session exists without Resolution
- Resolution exists without LEGITIMACY receipt
- LEGITIMACY receipt exists without Resolution
- supersession mutation exists without the Resolution that caused it

Runtime consequences of detecting such corruption are defined in ENG-INTEGRITY.

ENG-PERSISTENCE defines that these states must never be produced by a compliant persistence implementation.

---

# 4. Terminal Closure Atomic Boundary

## ENG-PERSISTENCE-05 — Closure Atomic Commit Set

When a session transitions to CLOSED without accepted legitimacy creation, the following must commit atomically:

1. session state transition to CLOSED
2. EXPLORATION receipt creation

No Resolution is created.

Partial persistence of terminal closure artifacts is forbidden.

---

# 5. Audit Separation

## ENG-PERSISTENCE-06 — Audit Is Outside Legitimacy Atomicity

Audit events are not part of the legitimacy atomic durability boundary.

Audit emission:

- must not be required for legitimacy validity
- must not be required for terminal receipt validity
- may fail independently after successful legitimacy persistence

If legitimacy commit succeeds and audit emission fails, legitimacy remains valid.

Audit semantics are defined in ENG-AUD.  
ENG-PERSISTENCE defines only that audit is outside the legitimacy commit set.

---

## ENG-PERSISTENCE-07 — Audit May Be Reconstructed, Legitimacy Artifacts May Not

Audit may be reconstructed from domain facts by external systems if needed.

Receipts and Resolutions must never be reconstructed after a failed or partial legitimacy commit.

If they were not durably committed, they must be treated as never having existed.

---

# 6. Crash Recovery Guarantees

## ENG-PERSISTENCE-08 — Pre-Commit Crash

If a crash occurs before atomic commit completes:

- session must remain non-ACCEPTED for legitimacy creation attempts
- no Resolution may exist from that acceptance attempt
- no receipt may exist from that acceptance attempt
- no supersession mutation from that acceptance attempt may exist

The system must behave as though the acceptance never happened.

---

## ENG-PERSISTENCE-09 — Post-Commit Crash

If a crash occurs after atomic commit completes:

- session must already be durably ACCEPTED, or durably CLOSED as applicable
- all artifacts inside the relevant atomic boundary must already exist
- no replay or reconstruction may be required to recover legitimacy artifacts
- the supersession graph must already reflect the committed structural result

The Engine must never reconstruct committed legitimacy artifacts after crash recovery.

It may only reload them.

---

## ENG-PERSISTENCE-10 — No Post-Crash Fabrication

Post-crash recovery must not fabricate:

- Resolution objects
- receipt objects
- supersession mutations
- session terminal state transitions

Recovery may only observe durable state that already exists.

If durable state is incomplete, corruption has occurred.

Runtime response to that corruption is governed by ENG-INTEGRITY.

---

# 7. Persistence Relationship to Rehydration

## ENG-PERSISTENCE-11 — Runtime Integrity Consumes Persistence Outcomes

ENG-PERSISTENCE does not define rehydration halt or degraded-mode policy.

It defines the persistence assumptions that rehydration may rely on.

ENG-INTEGRITY consumes those assumptions when validating:

- session / Resolution / receipt correspondence
- receipt existence
- supersession coherence
- rule provenance consistency
- crash safety assumptions

If persistence guarantees were violated, ENG-INTEGRITY determines runtime consequences.

---

## ENG-PERSISTENCE-12 — Historical Stability Across Later Lifecycle Changes

Later lifecycle changes to a referenced Resolution, including:

- UNDER_REVIEW
- RETIRED
- SUPERSEDED

do not invalidate the historical legitimacy artifacts that were already durably committed.

ENG-PERSISTENCE defines only that already committed legitimacy artifacts remain part of durable history.

Usability semantics for later states belong to ENG-REVIEW-RETIRED.  
Graph meaning belongs to ENG-SUPERSESSION.

---

# 8. Storage Adapter Requirements

## ENG-PERSISTENCE-13 — Required Storage Capability

The storage adapter must support:

- atomic durable commit across all objects in the defined boundary
- all-or-nothing transactional persistence
- crash-safe durability

ENG-PERSISTENCE is storage-agnostic about implementation technology.

The guarantee is mandatory, regardless of storage mechanism.

Possible compliant implementation strategies may include:

- ACID transactions
- atomic write batches
- write-ahead logs with commit markers
- equivalent all-or-nothing durable commit models

---

## ENG-PERSISTENCE-14 — Persistence Order Must Not Escape the Atomic Boundary

The implementation must not expose externally visible partial orderings that violate atomicity.

Specifically, it must not durably expose:

- session terminal state before the corresponding receipt when both are in the same boundary
- Resolution before the ACCEPTED session if atomicity is not yet complete
- supersession mutation before the Resolution that justifies it
- receipt creation as an asynchronous follow-up to accepted legitimacy creation

Intermediate write steps may exist internally, but they must not become durable committed truth independently.

---

# 9. Prohibited Behaviors

## ENG-PERSISTENCE-15 — No Lazy or Split Legitimacy Persistence

The Engine or its storage adapter must never:

- write Resolution asynchronously after session acceptance
- write receipt asynchronously after session acceptance
- apply supersession mutation outside the legitimacy atomic boundary
- repair partial legitimacy commits automatically
- generate receipts lazily after terminal transition
- reconstruct receipts from audit
- reconstruct Resolutions from receipts or audit after failed persistence

These are persistence violations, not merely behavioral preferences.

---

# 10. Non-Legitimacy Transitions

## ENG-PERSISTENCE-16 — Ordinary State Mutations Are Not Legitimacy Atomic Boundaries

State transitions that do not create legitimacy do not require the multi-object legitimacy atomic boundary.

Examples may include ordinary session lifecycle changes outside terminal acceptance and closure.

ENG-PERSISTENCE defines only that:

- ACCEPTED legitimacy creation requires the legitimacy atomic boundary
- CLOSED terminal closure requires the closure atomic boundary

Other mutation grouping requirements may be defined elsewhere, but are not legitimacy persistence boundaries unless explicitly stated.

---

## ENG-PERSISTENCE-17 — Administrative Usability Transitions Are Outside the Legitimacy Boundary

Administrative lifecycle transitions affecting forward usability, including transitions such as:

- ACTIVE → UNDER_REVIEW
- UNDER_REVIEW → ACTIVE
- ACTIVE → RETIRED

are not part of the legitimacy atomic boundary.

These transitions must not rewrite:

- historical receipts
- prior session history
- previously committed legitimacy artifacts
- supersession graph structure unless separately authorized by graph rules elsewhere

ENG-PERSISTENCE defines only that such transitions are not legitimacy creation commits.

---

# 11. Determinism Guarantee

## ENG-PERSISTENCE-18 — Persistence Must Not Introduce Nondeterminism

Given identical valid pre-commit structural state and identical accepted decision outcome, persistence must preserve deterministic artifacts.

Persistence must not alter:

- Resolution content
- receipt content
- supersession mutation content
- provenance fields
- hash-bearing artifact content

Artifact construction rules belong elsewhere.  
ENG-PERSISTENCE guarantees that persistence does not distort them.

---

# 12. Evidentiary Integrity

## ENG-PERSISTENCE-19 — Legitimacy Artifacts Must Remain Admissible as a Durable Pair

For accepted legitimacy, the admissible durable artifact set is:

- Resolution
- corresponding LEGITIMACY receipt

with the associated accepted session state and any committed supersession mutation.

These artifacts must remain:

- indivisible
- durable
- deterministic
- verifiable

Audit remains supplemental and non-constitutive.

---

# 13. Relationship to Rule Provenance

## ENG-PERSISTENCE-20 — Provenance Fields Are Persistence-Critical Structural Content

Where governing artifact specifications require:

- engine_version
- spec_set_hash

ENG-PERSISTENCE requires that those fields persist atomically with the artifact they belong to.

ENG-PERSISTENCE does not define:

- provenance semantics
- historical spec-set compatibility policy

Those belong to ENG-SPECVERIFY.

It defines only that provenance-bearing artifacts must not be partially persisted.

---

# 14. Relationship to Receipts

## ENG-PERSISTENCE-21 — Receipt Emission Is Consumed, Not Defined, Here

ENG-RECEIPT defines:

- receipt types
- receipt structure
- receipt immutability

ENG-PERSISTENCE defines only that required receipts must be durably created inside the correct atomic boundary.

ENG-PERSISTENCE must not redefine receipt schema or canonical structure.

---

# 15. Relationship to Runtime Integrity

## ENG-PERSISTENCE-22 — Integrity Enforces Trust in Persistence Guarantees

ENG-INTEGRITY may use persistence outcomes and corruption evidence to determine:

- halt
- degraded mode
- restore refusal

ENG-PERSISTENCE does not define those runtime policies.

It defines the durability guarantees that those policies depend on.

---

# 16. Engine Invariants

- legitimacy creation commits atomically as session + Resolution + supersession mutation + LEGITIMACY receipt
- terminal non-legitimacy closure commits atomically as session + EXPLORATION receipt
- audit is outside legitimacy atomicity
- crash before commit yields no legitimacy artifact
- crash after commit requires no artifact reconstruction
- receipts and Resolutions must never be fabricated during recovery
- storage adapter must provide all-or-nothing durability for required commit sets
- provenance-bearing structural artifacts must persist atomically with their commit boundary
- administrative usability transitions do not rewrite historical legitimacy artifacts
- dependent specifications must consume ENG-PERSISTENCE rather than redefine atomic durability rules

---

# 17. Mental Model

ENG-PERSISTENCE defines durable commit truth.

It answers:

- what must persist together
- what may never partially persist
- what crash recovery may rely on
- what may never be reconstructed after failure

It does not answer:

- whether acceptance is valid
- what a receipt contains
- how a Resolution becomes structurally ACTIVE
- whether the Engine must halt on corruption
- how bytes are canonically hashed

Those belong elsewhere.

ENG-PERSISTENCE is the durability layer.  
Other specifications must consume it rather than duplicate its atomicity guarantees.