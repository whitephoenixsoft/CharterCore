# ENG-RECEIPT — Engine Session Receipt Specification (V1)
Status: DRAFT
Applies to: Engine Core (V1 Solo Mode)
Scope: Deterministic session receipts for audit, reconstruction, and legitimacy verification

---

# 1. Purpose

This specification defines how the Engine produces canonical session receipts:

- Provide immutable, verifiable records of session closure
- Capture participant, candidate, and stance snapshots
- Record acceptance and abandonment deterministically
- Enable cross-referencing for CLI REVIEW receipts
- Facilitate restore and reconstruction of historical session state

Receipts are the **engine-authoritative proof** of what happened.

---

# 2. Receipt Types

## 2.1 LEGITIMACY

- Emitted when a session **ACCEPTED** a candidate
- Records full snapshots for audit and legal admissibility

## 2.2 EXPLORATION

- Optional for sessions closed without acceptance
- Records closure of PRE_STANCE sessions or abandoned deliberation

---

# 3. Receipt Structure

A session receipt must include the following fields:

- `receipt_type` (LEGITIMACY | EXPLORATION)
- `receipt_id` (engine-generated UUIDv7)
- `session_id`
- `engine_version` (for schema and logic determinism)
- `topic`
- `area_id`
- `authority_snapshot_id`
- `scope_snapshot_id`
- `participant_snapshot` (set of participant IDs at snapshot)
- `candidate_snapshot` (list of candidates, immutable)
- `stance_snapshot` (list of stances including solo mode implicit votes)
- `annotations` (optional, immutable)
- `session_state_at_close` (ACCEPTED | CLOSED)
- `session_phase_at_close` (TERMINAL)
- `acceptance_result` (SUCCESS | ABANDONED)
- `participant_reconfirmation` (if session resumed prior to acceptance)
- `lineage_references` (optional list of receipt_ids referenced)
- `created_at` (timestamp)
- `content_hash` (deterministic hash of all above content)

---

# 4. Generation Triggers

1. **Session Accepted**
   - Produce LEGITIMACY receipt
   - Freeze participant, candidate, and vote sets
   - Include implicit vote if in solo mode
   - Capture any participant reconfirmation events (resume with PRE_STANCE participants)
   
2. **Session Closed without Acceptance**
   - Produce EXPLORATION receipt
   - Capture final participant set
   - Capture candidates and annotations
   - Mark `acceptance_result = ABANDONED`

3. **Session Resume**
   - If participants were revalidated, record `participant_reconfirmation` in subsequent receipt

---

# 5. Deterministic Guarantees

- Receipts must be **fully reconstructable** from session audit logs
- `content_hash` must be stable across:
  - Programming language
  - Storage backends
  - Operating system
- Any receipt mismatch indicates non-determinism or engine fault

---

# 6. Immutability

- Once created, a receipt is immutable
- No field may change
- Session modifications after receipt creation do **not** alter prior receipts
- Receipts survive restore operations and can be re-imported for historical verification

---

# 7. Integration with Restore / Baseline

- **Restore Mode:** Include receipts with domain objects to ensure historical legitimacy proof
- **Baseline Review:** Receipts can be aggregated by the CLI to create REVIEW receipts
- Receipts reference **lineage_references** to reconstruct session and review history

---

# 8. Audit & Legal Alignment

- Receipts provide canonical audit trail
- `content_hash` allows tamper-evident verification
- May be used as admissible evidence in compliance or legal contexts
- CLI may display receipts or serialize them for external verification

---

# 9. Solo Mode Considerations

- Implicit ACCEPT vote is included in `stance_snapshot`
- Participant set in solo mode is always single
- Any session resume reconfirmation is recorded in `participant_reconfirmation`

---

# 10. Mental Model

- Receipts are **frozen session state artifacts**
- LEGITIMACY receipts = accepted sessions  
- EXPLORATION receipts = closed/abandoned sessions  
- Participant reconfirmation events are explicitly recorded  
- Receipts are independent of audit events but cross-referenceable
- `content_hash` ensures deterministic integrity

---

# Alignment

- ENG-DOMAIN  
- ENG-SESSION  
- ENG-API (retrieval API)  
- ENG-IMPORT  
- ENG-AUD  

Receipts formalize session closure, enable reconstruction, and anchor legitimacy proofs.