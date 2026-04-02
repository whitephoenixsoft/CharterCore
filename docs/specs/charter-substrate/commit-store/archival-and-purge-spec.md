# Charter Commit Store — Archival & Purge Specification

Status: FOUNDATIONAL  
Intent: Define the archival and purge model across all runtime-managed data and commit artifacts  
Scope: Runtime → CCS archival pipeline, archive bundle commits, runtime purge, forward-compatible commit-store purge  
Does NOT Define: commit structure beyond archival extension (CCS), runtime UX, legitimacy semantics, alignment, or guidance  

---

# 1. Purpose

This specification defines how Charter:

- preserves runtime-managed data as immutable commit artifacts  
- safely reduces local storage without compromising integrity  
- provides a structured and auditable path for data removal  
- maintains consistency across local and federated environments  

Archival and purge are distinct:

- **Archival** preserves data  
- **Purge** removes local copies after preservation  

---

# 2. Core Principle

> Nothing is removed until it is safely preserved.

The system guarantees:

- preservation precedes removal  
- removal is explicit and auditable  
- meaning is never altered during storage transitions  

The system must not:

- delete data implicitly  
- rewrite artifacts  
- infer meaning during archival or purge  

---

# 3. Runtime-Orchestrated Storage Model

The Runtime Layer orchestrates all local data stores.

---

## 3.1 Runtime-Managed Stores

The Runtime Layer manages multiple categories of data:

### A. Workflow Store (Non-Legitimacy)
- deliberates  
- breakouts  
- synthesis artifacts  
- baseline review workspaces  
- draft candidates  

Characteristics:
- mutable within workflow constraints  
- generally purgeable after archival  

---

### B. Legitimacy Store (Integrity-Critical)
- sessions  
- resolutions  
- authority and scope records  
- receipts  
- supersession relationships  

Characteristics:
- immutable or append-only at the semantic level  
- required for legitimacy validation and audit  

---

### C. Derived / Auxiliary Stores (Optional)
- indexes  
- caches  
- metadata  

Characteristics:
- rebuildable  
- not primary sources of truth  

---

## 3.2 Commit Store (Preserved Artifacts)

The Commit Store contains:

- immutable commits  
- resolution commits  
- receipt commits  
- archive bundle commits  
- imported artifacts  

Characteristics:

- append-only by default  
- immutable  
- portable  
- queryable  

---

## 3.3 Key Distinction

- Runtime-managed stores = **active and operational data**  
- Commit Store = **preserved, immutable artifacts**  

---

# 4. Archivable vs Purgeable

## 4.1 Archivable

All runtime-managed data is **archivable**.

This includes:

- workflow artifacts  
- legitimacy artifacts  

Archival produces immutable commits and does not affect meaning.

---

## 4.2 Purgeable

Not all data is purgeable.

- Workflow artifacts → generally purgeable  
- Legitimacy artifacts → **conditionally purgeable**  

Purgeability depends on whether removal preserves system integrity.

---

# 5. Definition: Archival

Archival is:

> An explicit, auditable transformation of runtime-managed data into immutable commit artifacts.

Archival:

- preserves identity, hashes, lineage, and provenance  
- produces new commits  
- does not interpret or modify meaning  

---

# 6. Archive Bundle Commit

## 6.1 Definition

An **Archive Bundle Commit** is a commit that encapsulates a set of runtime artifacts.

---

## 6.2 Properties

An Archive Bundle Commit:

- contains serialized artifacts from runtime stores  
- preserves original identifiers and hashes  
- may include dependency closure  
- may include contextual artifacts (narrative mode)  
- may optionally reference original runtime objects  

---

## 6.3 Structural Role

- canonical archival unit  
- replaces ad hoc export packaging  
- transportable via relay  
- rehydratable by runtime  

---

## 6.4 Non-Interpretation Guarantee

Archive Bundle Commits are:

- opaque to CCS, Commit Store, and Relay  
- non-authoritative  
- not interpreted during storage or transport  

