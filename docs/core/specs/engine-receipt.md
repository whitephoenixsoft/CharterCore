# ENG-RECEIPT — Engine Session Receipt Specification

Status: FROZEN (v9 – Canonical Ordering & Proposal Epoch Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic session receipts for audit, reconstruction, legitimacy verification, and federation portability

Authority: Subordinate to ENG-DOMAIN, ENG-SESSION, ENG-ERROR, ENG-INTEGRITY, ENG-API, ENG-SPECVERIFY

---

# 1. Purpose

This specification defines how the Engine produces canonical session receipts.

Receipts:

- Provide immutable, verifiable records of session terminal transition
- Capture frozen governance and round-segmented structural state
- Preserve participant and candidate epoch integrity
- Record deterministic acceptance or abandonment outcomes
- Bind session outcomes to the exact rule set that evaluated them
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
- Rule set identity used to evaluate the decision

LEGITIMACY receipts anchor governance history.

---

## 2.2 EXPLORATION

Emitted when a session transitions atomically to CLOSED without acceptance.

Captures:

- Full ordered round history
- Final non-accepted state
- Explicit abandonment outcome
- Rule set identity used during the session

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

---

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

---

## 4.2 Round Ordering

Rounds must be ordered by ascending `round_index`.

Rules:

- Round indices must be contiguous
- No re-numbering permitted
- Serialization order must match round_index order

---

# 5. Receipt Structure

A receipt must contain the following canonical fields:

- receipt_type (LEGITIMACY | EXPLORATION)
- receipt_id (UUIDv7, engine-generated)
- session_id
- area_id
- engine_version
- spec_set_hash
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

# 6. Rule Context Binding

Receipts must record the rule set that evaluated the session.

Rule context fields:

- engine_version
- spec_set_hash

Definitions:

engine_version  
Identifies the Engine binary version that executed the session.

spec_set_hash  
Identifies the deterministic specification set embedded in the Engine.

spec_set_hash must match the value defined by ENG-SPECVERIFY for the executing Engine.

Rule context fields provide rule provenance for the receipt.

They allow later systems to determine:

- which Engine produced the receipt
- which specification set governed the evaluation

---

# 7. Canonical Serialization Rule Context

spec_set_hash must participate in canonical serialization.

Therefore:

content_hash = hash(canonical_receipt_content)

Where canonical_receipt_content includes:

- engine_version
- spec_set_hash
- hash_algorithm

This ensures that the receipt integrity hash binds:

- the decision outcome
- the exact rule system used to produce it

Changing spec_set_hash must change content_hash.

This prevents silent reinterpretation of receipts under different rule sets.

---

# 8. Round Structure

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

# 9. Participant Snapshot Semantics

Participants represent participation epochs.

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

participant_set must be ordered lexicographically by participant_id.

---

# 10. Candidate Snapshot Semantics

Candidates represent round-scoped proposal epochs.

Each candidate_set entry must include:

- candidate_id (UUIDv7)
- session_id
- area_id
- round_index
- candidate_content

Rules:

- candidate_id unique within the round
- candidate_id never reused across rounds
- Candidate content mutable during PRE_STANCE
- Candidate content immutable once VOTING begins
- Only candidates active in that round appear

candidate_set must be ordered lexicographically by candidate_id.

---

# 11. Constraint Snapshot Semantics

Each constraint_set entry must include:

- constraint_id
- session_id
- area_id
- round_index
- constraint_type
- constraint_parameters

constraint_set must be ordered lexicographically by constraint_id.

---

# 12. Vote Snapshot Semantics

vote_set must include:

- vote_id
- session_id
- area_id
- round_index
- participant_id
- candidate_id
- stance (ACCEPT | REJECT | ABSTAIN)

vote_set must be ordered lexicographically by vote_id.

---

# 13. Solo Mode

If session operated in Solo Mode:

- Implicit ACCEPT vote must be inserted deterministically during acceptance
- Exactly one participation epoch must exist in the final round
- The vote must appear in the final round snapshot

Solo Mode does not bypass validation.

Solo Mode does not bypass receipt emission.

---

# 14. Canonical Serialization & Hashing

content_hash must be computed over canonical serialized receipt content using ENG-CANON rules.

Canonicalization guarantees:

- deterministic field ordering
- deterministic round ordering
- deterministic set ordering
- stable UTF-8 JSON encoding
- cross-language reproducibility

---

# 15. Deterministic Guarantees

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

# 16. Immutability

Receipts are immutable once emitted.

No field may be modified.

Receipts survive restore and rehydration.

Later governance changes do not alter prior receipts.

Receipts are append-only artifacts.

---

# 17. Integration with Rehydration & Degraded Mode

During rehydration:

- Receipt integrity must be revalidated
- Hash must be recomputed and compared
- Round contiguity must be validated

Structural violation results in initialization failure.

In degraded mode:

- No new receipts may be emitted
- Existing receipts remain readable
- Integrity defects must not be masked

---

# 18. Audit Alignment

Receipts:

- Are distinct from audit events
- Do not replace lifecycle audit
- Represent only terminal structural outcome

Audit is descriptive runtime telemetry.

Receipt is terminal structural artifact.

---

# 19. Mental Model

Rounds represent structural legitimacy epochs.

Pause or temporary block requires resume.

Resume creates a new round.

Acceptance finalizes the current round atomically.

Participant and candidate IDs represent participation and proposal epochs.

UUID defines identity.  
content_hash defines integrity.  
spec_set_hash defines the rule system under which the receipt was created.

Timestamps are informational only.

Receipts describe legitimacy.

They do not create it.