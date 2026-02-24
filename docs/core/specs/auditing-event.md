# ENG-AUD  
Audit Event Specification  
Status: FROZEN (v2)  
Applies to: Engine Core (V1/V2+)  

This document must be interpreted in conjunction with:

- ENG-DOMAIN  
- ENG-DECISION  
- ENG-SUPERSESSION  
- ENG-REVIEW-RETIRED  
- ENG-INTEGRITY  

If conflict exists, structural and legitimacy rules defined elsewhere take precedence.

Audit is observational only.

---

# 1. Purpose

## ENG-AUD-01 — Audit Preserves Accountability

Audit records exist to answer:

- What action occurred?
- When did it occur?
- Under which Authority and Scope context?
- Was the action explicit?

Audit records:

- Do not create legitimacy.
- Do not alter structural state.
- Do not confer authority.
- Do not participate in ACTIVE derivation.
- Do not participate in restore.
- Do not participate in acceptance validation.

Fail if:

- Audit data influences acceptance.
- Audit data influences restore behavior.
- Removing audit changes engine legitimacy outcomes.

Audit is memory, not law.

---

# 2. Core Principles

## ENG-AUD-02 — Append-Only

Audit events:

- Are immutable.
- Are never edited.
- Are never deleted implicitly.

Corrections require new audit events.

Fail if:

- Audit records are modified.
- Audit history is rewritten.

---

## ENG-AUD-03 — Engine Is Audit Producer Only

The engine:

- Emits audit events as side effects of actions.
- Must not consult audit events during restore.
- Must not consult audit events during evaluation.
- Must not consult audit events during acceptance.

Audit corruption must not alter legitimacy derivation.

Structural integrity (ENG-INTEGRITY) must not depend on audit availability.

Fail if:

- Engine logic depends on audit queries.
- Restore requires audit records to derive state.

---

# 3. Audit Scope Model

## ENG-AUD-04 — Every Event Belongs to Exactly One Audit Scope

Each audit event must belong to exactly one of:

- GLOBAL
- AREA:<area_id>

Audit scope defines retention and organizational boundary only.

Audit scope does not affect legitimacy.

Fail if:

- An auditable action exists without a scope.
- Scope determines legitimacy evaluation.

---

## ENG-AUD-05 — GLOBAL Scope Is Immutable

The GLOBAL audit scope:

- Must always exist.
- Must not be deleted.
- Must not be retired.
- Must not be superseded.

Used for:

- Area creation or deletion
- Import operations
- Hash migrations
- Reference rewrites
- Cross-area structural events

Fail if:

- GLOBAL scope is missing.
- GLOBAL scope is mutable.

---

# 4. Auditable Actions

## ENG-AUD-06 — Explicit Engine Actions Must Emit Audit Events

The following engine actions must generate audit events:

- Area creation and deletion
- Session start, pause, resume, close
- Candidate addition (before freeze)
- First stance (freeze trigger)
- Resolution acceptance
- Resolution supersession (implicit via acceptance)
- Resolution retirement
- Acceptance of Resolution occupying Authority slot
- Acceptance of Resolution occupying Scope slot
- Import start, close, accept, reject, consolidate
- Hash migration
- Reference creation, update, deletion
- fsck execution (diagnostic only)

Fail if:

- Any above occurs without emitting audit.

Audit emission must not alter acceptance atomicity.

---

# 5. Identity Rules

## ENG-AUD-07 — Audit Event Identity

Each audit event must include:

- event_id: UUID version 7
- occurred_at: timestamp (informational only)

All engine-generated identifiers must be UUIDv7.

Audit identity must be globally unique.

Event ordering must not be derived from UUID timestamp for legitimacy purposes.

Fail if:

- Non-UUIDv7 identifiers are used.
- Identity format varies across implementations.

---

# 6. Canonical Audit Event Structure

## ENG-AUD-08 — Stable Event Shape

Every audit event must include:

{
  "event_id": "<uuidv7>",
  "event_type": "<ENUM>",
  "occurred_at": "<timestamp>",
  "actor": "<opaque string or null>",
  "scope": "GLOBAL | AREA:<area_id>",
  "subject": {
    "object_type": "<type>",
    "object_id": "<uuidv7 or other immutable id>"
  },
  "context": {
    "authority_resolution_id": "<uuidv7 or null>",
    "scope_resolution_id": "<uuidv7 or null>",
    "session_id": "<uuidv7 or null>"
  },
  "details": { ... non-semantic data ... }
}

Rules:

- details is non-semantic.
- Missing fields must be explicit null.
- Event shape must remain stable across versions.
- Field omission is not permitted.
- Audit payload must not be used to reconstruct state.

Fail if:

- Event structure varies implicitly.
- Context or subject fields are omitted.
- Audit payload is used to reconstruct state.

---

# 7. Actor Semantics

## ENG-AUD-09 — Actors Are Informational Only

- actor is an opaque identifier.
- Actor identity has no authority implication.
- Actor may be null.
- Actor does not imply permissions.

Fail if:

- Actor alters legitimacy.
- Actor is used for permission enforcement.

Authorization belongs outside engine legitimacy logic.

---

# 8. Relationship to Legitimacy

## ENG-AUD-10 — Audit Never Creates Legitimacy

- Audit may reference acceptance.
- Audit must never imply acceptance.
- A Resolution must exist in domain storage to be legitimate.
- Audit cannot substitute for session acceptance.

Fail if:

- Resolution exists only in audit.
- Audit implies consent.
- Audit influences ACTIVE derivation.

---

# 9. Import & Upgrade Semantics

## ENG-AUD-11 — Import Must Be Fully Audited

Import operations must emit:

- Import started
- Import mode (RESTORE or CONSOLIDATE)
- Object counts
- Structural validation outcomes
- Hash mismatches
- Review outcomes
- Import closed

Import audit must not substitute for structural validation.

Fail if:

- Import side effects are unaudited.
- Audit replaces structural validation.

---

## ENG-AUD-12 — Hash Migration Must Be Audited

Hash migration must emit:

- Old hash version
- New hash version
- Affected object count
- Reference rebinding summary

Object identity must not change during hash migration.

Fail if:

- Hash changes occur silently.
- Identity is altered during migration.

---

# 10. fsck Interaction

## ENG-AUD-13 — fsck Is Observational

- fsck may emit audit events.
- fsck must not mutate state.
- fsck must not perform automatic repair.
- fsck must not consult audit to repair state.

Fail if:

- fsck modifies domain objects.
- fsck performs hidden reconciliation.

---

# 11. Storage Guarantees

## ENG-AUD-14 — Audit Outlives Subjects

Audit records must remain accessible even if:

- Area is deleted
- Session is closed
- Objects become unreachable

Audit retention must not affect structural restore.

Audit corruption must not trigger StructuralIntegrityFailure unless domain data is also corrupted.

Fail if:

- Deleting a subject deletes its audit.
- Restore requires audit presence.

---

# 12. Query & Export Semantics

## ENG-AUD-15 — Queries Are Read-Only

Audit logs must be queryable and filterable.

Queries:

- Must not alter engine state.
- Must not influence legitimacy decisions.

Fail if:

- Engine decisions depend on audit queries.

---

## ENG-AUD-16 — Export Is Deterministic

Audit export may include events.

If exported:

- Event ordering must be preserved.
- Scope must be preserved.
- No silent filtering is permitted.

Export must not alter domain state.

---

# 13. Design Guarantees

Audit guarantees:

- No silent authority shifts.
- No invisible legitimacy changes.
- No untraceable imports.
- No untraceable identity migrations.
- No invisible structural rewrite.

Audit does not guarantee:

- Legitimacy correctness.
- Structural integrity.
- Consensus.

Those are governed elsewhere.

---

# Mental Model

- Legitimacy lives in sessions.
- Structure lives in domain objects and supersession edges.
- Integrity lives in restore and halt rules.
- Accountability lives in audit.
- Audit observes — it never acts.

---

# Non-Goal

Audit is not:

- A narrative log.
- A reasoning log.
- A command replay system.
- A consensus layer.

Audit is institutional memory.

It records explicit action without influencing outcome.