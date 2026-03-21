# ENG-RECEIPT — Terminal Receipt Structure, Canonical Form & Rule Provenance

Status: REFACTORED (v10 – Reference-Driven Model)  
Applies to: Engine Core (V1/V2+)  
Scope: Immutable terminal receipts for legitimacy history, reconstruction, verification, and portability

Authority: Foundational authority for receipt structure and receipt immutability.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SESSION
- ENG-CANON
- ENG-SPECVERIFY
- ENG-PERSISTENCE
- ENG-INTEGRITY
- ENG-REVIEW-RETIRED
- ENG-ERROR

---

# 1. Purpose

ENG-RECEIPT defines the canonical terminal receipt artifact emitted by the Engine.

It is the authoritative specification for:

- receipt types
- receipt field structure
- round snapshot structure inside receipts
- receipt immutability
- receipt rule provenance fields
- receipt-level deterministic content requirements
- receipt historical stability across later governance lifecycle changes

ENG-RECEIPT does not define:

- session lifecycle transitions
- acceptance validation rules
- structural halt conditions
- canonical encoding algorithm details
- atomic commit boundaries
- UNDER_REVIEW / RETIRED usability rules
- supersession graph semantics

Those are defined respectively in:

- ENG-SESSION
- ENG-DECISION
- ENG-INTEGRITY
- ENG-CANON
- ENG-PERSISTENCE
- ENG-REVIEW-RETIRED
- ENG-SUPERSESSION

Receipts do not create legitimacy.  
They are the immutable terminal artifacts emitted when legitimacy or closure has already been determined elsewhere.

---

# 2. Receipt Types

## ENG-RECEIPT-01 — LEGITIMACY

A LEGITIMACY receipt is emitted for a session that terminally transitions to ACCEPTED.

It records:

- the governance snapshot context
- the ordered round history
- the final accepted round
- the terminal accepted outcome
- the rule identity under which the session was evaluated

A LEGITIMACY receipt may reference a Resolution whose later lifecycle state changes, but those later changes do not alter the receipt.

---

## ENG-RECEIPT-02 — EXPLORATION

An EXPLORATION receipt is emitted for a session that terminally transitions to CLOSED without accepted legitimacy creation.

It records:

- the ordered round history
- the terminal non-accepted outcome
- the rule identity under which the session closed

An EXPLORATION receipt does not create legitimacy.

---

# 3. Receipt Emission Boundary

## ENG-RECEIPT-03 — Terminal Artifact Only

A receipt may be emitted only on terminal session transition.

Required mapping:

- ACCEPTED → LEGITIMACY
- CLOSED → EXPLORATION

Receipt emission is not defined here as a persistence operation.  
Atomic commit and durability are defined in ENG-PERSISTENCE.

ENG-RECEIPT defines only that:

- exactly one terminal receipt exists per session
- receipt type must match terminal outcome
- receipt content must reflect the terminal state truthfully and deterministically

Failure semantics and runtime enforcement are defined elsewhere.

---

## ENG-RECEIPT-04 — Terminal Uniqueness

Each session must have exactly one terminal receipt.

ENG-RECEIPT is the authority for receipt uniqueness as an artifact rule.  
ENG-INTEGRITY is the runtime authority for verifying that uniqueness during restore or rehydration.

---

# 4. Receipt Structure

## ENG-RECEIPT-05 — Canonical Structural Fields

A receipt must contain the following structural fields:

- receipt_type
- receipt_id
- session_id
- area_id
- engine_version
- spec_set_hash
- authority_snapshot_id
- scope_snapshot_id
- rounds
- final_round_index
- session_state_at_close
- acceptance_result
- hash_algorithm
- content_hash
- schema_version

Optional or informational fields may include:

- problem_statement
- annotations
- created_at
- lineage_references

Whether a field participates in canonical hashing is not defined here.  
That is defined in ENG-CANON.

Rule identity meaning is not defined here.  
That is defined in ENG-SPECVERIFY.

Object field definitions are structurally aligned with ENG-DOMAIN.

---

## ENG-RECEIPT-06 — Receipt Identity

receipt_id must be:

- engine-generated
- UUIDv7
- immutable

Identifier generation rules are defined in ENG-DOMAIN.

---

# 5. Rule Provenance Binding

## ENG-RECEIPT-07 — Rule Context Fields

Each receipt must include:

