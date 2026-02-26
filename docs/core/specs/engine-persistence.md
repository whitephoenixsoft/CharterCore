# ENG-PERSISTENCE — Atomic Commit & Crash Recovery Specification  
Status: FROZEN (v1 – Acceptance Atomicity Defined)  
Applies to: Engine Core (V1/V2+)  
Scope: Durable commit boundaries, crash recovery guarantees, and rehydration integrity  

Authority: Subordinate to ENG-SESSION, ENG-DECISION, ENG-RECEIPT, ENG-SUPERSESSION, ENG-INTEGRITY  

---

# 1. Purpose

This document defines:

- The atomic durability boundary for legitimacy creation  
- Crash recovery guarantees  
- Storage adapter requirements  
- Rehydration integrity verification  
- Separation of legitimacy artifacts from audit artifacts  

This specification governs persistence behavior.  

It does not mandate a specific storage technology.  
It mandates atomic guarantees.

---

# 2. Foundational Principle

## ENG-PERSISTENCE-01 — Legitimacy Events Are Indivisible

Legitimacy creation must be durably atomic.

A legitimacy event must either:

- Fully persist  
- Or not exist at all  

Partial persistence is structural corruption.

---

# 3. Acceptance Atomic Boundary

## ENG-PERSISTENCE-02 — Mandatory Atomic Commit Set

When a session transitions to ACCEPTED, the following objects must commit in a single durable atomic operation:

1. Session state mutation → ACCEPTED  
2. Resolution creation  
3. Supersession graph mutation (if applicable)  
4. LEGITIMACY receipt creation  

These four elements form the Legitimacy Atomic Boundary.

All must commit or none must commit.

Fail if:

- Resolution exists without ACCEPTED session  
- ACCEPTED session exists without Resolution  
- Resolution exists without LEGITIMACY receipt  
- Receipt exists without Resolution  
- Supersession edges exist without Resolution  

---

# 4. Terminal Closure Atomic Boundary

## ENG-PERSISTENCE-03 — Closure Atomic Commit

When a session transitions to CLOSED (without acceptance):

The following must commit atomically:

1. Session state mutation → CLOSED  
2. EXPLORATION receipt creation  

No Resolution is created.

Partial persistence is forbidden.

---

# 5. Audit Separation Rule

## ENG-PERSISTENCE-04 — Audit Is Outside the Atomic Boundary

Audit events are append-only and descriptive.

Audit emission:

- Must not be required for legitimacy validity  
- Must not be part of the acceptance atomic boundary  
- May fail independently of legitimacy persistence  

If:

- Acceptance commit succeeds  
- Audit emission fails due to crash  

The legitimacy event remains valid.

Audit may be reconstructed from domain objects.

Receipts may not be reconstructed.

---

# 6. Crash Recovery Guarantees

## ENG-PERSISTENCE-05 — Pre-Commit Crash

If crash occurs before atomic commit:

- Session remains non-ACCEPTED  
- No Resolution exists  
- No receipt exists  

System behaves as though acceptance never occurred.

---

## ENG-PERSISTENCE-06 — Post-Commit Crash

If crash occurs after atomic commit:

- Session must be ACCEPTED  
- Resolution must exist  
- Receipt must exist  
- Supersession graph must be consistent  

System must not require replay or repair to reconstruct legitimacy.

---

# 7. Rehydration Integrity Verification

## ENG-PERSISTENCE-07 — Mandatory Integrity Checks

Upon engine rehydration:

For every session where state = ACCEPTED:

Engine must verify:

1. Exactly one Resolution references session_id  
2. Exactly one LEGITIMACY receipt references session_id  
3. Receipt participant snapshot matches frozen session participant set  
4. Receipt candidate snapshot matches Resolution snapshot  
5. Receipt stance snapshot matches recorded votes  
6. Receipt content_hash verifies deterministically  
7. Supersession graph integrity holds  

Fail if any condition fails.

Engine must refuse initialization.

No auto-repair permitted.

---

## ENG-PERSISTENCE-08 — Closure Verification

For every session where state = CLOSED:

- Exactly one EXPLORATION receipt must exist  
- Receipt snapshot must match session terminal state  

Fail on mismatch.

---

# 8. Storage Adapter Requirements

## ENG-PERSISTENCE-09 — Atomic Storage Capability

The storage layer must support:

- Atomic durable commit across multiple object writes  
- All-or-nothing transactional behavior  
- Crash-safe durability  

Acceptable implementation models include:

- ACID database transaction  
- Atomic write batch in key-value store  
- Write-ahead log with commit marker  
- Single-file append with commit boundary  

The specification is storage-agnostic.

The guarantee is not optional.

---

# 9. Prohibited Behaviors

Engine must not:

- Write Resolution before session state  
- Write receipt asynchronously after acceptance  
- Write supersession edges outside acceptance transaction  
- Repair partial commits automatically  
- Generate receipts lazily  
- Reconstruct receipts from audit  

Partial durability is structural corruption.

---

# 10. Non-Legitimacy Transitions

State transitions that do not create legitimacy:

- ACTIVE → PAUSED  
- ACTIVE → BLOCK_TEMPORARY  
- BLOCK_TEMPORARY → ACTIVE  
- BLOCK_PERMANENT transitions  

Do not require atomic grouping beyond their own state mutation.

Only ACCEPTED and CLOSED transitions require atomic multi-object commit.

---

# 11. Determinism Guarantee

Given identical:

- Pre-acceptance state  
- Votes  
- Authority  
- Constraints  

Acceptance must produce identical:

- Resolution content  
- Receipt content  
- Supersession edges  
- content_hash  

Persistence must not introduce nondeterminism.

---

# 12. Legal & Evidentiary Integrity

Legitimacy artifacts (Resolution + Receipt) must be:

- Indivisible  
- Durable  
- Deterministic  
- Verifiable  

Partial persistence invalidates structural trust.

Receipt integrity provides:

- Cryptographic anchoring (via content_hash)  
- Snapshot immutability  
- Verifiable linkage to session and resolution  

Audit is supplemental.

Receipt + Resolution form the admissible legitimacy pair.

---

# 13. Summary Guarantees

- Acceptance is a single durable event  
- Resolution + Session + Supersession + Receipt commit together  
- Audit is outside atomic boundary  
- Crash before commit → no legitimacy  
- Crash after commit → full legitimacy preserved  
- Rehydration must validate structural consistency  
- No auto-repair allowed  
- Storage adapter must guarantee atomic durability  

---

# Mental Model

Legitimacy is a commit.

Not a sequence.  
Not an event stream.  
Not a best effort.

A legitimacy event either exists completely  
or it never happened.