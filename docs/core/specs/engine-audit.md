# ENG-AUD — Audit Event Specification
Status: DRAFT (Adjusted for V3.1 – Governance & Incremental Compilation)
Applies to: Engine Core (V1/V2+)

Authority: Subordinate to ENG-DOMAIN, ENG-SESSION, ENG-RECEIPT, ENG-ERROR, ENG-INTEGRITY, ENG-API

If conflict exists, structural and legitimacy rules defined elsewhere take precedence.

Audit is observational only.

---

# 1. Purpose

## ENG-AUD-01 — Audit Preserves Accountability

Audit records exist to answer:

- What explicit engine action occurred?  
- When did it occur?  
- Under which Authority and Scope snapshot context?  
- Which structural object was affected?  

Audit records:

- Do not create legitimacy  
- Do not alter structural state  
- Do not confer authority  
- Do not participate in ACTIVE derivation or evaluation  
- Do not participate in restore or acceptance validation  

Fail if:

- Audit data influences acceptance  
- Audit data influences restore behavior  
- Removing audit changes legitimacy outcomes  
- Audit ordering alters deterministic engine behavior  

Audit is memory, not law.  

Incremental compilation and session replays must **ignore audit events**; only the resolution index and canonical ordering matter.

---

# 2. Core Principles

## ENG-AUD-02 — Append-Only

- Audit events are immutable, never edited or deleted implicitly  
- Corrections require new audit events  

Fail if:

- Audit records modified in place  
- Audit history rewritten  
- Events silently dropped  

## ENG-AUD-03 — Engine Is Audit Producer Only

- Engine emits audit events as side effects of successful commands  
- Must not consult audit events for:
  - Rehydration  
  - Evaluation  
  - Acceptance  
  - Blocking decisions  
  - Supersession derivation  
- Must not reconstruct or repair state using audit  

Fail if:

- Engine logic depends on audit queries  
- Restore requires audit records  
- Acceptance behavior differs based on audit presence  

---

# 3. Deterministic Emission Rules

## ENG-AUD-04 — Emission After Commit

- Audit emission occurs only after successful state mutation and terminal commit  
- Not emitted for failed commands or evaluation-only operations  
- Does not affect transaction atomicity  

---

# 4. Audit Scope Model

## ENG-AUD-05 — Exactly One Scope Per Event

- Each audit event belongs to exactly one scope: GLOBAL or AREA:<area_id>  
- Scope defines organizational boundary only  

Fail if:

- Auditable action exists without scope  
- Scope influences legitimacy, restore, or evaluation  

## ENG-AUD-06 — GLOBAL Scope Is Immutable

- GLOBAL scope must always exist  
- Cannot be deleted, superseded, or renamed  
- Used for area lifecycle, import/export, hash migrations, cross-area structural events, engine initialization/halt  

Fail if:

- GLOBAL scope missing  
- GLOBAL scope mutable  

---

# 5. Auditable Actions

## ENG-AUD-07 — Required Audit Emissions

Successful engine actions that must emit audit events include:

Lifecycle:

- Session creation, pause, resume, close, terminal transition (ACCEPTED/CLOSED)

Participation:

- Participant added/removed  
- Resume-triggered participant reset  

Structure:

- Candidate added/removed (PRE_STANCE only)  
- First/subsequent vote recorded  
- Resolution created (ACCEPTED)  
- Resolution superseded  
- Resolution retired  
- Resolution state change to UNDER_REVIEW  

Governance:

- Authority slot filled  
- Scope slot filled  
- BLOCK_TEMPORARY / BLOCK_PERMANENT entered  

Integrity & Maintenance:

- Engine rehydration success  
- Rehydration structural failure  
- Degraded mode activation  
- fsck execution (diagnostic only)  
- Hash migration  

Import/Export:

- Import start, validation result, completion  
- Export operation  

Fail if:

- Any state mutation above occurs without emitting audit  
- Audit emission changes engine outcome  