- engine_version
- spec_set_hash

These fields bind the receipt to the exact Engine rule system that produced it.

ENG-RECEIPT requires their presence and immutability.  
ENG-SPECVERIFY defines their meaning and verification behavior.

---

## ENG-RECEIPT-08 — Provenance Survives Separation

Receipt rule provenance must remain present even if the receipt is separated from the originating runtime context or exported independently.

This ensures that later systems can determine:

- which Engine produced the receipt
- which specification set governed the session outcome

---

# 6. Round Snapshot Model

## ENG-RECEIPT-09 — Receipts Preserve Full Round History

Receipts contain ordered round snapshots.

Round creation semantics belong to ENG-SESSION.  
ENG-RECEIPT defines how rounds are preserved once terminalized.

Each round snapshot must include:

- round_index
- round_state
- participant_set
- candidate_set
- constraint_set
- vote_set

A receipt records full structural round state.  
It does not record diffs.

---

## ENG-RECEIPT-10 — Round Ordering

Rounds in a receipt must appear in ascending round_index order.

They must be:

- contiguous
- not renumbered
- deterministically ordered

Round lifecycle creation is defined in ENG-SESSION.  
ENG-RECEIPT defines only their terminal representation.

---

# 7. Snapshot Set Semantics

## ENG-RECEIPT-11 — Participant Snapshot Set

Participant snapshots represent participation epochs as they existed in that round.

Receipt participants must preserve:

- participant_id
- session_id
- area_id
- round_index
- display_name

Epoch integrity semantics are defined in ENG-DOMAIN and ENG-SESSION.  
Runtime validation of epoch consistency belongs to ENG-INTEGRITY.

---

## ENG-RECEIPT-12 — Candidate Snapshot Set

Candidate snapshots represent proposal epochs as they existed in that round.

Receipt candidates must preserve:

- candidate_id
- session_id
- area_id
- round_index
- candidate_content

Candidate lifecycle semantics are defined in ENG-DOMAIN and ENG-SESSION.

---

## ENG-RECEIPT-13 — Constraint Snapshot Set

Constraint snapshots represent the full declared constraints for that round.

Receipt constraints must preserve the structural fields defined in ENG-DOMAIN.

Constraint evaluation is not defined here.  
It belongs to ENG-DECISION.

---

## ENG-RECEIPT-14 — Vote Snapshot Set

Vote snapshots represent the full vote state for that round.

Receipt votes must preserve:

- vote_id
- session_id
- area_id
- round_index
- participant_id
- candidate_id
- stance

Vote semantics and acceptance behavior are defined elsewhere.  
ENG-RECEIPT records their terminal structural history.

---

# 8. Ordering Requirements Inside Receipts

## ENG-RECEIPT-15 — Deterministic Set Ordering

Receipt snapshot sets must be deterministically ordered.

Expected ordering keys are defined by the object identity rules and canonical serialization rules consumed from:

- ENG-DOMAIN
- ENG-CANON

ENG-RECEIPT requires that ordering be deterministic and stable.  
ENG-CANON defines the exact canonical encoding rules.

---

# 9. Relationship to Canonical Serialization

## ENG-RECEIPT-16 — Receipt Content Must Be Canonicalizable

Receipt content must be representable in a deterministic canonical form.

ENG-RECEIPT does not define the byte-level canonical serialization algorithm.  
That belongs to ENG-CANON.

ENG-RECEIPT requires that receipt structure be complete and stable enough for ENG-CANON to produce deterministic hashing.

---

## ENG-RECEIPT-17 — content_hash Is Receipt Integrity Output

Each receipt must contain:

- hash_algorithm
- content_hash

ENG-RECEIPT requires that these exist and remain immutable.

ENG-CANON defines:

- what bytes are hashed
- how fields are encoded
- how ordering works

ENG-SPECVERIFY defines the significance of spec_set_hash within that canonical content.

---

# 10. Relationship to Session Lifecycle

## ENG-RECEIPT-18 — Receipts Reflect Terminal Session Truth

ENG-RECEIPT does not define session lifecycle or acceptance rules.

It depends on ENG-SESSION and ENG-DECISION for:

- when a session becomes terminal
- whether the outcome is ACCEPTED or CLOSED
- what round state exists at terminalization

ENG-RECEIPT defines how that terminal truth is captured as an immutable artifact.

---

