# Charter Core — Audit Event Specification

Document ID: ENG-AUD  
Status: FROZEN (v1)  
Audience: Charter Core engine implementers  
Scope: Engine-internal only (never legitimizing)  

---

## 1. Purpose

### ENG-AUD-01 — Audit Preserves Accountability

Audit records exist to answer:

- What happened?  
- When did it happen?  
- Under which Authority and Scope context?  
- Was it explicit?  

Audit records:

- Do **not** create legitimacy  
- Do **not** alter outcomes  
- Do **not** confer authority  

Fail if:

- Audit data influences acceptance or legitimacy  
- Removing audit changes engine behavior  

---

## 2. Core Principle

### ENG-AUD-02 — Append-Only

Audit events:

- Are immutable  
- Are never edited  
- Are never deleted implicitly  

Corrections require **new audit events**.

Fail if:

- Audit records are modified  
- Audit history is rewritten  

---

## 3. Audit Scope Model

### ENG-AUD-03 — Every Event Belongs to a Scope

- Each audit event must belong to exactly **one scope**  
- Scopes define **retention**, not legitimacy  

Required scopes:

- `GLOBAL` (non-deletable)  
- `AREA:<area_id>`  

Fail if:

- An auditable action exists without a scope  
- Area deletion removes the only audit record  

### ENG-AUD-04 — Global Audit Scope Is Immutable

- `GLOBAL` audit scope **must always exist**  
- Must **not** be deleted or retired  

Used for:

- Area creation/deletion  
- Import operations  
- Hash upgrades  
- Reference rewrites  

Fail if:

- Global scope is missing or mutable  

---

## 4. Auditable Actions

### ENG-AUD-05 — Explicit Engine Actions Must Emit Audit Events

The following actions **must** generate audit records:

- Area creation / deletion  
- Session start / pause / resume / close  
- Candidate addition (pre-freeze)  
- First stance (candidate freeze trigger)  
- Resolution acceptance / supersession / retirement  
- Authority resolution change  
- Scope resolution change  
- Import start / close / accept / reject / consolidate  
- Hash migration  
- Reference creation / update / deletion  
- fsck execution (diagnostic only)  

Fail if:

- Any above occurs silently  

---

## 5. Audit Event Structure

### ENG-AUD-06 — Canonical Audit Event Shape

Every audit event must include:

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
  "details": { ... engine-defined, non-semantic ... }
}

Rules:

- `details` is non-semantic  
- Missing fields **must** be explicit nulls  
- No inferred values  
- Event shape is stable across versions  

Fail if:

- Context or subject fields are omitted  
- Event structure varies implicitly  

---

## 6. Actor Semantics

### ENG-AUD-07 — Actors Are Informational

- `actor` is an opaque identifier  
- Has **no authority implications**  
- May be `null` for automated actions  

Fail if:

- Actor identity alters legitimacy  
- Actor implies permissions  

---

## 7. Relationship to Legitimacy

### ENG-AUD-08 — Audit Never Creates Legitimacy

- Audit events may reference legitimacy actions  
- Must **never** be used to infer acceptance  
- Must **never** substitute for sessions  

Fail if:

- Resolution exists only in audit  
- Audit implies consent  

---

## 8. Import & Upgrade Semantics

### ENG-AUD-09 — Import Is Fully Audited

Import operations **must** emit:

- Import started  
- Import mode (`RESTORE` / `CONSOLIDATE`)  
- Object counts  
- Hash mismatches  
- Review outcomes  
- Import closed  

Fail if:

- Any import side effect is unaudited  

### ENG-AUD-10 — Hash Migration Is Audited

- Hash upgrades **must** emit:

  - Old hash version  
  - New hash version  
  - Affected object count  
  - Reference rebinding summary  

Fail if:

- Hashes change without audit trail  

---

## 9. fsck Interaction

### ENG-AUD-11 — fsck Is Auditable but Non-Mutating

- fsck **may** emit audit events  
- Must **not** modify state  
- Must **not** repair automatically  

Fail if:

- fsck mutates data or performs hidden repairs  

---

## 10. Storage Guarantees

### ENG-AUD-12 — Audit Outlives Subjects

- Audit records must remain accessible even if:

  - Area is deleted  
  - Session is removed  
  - Objects become unreachable  

Fail if:

- Subject deletion removes audit history  

---

## 11. Query Semantics

### ENG-AUD-13 — Audit Queries Are Read-Only

- Audit logs must be queryable and filterable  
- Queries **must not** affect legitimacy or engine state  

Fail if:

- Engine decisions depend on audit queries  

---

## 12. Export Semantics

### ENG-AUD-14 — Audit Export Is Optional but Deterministic

- Audit exports **may** include events  
- If included:

  - Ordering must be preserved  
  - Scope must be preserved  
  - No filtering by relevance  

Fail if:

- Audit is partially exported without warning  

---

## 13. Design Guarantees

### ENG-AUD-15 — Audit Is Engine Memory

Audit guarantees:

- No silent authority shifts  
- No invisible legitimacy changes  
- No untraceable imports or upgrades  

---

## Mental Model

- Legitimacy lives in sessions  
- History lives in objects  
- Accountability lives in audit  
- Audit observes — it never acts  

---

## Non-Goal

Audit is **not** a narrative log:

- Does not provide human-readable story  
- Does not explain reasoning  
- Does not replay commands  

Audit exists solely to preserve accountability and explicitness.  

---

## Why This Matters

Without audit:

- Area deletion is dangerous  
- Imports are unverifiable  
- Hash upgrades are untrustworthy  
- Long-term legitimacy collapses  

With audit:

- Charter preserves defensible institutional memory  
- Actions can be explained precisely, without altering outcomes
