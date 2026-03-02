# ENG-AUD — Audit Event Specification  
Status: FROZEN (v3 – Deterministic Alignment & Engine Isolation Clarified)  
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

- Do not create legitimacy.
- Do not alter structural state.
- Do not confer authority.
- Do not participate in ACTIVE derivation.
- Do not participate in restore.
- Do not participate in acceptance validation.
- Do not participate in EvaluationReport generation.

Fail if:

- Audit data influences acceptance.
- Audit data influences restore behavior.
- Removing audit changes legitimacy outcomes.
- Audit ordering alters deterministic engine behavior.

Audit is memory, not law.

---

# 2. Core Principles

## ENG-AUD-02 — Append-Only

Audit events:

- Are immutable.
- Are never edited.
- Are never deleted implicitly.
- Must not be rewritten during export or import.

Corrections require new audit events.

Fail if:

- Audit records are modified in place.
- Audit history is rewritten.
- Events are silently dropped.

---

## ENG-AUD-03 — Engine Is Audit Producer Only

The Engine:

- Emits audit events as side effects of successful commands.
- Must not consult audit events during:
  - rehydration
  - evaluation
  - acceptance
  - blocking decisions
  - supersession derivation
- Must not reconstruct state from audit.
- Must not repair state using audit.

Audit corruption must not alter legitimacy derivation.

Structural integrity (ENG-INTEGRITY) must not depend on audit availability.

Fail if:

- Engine logic depends on audit queries.
- Restore requires audit records.
- Acceptance behavior differs based on audit presence.

---

# 3. Deterministic Emission Rules

## ENG-AUD-04 — Emission After Commit

Audit emission must occur only after:

- A successful state mutation
- Atomic acceptance commit
- Terminal transition commit

Audit must not:

- Be emitted for failed commands
- Be emitted for evaluation-only operations
- Precede atomic commit

If a command fails validation:

- No audit event must be emitted.

Audit emission must not affect transaction atomicity.

---

# 4. Audit Scope Model

## ENG-AUD-05 — Exactly One Scope Per Event

Each audit event must belong to exactly one scope:

- GLOBAL
- AREA:<area_id>

Scope defines organizational boundary only.

Scope must not:

- Influence legitimacy
- Influence restore
- Influence evaluation
- Influence acceptance

Fail if:

- An auditable action exists without scope.
- Scope determines legitimacy behavior.

---

## ENG-AUD-06 — GLOBAL Scope Is Immutable

The GLOBAL audit scope:

- Must always exist.
- Must not be deleted.
- Must not be superseded.
- Must not be renamed.

Used for:

- Area lifecycle operations
- Import/export operations
- Hash migrations
- Cross-area structural events
- Engine initialization and halt events

Fail if:

- GLOBAL scope is missing.
- GLOBAL scope becomes mutable.

---

# 5. Auditable Actions

## ENG-AUD-07 — Required Audit Emissions

The following successful engine actions must emit audit events:

Lifecycle:
- Session creation
- Session pause
- Session resume
- Session close
- Session terminal transition (ACCEPTED or CLOSED)

Participation:
- Participant added
- Participant removed
- Resume-triggered participant reset

Structure:
- Candidate added (PRE_STANCE only)
- Candidate removed (PRE_STANCE only)
- First vote recorded (freeze trigger)
- Subsequent vote recorded
- Resolution created (via acceptance)
- Resolution superseded (implicit via acceptance)
- Resolution retired

Governance:
- Authority slot filled
- Scope slot filled
- BLOCK_TEMPORARY entered
- BLOCK_PERMANENT entered

Integrity & Maintenance:
- Engine rehydration success
- Rehydration structural failure
- Degraded mode activation
- fsck execution (diagnostic only)
- Hash migration

Import/Export:
- Import start
- Import validation result
- Import completion
- Export operation

Fail if:

- Any successful state mutation above occurs without emitting audit.
- Audit emission changes engine outcome.

---

# 6. Identity Rules

## ENG-AUD-08 — Audit Event Identity

Each audit event must include:

- `event_id` — UUIDv7
- `occurred_at` — UTC ISO 8601 timestamp with timezone (informational only)

Rules:

- All event_id values must be UUIDv7.
- Identity must be globally unique.
- Timestamp must not influence legitimacy.
- Event ordering must not be derived from UUID timestamp for legitimacy logic.

Audit identity exists solely for traceability.

Fail if:

