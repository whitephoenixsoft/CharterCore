# ENG-IMPORT — Engine Import & Area Activation Specification (v3)  
Status: FROZEN (v3 – Governance Slot Validation Integrated)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic Area Restore & Baseline Consolidation  
Purpose: Define import semantics, governance slot enforcement, receipt validation, and evaluation readiness.

---

# 1. Purpose

This document specifies how the Engine handles import of domain objects for:

1. Area Restore (Full Overwrite)  
2. Baseline Consolidation (Incremental / Review Mode)

Goals:

- Preserve structural determinism  
- Enforce invariant compliance  
- Validate session receipt integrity  
- Enforce governance slot constraints  
- Maintain audit and legitimacy traceability  
- Keep the engine isolated from storage or CLI logic  

Receipts are first-class structural artifacts and are validated during restore.

Governance slots are structural invariants and must be validated during restore and rehydration.

---

# 2. Engine vs Host Responsibilities

## ENG-IMPORT-01 — Host / CLI Responsibilities

The host/CLI is responsible for:

- Reading and writing files  
- Selecting import mode (Restore / Consolidate)  
- Validating file integrity (hash, signature, envelope)  
- Providing canonical domain objects to the engine  
- Handling user review in baseline mode  

The host must provide a complete and canonical graph.

The engine never reconstructs missing governance.

---

## ENG-IMPORT-02 — Engine Responsibilities

The Engine is responsible for:

- Structural validation of imported objects  
- Governance slot exclusivity validation  
- Governance slot presence validation (Restore mode)  
- Supersession graph acyclicity  
- Participant and candidate snapshot integrity  
- BLOCK_PERMANENT hygiene enforcement  
- Session receipt validation  
- Deterministic readiness evaluation  

The Engine must not:

- Discover missing objects outside provided graph  
- Modify object content  
- Generate new receipts during validation  
- Perform storage operations  
- Reconstruct missing Authority or Scope  

Fail if:

- Structural validation fails  
- Receipt integrity fails  
- Governance slot invariants fail  
- Snapshot alignment fails  

---

# 3. Governance Slot Model

Per Area, the following slots exist:

- Authority slot  
- Scope slot  

Slots are exclusive.

Derived Area Governance State (not stored):

- UNINITIALIZED — no ACTIVE Authority  
- AUTHORITY_DEFINED — exactly one ACTIVE Authority, no ACTIVE Scope  
- FULLY_GOVERNED — exactly one ACTIVE Authority and one ACTIVE Scope  

Restore validation depends on mode.

---

# 4. Import Modes

## 4.1 Area Restore (Full Overwrite)

Engine receives a complete Area graph including:

- Sessions  
- Candidates  
- Votes  
- Resolutions  
- Scopes  
- Session Receipts  

Receipts are mandatory in Restore mode.

A Restore operation without required receipts is invalid.

---

### 4.1.1 Governance Slot Requirements (Restore Mode)

For a successful full restore:

The Area must contain:

- Exactly one ACTIVE Authority Resolution  
- Exactly one ACTIVE Scope  

Fail if:

- No ACTIVE Authority present  
- More than one ACTIVE Authority present  
- No ACTIVE Scope present  
- More than one ACTIVE Scope present  
- Orphaned Authority or Scope references exist  
- Governance slots structurally inconsistent  

The Engine must not:

- Infer missing Scope  
- Infer missing Authority  
- Promote UNDER_REVIEW objects  
- Auto-repair slot violations  

Restore is all-or-nothing.

---

### 4.1.2 Receipt Requirements (Restore Mode)

For every session in state:

- ACCEPTED  
- CLOSED  

A corresponding receipt must exist.

The Engine must validate:

1. receipt_id is valid UUIDv7  
2. receipt_type matches session closure type  
3. participant_snapshot matches frozen session participants  
4. candidate_snapshot matches frozen candidates  
5. stance_snapshot matches recorded votes (including implicit solo vote)  
6. participant_reconfirmation history aligns with session resume events  
7. content_hash matches deterministic serialization  
8. acceptance_result aligns with session state  

