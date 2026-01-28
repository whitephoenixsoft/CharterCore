# Charter Core — Audit Event Specification

Document ID: ENG-AUD
Status: FROZEN (v1)
Audience: Charter Core engine implementers
Scope: Engine-internal only (never legitimizing)

---
## 1. Purpose

## ENG-AUD-01 — Audit Exists to Preserve Accountability

Audit records exist to answer:
- What happened?
- When did it happen?
- Under which authority and context?
- Was it explicit?

Audit records:
- Do not create legitimacy
- Do not alter outcomes
- Do not confer authority

Fail if:
- Audit data influences acceptance
- Removing audit changes engine behavior

---
## 2. Core Principle

## ENG-AUD-02 — Audit Is Append-Only

Audit events:
- Are immutable
- Are never edited
- Are never deleted implicitly

Corrections require new audit events.

Fail if:
- Audit records are modified
- Audit history is rewritten

---
## 3. Audit Scope Model

## ENG-AUD-03 — Every Event Belongs to an Audit Scope

Every audit event MUST belong to exactly one scope.
Scopes define retention, not legitimacy.

Required scopes:
- GLOBAL (non-deletable)
- AREA:<area_id>

Fail if:
- An auditable action exists without a scope
- Area deletion removes the only audit record

## ENG-AUD-04 — Global Audit Scope Is Immutable

The GLOBAL audit scope:
- MUST always exist
- MUST NOT be deleted
- MUST NOT be retired

Used for:
- Area creation
- Area deletion
- Import operations
- Hash upgrades
- Ref rewrites

Fail if:
- Global scope is missing or mutable

---
## 4. What Is Auditable

## ENG-AUD-05 — Explicit Engine Actions Are Auditable

The following actions MUST emit audit events:
- Area creation
- Area deletion
- Session start / pause / resume / close
- Candidate addition (before freeze)
- First stance (candidate freeze trigger)
- Resolution acceptance
- Resolution supersession
- Resolution retirement
- Authority change
- Scope change
- Import begin / close
- Import accept / reject / consolidate
- Hash migration
- Ref creation, update, deletion
- fsck execution (diagnostic only)

Fail if:
- Any of the above occurs silently

---
## 5. Audit Event Structure

### ENG-AUD-06 — Canonical Audit Event Shape

Every audit event MUST include:
```Json
{
  "event_id": "<stable id>",
  "event_type": "<ENUM>",
  "occurred_at": "<timestamp>",
  "actor": "<opaque string or null>",
  "scope": "GLOBAL | AREA:<area_id>",
  "subject": {
    "object_type": "<type>",
    "object_id": "<id or hash>"
  },
  "context": {
    "authority_resolution_id": "<id or null>",
    "scope_resolution_id": "<id or null>",
    "session_id": "<id or null>"
  },
  "details": { ... engine-defined ... }
}
```

Rules:
- details is non-semantic
- Missing fields MUST be explicit nulls
- No inferred values

Fail if:
- Context is omitted
- Event structure varies implicitly

---
## 6. Actor Semantics

## ENG-AUD-07 — Actors Are Informational

actor:
- Is an opaque identifier
- Has no authority implications
- May be null (automation)

Fail if:
- Actor identity alters legitimacy
- Actor implies permissions

---
## 7. Relationship to Legitimacy

## ENG-AUD-08 — Audit Never Creates Legitimacy

Audit events:
- May reference legitimacy actions
- Must never be used to infer acceptance
- Must never substitute for sessions

Fail if:
- A resolution exists only in audit
- Audit implies consent

---
## 8. Import & Upgrade Semantics

## ENG-AUD-09 — Import Is Fully Audited

Import operations MUST emit:
- Import started
- Import mode (RESTORE / CONSOLIDATE)
- Object counts
- Hash mismatches
- Review outcomes
- Import closed

Fail if:
- Import side effects are unaudited

## ENG-AUD-10 — Hash Migration Is Audited

Hash upgrades MUST emit:
- Old hash versio
- New hash version
- Affected object count
- Ref rebinding summary

Fail if:
- Hashes change without audit trail

---
## 9. fsck Interaction

## ENG-AUD-11 — fsck Is Auditable but Non-Mutating

fsck:
- MAY emit audit events
- MUST NOT modify state
- MUST NOT repair automatically

Fail if:
- fsck mutates data
- fsck performs hidden repairs

---
## 10. Storage Guarantees

## ENG-AUD-12 — Audit Outlives Subjects

Audit records MUST remain accessible even if:
- An Area is deleted
- A Session is removed
- Objects become unreachable

Fail if:
- Subject deletion removes its audit history

---
## 11. Query Semantics

## ENG-AUD-13 — Audit Queries Are Read-Only

Audit logs:
- Are queryable
- Are filterable
- Are never authoritative

Fail if:
- Engine decisions depend on audit queries

---
## 12. Export Semantics

## ENG-AUD-14 — Audit Export Is Optional but Deterministic

Exports MAY include audit events.

If included:
- Ordering MUST be preserved
- Scope MUST be preserved
- No filtering by relevance

Fail if:
- Audit is partially exported without warning

---
## 13. Design Guarantees

## ENG-AUD-15 — Audit Is the Engine’s Memory

Audit guarantees:
- No silent authority shifts
- No invisible legitimacy changes
- No untraceable imports or upgrades

---
## Mental Model 

- Legitimacy lives in sessions
- History lives in objects
- Accountability lives in audit
- Audit watches — it never acts

---
## Non-Goal — Audit Is Not a Narrative Log

Audit events are not intended to provide:
- a human-readable story of changes
- a chronological explanation of reasoning
- a replayable command history

Audit exists solely to preserve accountability and explicitness.

---
## Why This Matters

Without this spec:
- Area deletion is dangerous
- Imports are unverifiable
- Hash upgrades are untrustworthy
- Long-term legitimacy collapses

With it:
- Charter becomes defensible institutional memory
- You can explain why something exists — not just that it exists