- Non-UUIDv7 identifiers are used.
- Timestamp influences structural derivation.
- Identity format varies across implementations.

---

# 7. Canonical Audit Event Structure

## ENG-AUD-09 — Stable Event Shape

Every audit event must include the following fields:

- `event_id`
- `event_type`
- `occurred_at`
- `actor`
- `scope`
- `subject`
- `context`
- `details`

### subject

Must include:

- `object_type`
- `object_id`

### context

Must include explicit nulls where not applicable:

- `area_id`
- `session_id`
- `authority_resolution_id`
- `scope_resolution_id`

### details

- Non-semantic data only
- Must not contain canonical domain objects
- Must not be required for restore
- Must not be required for legitimacy derivation

Rules:

- Field omission is not permitted.
- Null must be explicit.
- Shape must remain stable across versions.
- Event structure must be forward-compatible.

Fail if:

- Event structure varies implicitly.
- Context or subject fields are omitted.
- Audit payload is used to reconstruct state.

---

# 8. Actor Semantics

## ENG-AUD-10 — Actor Is Informational

- `actor` is an opaque identifier.
- Actor may be null.
- Actor has no authority implication.
- Actor does not imply permissions.

Authorization belongs outside the engine.

Fail if:

- Actor alters legitimacy.
- Actor is used for permission enforcement.
- Actor identity affects deterministic behavior.

---

# 9. Relationship to Legitimacy

## ENG-AUD-11 — Audit Never Creates Legitimacy

- Audit may reference acceptance.
- Audit must not imply acceptance.
- A Resolution must exist in domain storage to be legitimate.
- Audit cannot substitute for session acceptance.
- Audit cannot imply consent or quorum.

Fail if:

- Resolution exists only in audit.
- ACTIVE derivation consults audit.
- Acceptance can be inferred from audit alone.

---

# 10. Ordering & Export

## ENG-AUD-12 — Deterministic Ordering

Audit logs must preserve deterministic ordering.

Ordering rule for export:

- Lexicographic by `event_id`

Ordering must not:

- Depend on storage iteration order.
- Depend on timestamp for legitimacy.

Export must:

- Preserve scope.
- Preserve event identity.
- Not filter implicitly.
- Not mutate event shape.

Export must not alter domain state.

---

# 11. Rehydration & Degraded Mode

## ENG-AUD-13 — Rehydration Independence

Rehydration must:

- Not require audit presence.
- Not require audit ordering.
- Not reconstruct state from audit.

If audit is corrupted:

- Engine legitimacy must remain derivable from domain objects.
- StructuralIntegrityFailure must not depend solely on audit corruption.

Audit is external to legitimacy computation.

---

## ENG-AUD-14 — Degraded Mode Constraints

In degraded mode:

- Audit remains append-only.
- Mutating commands remain prohibited.
- Audit must not enable partial legitimacy reconstruction.

Degraded mode must not:

- Silence audit emission for allowed read-only operations.
- Alter prior audit events.

---

# 12. fsck Interaction

## ENG-AUD-15 — fsck Is Observational

- fsck may emit audit events.
- fsck must not mutate domain objects.
- fsck must not repair automatically.
- fsck must not consult audit to repair state.

Fail if:

- fsck modifies structural objects.
- fsck rewrites audit.
- fsck performs hidden reconciliation.

---

# 13. Storage Guarantees

## ENG-AUD-16 — Audit Outlives Subjects

Audit records must remain accessible even if:

- Area is deleted.
- Session is closed.
- Objects become unreachable.
- Resolutions are superseded.

Deleting domain objects must not delete historical audit.

Audit retention must not affect restore semantics.

Fail if:

- Deleting a subject deletes its audit.
- Restore requires audit presence.
- Audit absence alters legitimacy.

---

# 14. Design Guarantees

Audit guarantees:

- No silent legitimacy transitions.
- No invisible supersession.
- No invisible Authority or Scope slot changes.
- No invisible import or migration.
- No invisible participant epoch reset.

Audit does not guarantee:

- Legitimacy correctness.
- Structural integrity.
- Deterministic evaluation.
- Consensus validity.

Those are governed elsewhere.

---

# Mental Model

Legitimacy lives in sessions.  
Structure lives in domain objects and supersession edges.  
Integrity lives in rehydration and halt rules.  
Determinism lives in evaluation and canonicalization.  
Receipts freeze terminal state.  
Audit records explicit action.  

Audit observes.  
It never acts.  
It never decides.  
It never legitimizes.