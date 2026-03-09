# ENG-DOMAIN — Domain Object Schema
Canonical Engine Object Definitions

Status: DRAFT (v15.1 – Spec Verify & Governance State Alignment)
Applies to: Engine Core (V1/V2+)

---

# 1. Purpose

Freezes canonical domain object schemas.

Behavioral rules remain in:

- ENG-SESSION, ENG-DECISION, ENG-SUPERSESSION, ENG-API, ENG-INTEGRITY, ENG-ERROR, ENG-RECEIPT, ENG-AUD, ENG-CANON, ENG-SPECVERIFY

This version integrates:

- UNDER_REVIEW / RETIRED semantics
- Incremental compilation deterministic behavior
- Spec verify binding (spec_set_hash)
- Consistency with audit, receipt, and governance rules

---

# 2. General Principles

## 2.1 Immutability

All persisted domain objects immutable once written.

- Objects replaced only via Engine-controlled lifecycle operations (acceptance, supersession, retirement, session re-evaluation)
- RETIRED and UNDER_REVIEW transitions **do not mutate historical objects**, only affect forward usability

---

## 2.2 Canonical Serialization

- ENG-CANON required  
- Deterministic field and enum ordering  
- Canonical UTF-8 JSON encoding  
- Explicit nullability  
- Applies to receipts, resolutions, sessions, participants, candidates, constraints, votes  

Incremental compilation **relies on canonical serialization** to compare historical vs current state deterministically.

---

## 2.3 Informational vs Structural Data

- Structural fields affect legitimacy, governance semantics, receipt identity, rule provenance, canonical hash  
- Informational cross-area references are opaque and **never influence legitimacy or ACTIVE derivation**  
- spec_set_hash is structural, binds the object to the exact rule system

---

## 2.4 Lifecycle State

- Represented as a single enum field  
- Resolutions support: ACTIVE, SUPERSEDED, UNDER_REVIEW, RETIRED  
- Scope supports: ACTIVE, SUPERSEDED, UNDER_REVIEW  
- Authority supports: ACTIVE, SUPERSEDED  
- Incremental compilation **evaluates state deterministically using the resolution index and accepted_at timestamps if needed, ignoring audit timestamps**

---

## 2.5 Schema Versioning

- All domain objects include schema_version  
- Major version: breaking structural changes  
- Minor version: additive, non-legitimacy affecting changes  
- Unknown structural enums/fields → restore failure

---

## 2.6 Identifier Generation

- All Engine-generated IDs are UUIDv7  
- Caller-provided IDs prohibited  
- Timestamp components **never influence legitimacy or restore precedence**  
- participant_id session-scoped, candidate_id round-scoped, vote_id bound to single round

---

## 2.7 Area Identity

- area_id opaque, externally provided  
- Engine operates on exactly one Area at runtime  
- Cross-area references are informational

---

# 3. Schema Versioning & Compatibility

- Unknown enum/structural field → UNSUPPORTED_SCHEMA_VERSION  
- Minor versions additive only  
- Major version increments require spec verification  
- Incremental compilation uses **spec_set_hash** to determine rule context equivalence

---

# 4. Single-Area Structural Model

- All objects must share area_id  
- Supersession strictly Area-local  
- Governance slots Area-local  
- Cross-area references informational only

---

# 5. Governance Slot Model

- Authority and Scope slots exclusive  
- At most one ACTIVE Authority  
- At most one ACTIVE Scope  
- Slot exclusivity enforced at restore  
- Violations → StructuralIntegrityFailure

---

# 6. Structural vs Informational References

## 6.1 Structural References

- Affect legitimacy and must resolve at restore  
- Examples:  
  - Session → Authority/Scope Resolution  
  - Resolution → originating Session  
  - Resolution → superseded Resolution  
  - Receipt → Session / Resolution (LEGITIMACY)  
- Structural references **never cross Areas**

## 6.2 Informational Cross-Area References

- May reference external Areas or Resolutions  
- Must **never affect legitimacy or ACTIVE derivation**  
- Must **never trigger restore failure**  

---

# 7. CrossAreaReference Schema

- Fields: external_area_id, external_area_label, external_resolution_id (nullable), external_resolution_label (nullable), created_at, schema_version  
- Engine **never dereferences them**  

---

# 8. Participant Schema