## ENG-RECEIPT-19 — Acceptance Finalizes the Current Round, Not a New One

If a session is accepted, the receipt records the final accepted round as it existed at the time of acceptance.

Round creation rules remain in ENG-SESSION.  
ENG-RECEIPT records, but does not create, round history.

---

# 11. Historical Stability Across Later Lifecycle Changes

## ENG-RECEIPT-20 — Later Governance Changes Do Not Rewrite Receipts

If a referenced Resolution later becomes:

- UNDER_REVIEW
- RETIRED
- SUPERSEDED

the historical receipt remains valid as a record of what occurred at acceptance or closure time.

ENG-REVIEW-RETIRED defines forward usability semantics.  
ENG-SUPERSESSION defines later graph meaning.  
ENG-RECEIPT defines that neither retroactively rewrites historical receipts.

---

# 12. Integration with Restore and Rehydration

## ENG-RECEIPT-21 — Receipt Validation Consumed by Runtime Integrity

ENG-RECEIPT is not the runtime halt authority.

It defines the artifact structure that runtime systems must validate.

ENG-INTEGRITY consumes ENG-RECEIPT, ENG-CANON, and ENG-SPECVERIFY to determine:

- whether receipt structure is valid
- whether runtime may proceed
- whether degraded mode or halt is required

---

# 13. Integration with Atomic Persistence

## ENG-RECEIPT-22 — Receipt Emission Requires Atomic Persistence but Does Not Define It

Receipts must be emitted as part of terminal state persistence.

ENG-RECEIPT defines the artifact requirement.  
ENG-PERSISTENCE defines the atomic boundary within which the receipt is durably created.

ENG-RECEIPT must not redefine commit ordering or crash recovery rules.

---

# 14. Solo Mode Relationship

## ENG-RECEIPT-23 — Implicit Votes Must Still Be Preserved Structurally

If session logic introduces an implicit solo vote, that vote must appear in the terminal receipt exactly as part of the final round snapshot.

ENG-SESSION and ENG-DECISION define when that vote exists.  
ENG-RECEIPT defines that once it exists at terminalization, it must be preserved structurally.

---

# 15. Deterministic Guarantees

## ENG-RECEIPT-24 — Identical Terminal State Produces Identical Receipt Content

Given identical terminal session state, receipt content must be identical in structure.

Byte-identical canonical representation is governed by ENG-CANON.  
Rule identity consistency is governed by ENG-SPECVERIFY.

ENG-RECEIPT defines the structural sameness requirement for the artifact.

---

# 16. Immutability

## ENG-RECEIPT-25 — Receipts Are Immutable

Once emitted, a receipt must not be modified.

This includes:

- structural fields
- round snapshots
- rule provenance fields
- hash fields

Runtime enforcement belongs to ENG-INTEGRITY.  
Persistence durability belongs to ENG-PERSISTENCE.

ENG-RECEIPT is the authority for artifact immutability itself.

---

# 17. Audit Relationship

## ENG-RECEIPT-26 — Receipt Is Not Audit

Receipts are distinct from audit records.

Receipts:

- are terminal structural artifacts
- are immutable
- preserve final legitimacy or closure history

Audit is observational telemetry defined in ENG-AUD.

ENG-RECEIPT does not create, replace, or mutate audit records.

---

# 18. Engine Invariants

- each terminal session has exactly one receipt
- receipt type matches terminal outcome
- receipts preserve ordered full round snapshots
- receipts preserve rule provenance fields
- receipts are immutable
- later UNDER_REVIEW / RETIRED / SUPERSEDED transitions do not rewrite receipts
- canonical hashing requirements are consumed from ENG-CANON
- runtime trust decisions are consumed by ENG-INTEGRITY
- atomic durability is consumed from ENG-PERSISTENCE
- rule identity semantics are consumed from ENG-SPECVERIFY

---

# 19. Mental Model

ENG-RECEIPT defines the terminal artifact layer.

It answers:

- what a receipt is
- what fields a receipt must contain
- how round history is preserved
- how rule provenance is embedded
- why receipts remain historically stable

It does not answer:

- whether acceptance was valid
- whether the Engine must halt
- how bytes are canonically encoded
- how receipts are durably committed
- how UNDER_REVIEW or RETIRED affect forward usability

Those belong elsewhere.

ENG-RECEIPT is the artifact specification.  
Other specifications must consume it rather than duplicate it.