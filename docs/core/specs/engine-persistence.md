# ENG-PERSISTENCE — Atomic Commit, Durability & Crash Recovery

Status: REFACTORED (v3 – Candidate-Centric Atomic Boundary & Structural Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Atomic durability boundaries, crash recovery guarantees, and persistence safety requirements  

Authority: Foundational authority for atomic persistence boundaries and crash recovery guarantees.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-STRUCTURE
- ENG-RECEIPT
- ENG-INTEGRITY
- ENG-SPECVERIFY
- ENG-USABILITY
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
- structural graph semantics  
- receipt structure  
- canonical serialization rules  
- runtime halt policy  
- usability semantics  

Those are defined respectively in:

- ENG-DECISION  
- ENG-SESSION  
- ENG-STRUCTURE  
- ENG-RECEIPT  
- ENG-CANON  
- ENG-INTEGRITY  
- ENG-USABILITY  

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

## ENG-PERSISTENCE-02 — Candidate-Centric Atomic Commit Set

When a session transitions to ACCEPTED, the Engine must durably commit the **accepted candidate outcome** in a single atomic operation.

The atomic commit set must include:

1. session state transition to ACCEPTED  
2. persistence of the accepted candidate outcome  
3. structural graph mutation, if required by the candidate action  
4. LEGITIMACY receipt creation  

All elements must commit or none must commit.

---

## ENG-PERSISTENCE-02A — Accepted Candidate Outcome Persistence

The accepted candidate outcome must be durably realized according to the candidate_action_type.

Depending on the candidate action, this may include:

- creation of a new Resolution  
- mutation of Resolution lifecycle state (e.g., retirement)  
- structural graph updates (e.g., supersession edges)  
- persistence of any required provenance-bearing structural artifacts  

ENG-PERSISTENCE does not define candidate semantics.  
ENG-DOMAIN and ENG-DECISION define candidate action meaning.

ENG-PERSISTENCE defines only that the resulting structural effects must be committed atomically.

---

## ENG-PERSISTENCE-03 — Provenance Fields Must Be Atomically Persisted

Any artifact created or mutated as part of accepted candidate outcome persistence that requires provenance fields must include:

- engine_version  
- spec_set_hash  

These fields must be committed atomically with the artifact they belong to.

This includes, where applicable:

- newly created Resolution objects  
- other provenance-bearing structural artifacts  

ENG-PERSISTENCE defines the durability requirement only.  
Meaning of those fields belongs to ENG-SPECVERIFY.

---

## ENG-PERSISTENCE-04 — Invalid Partial Acceptance States

The following persisted states are forbidden:

- ACCEPTED session without corresponding accepted candidate outcome  
- accepted candidate outcome artifacts without ACCEPTED session  
- LEGITIMACY receipt without ACCEPTED session  
- ACCEPTED session without LEGITIMACY receipt  
- structural graph mutation without the accepted candidate action that caused it  
- provenance-bearing artifacts missing required provenance fields  

Runtime consequences of detecting such corruption are defined in ENG-INTEGRITY.

ENG-PERSISTENCE defines that these states must never be produced.

---

# 4. Terminal Closure Atomic Boundary

## ENG-PERSISTENCE-05 — Closure Atomic Commit Set

When a session transitions to CLOSED without accepted legitimacy creation, the following must commit atomically:

1. session state transition to CLOSED  
2. EXPLORATION receipt creation  

No accepted candidate outcome is persisted.

Partial persistence of closure artifacts is forbidden.

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

---

## ENG-PERSISTENCE-07 — Audit May Be Reconstructed, Legitimacy Artifacts May Not

Audit may be reconstructed externally.

The following must never be reconstructed:

- accepted candidate outcome artifacts  
- receipts  
- structural graph mutations  

If they were not durably committed, they must be treated as never having existed.

---

# 6. Crash Recovery Guarantees

## ENG-PERSISTENCE-08 — Pre-Commit Crash

If a crash occurs before atomic commit completes:

- session must remain non-ACCEPTED  
- no accepted candidate outcome artifacts may exist  
- no receipt may exist  
- no structural graph mutation may exist  

The system must behave as though the acceptance never occurred.

---

## ENG-PERSISTENCE-09 — Post-Commit Crash

If a crash occurs after atomic commit completes:

- session must already be ACCEPTED or CLOSED  
- all artifacts in the atomic boundary must exist  
- no reconstruction is required  
- structural graph must reflect the committed outcome  

The Engine must only reload persisted artifacts.

---

## ENG-PERSISTENCE-10 — No Post-Crash Fabrication

Post-crash recovery must not fabricate:

- accepted candidate outcomes  
- Resolution objects  
- receipts  
- structural graph mutations  
- session terminal state transitions  

If durable state is incomplete, corruption has occurred.

---

# 7. Persistence Relationship to Rehydration

## ENG-PERSISTENCE-11 — Runtime Integrity Consumes Persistence Guarantees

ENG-PERSISTENCE defines guarantees.

ENG-INTEGRITY consumes them to validate:

- session / receipt / outcome consistency  
- structural graph correctness  
- provenance integrity  
- crash safety assumptions  

---

## ENG-PERSISTENCE-12 — Historical Stability Across Later Usability Changes

Later usability transitions, including:

- ACTIVE → ON_HOLD  
- ON_HOLD → ACTIVE  
- ACTIVE → RETIRED  

do not invalidate previously committed legitimacy artifacts.

These transitions:

- do not rewrite receipts  
- do not alter prior accepted candidate outcomes  
- do not retroactively change structural history  

ENG-USABILITY defines forward-use semantics.  
ENG-STRUCTURE defines graph meaning.

---

# 8. Storage Adapter Requirements

## ENG-PERSISTENCE-13 — Required Storage Capability

The storage adapter must support:

- atomic multi-object commit  
- all-or-nothing durability  
- crash-safe persistence  

Implementation is storage-agnostic but guarantees are mandatory.

---

## ENG-PERSISTENCE-14 — No External Visibility of Partial State

The system must not expose:

- ACCEPTED session without receipt  
- receipt without session  
- structural graph mutation without corresponding candidate outcome  
- partial persistence of atomic commit set  

Internal steps may exist but must not become externally visible.

---

# 9. Prohibited Behaviors

## ENG-PERSISTENCE-15 — No Split or Deferred Legitimacy Persistence

The Engine must never:

- persist candidate outcome separately from session acceptance  
- persist receipt asynchronously after acceptance  
- apply structural graph mutation outside atomic boundary  
- reconstruct legitimacy artifacts after failure  
- generate receipts lazily  
- derive artifacts from audit  

---

# 10. Non-Legitimacy Transitions

## ENG-PERSISTENCE-16 — Non-Legitimacy Mutations Are Not Atomic Boundaries

Only:

- ACCEPTED  
- CLOSED  

define persistence atomic boundaries.

Other mutations are outside this scope unless explicitly defined elsewhere.

---

## ENG-PERSISTENCE-17 — Usability Transitions Are Outside Legitimacy Boundaries

Usability transitions:

- ACTIVE ↔ ON_HOLD  
- ACTIVE → RETIRED  

are not legitimacy events.

They must not:

- rewrite receipts  
- alter structural graph truth  
- mutate historical outcomes  

---

# 11. Determinism Guarantee

## ENG-PERSISTENCE-18 — Persistence Must Preserve Determinism

Persistence must not alter:

- candidate outcome content  
- receipt content  
- structural graph state  
- provenance fields  

---

# 12. Evidentiary Integrity

## ENG-PERSISTENCE-19 — Legitimacy Artifacts Form an Indivisible Set

For accepted sessions, the durable legitimacy set consists of:

- accepted candidate outcome artifacts  
- corresponding LEGITIMACY receipt  
- ACCEPTED session state  
- structural graph state  

These must remain:

- indivisible  
- verifiable  
- deterministic  

---

# 13. Relationship to Rule Provenance

## ENG-PERSISTENCE-20 — Provenance Is Structurally Required

Provenance fields must:

- exist where required  
- be persisted atomically  
- never be partially written  

Meaning belongs to ENG-SPECVERIFY.

---

# 14. Relationship to Receipts

## ENG-PERSISTENCE-21 — Receipt Emission Is Consumed

ENG-RECEIPT defines receipt structure.

ENG-PERSISTENCE defines only:

- receipts must be created atomically  
- receipts must not be partially persisted  

---

# 15. Relationship to Runtime Integrity

## ENG-PERSISTENCE-22 — Integrity Enforces Trust

ENG-INTEGRITY determines:

- halt  
- degraded mode  

based on persistence correctness.

---

# 16. Engine Invariants

- acceptance commits atomically as session + candidate outcome + structural mutation + receipt  
- closure commits atomically as session + receipt  
- audit is outside atomic boundary  
- no partial legitimacy persistence allowed  
- no reconstruction of legitimacy artifacts  
- provenance fields persist atomically  
- usability transitions do not alter history  

---

# 17. Mental Model

ENG-PERSISTENCE defines durable truth.

It answers:

- what must persist together  
- what must never partially persist  
- what crash recovery may assume  

It does not define:

- legitimacy  
- structure  
- usability  
- decision logic  

It guarantees that once legitimacy exists, it is permanent and indivisible.