---

# 6. Identity Rules

## ENG-AUD-08 — Audit Event Identity

Each audit event includes:

- `event_id` — UUIDv7  
- `occurred_at` — UTC ISO 8601 timestamp with timezone  

Rules:

- event_id must be UUIDv7 and globally unique  
- Timestamp is **informational only**  
- Event ordering for legitimacy is **never derived from timestamp**  

Incremental compilation sessions and replay use canonical index, not audit timestamps, for deterministic ordering.

Fail if:

- Non-UUIDv7 identifiers used  
- Timestamp influences structural derivation or acceptance  

---

# 7. Canonical Audit Event Structure

## ENG-AUD-09 — Stable Event Shape

Every audit event includes:

- `event_id`  
- `event_type`  
- `occurred_at`  
- `actor`  
- `scope`  
- `subject`  
- `context`  
- `details`  

### subject

- `object_type`  
- `object_id`  

### context

- explicit nulls where not applicable:  
  - `area_id`  
  - `session_id`  
  - `authority_resolution_id`  
  - `scope_resolution_id`  

### details

- Non-semantic only  
- Must not contain canonical domain objects  
- Must not be required for restore or legitimacy derivation  

Fail if:

- Event structure varies  
- Context or subject omitted  
- Audit payload used to reconstruct state  

---

# 8. Actor Semantics

## ENG-AUD-10 — Actor Is Informational

- `actor` opaque identifier, may be null  
- Does not confer authority  
- Does not imply permissions  

Fail if:

- Actor alters legitimacy or deterministic behavior  

---

# 9. Relationship to Legitimacy

## ENG-AUD-11 — Audit Never Creates Legitimacy

- Audit may reference acceptance  
- Must not imply acceptance  
- Resolution must exist in domain storage to be legitimate  
- Cannot substitute for session acceptance or imply quorum  

Fail if:

- Resolution exists only in audit  
- ACTIVE derivation consults audit  
- Acceptance inferred from audit alone  

---

# 10. Ordering & Export

## ENG-AUD-12 — Deterministic Ordering

- Audit logs preserve deterministic ordering  
- Lexicographic by `event_id` for export  
- Must not depend on storage iteration or timestamp for legitimacy  

Export must:

- Preserve scope and event identity  
- Not filter implicitly  
- Not mutate event shape  

---

# 11. Rehydration & Degraded Mode

## ENG-AUD-13 — Rehydration Independence

- Engine does not require audit presence or ordering  
- Corrupted audit does not affect structural or legitimacy derivation  

## ENG-AUD-14 — Degraded Mode Constraints

- Audit remains append-only  
- Mutating commands prohibited  
- Audit does not enable partial legitimacy reconstruction  

---

# 12. fsck Interaction

## ENG-AUD-15 — fsck Is Observational

- fsck may emit audit events  
- Must not mutate domain objects or repair state using audit  

Fail if:

- fsck modifies objects or rewrites audit  

---

# 13. Storage Guarantees

## ENG-AUD-16 — Audit Outlives Subjects

- Audit must remain accessible even if objects deleted, sessions closed, resolutions superseded  
- Deleting domain objects must not delete audit  

Fail if:

- Audit absence alters legitimacy  
- Restore requires audit presence  

---

# 14. Design Guarantees

Audit guarantees:

- No silent legitimacy transitions  
- No invisible supersession  
- No invisible Authority or Scope slot changes  
- No invisible import/migration  
- No invisible participant epoch reset  

Audit does **not** guarantee:

- Legitimacy correctness  
- Structural integrity  
- Deterministic evaluation  
- Consensus validity  

Those governed elsewhere.

---

# 15. Mental Model

- Legitimacy lives in sessions  
- Structure lives in domain objects and supersession edges  
- Integrity lives in rehydration and halt rules  
- Determinism lives in evaluation and canonicalization  
- Receipts freeze terminal state  
- Audit records explicit action  

Audit observes only.  
