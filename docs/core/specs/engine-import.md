# ENG-IMPORT — Engine Import & Area Activation Specification (v2)
Status: FROZEN (v2 – Solo Mode + Receipts)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic Area Restore & Baseline Consolidation  
Purpose: Define import semantics for Charter Core Engine, structural enforcement, receipt validation, and evaluation readiness.

---

# 1. Purpose

This document specifies how the Engine handles import of domain objects for:

1. Area Restore (Full Overwrite)  
2. Baseline Consolidation (Incremental / Review Mode)

Goals:

- Preserve structural determinism  
- Enforce invariant compliance  
- Validate session receipt integrity  
- Maintain audit and legitimacy traceability  
- Keep the engine isolated from storage or CLI logic  

Receipts are first-class structural artifacts and are validated during restore.

---

# 2. Engine vs Host Responsibilities

## ENG-IMPORT-01 — Host / CLI

The host/CLI is responsible for:

- Reading and writing files  
- Selecting import mode (Restore / Consolidate)  
- Validating file integrity (hash, signature, envelope)  
- Providing canonical domain objects to the engine  
- Handling user review in baseline mode  

The host must provide a complete and canonical graph.

---

## ENG-IMPORT-02 — Engine Responsibilities

The Engine is responsible for:

- Structural validation of imported objects  
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

Fail if:

- Structural validation fails  
- Receipt integrity fails  
- Snapshot alignment fails  

---

# 3. Import Modes

## 3.1 Area Restore (Full Overwrite)

Engine receives a complete Area graph including:

- Sessions  
- Candidates  
- Votes  
- Resolutions  
- Scopes  
- Session Receipts  

Receipts are mandatory in Restore mode.

A Restore operation without receipts is invalid.

---

### 3.1.1 Receipt Requirements (Restore Mode)

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

- Receipt missing for closed or accepted session  
- Snapshot mismatch detected  
- Hash mismatch detected  
- Receipt references nonexistent session  

---

### 3.1.2 Structural Enforcement (Restore)

Engine enforces:

- Supersession graph acyclicity  
- Exclusive slot constraints  
- Participant set completeness  
- Candidate and vote snapshot completeness  
- BLOCK_PERMANENT hygiene  

The Engine does not:

- Alter domain objects  
- Recompute or regenerate receipts  
- Implicitly accept candidates  

Imported Area becomes fully active only if validation passes.

Partial restore is rejected.

---

## 3.2 Baseline Consolidation (Incremental / Review Mode)

Engine receives partial graph including:

- Resolutions  
- Sessions (optional)  
- Candidates  
- Optional receipts  

Receipts are optional in Baseline mode.

If receipts are provided, they must pass integrity validation.

Engine behavior:

- Structural validation only  
- Imported resolutions marked UNDER_REVIEW  
- No automatic ACCEPTED state creation  
- Supersession edges validated against local graph  

The Engine does not:

- Generate new receipts  
- Modify existing receipts  
- Create legitimacy automatically  

Host handles review aggregation and CLI-level REVIEW receipt creation.

Fail if:

- Supersession cycles detected  
- References unresolved  
- Receipt integrity fails (if provided)  

---

# 4. Object Graph Requirements

## ENG-IMPORT-03 — Required Object Properties

Every imported object must have:

- Valid UUIDv7 identifier  
- Deterministic schema version  
- All references resolved within provided graph (or allowed local graph in baseline mode)

Fail if:

- References missing  
- Duplicate ACTIVE authority or scope detected  
- Structural invariants violated  

---

## ENG-IMPORT-04 — Receipt Integrity Model

Receipts are immutable structural artifacts.

Engine must validate:

- Canonical serialization consistency  
- content_hash determinism  
- Snapshot alignment with session  
- No mutation since issuance  

Receipt presence never implies legitimacy correctness.

Receipt mismatch implies structural corruption.

Fail loudly.

---

# 5. Governance Hygiene Enforcement

Engine enforces:

- Exclusive Authority / Scope per Area  
- Participant set completeness  
- Candidate snapshot consistency  
- BLOCK_PERMANENT invariants  

Receipts do not override hygiene.

Fail if:

- Multiple ACTIVE Authorities detected  
- Participant snapshot empty  
- Candidate snapshot incomplete  

---

# 6. Deterministic Import Guarantees

Given identical:

- Input graph  
- Mode (Restore or Baseline)

Engine must produce identical:

- Validation outcome  
- Invariant enforcement result  
- Receipt integrity result  

Import must not depend on:

- Storage ordering  
- File order  
- External timestamps  
- Runtime environment  

Fail if:

- Two identical imports produce different validation results  

---

# 7. Failure Semantics

Any structural or receipt violation results in:

- Explicit failure  
- Deterministic EvaluationReport  
- No mutation of existing engine state  
- Full rejection of imported graph  

No partial activation allowed.

No implicit repair permitted.

---

# 8. Engine API Interaction

The following API calls support import:

- validate_imported_area(domain_objects)  
- simulate_restore(domain_objects)  
- rehydrate_engine(domain_objects)  

Receipt validation occurs during:

- validate_imported_area  
- rehydrate_engine  

simulate_restore must not generate new receipts.

---

# 9. Mental Model

Restore:

- Full graph required  
- Receipts mandatory  
- Receipts validated  
- Area activated only if fully consistent  

Baseline:

- Partial graph allowed  
- Receipts optional  
- No automatic legitimacy  
- CLI performs review and aggregation  

Host provides facts.  
Engine validates structure.  
Receipts anchor closure.  
Nothing mutates unless explicitly commanded.

---

# 10. Alignment

- ENG-ENGINE-INITIALIZATION  
- ENG-DOMAIN  
- ENG-SESSION  
- ENG-DECISION  
- ENG-SUPERSESSION  
- ENG-RECEIPT  
- ENG-AUD  
- ENG-API  

Violation constitutes structural engine import failure.