# Charter — Archival Model (Conceptual Specification)

Status: FOUNDATIONAL (Conceptual)  
Applies to: CLI Runtime, Persistence Layer, Relay Interaction  
Does NOT apply to: Engine legitimacy semantics  

---

## 1. Purpose

This document defines the conceptual model for **archival in Charter**.

Archival exists to:

- manage long-term storage growth  
- support institutional lifecycle management  
- enable durable, portable historical preservation  
- separate active working state from inactive history  

Archival is **not deletion** and **not governance**.

---

## 2. Core Principle

> Archiving changes storage posture, not meaning.

Archival must never:

- change legitimacy  
- alter authority  
- modify lineage  
- reinterpret history  
- remove provenance  

All archived artifacts retain full identity and meaning.

---

## 3. Definition: Archival

Archival is:

> An explicit, auditable operation that relocates a set of artifacts from active storage into a preserved, self-contained archival form.

Archival produces:

- a new storage boundary (typically an Area or export)
- a preserved subset of artifacts
- optional references from the originating system

Archival does not:

- mutate artifacts
- compress or summarize meaning
- delete history

---

## 4. Archival Unit

The fundamental unit of archival is:

> A subset of artifacts extracted into a new Area or export bundle.

Properties:

- preserves original IDs
- preserves object hashes
- preserves lineage and references
- preserves spec and engine provenance

Archived Areas behave as **first-class Areas**.

---

## 5. Identity and Integrity

### 5.1 Identity Preservation

All archived artifacts MUST:

- retain original IDs
- retain object hashes
- retain timestamps
- retain provenance fields

Archival must not re-materialize new identities.

---

### 5.2 Integrity Preservation

Archived artifacts MUST:

- pass all integrity checks
- preserve referential relationships
- maintain canonical serialization

Archival is a relocation, not a transformation.

---

## 6. Dependency and Closure

Archival operates on a **graph of artifacts**.

### 6.1 Closure Principle

By default, archival SHOULD produce a **closure-complete subgraph**.

This means:

- all referenced artifacts required for interpretation are included
- no required dependencies are omitted

---

### 6.2 Non-Closure Mode (Optional)

Archival MAY allow partial extraction:

- references may point to external artifacts
- archive is not self-contained

This mode must be:

- explicit  
- clearly marked  
- treated as dependent on external context  

---

## 7. Archival Classes (Conceptual)

Artifacts may be eligible for archival based on classification:

### 7.1 Non-Legitimacy Workflow

- deliberates
- breakouts
- synthesis artifacts
- baseline review workspaces
- rejected or abandoned proposals

---

### 7.2 Historical Workflow (Cold)

- closed sessions without acceptance
- review receipts
- exploration receipts

---

### 7.3 Convenience Artifacts

- participant groups
- draft candidate sets
- derived or temporary state

---

### 7.4 Canonical Core (Default Exclusion)

Typically not archived by default:

- active resolutions
- legitimacy receipts
- authority and scope state

These may be archived only under explicit policy.

---

## 8. Archival Workflow (Conceptual)

Archival follows a structured pattern:

### 8.1 Candidate Selection

Artifacts are selected via:

- explicit user selection  
- time-based filters  
- state-based filters  
- relevance criteria  

---

### 8.2 Archival Review

Selected artifacts are reviewed:

- inclusion is explicit  
- dependencies are evaluated  
- closure behavior is determined  

This stage is **non-legitimizing**.

---

### 8.3 Archival Closure

Upon closure:

- archival set is finalized  
- an archival record (receipt or audit event) is emitted  
- artifacts are packaged into a new Area or export  

---

### 8.4 Archival Movement

Artifacts are:

- relocated to cold storage  
- exported (CCE or equivalent)  
- optionally pushed to relay  

---

## 9. Archival Outputs

Archival may produce:

- a new Area containing archived artifacts  
- a CCE export bundle  
- a relay-published archive  
- local cold storage structures  

All outputs must preserve:

- identity  
- lineage  
- provenance  

---

## 10. References to Archived Artifacts

The originating system MAY:

- maintain soft references to archived Areas  
- record archival locations  

These references are:

- informational  
- non-legitimizing  
- non-binding  

Archived Areas must remain self-contained.

---

## 11. Relationship to Relay

Relay may act as:

- archival storage  
- distribution mechanism  
- redundancy layer  

Relay must:

- preserve artifacts immutably  
- not interpret or merge archives  
- treat archived data identically to other commits  

---

## 12. Relationship to Export (CCE)

CCE is the canonical archival format for legitimacy-bearing data.

Archival may:

- produce CCE exports  
- include spec_set_hash and engine_version  
- preserve deterministic integrity  

Archival must not:

- alter export semantics  
- redefine restore behavior  

---

## 13. Archival vs Deletion

Archival:

- preserves data  
- preserves identity  
- preserves meaning  
- remains reversible  

Deletion:

- destroys data  
- breaks lineage  
- removes proof  
- is irreversible  

These operations must never be conflated.

---

## 14. Invariants

- Archival must not change legitimacy  
- Archival must preserve identity  
- Archival must preserve provenance  
- Archival must be explicit and auditable  
- Archival must not introduce hidden dependencies  
- Archived artifacts must remain interpretable independently (if closure-complete)  
- Archival must not compress or reinterpret history  

Violation of these invariants breaks Charter’s integrity guarantees.

---

## 15. Mental Model

Archival is:

- moving history to a different shelf  
- not rewriting the book  
- not summarizing the book  
- not discarding the book  

The system remembers everything.  
Archival determines where it is kept.

---

## 16. Final Principle

Charter does not forget.

It only changes how history is stored, accessed, and transported.