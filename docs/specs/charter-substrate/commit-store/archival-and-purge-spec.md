# Charter Commit Store — Archival & Purge Specification

Status: FOUNDATIONAL  
Intent: Define the archival and purge model for runtime artifacts and commit artifacts  
Scope: Runtime → CCS archival pipeline, archive bundle commits, local purge, future commit-store purge  
Does NOT Define: commit structure beyond archival extension (CCS), runtime workflows in detail, legitimacy semantics, alignment, or guidance  

---

# 1. Purpose

This specification defines how Charter:

- preserves runtime artifacts as immutable commit artifacts  
- reduces active storage safely and explicitly  
- enables controlled removal of local data  
- maintains integrity across local and federated environments  

Archival and purge are **separate but related workflows**:

- **Archival** creates preserved, immutable artifacts  
- **Purge** removes local data only after preservation is verified  

---

# 2. Core Principle

> Nothing is removed until it is safely preserved.

The system must guarantee:

- preservation precedes removal  
- removal is always explicit  
- meaning is never altered during storage transitions  

At no point may the system:

- delete data implicitly  
- rewrite or reinterpret artifacts  
- destroy data without user intent  

---

# 3. Storage Domains

Charter operates across two primary storage domains.

---

## 3.1 Runtime Store (Working State)

Holds:

- sessions  
- deliberates  
- breakouts  
- synthesis artifacts  
- baseline review workspaces  
- candidates and workflow state  

Characteristics:

- mutable within workflow constraints  
- optimized for active use  
- eligible for archival and purge  

---

## 3.2 Commit Store (Preserved Artifacts)

Holds:

- immutable commits (all types)  
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

- Runtime Store = **working state**  
- Commit Store = **preserved truth artifacts**  

Archival moves data from Runtime → Commit Store.  
Purge removes data from local storage only.

---

# 4. Definition: Archival

Archival is:

> An explicit, auditable process that transforms runtime artifacts into immutable commit artifacts.

Archival:

- preserves identity, hashes, lineage, and provenance  
- does not modify or reinterpret artifacts  
- produces new commits representing preserved data  

Archival changes **storage posture**, not meaning.

---

# 5. Archive Bundle Commit

## 5.1 Definition

An **Archive Bundle Commit** is a commit that encapsulates a set of runtime artifacts for preservation.

It is:

- a first-class commit type within CCS  
- immutable and append-only  
- self-contained or partially dependent (based on mode)  

---

## 5.2 Properties

An Archive Bundle Commit:

- contains serialized runtime artifacts  
- preserves original identifiers and hashes  
- may include dependency closure  
- may include contextual artifacts (narrative mode)  
- may include optional references to original runtime objects  

---

## 5.3 Structural Role

The Archive Bundle Commit:

- replaces temporary storage constructs as the canonical archival unit  
- serves as the durable boundary between working state and preserved artifacts  
- is transportable via relay  
- is rehydratable by runtime systems  

---

## 5.4 Non-Interpretation Guarantee

The Archive Bundle Commit:

- is opaque to CCS, Commit Store, and Relay  
- carries no intrinsic authority  
- is not interpreted during storage or transport  

Interpretation occurs only in higher layers (e.g., runtime rehydration or guidance).

---

# 6. Archival Pipeline

Archival proceeds through explicit stages.

---

## 6.1 Stage 1 — Candidate Selection

Artifacts are selected from the Runtime Store using:

- explicit selection  
- time-based filters  
- state-based filters  
- workflow classification  

---

## 6.2 Stage 2 — Archival Review

A non-legitimizing review process validates:

- inclusion  
- dependency completeness  
- archival mode  

This stage:

- does not create authority  
- does not modify artifacts  
- ensures safe packaging  

---

## 6.3 Stage 3 — Packaging

Artifacts are packaged into an Archive Bundle:

- no transformation of content  
- preservation of IDs and hashes  
- optional dependency expansion  

Implementation details (e.g., temporary areas) are runtime concerns.

---

## 6.4 Stage 4 — Commit Creation (CCS)

The packaged archive is committed as an:

- Archive Bundle Commit  