---

# 7. Archival Pipeline

---

## 7.1 Stage 1 — Selection

Artifacts selected from any runtime-managed store.

---

## 7.2 Stage 2 — Review

Non-legitimizing validation of:

- inclusion  
- dependency completeness  
- archival mode  

---

## 7.3 Stage 3 — Packaging

Artifacts grouped into an archive bundle.

- no transformation  
- IDs and hashes preserved  

---

## 7.4 Stage 4 — Commit Creation (CCS)

Archive bundle becomes an:

- Archive Bundle Commit  

---

## 7.5 Stage 5 — Persistence

Commit stored in Commit Store.

---

## 7.6 Stage 6 — Purge Eligibility

Original artifacts become eligible for purge.

---

# 8. Archival Modes

## MINIMAL
- selected artifacts only  

## CLOSURE
- dependency-complete  

## NARRATIVE
- closure + contextual history  

---

> Modes affect completeness, not meaning.

---

# 9. Runtime Purge

## 9.1 Definition

> Explicit removal of runtime-managed data after archival.

---

## 9.2 Requirements

- user-initiated  
- auditable  
- preservation verified  

---

## 9.3 Safety Model

System:

- must surface dependency impact  
- must allow explicit override  

System must not:

- infer semantic importance  
- enforce hidden authority  

---

## 9.4 Guarantees

- archived data remains intact  
- legitimacy remains valid  
- commit store unchanged  

---

# 10. Legitimacy-Sensitive Constraints

## 10.1 Preservation Rule

Legitimacy-critical artifacts MUST NOT be removed if removal would break:

- resolution validation  
- authority or scope reconstruction  
- supersession graph integrity  
- auditability  

---

## 10.2 Override Model

The system may allow purge with:

- explicit user override  
- full visibility into consequences  
- audit trail  

---

## 10.3 Responsibility Boundary

- Runtime enforces constraints  
- Legitimacy Engine defines what is required  
- Commit Store does not interpret importance  

---

# 11. Future: Commit Store Purge

## 11.1 Status

Out of scope for current implementation, but structurally supported.

---

## 11.2 Principles

- explicit and multi-stage  
- review-driven  
- auditable  
- does not rewrite history  

---

## 11.3 Invariant

> Local removal does not imply global deletion.

---

# 12. Federation Model

Artifacts may exist in:

- local commit store  
- relay systems  
- exported bundles  

---

## Invariant

> Absence locally ≠ absence globally

---

# 13. Relationship to Export (CCE)

## 13.1 Pre-Commit Store Model

- export artifacts (CCE)  
- store externally (e.g., source control)  
- re-import as needed  

---

## 13.2 Commit Store Model

- archive → commit  
- store locally  
- optionally relay  

---

## 13.3 Invariant

> Commit Store formalizes and internalizes export-based preservation.

---

# 14. Relationship to CCS

CCS defines:

- commit structure  
- archive bundle commit type  

This spec defines:

- how archival produces commits  

---

# 15. Relationship to Runtime

Runtime:

- orchestrates all archival and purge  
- manages all local stores  

---

# 16. Relationship to Commit Store

Commit Store:

- stores commits immutably  
- does not interpret or enforce archival  

---

# 17. Relationship to Relay (CRS)

Relay:

- transports commits  
- stores immutably  
- does not interpret  

---

# 18. Invariants

- Archival does not change meaning  
- All archived data becomes commits  
- Archive Bundle Commits are immutable  
- Purge requires prior preservation  
- Purge is explicit and auditable  
- Legitimacy integrity must be preserved  
- No layer may reinterpret artifacts  
- Local removal does not imply global deletion  

---

# 19. Mental Model

Archival:

- converts active data into preserved artifacts  

Purge:

- removes local copies after preservation  

The system:

- does not forget  
- only changes where data resides  

---

# 20. Final Principle

Charter preserves before it removes.

Archival creates safety.  
Purge requires proof.  
Integrity is never compromised.