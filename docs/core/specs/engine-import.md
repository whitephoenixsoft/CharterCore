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

Import operations must not create legitimacy.

Legitimacy artifacts (Resolution + LEGITIMACY receipt) must already exist and must be historically complete when imported.

Import mechanisms may only insert previously finalized legitimacy artifacts into the domain graph.

Any attempt to import incomplete legitimacy artifacts must fail.

---

# 2. Engine vs Host Responsibilities

## ENG-IMPORT-01 — Host / CLI Responsibilities

The host/CLI is responsible for:

- Reading and writing files  
- Selecting import mode (Restore / Consolidate)  
- Validating file integrity (hash, signature, envelope)  
- Providing canonical domain objects to the engine  
- Handling user review in baseline mode  
- Rule provenance verification (engine_version, spec_set_hash)
- Canonical receipt structure validation
- Deterministic round snapshot validation
- Lifecycle state validation for imported Resolutions
- Enforcement that imported legitimacy artifacts are historically complete

The Engine must not:

- evaluate acceptance rules during import
- simulate session acceptance
- generate new Resolutions
- generate new receipts

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
- Resolutions  
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
2. receipt_type matches session_state
3. round snapshots are contiguous
4. participant_set matches receipt round structure
5. candidate_set matches receipt round structure
6. vote_set matches receipt round structure
7. canonical serialization is valid
8. content_hash recomputation matches stored value
9. spec_set_hash exists and is valid
10. engine_version exists
11. rule provenance fields match the originating Resolution

Fail if any verification fails.

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

## 4.2 Incremental Compilation

The Engine receives a partial domain graph containing historical legitimacy artifacts.

The graph may include:

- Resolutions
- Sessions
- Receipts

Receipts are required to prove legitimacy for all terminal sessions.

The imported graph must pass structural integrity validation as defined in ENG-INTEGRITY.

Governance completeness may be temporarily absent during incremental compilation.

However, any attempt to rehydrate the graph into an active evaluation context must enforce governance slot requirements.

The Engine must not:

- Modify existing receipts
- Create legitimacy automatically
- Reconstruct governance implicitly

Imported resolutions may be marked UNDER_REVIEW per ENG-REVIEW.

Validation must fail if:

- Supersession cycles are detected
- Structural references are unresolved
- Receipt integrity validation fails
- Governance slot exclusivity is violated

---

# 5. Object Graph Requirements

## ENG-IMPORT-03 — Required Object Properties

Every imported object must have:

- Valid UUIDv7 identifier  
- Deterministic schema version  
- All references resolved within provided graph (or allowed local graph in baseline mode)

Imported Resolutions must satisfy lifecycle constraints defined in ENG-REVIEW-RETIRED:

Allowed lifecycle states during import:

- ACTIVE
- UNDER_REVIEW
- RETIRED
- SUPERSEDED

State transitions must not be inferred by the Engine during import.

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

Lifecycle state usability must not be altered during import.

Import may insert Resolutions in UNDER_REVIEW or RETIRED state, but must not transition states during validation.

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

Specification verification must occur during import validation.

If an imported artifact references a spec_set_hash different from the executing Engine:

- MATCH → full verification allowed
- LEGACY_MATCH → verification allowed in historical compatibility mode
- SPEC_SET_UNKNOWN → artifact may be stored but legitimacy must not be reinterpreted

This rule aligns with ENG-SPECVERIFY.

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

- rehydrate_engine(domain_objects)  

Governance slot validation occurs during:

- rehydrate_engine  

Restore activation requires:

- FULLY_GOVERNED state  
- session receipts

Incremental compilation operations correspond to the following Engine API calls:

- begin_incremental_compilation
- stage_historical_session
- stage_historical_resolution
- stage_historical_receipt
- finalize_incremental_compilation

Rehydration remains the only mechanism that activates an Area for runtime evaluation.

Import operations must not bypass rehydration validation.
 
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

Import does not create legitimacy.

Import only introduces historical legitimacy artifacts into the domain graph.

Incremental compilation allows historical DAG construction before runtime rehydration.

Rehydration remains the only mechanism that activates legitimacy evaluation.

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
