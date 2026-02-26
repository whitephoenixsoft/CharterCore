# ENG-RECEIPT — Engine Session Receipt Specification (V2)
Status: DRAFT  
Applies to: Engine Core (V1 Solo Mode, forward-compatible)  
Scope: Deterministic session receipts for audit, reconstruction, legitimacy verification, and federation portability

---

# 1. Purpose

This specification defines how the Engine produces canonical session receipts.

Receipts:

- Provide immutable, verifiable records of session closure  
- Capture participant, candidate, and stance snapshots  
- Record acceptance or abandonment deterministically  
- Enable reconstruction of chronological governance history  
- Provide portable legitimacy artifacts for federation and external verification  

Receipts are **engine-authoritative descriptive artifacts** of session outcomes.

Receipts do not create legitimacy.  
They describe legitimacy created by ACCEPTED sessions.

---

# 2. Receipt Types

## 2.1 LEGITIMACY

Emitted when a session transitions to ACCEPTED.

Captures:

- Full governance context
- Frozen participant set
- Frozen candidate set
- Final stance set (including implicit votes)
- Deterministic acceptance result

This receipt anchors legitimacy history.

## 2.2 EXPLORATION

Emitted when a session transitions to CLOSED without acceptance.

Captures:

- Final participant set
- Final candidate set
- Closure context
- Explicit abandonment

Does not create legitimacy.

---

# 3. Receipt Structure

A receipt must include the following fields:

- `receipt_type` (LEGITIMACY | EXPLORATION)
- `receipt_id` (engine-generated UUIDv7)
- `session_id`
- `engine_version`
- `topic`
- `area_id`
- `authority_snapshot_id`
- `scope_snapshot_id`
- `participant_snapshot` (ordered, deterministic set)
- `candidate_snapshot` (ordered, deterministic list)
- `stance_snapshot` (explicit and implicit votes, deterministic order)
- `annotations` (optional, immutable)
- `session_state_at_close` (ACCEPTED | CLOSED)
- `session_phase_at_close` (TERMINAL)
- `acceptance_result` (SUCCESS | ABANDONED)
- `participant_reconfirmation` (list of reconfirmation events, if any)
- `lineage_references` (optional list of related receipt_ids)
- `created_at` (timestamp, informational only)
- `hash_algorithm` (string identifier, e.g., SHA-256)
- `content_hash` (hash of canonical serialized receipt content)

---

# 4. Canonical Serialization & Hashing

## 4.1 Canonical Form

The `content_hash` must be computed over a canonical serialization of the receipt.

Canonicalization rules must ensure:

- Deterministic field ordering  
- Deterministic list ordering  
- Stable encoding format  
- Cross-language reproducibility  

Any deviation invalidates determinism.

## 4.2 Hash Semantics

- `content_hash` is integrity metadata only.  
- Hash does not influence legitimacy.  
- Hash does not influence acceptance.  
- Hash does not influence restore.  
- Hash does not influence identity resolution.  

Identity is defined exclusively by `receipt_id` (UUIDv7).

## 4.3 Algorithm Migration

- `hash_algorithm` must be explicitly declared.  
- Hash algorithm changes do not alter receipt identity.  
- Historical receipts remain valid under their original algorithm.  

Hash migration never alters legitimacy history.

---

# 5. Generation Triggers

## 5.1 Session Accepted

When a session transitions to ACCEPTED:

- Produce LEGITIMACY receipt  
- Freeze participant, candidate, and stance sets  
- Insert implicit solo vote if applicable  
- Capture participant reconfirmation events  
- Compute canonical hash  
- Persist receipt atomically with acceptance  

## 5.2 Session Closed Without Acceptance

When a session transitions to CLOSED:

- Produce EXPLORATION receipt  
- Capture final participant and candidate state  
- Mark `acceptance_result = ABANDONED`  
- Compute canonical hash  

## 5.3 Resume & Reconfirmation

If a session resumes and requires participant reconfirmation:

- Each reconfirmation event must be recorded  
- Final receipt must include the reconfirmation history  
- Chronology must be reconstructable independent of timestamps  

---

# 6. Deterministic Guarantees

Receipts must be:

- Fully reconstructable from engine state  
- Deterministic across implementations  
- Canonically hashable across environments  
- Independent of storage backend  

Given identical session state, the receipt and its hash must be identical.

A mismatch indicates:

- Non-determinism  
- Engine defect  
- Canonicalization violation  

---

# 7. Immutability

- Receipts are immutable once emitted.  
- No field may be modified.  
- Session modifications after receipt creation do not alter prior receipts.  
- Receipts survive restore operations.  

Receipts are append-only artifacts.

---

# 8. Integration with Restore & Import

## 8.1 Restore Mode

- Receipts may be included with domain objects.  
- Receipt hash may be revalidated.  
- Restore must not alter receipt content.  

## 8.2 Relaxed Import / CLI Baseline Review

- Receipts may be absent in proposal-only imports.  
- No legitimacy is inferred from imported receipts without structural validation.  
- CLI may aggregate receipts into higher-level REVIEW artifacts.  

---

# 9. Audit & Legal Alignment

Receipts provide:

- Canonical closure proof  
- Deterministic participant record  
- Explicit vote record  
- Tamper-evident integrity (via hash)  

Receipts may be:

- Serialized externally  
- Anchored in external audit systems  
- Used in compliance or legal contexts  

Receipts remain descriptive; legitimacy originates in session acceptance.

---

# 10. Solo Mode Considerations

- Implicit ACCEPT vote must appear in `stance_snapshot`.  
- Participant set contains exactly one member.  
- Any resume-triggered reconfirmation must be recorded.  

Solo Mode does not bypass receipt requirements.

---

# 11. Mental Model

- Receipts are frozen session-state artifacts.  
- LEGITIMACY receipts anchor governance history.  
- EXPLORATION receipts anchor deliberation history.  
- UUID defines identity.  
- Hash defines integrity.  
- Timestamps are informational only.  
- Receipts describe legitimacy; they do not create it.  

---

# Alignment

- ENG-INTEGRITY  
- ENG-DOMAIN  
- ENG-SESSION  
- ENG-API  
- ENG-IMPORT  
- ENG-AUD  

Receipts formalize session closure, enable deterministic reconstruction, and provide portable legitimacy artifacts for federation and long-term verification.