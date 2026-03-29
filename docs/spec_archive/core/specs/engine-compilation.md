# ENG-COMPILATION  
Historical DAG Reconstruction & Incremental Compilation Model  

Status: REFACTORED (v3 – Structure-Aligned, Terminology-Converged)  
Applies to: Engine Core (V1/V2+)  

Authority: Subordinate to ENG-INTEGRITY, ENG-STRUCTURE, ENG-RECEIPT, ENG-SPECVERIFY  

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-STRUCTURE
- ENG-USABILITY
- ENG-INTEGRITY
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

---

# 1. Purpose

ENG-COMPILATION defines how the Engine reconstructs a valid structural legitimacy DAG from historical artifacts.

It is the authoritative specification for:

- deterministic reconstruction of accepted history  
- structural graph reconstruction  
- resolution index construction  
- incremental ingestion of historical artifacts  
- structural conflict detection during ingestion  
- canonical ordering independent of timestamps  

Compilation does not:

- create legitimacy  
- re-evaluate legitimacy  
- replay acceptance logic  
- modify historical outcomes  

Legitimacy is defined exclusively by:

- ACCEPTED sessions  
- emitted receipts  

Compilation reconstructs what already happened.

---

# 2. Compilation vs Runtime

## ENG-COMP-01 — Separation of Concerns

Runtime Mode:

- evaluates sessions  
- performs acceptance  
- creates structural artifacts (including Resolutions where applicable)  
- emits receipts  

Compilation Mode:

- ingests historical artifacts  
- verifies receipts  
- reconstructs structural graph  
- derives structural ACTIVE sets  

Compilation must not:

- call acceptance logic  
- generate new receipts  
- alter session outcomes  

---

# 3. Source of Truth

## ENG-COMP-02 — Receipt-Driven Reconstruction

The canonical source of historical legitimacy is:

- LEGITIMACY receipts  
- associated structural artifacts (including Resolutions)  

Receipts define:

- which sessions were accepted  
- the final round state  
- governance context  
- rule identity (`spec_set_hash`)  

Sessions are not authoritative for accepted historical truth during compilation.

If a receipt and session disagree:

- receipt prevails  

---

# 4. Resolution Index

## ENG-COMP-03 — Deterministic Index Construction

The Engine must construct a resolution index from:

- all Resolutions in the Area  
- all LEGITIMACY receipts  

The index must include:

- resolution_id  
- originating_session_id  
- state  
- superseded_by (if present)  
- spec_set_hash  

The index must:

- include all resolutions regardless of state  
- be fully derivable from domain objects  
- be deterministic across implementations  

The resolution index represents the structural node set used for graph reconstruction.

---

# 5. Ordering Model

## ENG-COMP-04 — No Timestamp Authority

Compilation must not depend on:

- timestamps (`accepted_at`, `created_at`, etc.)  
- audit ordering  
- storage ordering  

Canonical ordering is derived from:

- structural relationships  
- deterministic identifier ordering  
- resolution index consistency  

If additional ordering is required:

- it must be derived from canonical identifiers (e.g., lexicographic ordering)  
- it must not imply legitimacy precedence  

---

# 6. Structural Graph Reconstruction

## ENG-COMP-05 — Graph Reconstruction

Compilation must:

- reconstruct structural edges from Resolution objects  
- reconstruct the structural graph, including supersession relationships  
- validate graph acyclicity  
- enforce Area-local constraints  

Structural graph semantics are defined in ENG-STRUCTURE.

Compilation must not redefine:

- structural conflict rules  
- acceptance race semantics  

---

# 7. Conflict Handling

## ENG-COMP-06 — Structural Conflict Detection Only

Compilation detects structural inconsistencies such as:

- conflicting structural edges  
- multiple incompatible successors for the same target  
- missing structural references  
- cycles  
- invalid state combinations  

On conflict:

- compilation must fail (StructuralIntegrityFailure)  
- no heuristic resolution permitted  

Compilation must not:

- choose a “winner”  
- reject previously accepted sessions  
- reinterpret legitimacy  

---

# 8. Incremental Compilation

## ENG-COMP-07 — Incremental Ingestion

Incremental compilation allows staged ingestion of historical artifacts.

Requirements:

- all structural dependencies must be present or staged  
- receipts must validate (ENG-RECEIPT + ENG-CANON)  
- spec_set_hash must pass verification (ENG-SPECVERIFY)  

The Engine must:

- merge new artifacts into the existing graph  
- rebuild the resolution index deterministically  
- re-run structural validation (ENG-INTEGRITY)  

If dependencies are missing:

- fail with deterministic error  
- do not partially apply  

---

# 9. Administrative State Handling

## ENG-COMP-08 — Administrative States Are Non-Structural

Resolution states:

- ON_HOLD  
- RETIRED  

affect:

- forward usability  

They do not affect:

- historical legitimacy  
- structural graph reconstruction  
- DAG structure  

Compilation must treat them as:

- non-structural metadata layered on top of the graph  

Usability semantics are defined in ENG-USABILITY.

---

# 10. Receipt Verification

## ENG-COMP-09 — Mandatory Receipt Validation

For each LEGITIMACY receipt:

- canonical serialization must validate (ENG-CANON)  
- content_hash must match  
- spec_set_hash must be verified (ENG-SPECVERIFY)  

Failure:

- must result in compilation failure  

Compilation must not proceed with invalid receipts.

Runtime consequences of invalid receipts are defined in ENG-INTEGRITY.

---

# 11. Determinism Guarantee

## ENG-COMP-10 — Fully Deterministic Reconstruction

Given identical inputs:

- domain objects  
- receipts  
- structural artifacts  

All implementations must produce identical:

- resolution index  
- structural graph  
- ACTIVE set  

Compilation must not depend on:

- timestamps  
- environment  
- execution order  

---

# 12. Audit Interaction

## ENG-COMP-11 — Audit Is Ignored

Compilation must not:

- read audit logs  
- derive ordering from audit  
- depend on audit presence  

Audit may record compilation activity, but:

- must not influence outcomes  

---

# 13. Failure Semantics

## ENG-COMP-12 — Fail-Closed Behavior

Compilation must fail if:

- structural inconsistencies exist  
- receipts fail validation  
- structural graph is invalid  
- required references are missing  

Failure must be:

- deterministic  
- non-mutating  
- complete (no partial graph acceptance)  

---

# 14. Engine Invariants

- compilation never creates legitimacy  
- compilation never replays acceptance  
- compilation never overrides receipts  
- compilation never uses timestamps for authority  
- compilation never resolves conflicts heuristically  
- compilation always derives from canonical artifacts  
- compilation is fully deterministic  

---

# 15. Mental Model

Compilation reconstructs the past.

- Receipts define what happened  
- Structural artifacts define graph structure  
- ENG-STRUCTURE defines graph meaning  
- ENG-INTEGRITY defines validity  

Compilation does not decide history.

It proves that history is internally consistent.