- participant_id unique within session, never reused  
- Belongs to exactly one round  
- Removal terminates epoch  
- Resume generates new epoch  
- display_name unique within round  
- Forward compatibility guaranteed  
- Used by incremental compilation for deterministic round comparison  

---

# 9. Candidate Schema

- candidate_id unique within round, never reused  
- Belongs to exactly one round  
- candidate_content immutable once VOTING begins  
- Candidate identity **round-scoped proposal epoch**  
- Supports incremental compilation by canonical round comparison

---

# 10. Constraint Schema

- constraint_id unique  
- Belongs to exactly one round  
- Immutable after VOTING begins  
- Evaluated in ENG-DECISION

---

# 11. Vote Schema

- vote_id unique, session and round bound  
- One vote per participant per candidate  
- Immutable once recorded  
- Votes never cross rounds  
- Abstain is explicit and first-class

---

# 12. Session Schema

- session_id, area_id, session_type (AUTHORITY | SCOPE | REGULAR)  
- authority_id, scope_id (nullable during bootstrap)  
- phase (PRE_STANCE | VOTING | TERMINAL)  
- state (ACTIVE | BLOCK_TEMPORARY | BLOCK_PERMANENT | ACCEPTED | CLOSED)  
- round_index, participants, candidates, constraints, votes  
- terminal_receipt_id required for ACCEPTED/CLOSED  
- cross_area_references optional, annotations optional  
- created_at, updated_at, schema_version  

Rules:

- Round index increments deterministically on resume  
- Historical rounds only in receipts  
- Session evaluation **blocks if referenced Resolutions are UNDER_REVIEW or RETIRED**  
- Incremental compilation evaluates only **canonical session and resolution index**, ignoring audit timestamps  

---

# 13. Resolution Schema

- resolution_id, area_id, originating_session_id, authority_snapshot_id, scope_snapshot_id, accepted_candidate_id, engine_version, spec_set_hash  
- state (ACTIVE | SUPERSEDED | UNDER_REVIEW | RETIRED)  
- superseded_by nullable, cross_area_references optional, annotations optional, created_at, schema_version  

Rules:

- Created only via session acceptance  
- Supersession Area-local, immutable  
- RETIRED and UNDER_REVIEW affect forward usability, not historical legitimacy  
- spec_set_hash **binds resolution to exact rule context for spec verification**  

---

# 14. Receipt Schema

- receipt_id, session_id, resolution_id (nullable), receipt_type (LEGITIMACY | EXPLORATION), area_id, engine_version, spec_set_hash  
- authority_snapshot_id, scope_snapshot_id, problem_statement optional  
- rounds, final_round_index, session_state_at_close, acceptance_result, annotations optional  
- created_at, hash_algorithm, content_hash, schema_version  

Rules:

- LEGITIMACY receipts must reference the accepted Resolution  
- EXPLORATION receipts resolution_id null  
- Round sequence contiguous, deterministic  
- content_hash computed over canonical serialization  
- spec_set_hash binds legitimacy artifact to rule system  

---

# 15. Round Snapshot Schema

- round_index, round_state (COMPLETED | FINAL_ACCEPTED | ABANDONED)  
- participant_set, candidate_set, constraint_set, vote_set  
- Sets ordered lexicographically by UUID  
- Used by incremental compilation to compare prior vs current rounds

---

# 16. Supersession Encoding

- superseded_by immutable, Area-local, applied only via session acceptance  
- Incremental compilation **consults resolution index and supersession graph** to determine canonical ACTIVE set

---

# 17. Deterministic Encoding Requirements

- ENG-CANON required  
- Deterministic field ordering and enum values  
- Lexicographic ordering of sets  
- Canonical UTF-8 JSON encoding  
- Hash includes only recognized structural fields

---

# 18. Engine Invariants

- Single active Area at runtime  
- Domain graph shares same area_id  
- Governance slots exclusive  
- Structural references resolve locally  
- Cross-area references informational only  
- UUIDv7 for all IDs, never reused per rules  
- Votes and constraints bound to a single round  
- terminal_receipt_id required for terminal sessions  
- LEGITIMACY receipts bind to Resolution  
- spec_set_hash must match Engine manifest (Spec Verify)  
- Round indices contiguous  
- Deterministic encoding mandatory  
- Forward and backward compatibility enforced  
- Incremental compilation deterministic  
- Legitimacy independent of timestamps, ordering, or audit

Violation constitutes critical Engine failure.