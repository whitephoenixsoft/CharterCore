# ENG-RECEIPT — Engine Session Receipt Specification  
Status: FROZEN (v4 – Deterministic Canonicalization & Multi-Error Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic session receipts for audit, reconstruction, legitimacy verification, and federation portability  

Authority: Subordinate to ENG-DOMAIN, ENG-SESSION, ENG-ERROR, ENG-INTEGRITY, ENG-API  

---

# 1. Purpose

This specification defines how the Engine produces canonical session receipts.

Receipts:

- Provide immutable, verifiable records of session terminal transition  
- Capture frozen governance, participant epoch, candidate, and stance snapshots  
- Record deterministic acceptance or abandonment outcomes  
- Enable reconstruction of governance chronology  
- Provide portable legitimacy artifacts  

Receipts are engine-authoritative descriptive artifacts.

Receipts do not create legitimacy.  
Legitimacy is created only by successful session acceptance.

Receipts formalize and freeze closure.

Participant identity within receipts is epoch-based and session-scoped.

---

# 2. Receipt Types

## 2.1 LEGITIMACY

Emitted when a session transitions to ACCEPTED.

Captures:

- Governance context snapshot  
- Final participant epoch set  
- Frozen candidate set  
- Final stance set (explicit and implicit votes)  
- Deterministic acceptance outcome  

LEGITIMACY receipts anchor governance history.

---

## 2.2 EXPLORATION

Emitted when a session transitions to CLOSED without acceptance.

Captures:

- Final participant epoch set  
- Final candidate set  
- Final stance state (if any)  
- Explicit abandonment outcome  

Does not create legitimacy.

---

# 3. Generation Rules

## 3.1 Terminal Transition Requirement

Exactly one receipt must be emitted per session.

Receipt emission is:

- Atomic with terminal state transition  
- Deterministic  
- Immutable  

If:

- ACCEPTED → LEGITIMACY receipt  
- CLOSED → EXPLORATION receipt  

No receipt may be emitted outside a TERMINAL transition.

Fail if:

- Terminal state reached without receipt  
- Receipt emitted without terminal transition  
- More than one receipt exists for a session  

---

## 3.2 Acceptance Preconditions

LEGITIMACY receipt may only be emitted after:

- Full deterministic validation pass  
- All violations accumulated  
- No blocking conditions remain  
- Outcome derived  

Receipt must reflect the fully frozen state created during acceptance commit.

---

# 4. Receipt Structure

A receipt must contain the following canonical fields:

- `receipt_type` (LEGITIMACY | EXPLORATION)  
- `receipt_id` (UUIDv7, engine-generated)  
- `session_id`  
- `area_id`  
- `engine_version`  
- `authority_snapshot_id`  
- `scope_snapshot_id`  
- `participant_snapshot` (deterministically ordered)  
- `candidate_snapshot` (deterministically ordered)  
- `stance_snapshot` (deterministically ordered)  
- `participant_reconfirmation` (ordered reconfirmation history, if any)  
- `annotations` (optional, immutable)  
- `session_state_at_close` (ACCEPTED | CLOSED)  
- `session_phase_at_close` (TERMINAL)  
- `acceptance_result` (SUCCESS | ABANDONED)  
- `created_at` (UTC ISO 8601 with timezone, informational only)  
- `hash_algorithm`  
- `content_hash`  

Optional:

- `lineage_references` (deterministically ordered list of related receipt_ids)

All fields participate in canonical serialization unless explicitly marked informational.

`created_at` is informational and must not influence ordering, identity, or legitimacy.

---

# 5. Participant Snapshot Semantics

## 5.1 Participation Epoch Model

Participants represent participation epochs, not human identities.

Each `participant_snapshot` entry must include:

- `participant_id` (UUIDv7)  
- `display_name` (string at time of closure)  

Rules:

- `participant_id` represents a single non-reusable participation epoch  
- Only the final active epoch set at closure appears  
- No merging of historical epochs permitted  
- No inference of identity continuity  
- No rewriting of prior epochs  

If a participant left and rejoined:

- Only the final active epoch appears  
- Prior epochs are not included  

Participant identity continuity across resumes is not assumed.

---

## 5.2 Deterministic Ordering

`participant_snapshot` must be ordered lexicographically by `participant_id`.

Ordering must not depend on:

- Timestamp  
- Insertion order  
- Display name  
- Vote order  

Identical session state → identical participant ordering across implementations.

---

## 5.3 Reconfirmation Recording

If resume events occurred:

- Each reconfirmation event must be recorded in `participant_reconfirmation`  
- Ordering must be deterministic  
- Reconstruction must not rely on timestamps  

Reconfirmation history is descriptive only and does not alter legitimacy.

---

# 6. Candidate Snapshot Semantics

`candidate_snapshot` must:

- Contain the fully frozen candidate set  
- Preserve canonical candidate content  
- Be ordered deterministically  

Ordering rule:

- Lexicographic by `candidate_id`  

Candidate content must match frozen state at transition.

---

# 7. Stance Snapshot Semantics

`stance_snapshot` must:

- Include all explicit votes  
- Include implicit Solo Mode vote if applicable  
- Reference participant_id and candidate_id  
- Be ordered deterministically  

Ordering rule:

1. Lexicographic by candidate_id  
2. Then lexicographic by participant_id  

Votes are immutable once recorded.

No inference permitted.

---

# 8. Canonical Serialization & Hashing

## 8.1 Canonicalization Requirement

`content_hash` must be computed over canonical serialized receipt content.

Canonicalization must ensure:

- Deterministic field ordering  
- Deterministic list ordering  
- Stable encoding format  
- Cross-language reproducibility  
- UTF-8 encoding  

EvaluationReport ordering rules do not apply here; receipt ordering rules are defined in this specification.

Deviation constitutes engine defect.

---

## 8.2 Hash Semantics

- `content_hash` ensures integrity only  
- Does not influence legitimacy  
- Does not influence acceptance  
- Does not influence restore  
- Does not influence identity  

Identity is defined exclusively by `receipt_id`.

---

## 8.3 Algorithm Declaration

- `hash_algorithm` must be explicitly declared  
- Algorithm migration does not alter receipt identity  
- Historical receipts remain valid under their original algorithm  

Hash changes never rewrite legitimacy history.

---

# 9. Deterministic Guarantees

Given identical session state at terminal transition:

- Receipt content must be identical  
- Field ordering must be identical  
- Snapshots must be identical  
- `content_hash` must be identical  

A mismatch indicates:

- Non-determinism  
- Canonicalization violation  
- Engine defect  

Receipts must be reconstructable from engine state.

---

# 10. Immutability

- Receipts are immutable once emitted  
- No field may be modified  
- Receipts survive restore and rehydration  
- Later governance changes do not alter prior receipts  

Receipts are append-only artifacts.

---

# 11. Integration with Rehydration & Degraded Mode

## 11.1 Rehydration Validation

During rehydration:

- Receipt integrity may be revalidated  
- Hash may be recomputed and compared  
- Structural references must be validated  

Structural violation results in initialization failure.

Non-fatal integrity issues may trigger degraded read-only mode per ENG-INTEGRITY.

---

## 11.2 Degraded Mode

In degraded mode:

- No new receipts may be emitted  
- Existing receipts remain readable  
- Integrity defects must not be masked  

Degraded mode prohibits partial legitimacy compilation.

---

# 12. Solo Mode

If session operated in Solo Mode:

- Implicit ACCEPT vote must appear in `stance_snapshot`  
- Exactly one participation epoch must appear  
- Any resume-triggered reconfirmation must be recorded  

Solo Mode does not bypass receipt emission.

---

# 13. Audit Alignment

Receipts:

- Are distinct from audit events  
- Do not replace lifecycle audit  
- Do not encode EvaluationReport errors  
- Represent only terminal state outcome  

Audit is descriptive runtime telemetry.  
Receipt is terminal structural artifact.

---

# 14. Mental Model

Receipts freeze terminal session state.

Participant IDs represent participation epochs.  
Only final epoch set appears.  
No historical merging.  

LEGITIMACY receipts anchor governance history.  
EXPLORATION receipts anchor deliberation history.  

UUID defines identity.  
Hash defines integrity.  
Timestamps are informational only.  

Receipts describe legitimacy.  
They do not create it.