# ENG-RECEIPT — Engine Session Receipt Specification (V3)
Status: DRAFT (Participant Epoch Snapshot Clarified)  
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

Participant identity within receipts is epoch-based and session-scoped.

---

# 2. Receipt Types

## 2.1 LEGITIMACY

Emitted when a session transitions to ACCEPTED.

Captures:

- Full governance context  
- Final participant epoch set  
- Frozen candidate set  
- Final stance set (including implicit votes)  
- Deterministic acceptance result  

This receipt anchors legitimacy history.

## 2.2 EXPLORATION

Emitted when a session transitions to CLOSED without acceptance.

Captures:

- Final participant epoch set  
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
- `participant_snapshot` (ordered, deterministic set of participation epochs)  
- `candidate_snapshot` (ordered, deterministic list)  
- `stance_snapshot` (explicit and implicit votes, deterministic order)  
- `annotations` (optional, immutable)  
- `session_state_at_close` (ACCEPTED | CLOSED)  
- `session_phase_at_close` (TERMINAL)  
- `acceptance_result` (SUCCESS | ABANDONED)  
- `participant_reconfirmation` (ordered list of reconfirmation events, if any)  
- `lineage_references` (optional list of related receipt_ids)  
- `created_at` (timestamp, informational only)  
- `hash_algorithm` (string identifier, e.g., SHA-256)  
- `content_hash` (hash of canonical serialized receipt content)

---

# 4. Participant Snapshot Semantics

## 4.1 Participation Epoch Model

Participants in receipts represent **participation epochs**, not abstract human identities.

Each entry in `participant_snapshot` must include:

- `participant_id` (UUIDv7, engine-generated)  
- `display_name` (string at time of acceptance/closure)  

Rules:

- `participant_id` represents a single, non-reusable participation epoch.  
- IDs in the snapshot correspond exclusively to the final active epoch set at acceptance or closure.  
- No historical epoch merging is permitted.  
- If a participant left and rejoined, the receipt must contain only the final epoch instance active at closure.  
- Prior epochs are not merged, rewritten, or inferred.  

Participant identity continuity across resumes is not assumed by the Engine.

---

## 4.2 Deterministic Ordering

`participant_snapshot` must be serialized in deterministic order.

Ordering rule:

- Lexicographic ordering by `participant_id`  

No ordering may depend on:

- Timestamp  
- Addition order  
- Display name  
- Vote order  

Given identical session state, participant ordering must be identical across implementations.

---

## 4.3 Reconfirmation Recording

If the session experienced resume events requiring reconfirmation:

- Each reconfirmation event must be recorded in `participant_reconfirmation`  
- Reconfirmation history must allow deterministic reconstruction of participation epochs  
- Chronology must be reconstructable independent of timestamps  

The receipt captures the final epoch set and the reconfirmation history that led to it.

---

# 5. Canonical Serialization & Hashing

## 5.1 Canonical Form

The `content_hash` must be computed over canonical serialization of the receipt.

Canonicalization must ensure:

- Deterministic field ordering  
- Deterministic list ordering  
- Stable encoding format  
- Cross-language reproducibility  

Any deviation invalidates determinism.

## 5.2 Hash Semantics

- `content_hash` is integrity metadata only.  
- Hash does not influence legitimacy.  
- Hash does not influence acceptance.  
- Hash does not influence restore.  
- Hash does not influence identity resolution.  

Identity is defined exclusively by `receipt_id`.

## 5.3 Algorithm Migration

- `hash_algorithm` must be explicitly declared.  
- Hash algorithm changes do not alter receipt identity.  
- Historical receipts remain valid under their original algorithm.  

Hash migration never alters legitimacy history.

---

# 6. Generation Triggers

## 6.1 Session Accepted

When a session transitions to ACCEPTED:

- Produce LEGITIMACY receipt  
- Freeze participant epoch snapshot  
- Freeze candidate snapshot  
- Freeze stance snapshot  
- Insert implicit solo vote if applicable  
- Capture participant reconfirmation events  
- Compute canonical hash  
- Persist receipt atomically with acceptance  

## 6.2 Session Closed Without Acceptance

When a session transitions to CLOSED:

- Produce EXPLORATION receipt  
- Capture final participant epoch snapshot  
- Capture final candidate state  
- Mark `acceptance_result = ABANDONED`  
- Compute canonical hash  

## 6.3 Resume & Reconfirmation

On resume:

- Prior participation epochs are terminated (per ENG-DECISION)  
- Re-additions generate new participant_id values  
- Receipt must reflect only the final epoch set  
- Historical epoch reuse is prohibited  

---

# 7. Deterministic Guarantees

Receipts must be:

- Fully reconstructable from engine state  
- Deterministic across implementations  
- Canonically hashable across environments  
- Independent of storage backend  

Given identical session state, the receipt and its `content_hash` must be identical.

A mismatch indicates:

- Non-determinism  
- Engine defect  
- Canonicalization violation  

---

# 8. Immutability

- Receipts are immutable once emitted.  
- No field may be modified.  
- Session modifications after receipt creation do not alter prior receipts.  
- Receipts survive restore operations.  

Receipts are append-only artifacts.

---

# 9. Integration with Restore & Import

## 9.1 Restore Mode

- Receipts may be included with domain objects.  
- Receipt hash may be revalidated.  
- Restore must not alter receipt content.  

## 9.2 Relaxed Import / CLI Baseline Review

- Receipts may be absent in proposal-only imports.  
- No legitimacy is inferred from imported receipts without structural validation.  
- CLI may aggregate receipts into higher-level REVIEW artifacts.  

---

# 10. Audit & Legal Alignment

Receipts provide:

- Canonical closure proof  
- Deterministic participant epoch record  
- Explicit vote record  
- Tamper-evident integrity (via hash)  

Receipts may be:

- Serialized externally  
- Anchored in external audit systems  
- Used in compliance or legal contexts  

Receipts remain descriptive; legitimacy originates in session acceptance.

---

# 11. Solo Mode Considerations

- Implicit ACCEPT vote must appear in `stance_snapshot`.  
- Participant snapshot contains exactly one participation epoch.  
- Any resume-triggered reconfirmation must be recorded.  

Solo Mode does not bypass receipt requirements.

---

# 12. Mental Model

- Receipts are frozen session-state artifacts.  
- Participant IDs represent participation epochs.  
- Snapshot reflects the final epoch set only.  
- No historical epoch merging.  
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
- ENG-DECISION  
- ENG-API  
- ENG-IMPORT  
- ENG-AUD  

Receipts formalize session closure, enforce deterministic participation epochs, enable reconstruction, and provide portable legitimacy artifacts for federation and long-term verification.