# ENG-RECEIPT — Engine Session Receipt Specification

Status: DRAFT (v9.1 – Under Review / Retired & Spec Verify Alignment)
Applies to: Engine Core (V1/V2+)
Scope: Deterministic session receipts for audit, reconstruction, legitimacy verification, and federation portability

Authority: Subordinate to ENG-DOMAIN, ENG-SESSION, ENG-ERROR, ENG-INTEGRITY, ENG-API, ENG-SPECVERIFY

---

# 1. Purpose

Receipts provide immutable, verifiable terminal session records.

Key adjustments:

- Include forward usability states: UNDER_REVIEW, RETIRED  
- Support incremental compilation comparisons via canonical round snapshots  
- Bind to exact rule system using spec_set_hash  
- Historical legitimacy preserved even if referenced Resolutions are later UNDER_REVIEW/RETIRED

Receipts **do not create legitimacy**; legitimacy is only via atomic session acceptance.

---

# 2. Receipt Types

## 2.1 LEGITIMACY

- Emitted on ACCEPTED terminal transition  
- Captures governance snapshots, ordered rounds, final accepted round, deterministic outcome, rule set identity  
- Resolution referenced may be ACTIVE, SUPERSEDED, UNDER_REVIEW, or RETIRED (forward usability rules applied only)  

## 2.2 EXPLORATION

- Emitted on CLOSED without acceptance  
- Captures ordered rounds, final state, explicit abandonment, rule set identity  
- Does not create legitimacy

---

# 3. Generation Rules

## 3.1 Terminal Transition Requirement

- One receipt per session  
- Atomic with terminal state transition, resolution creation (if ACCEPTED), and supersession application (if applicable)  
- Deterministic, immutable  
- ACCEPTED → LEGITIMACY  
- CLOSED → EXPLORATION  

Fail if:

- Terminal transition without receipt  
- Receipt emitted outside terminal transition  
- Multiple receipts per session  

## 3.2 Acceptance Preconditions

- LEGITIMACY receipt only if session state == ACTIVE, not BLOCK_PERMANENT, AcceptanceRuleSatisfied == true  
- Deterministic validation of all governance, round, and resolution references  
- Round frozen atomically  
- Forward usability blocks applied for UNDER_REVIEW/RETIRED referenced Resolutions

---

# 4. Round Segmentation Doctrine

## 4.1 Round Creation Rule

- Round 1 at session initialization  
- New round only on RESUME (from PAUSED or BLOCK_TEMPORARY)  
- Prohibited from BLOCK_PERMANENT  
- vote_set must be empty on resume  
- round_index increments by 1  
- All participant, candidate, constraint, vote sets begin fresh

## 4.2 Round Ordering

- Ascending round_index  
- Contiguous, no renumbering  
- Serialization order matches round_index  
- Used in incremental compilation to detect round changes deterministically

---

# 5. Receipt Structure

Canonical fields:

- receipt_type (LEGITIMACY | EXPLORATION)  
- receipt_id (UUIDv7, engine-generated)  
- session_id  
- area_id  
- engine_version  
- spec_set_hash (structural, must match ENG-SPECVERIFY manifest)  
- authority_snapshot_id  
- scope_snapshot_id  
- problem_statement (optional)  
- rounds (ordered sequence of round objects)  
- final_round_index  
- annotations (optional)  
- session_state_at_close (ACCEPTED | CLOSED)  
- acceptance_result (SUCCESS | ABANDONED)  
- created_at (UTC ISO 8601, informational only)  
- hash_algorithm  
- content_hash  
- lineage_references (optional, deterministically ordered list of related receipt_ids)

All fields except informational participate in canonical serialization.

---

# 6. Rule Context Binding

- engine_version → Engine binary executing the session  
- spec_set_hash → deterministic specification set  
- spec_set_hash must participate in canonical serialization  
- Changing spec_set_hash changes content_hash  
- Forward compatibility enforced for incremental compilation comparison

---

# 7. Canonical Serialization Rule Context

- content_hash = hash(canonical_receipt_content)  
- Includes engine_version, spec_set_hash, hash_algorithm  
- Ensures binding of session outcome to exact rule set  
- Guarantees deterministic comparison across Engines

---

# 8. Round Structure

Each round contains:

- round_index  
- round_state (COMPLETED | FINAL_ACCEPTED | ABANDONED)  
- participant_set (ordered lexicographically by participant_id)  
- candidate_set (ordered lexicographically by candidate_id)  
- constraint_set (ordered lexicographically by constraint_id)  
- vote_set (ordered lexicographically by vote_id)  

All sets represent full structural state; no diffs or inference permitted.

---

# 9. Participant Snapshot Semantics

- participant_id unique per epoch, session-scoped  
- display_name frozen per round snapshot  
- No merging of historical participation epochs  
- Ordered lexicographically by participant_id  
- Forward compatibility for incremental compilation

---

# 10. Candidate Snapshot Semantics

- candidate_id unique per round, never reused  
- candidate_content mutable PRE_STANCE, immutable once VOTING begins  
- Only active candidates appear  
- Ordered lexicographically  
- Forward compatibility for incremental compilation and canonical round comparison

---

# 11. Constraint Snapshot Semantics

- constraint_id, session_id, area_id, round_index, constraint_type, constraint_parameters  
- Ordered lexicographically  
- Immutable after VOTING begins

---

# 12. Vote Snapshot Semantics

- vote_id, session_id, area_id, round_index, participant_id, candidate_id, stance (ACCEPT | REJECT | ABSTAIN)  
- Ordered lexicographically  
- Immutable once recorded

---

# 13. Solo Mode

- Deterministic implicit ACCEPT vote if only one participant  
- Appears in final round snapshot  
- Session validation and receipt emission unaffected  
- Forward usability respects UNDER_REVIEW/RETIRED rules

---

# 14. Canonical Serialization & Hashing

- Deterministic field, round, and set ordering  
- Stable UTF-8 JSON  
- Cross-language reproducibility  
- Incremental compilation uses canonical round hashes to detect changes

---

# 15. Deterministic Guarantees

- Identical session state → identical receipt content, round order, set order, content_hash  
- Non-determinism indicates canonicalization or engine defect

---

# 16. Immutability

- Receipts immutable once emitted  
- Survive restore and rehydration  
- Historical governance changes (UNDER_REVIEW/RETIRED, supersession) do not alter past receipts

---

# 17. Integration with Rehydration & Degraded Mode

- Receipt integrity revalidated on restore  
- Hash recomputed and compared  
- Round contiguity validated  
- Structural violation → initialization failure  
- Degraded mode: no new receipts, existing readable, integrity defects not masked

---

# 18. Audit Alignment

- Receipts distinct from audit  
- Terminal structural artifact only  
- Does not create or modify audit  
- Audit remains observational telemetry

---

# 19. Mental Model

- Rounds = structural legitimacy epochs  
- Pause / BLOCK_TEMPORARY → RESUME creates new round  
- Acceptance finalizes current round atomically  
- Participant and candidate IDs = epoch identities  
- UUID = identity, content_hash = integrity, spec_set_hash = rule system  
- Timestamps informational only  
- Forward usability blocks for UNDER_REVIEW / RETIRED applied during session evaluation and receipt generation