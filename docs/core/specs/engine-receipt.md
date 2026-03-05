# ENG-RECEIPT — Engine Session Receipt Specification

Status: FROZEN (v7 – Domain Round Alignment & Constraint Snapshot Schema Integrated)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic session receipts for audit, reconstruction, legitimacy verification, and federation portability

Authority: Subordinate to ENG-DOMAIN, ENG-SESSION, ENG-ERROR, ENG-INTEGRITY, ENG-API


---

# 1. Purpose

This specification defines how the Engine produces canonical session receipts.

Receipts:

- Provide immutable, verifiable records of session terminal transition
- Capture frozen governance and round-segmented structural state
- Preserve participant and candidate epoch integrity
- Record deterministic acceptance or abandonment outcomes
- Enable reconstruction of procedural legitimacy boundaries
- Provide portable legitimacy artifacts independent of host audit logs

Receipts are engine-authoritative structural artifacts.

Receipts do not create legitimacy.  
Legitimacy is created only by successful atomic session acceptance.

Receipts formalize and freeze closure.

Participant and candidate identity within receipts is epoch-based and session-scoped.


---

# 2. Receipt Types

## 2.1 LEGITIMACY

Emitted when a session transitions atomically to ACCEPTED.

Captures:

- Governance context snapshot
- Full ordered round history
- Final accepted round
- Deterministic acceptance outcome

LEGITIMACY receipts anchor governance history.


## 2.2 EXPLORATION

Emitted when a session transitions atomically to CLOSED without acceptance.

Captures:

- Full ordered round history
- Final non-accepted state
- Explicit abandonment outcome

Does not create legitimacy.


---

# 3. Generation Rules

## 3.1 Terminal Transition Requirement

Exactly one receipt must be emitted per session.

Receipt emission is:

- Atomic with terminal state transition
- Atomic with resolution creation (if ACCEPTED)
- Atomic with supersession application (if applicable)
- Deterministic
- Immutable

If:

ACCEPTED → LEGITIMACY receipt  
CLOSED → EXPLORATION receipt

No receipt may be emitted outside a TERMINAL transition.

Fail if:

- Terminal state reached without receipt
- Receipt emitted without terminal transition
- More than one receipt exists for a session


## 3.2 Acceptance Preconditions

LEGITIMACY receipt may only be emitted after:

- Session state == ACTIVE
- Not BLOCK_PERMANENT
- AcceptanceRuleSatisfied == true
- Full deterministic validation pass
- All violations accumulated
- No blocking conditions remain

Acceptance finalizes the current round.  
Acceptance does not create a new round.

Receipt must reflect the fully frozen state created during atomic acceptance commit.


---

# 4. Round Segmentation Doctrine

## 4.1 Round Creation Rule

- Round 1 is created at session initialization.
- A new round is created if and only if a RESUME transition occurs.

RESUME:

- Permitted only from PAUSED or BLOCK_TEMPORARY
- Prohibited from BLOCK_PERMANENT
- Requires vote_set to be empty at time of resume

On RESUME:

- round_index increments by 1
- All structural sets (participant_set, candidate_set, constraint_set, vote_set) begin fresh for that round

No other state transition creates a round.


## 4.2 Round Ordering

- Rounds form an ordered sequence.
- Ordered strictly by ascending round_index.
- Round indices must be contiguous.
- No re-numbering permitted.


---

# 5. Receipt Structure

A receipt must contain the following canonical fields:

- receipt_type (LEGITIMACY | EXPLORATION)
- receipt_id (UUIDv7, engine-generated)
- session_id
- area_id
- engine_version
- authority_snapshot_id
- scope_snapshot_id
- problem_statement (optional, immutable across rounds)
- rounds (ordered sequence of round objects)
- final_round_index
- annotations (optional, immutable)
- session_state_at_close (ACCEPTED | CLOSED)
- acceptance_result (SUCCESS | ABANDONED)
- created_at (UTC ISO 8601 with timezone, informational only)
- hash_algorithm
- content_hash

Optional:

- lineage_references (deterministically ordered list of related receipt_ids)

All fields participate in canonical serialization unless explicitly marked informational.

created_at is informational only and must not influence ordering, identity, or legitimacy.


---

# 6. Round Structure

Each round must contain:

- round_index
- round_state (COMPLETED | FINAL_ACCEPTED | ABANDONED)
- participant_set (deterministically ordered)
- candidate_set (deterministically ordered)
- constraint_set (deterministically ordered)
- vote_set (deterministically ordered)

All sets represent full structural state for that round.

No diffs permitted.  
No inference permitted.


---

# 7. Participant Snapshot Semantics

## 7.1 Participation Epoch Model

Participants represent participation epochs, not human identities.

Each participant_set entry must include:

- participant_id (UUIDv7)
- session_id
- area_id
- round_index
- display_name

Rules:

- participant_id represents a single non-reusable participation epoch
- No merging of historical epochs permitted
- No inference of identity continuity
- display_name is legitimacy-bearing and frozen per round snapshot

If a participant left and rejoined:

- A new participant_id must exist
- Only epochs active in that round appear


## 7.2 Deterministic Ordering

participant_set must be ordered lexicographically by participant_id using canonical UUID byte ordering.

Ordering must not depend on:

- Timestamp
- Insertion order
- Display name
- Vote order

Identical round state → identical participant ordering across implementations.


---

# 8. Candidate Snapshot Semantics

Candidates represent round-scoped proposal epochs.

Each candidate_set entry must include:

- candidate_id (UUIDv7)
- session_id
- area_id
- round_index
- candidate_content

Rules:

- candidate_id unique within session
- candidate_id never reused
- Candidate content frozen at round finalization
- Only candidates active in that round appear


## 8.1 Deterministic Ordering

candidate_set must be ordered lexicographically by candidate_id using canonical UUID byte ordering.

Ordering must not depend on:

- Creation order
- Candidate content


---

# 9. Constraint Snapshot Semantics

Constraints represent mechanical acceptance gates active during the round.

Each constraint_set entry must include:

- constraint_id
- session_id
- area_id
- round_index
- constraint_type
- constraint_parameters

Rules:

- constraint definitions must match those active at round freeze
- constraint_set represents the full constraint configuration for the round
- constraints are immutable once VOTING begins
- only constraints active in that round appear

## 9.1 Deterministic Ordering

constraint_set must be ordered lexicographically by constraint_id.


---

# 10. Vote Snapshot Semantics

vote_set must include all explicit votes cast in that round.

Each vote entry must include:

- vote_id (UUIDv7)
- session_id
- area_id
- round_index
- participant_id
- candidate_id
- stance (ACCEPT | REJECT | ABSTAIN)

Rules:

- Includes implicit Solo Mode vote if applicable
- Votes immutable once recorded in a finalized round snapshot
- No inference permitted

vote_set must be ordered lexicographically by vote_id using canonical UUID byte ordering.


---

# 11. Solo Mode

If session operated in Solo Mode:

- Implicit ACCEPT vote must be inserted deterministically during acceptance
- Exactly one participation epoch must exist in the final round
- The vote must appear in the final round snapshot

Solo Mode does not bypass validation.

Solo Mode does not bypass receipt emission.


---

# 12. Canonical Serialization & Hashing

## 12.1 Canonicalization Requirement

content_hash must be computed over canonical serialized receipt content.

Canonicalization must ensure:

- Deterministic field ordering
- Deterministic round ordering
- Deterministic set ordering
- Stable UTF-8 JSON encoding
- Cross-language reproducibility


## 12.2 Hash Semantics

content_hash ensures integrity only.

It does not influence:

- legitimacy
- acceptance
- restore
- identity

Identity is defined exclusively by receipt_id.


## 12.3 Algorithm Declaration

hash_algorithm must be explicitly declared.

Algorithm migration:

- does not alter receipt identity
- does not rewrite historical receipts
- does not alter legitimacy history


---

# 13. Deterministic Guarantees

Given identical session state and identical round segmentation at terminal transition:

- Receipt content must be identical
- Round ordering must be identical
- Set ordering must be identical
- content_hash must be identical

Mismatch indicates:

- Non-determinism
- Canonicalization violation
- Engine defect

Receipts must be reconstructable from engine state.


---

# 14. Immutability

Receipts are immutable once emitted.

No field may be modified.

Receipts survive restore and rehydration.

Later governance changes do not alter prior receipts.

Receipts are append-only artifacts.


---

# 15. Integration with Rehydration & Degraded Mode

## 15.1 Rehydration Validation

During rehydration:

- Receipt integrity must be revalidated
- Hash must be recomputed and compared
- Round contiguity must be validated
- Snapshot round_index values must match their containing round
- Final round must match resolution snapshot (if ACCEPTED)

Structural violation results in initialization failure.


## 15.2 Degraded Mode

In degraded mode:

- No new receipts may be emitted
- Existing receipts remain readable
- Integrity defects must not be masked
- Degraded mode prohibits partial legitimacy compilation


---

# 16. Audit Alignment

Receipts:

- Are distinct from audit events
- Do not replace lifecycle audit
- Do not encode EvaluationReport errors
- Represent only terminal structural outcome

Audit is descriptive runtime telemetry.

Receipt is terminal structural artifact.


---

# 17. Mental Model

Rounds represent structural legitimacy epochs.

Pause or temporary block requires resume.

Resume creates a new round.

Acceptance finalizes the current round atomically.

Participant and candidate IDs represent participation and proposal epochs.

display_name and candidate_content are frozen per round snapshot.

UUID defines identity.  
content_hash defines integrity.

Timestamps are informational only.

Receipts describe legitimacy.

They do not create it.