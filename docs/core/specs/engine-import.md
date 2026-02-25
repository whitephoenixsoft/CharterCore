# ENG-IMPORT — Engine Import & Area Activation Specification (v1)
Status: FROZEN (v1 – Solo Mode)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic Area Restore & Baseline Consolidation  
Purpose: Define import semantics for Charter Core Engine, structural enforcement, and evaluation readiness.

---

# 1. Purpose

This document specifies how the Engine handles import of domain objects for:

1. **Area Restore (Full Overwrite)**  
   Replace an entire Area with a canonical export snapshot.

2. **Baseline Consolidation (Incremental / Review Mode)**  
   Merge external history into a baseline workspace for later evaluation.

Goals:

- Preserve structural determinism  
- Enforce invariant compliance  
- Maintain audit traceability  
- Keep the engine isolated from storage or CLI logic  

---

# 2. Engine vs Host Responsibilities

## ENG-IMPORT-01 — Host / CLI

The host/CLI is responsible for:

- Reading and writing files  
- Selecting import mode (Restore / Consolidate)  
- Validating file integrity (hash, signature)  
- Providing canonical domain objects to the engine  
- Handling conflicts and user review (baseline mode)

## ENG-IMPORT-02 — Engine

The Engine is responsible for:

- Structural validation of imported objects  
- Enforcing governance invariants  
- Participant and snapshot integrity  
- Supersession graph acyclicity  
- BLOCK_PERMANENT hygiene

The Engine **must not**:

- Discover missing objects outside the provided graph  
- Modify object content  
- Automatically accept resolutions (except full restore if valid)  
- Perform storage operations  

Fail if:

- Structural validation fails  
- Imported objects violate invariants  

---

# 3. Import Modes

## 3.1 Area Restore (Full Overwrite)

- Engine receives a **complete Area graph**:

  - All sessions, candidates, votes  
  - Authority resolutions  
  - Scope objects  
  - Accepted Resolutions

- Engine enforces:

  - Supersession graph integrity  
  - Exclusive slot constraints  
  - Participant snapshot consistency  
  - Candidate and vote snapshot completeness  
  - BLOCK_PERMANENT hygiene

- Engine **does not**:

  - Alter domain objects  
  - Implicitly accept candidates  
  - Modify votes or snapshots

- Imported Area becomes **fully activated** only if validation passes.  
- Partial restore is rejected.

Fail if:

- Any object references missing local objects  
- Supersession edges violate acyclic rules  
- Duplicate ACTIVE Authority or Scope in Area  
- Participant sets incomplete or empty  
- Candidate snapshot incomplete

---

## 3.2 Baseline Consolidation (Incremental / Review Mode)

- Engine receives **partial graph**:

  - Resolutions, sessions, candidates  
  - Optional annotations

- Engine behavior:

  - Each resolution marked **UNDER_REVIEW**  
  - No automatic creation of ACCEPTED resolutions  
  - Structural validation only  
  - Supersession edges checked within batch and against local objects  

- Host/CLI handles:

  - Batch evaluation  
  - User review and conflict resolution  
  - Manual acceptance via session mechanics

Fail if:

- Supersession edges form cycles  
- References point to missing objects in imported batch  

---

# 4. Object Graph Requirements

## ENG-IMPORT-03 — Required Objects

- Every imported object must have:

  - Valid UUIDv7 identifier  
  - Deterministic schema version  
  - All references resolved within provided graph or allowed local objects

Fail if:

- References point to missing or invalid objects  
- Imported object duplicates existing ACTIVE Authority/Scope (full restore)  

## ENG-IMPORT-04 — Shadow State Rules

- Imported objects are **structurally validated but not legitimized**  
- Only ACCEPTED resolutions from full restore are fully active  
- No automatic BLOCK_TEMPORARY sessions are created  
- Baseline mode objects may exist in **shadow/review state**

---

# 5. Governance Hygiene Enforcement

Engine enforces:

- Exclusive Authority / Scope per Area  
- Participant set completeness  
- Candidate snapshot consistency  
- BLOCK_PERMANENT invariants

Fail if:

- Multiple ACTIVE Authorities detected  
- Participant snapshot empty  
- Candidate or vote snapshot incomplete  

---

# 6. Deterministic Import Guarantees

- Identical input + mode → identical validation outcome  
- Supersession graph integrity deterministic  
- Eligibility for evaluation deterministic  

Fail if:

- Two identical imports produce differing results  

---

# 7. Failure Semantics

- Structural violations result in:

  - Explicit failure  
  - Deterministic EvaluationReport  
  - No mutation of existing engine state  
  - Full rejection of imported Area or batch

- No partial activation allowed  

---

# 8. Engine API Interaction

- **rehydrate_engine(domain_objects)** must be called post-import or post-simulation to load objects into evaluation memory  
- **simulate_restore(domain_objects)** allows preview of full restore effects without mutation  
- CLI may use **validate_imported_area(domain_objects)** for pre-rehydration validation  
- Baseline consolidation requires the host to iteratively accept resolutions via standard session API  

---

# 9. Mental Model

- Restore = full overwrite, validated, fully active if successful  
- Consolidation = incremental, under review, no automatic legitimacy  
- Host provides objects; engine validates structure  
- Nothing changes unless explicitly accepted via session mechanics  
- BLOCK_PERMANENT hygiene enforced consistently  
- Supersession remains graph-consistent and acyclic  

---

# 10. Alignment with Other Specifications

- ENG-ENGINE-INITIALIZATION  
- ENG-DOMAIN  
- ENG-SESSION  
- ENG-DECISION  
- ENG-SUPERSESSION  
- ENG-AUD  
- ENG-API  

Violation constitutes a structural engine import failure.