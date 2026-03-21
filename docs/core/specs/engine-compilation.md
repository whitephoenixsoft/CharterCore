# ENG-COMPILATION  
Historical DAG Reconstruction & Incremental Compilation Model  

Status: REFACTORED (v2 – Receipt-Driven Deterministic Reconstruction)  
Applies to: Engine Core (V1/V2+)  

Authority: Subordinate to ENG-INTEGRITY, ENG-SUPERSESSION, ENG-RECEIPT, ENG-SPECVERIFY  

---

# 1. Purpose

ENG-COMPILATION defines how the Engine reconstructs a valid legitimacy DAG from historical artifacts.

It is the authoritative specification for:

- deterministic reconstruction of accepted history  
- resolution index construction  
- incremental ingestion of historical artifacts  
- conflict detection during ingestion  
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
- creates resolutions  
- emits receipts  

Compilation Mode:

- ingests historical artifacts  
- verifies receipts  
- reconstructs supersession graph  
- derives ACTIVE sets  

Compilation must not:

- call acceptance logic  
- generate new receipts  
- alter session outcomes  

---

# 3. Source of Truth

## ENG-COMP-02 — Receipt-Driven Reconstruction

The canonical source of historical legitimacy is:

- LEGITIMACY receipts  
- associated Resolutions  

Sessions are informational during compilation.

Receipts define:

- which sessions were accepted  
- the final round state  
- governance context  
- rule identity (`spec_set_hash`)  

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

---

# 5. Ordering Model

## ENG-COMP-04 — No Timestamp Authority

Compilation must not depend on:

- timestamps (`accepted_at`, `created_at`, etc.)
- audit ordering  
- storage ordering  

Canonical ordering is derived from:

- supersession relationships  
- deterministic structural ordering  
- resolution index consistency  

If additional ordering is required for deterministic processing:

- it must be derived from canonical identifiers (e.g., lexicographic ordering)  
- it must not imply legitimacy precedence  

---

# 6. Supersession Reconstruction

## ENG-COMP-05 — Graph Reconstruction

Compilation must:

- reconstruct supersession edges from Resolution objects  
- validate graph acyclicity  
- enforce Area-local constraints  

Supersession semantics are defined in ENG-SUPERSESSION.

Compilation must not redefine:

- conflict rules  
- acceptance race semantics  

---

# 7. Conflict Handling

## ENG-COMP-06 — Structural Conflict Detection Only

Compilation detects structural inconsistencies such as:

- multiple resolutions superseding the same target inconsistently  
- missing supersession targets  
- cycles  
- invalid state transitions  

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

Incremental compilation allows partial ingestion of historical artifacts.

Requirements:

- all referenced objects must be present or staged  
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

- UNDER_REVIEW  
- RETIRED  

affect:

- runtime usability  

They do not affect:

- historical legitimacy  
- supersession reconstruction  
- DAG structure  

Compilation must treat them as:

- metadata on top of structural graph  

---

# 10. Receipt Verification

## ENG-COMP-09 — Mandatory Receipt Validation

For each LEGITIMACY receipt:

- canonical serialization must validate (ENG-CANON)  
- content_hash must match  
- spec_set_hash must be verified (ENG-SPECVERIFY)  

Failure → StructuralIntegrityFailure

Compilation must not proceed with invalid receipts.

---

# 11. Determinism Guarantee

## ENG-COMP-10 — Fully Deterministic Reconstruction

Given identical inputs:

- domain objects  
- receipts  
- resolutions  

All implementations must produce identical:

- resolution index  
- supersession graph  
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
- supersession graph invalid  
- references missing  

Failure must be:

- deterministic  
- non-mutating  
- complete (no partial graph acceptance)  

---

# 14. Engine Invariants

- Compilation never creates legitimacy  
- Compilation never replays acceptance  
- Compilation never overrides receipts  
- Compilation never uses timestamps for authority  
- Compilation never resolves conflicts heuristically  
- Compilation always derives from canonical artifacts  
- Compilation always deterministic  

---

# 15. Mental Model

Compilation reconstructs the past.

- Receipts define what happened  
- Resolutions define structure  
- Supersession defines evolution  
- Integrity defines validity  

Compilation does not decide history.

It proves history is internally consistent.