Fail if:

- Receipt missing  
- Snapshot mismatch detected  
- Hash mismatch detected  
- Receipt references nonexistent session  

---

### 4.1.3 Structural Enforcement (Restore)

Engine enforces:

- Supersession graph acyclicity  
- Exclusive governance slots  
- Participant set completeness  
- Candidate and vote snapshot completeness  
- BLOCK_PERMANENT hygiene  

The Engine does not:

- Alter domain objects  
- Recompute or regenerate receipts  
- Implicitly accept candidates  

Imported Area becomes active only if validation passes.

Partial restore is rejected.

---

## 4.2 Baseline Consolidation (Incremental / Review Mode)

Engine receives partial graph including:

- Resolutions  
- Sessions (optional)  
- Candidates  
- Optional receipts  

Receipts are optional in Baseline mode.

If receipts are provided, they must pass integrity validation.

Governance presence may be deferred in Baseline mode.

However:

- Any attempt to rehydrate into an active evaluation context must enforce governance slot requirements.
- simulate_restore must fail if governance slots are missing or exclusive constraints violated.

The Engine must not:

- Generate new receipts  
- Modify existing receipts  
- Create legitimacy automatically  
- Reconstruct governance implicitly  

Imported resolutions may be marked UNDER_REVIEW per ENG-REVIEW.

Fail if:

- Supersession cycles detected  
- References unresolved  
- Receipt integrity fails (if provided)  
- Governance slot exclusivity violated  

---

# 5. Object Graph Requirements

## ENG-IMPORT-03 — Required Object Properties

Every imported object must have:

- Valid UUIDv7 identifier  
- Deterministic schema version  
- All references resolved within provided graph (or allowed local graph in baseline mode)

Fail if:

- References missing  
- Duplicate ACTIVE Authority detected  
- Duplicate ACTIVE Scope detected  
- Structural invariants violated  

---

# 6. Governance Hygiene Enforcement

Engine enforces during restore and rehydration:

- Exactly one ACTIVE Authority (full restore)  
- Exactly one ACTIVE Scope (full restore)  
- No slot multiplicity  
- No orphaned supersession references  
- BLOCK_PERMANENT acceptance blocking  

Receipts do not override governance slot invariants.

Fail loudly on violation.

---

# 7. Deterministic Import Guarantees

Given identical:

- Input graph  
- Import mode  

Engine must produce identical:

- Validation outcome  
- Governance slot evaluation  
- Receipt integrity result  
- Readiness state  

Import must not depend on:

- Storage ordering  
- File order  
- External timestamps  
- Runtime environment  

Two identical imports must yield identical EvaluationReports.

---

# 8. Failure Semantics

Any structural, receipt, or governance violation results in:

- Explicit failure  
- Deterministic EvaluationReport  
- No mutation of existing engine state  
- Full rejection of imported graph  

No partial activation allowed.

No implicit repair permitted.

---

# 9. Engine API Interaction

Import validation uses:

- validate_imported_area(domain_objects)  
- simulate_restore(domain_objects)  
- rehydrate_engine(domain_objects)  

Governance slot validation occurs during:

- validate_imported_area  
- simulate_restore  
- rehydrate_engine  

Restore activation requires:

- FULLY_GOVERNED state  

Baseline mode may defer governance completeness until activation.

---

# 10. Mental Model

Restore:

- Full graph required  
- Governance slots required  
- Receipts mandatory  
- Structural validation strict  
- Area activates only if FULLY_GOVERNED  

Baseline:

- Partial graph allowed  
- Governance may be incomplete  
- No automatic legitimacy  
- Activation requires governance completeness  

Host provides facts.  
Engine validates structure.  
Governance must be structurally sound.  
Receipts anchor closure.  
Nothing is reconstructed implicitly.

---

# 11. Alignment

- ENG-DOMAIN  
- ENG-SESSION  
- ENG-DECISION  
- ENG-SUPERSESSION  
- ENG-RECEIPT  
- ENG-API  
- ENG-INTEGRITY  

Violation constitutes structural engine import failure.