This ensures:

- immutability  
- identity assignment  
- integration into commit graph  

---

## 6.5 Stage 5 — Commit Store Persistence

The Archive Bundle Commit is stored in the Commit Store:

- becomes durable  
- becomes queryable  
- may be exported or relayed  

---

## 6.6 Stage 6 — Purge Eligibility

Once preservation is verified:

- original runtime artifacts become eligible for purge  

They are not removed automatically.

---

# 7. Archival Modes

Archival modes define completeness.

---

## 7.1 MINIMAL

- includes only selected artifacts  
- does not include dependencies  
- may contain unresolved references  

---

## 7.2 CLOSURE

- includes all required dependencies  
- produces a self-contained archive  

---

## 7.3 NARRATIVE

- includes closure + contextual artifacts  
- preserves reasoning and evolution  

---

## 7.4 Mode Principle

> Modes affect completeness, not meaning.

All modes preserve:

- identity  
- hashes  
- lineage  

---

# 8. Definition: Purge (Runtime Store)

Purge is:

> An explicit operation that removes runtime artifacts after archival.

---

## 8.1 Requirements

Purge must be:

- user-initiated  
- explicit  
- auditable  
- preceded by verified archival  

---

## 8.2 Safety Model

The system may:

- warn about dependency breakage  
- surface impact  

The system must not:

- infer semantic importance  
- block purely on interpretation  

---

## 8.3 Guarantees

After purge:

- archived artifacts remain available  
- commit store remains intact  
- legitimacy is unaffected  

---

## 8.4 Scope

Runtime purge affects only:

- Runtime Store  

It does not affect:

- Commit Store  
- Relay  
- external archives  

---

# 9. Future Model: Commit Store Purge

Commit Store purge is **out of scope for this version**, but structurally anticipated.

---

## 9.1 Principles (Forward-Looking)

Commit Store purge must:

- be explicit and multi-stage  
- require review and selection  
- preserve federation awareness  
- never rewrite history  

---

## 9.2 Key Distinction

> Removing a commit locally does not erase it globally.

Commits may still exist in:

- relay systems  
- external exports  
- other local stores  

---

## 9.3 Non-Goal (Current Version)

This specification does not define:

- commit deletion mechanics  
- global coordination  
- retention policies  

---

# 10. Federation & Distribution

Charter operates in a federated environment.

---

## 10.1 Invariant

> Local absence does not imply global absence.

Artifacts may exist in:

- local commit store  
- relay systems  
- external archives  

---

## 10.2 Implications

- purge is local  
- archival enables portability  
- systems must not assume completeness  

---

# 11. Relationship to CCS

CCS defines:

- commit structure  
- archive bundle commit type  
- identity and integrity guarantees  

This specification defines:

- how archival produces commits  
- how commits are used for preservation  

---

# 12. Relationship to Runtime Layer

Runtime:

- selects artifacts  
- orchestrates archival pipeline  
- performs purge  

This specification does not define:

- runtime UX  
- workflow orchestration details  
- temporary workspace implementations  

---

# 13. Relationship to Commit Store

Commit Store:

- persists archive bundle commits  
- guarantees immutability  
- supports retrieval  

It does not:

- interpret archival content  
- enforce archival workflows  
- perform runtime purge  

---

# 14. Relationship to Relay (CRS)

Relay:

- transports archive bundle commits  
- stores them immutably  
- does not interpret completeness or meaning  

---

# 15. Invariants

- Archival must not change meaning  
- Archival must preserve identity and hashes  
- Archive Bundle Commits must be immutable  
- Commit Store must not mutate commits  
- Purge must not occur before preservation  
- Purge must be explicit and auditable  
- Local removal must not imply global deletion  
- No layer may reinterpret archival artifacts  

---

# 16. Mental Model

Archival is:

- converting working state into preserved artifacts  

Purge is:

- removing local working copies after preservation  

The system:

- does not forget  
- only changes where data resides  

---

# 17. Final Principle

Charter preserves before it removes.

Archival creates safety.  
Purge requires proof.  
Truth remains intact across all storage boundaries.