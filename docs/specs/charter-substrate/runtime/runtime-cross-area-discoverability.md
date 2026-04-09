# Charter Runtime — Cross-Area Discoverability Specification (Revised)

Status: FOUNDATIONAL  
Applies to: Runtime Layer, Charter Commit System (CCS), Commit Store, Reconciliation Review, Federation  
Does NOT define: transport protocols (CRS), legitimacy semantics, alignment computation, or guidance behavior  

---

# 1. Purpose

This document defines how **external Areas and their associated graphs become discoverable, accessible, and referenceable** within the Charter Runtime.

Discoverability exists to:

- make external Areas **identifiable**
- enable **query and acquisition** of foreign partial graphs
- support **reconciliation workflows** (forward and reverse)
- allow **safe cross-area referencing**
- enable **investigation and simulation over foreign structure**

Discoverability is a **precondition for structural modeling and simulation**.

If relationships cannot be discovered, they cannot be modeled.  
If they cannot be modeled, they cannot be simulated or reconciled.

---

# 2. Core Principle

> Discoverability provides access to foreign structure.  
> It does not grant authority, correctness, integration, or completeness.

A discovered Area is:

- visible  
- queryable  
- fetchable  
- inspectable  

It is not:

- locally valid  
- up-to-date  
- structurally integrated  
- authoritative  

---

# 3. Area Model

## 3.1 Areas as Partial Graph Holders

Each Area represents a **partial, local view of a broader distributed graph**.

An Area may contain:

- locally created commits  
- previously imported commits  
- lineage structures  
- observational artifacts  

No Area is assumed to contain a complete or globally consistent graph.

---

## 3.2 Distributed Reality

Across a federated system:

- different runtimes hold different subsets  
- histories may diverge  
- snapshots may be stale  

> Global consistency is not required.  
> Cohesion is achieved through explicit reconciliation.

---

# 4. Discoverability Capabilities

## 4.1 Identification

The Runtime must support identifying external Areas via:

- relay sources (CRS)  
- local registries  
- previously imported artifacts  
- manual references  

---

## 4.2 Query (Lightweight Discovery)

Before acquisition, the Runtime SHOULD support **lightweight queries**:

- Area existence  
- metadata (identity, scope, version hints)  
- available snapshots or references  

Querying:

- does not import data  
- does not create local state  
- enables informed acquisition decisions  

---

## 4.3 Acquisition (Fetch / Pull)

The Runtime must support acquiring a **foreign partial graph**:

- full Area snapshot  
- filtered subset  
- commit bundle  

Acquisition produces a **foreign, read-only graph representation**.

Only **durable CCS commits** are discoverable and fetchable.  
Transient systems (e.g., CSP feeds) are not part of discoverability unless materialized.

---

## 4.4 Local Availability

Once acquired, the foreign graph becomes:

- locally accessible  
- referenceable within controlled contexts  
- available for reconciliation workflows  
- usable for investigation and simulation  

---

## 4.5 No Implicit Synchronization

Discoverability does NOT:

- synchronize Areas automatically  
- maintain continuous consistency  
- update local copies without explicit action  

All updates require explicit acquisition.

---

# 5. Isolation of Foreign Graphs

## 5.1 Isolated Storage

Acquired foreign graphs MUST be stored in an **isolated, read-only environment**.

Properties:

- immutable  
- non-legitimizing  
- non-authoritative  
- separate from active runtime state  

---

## 5.2 Purpose of Isolation

Isolation ensures:

- safe inspection without side effects  
- prevention of accidental integration  
- clear boundary between foreign and local truth  

---

# 6. Relationship to Reconciliation

## 6.1 Reconciliation as Integration and Simulation Boundary

All interaction with foreign graph data MUST occur through **Reconciliation Review**.

Reconciliation is a **unified model** supporting:

- forward integration (toward legitimacy)  
- reverse integration (toward CDS simulation)  
- federation ingestion  

---

## 6.2 Forward Reconciliation (Legitimacy Path)

Foreign artifacts are:

→ evaluated  
→ accepted or rejected  
→ admitted through session  

Result:

- durable resolutions  
- durable structural relationships  

---

## 6.3 Reverse Reconciliation (Simulation Path)

Foreign artifacts may be:

→ transformed into CDS Items  
→ used for investigation and simulation  

Properties:

- produces **non-legitimate Items**  
- preserves lineage via `derived_from`  
- does not create or modify resolutions  

---

## 6.4 Core Rule

> Discoverability enables access.  
> Reconciliation determines how that access is used.

---

# 7. Reference Model

## 7.1 Reference Targets

Cross-area references may target:

- **Areas** (boundary-level reference)  
- **Resolutions** (legitimate artifacts)  
- **Items** (investigative artifacts via CDS)  

---

## 7.2 Area References (Boundary-Level)

An Area reference indicates:

- structural relevance of another Area  
- dependency, coordination, or relationship at the boundary level  
- acknowledgment of incomplete or evolving knowledge  

Area references:

- do NOT reference specific decisions  
- do NOT imply adoption  
- do NOT imply trust  
- do NOT imply completeness or freshness  

---

## 7.3 Resolution References (Artifact-Level)

A Resolution reference indicates:

- relevance of a specific accepted artifact  
- a concrete decision anchor  

Properties:

- precise  
- durable (after reconciliation)  

---

## 7.4 Item References (Investigative)

Item references indicate:

- exploratory or simulated relationships  
- pre-legitimacy structure  

Properties:

- non-authoritative  
- may evolve or be discarded  
- may later produce structural relationships via reconciliation  

---

## 7.5 Provisional vs Integrated References

### A. Provisional References

- created against isolated foreign graphs  
- exist only within reconciliation contexts or CDS  
- non-authoritative  
- non-durable  

---

### B. Integrated References

- created after successful reconciliation  
- durable within the local graph  
- visible to CSG and CAS  

---

## 7.6 Core Rule

> Unreviewed foreign artifacts may be referenced provisionally,  
> but must not become durable structural anchors.

---

# 8. Structural Scope of Discovery

Discovered graphs may include:

- resolution nodes (legitimate structure)  
- item nodes (investigative structure, if exported)  

CSG must preserve:

- node class distinction  
- origin of structure  
- non-authoritative nature of items  

---

# 9. CDS Interaction

Discoverability enables CDS to:

- seed Items from foreign signals or resolutions  
- observe foreign structure  
- surface candidate relationships  

These relationships:

- are non-authoritative  
- must pass through reconciliation to become structural  

---

# 10. Staleness and Supersession

## 10.1 No Freshness Guarantee

Discovered or fetched Areas may be:

- outdated  
- incomplete  
- superseded  

---

## 10.2 Stale Handling

Handled through:

- re-fetching  
- reconciliation  
- explicit investigation  

No automatic rebasing or mutation occurs.

---

## 10.3 Convergence Model

> Convergence occurs through explicit reconciliation, not synchronization.

---

# 11. Relationship to Federation

## 11.1 Federation as Structured Discoverability

Federation is:

- repeated acquisition of foreign graphs  
- governed by reconciliation  

It is not:

- synchronization  
- shared authority  
- implicit trust  

---

## 11.2 Federation Flow

CRS → Discoverability → Reconciliation → Local Structure or CDS

---

# 12. Relationship to CCS and Commit Store

## 12.1 CCS

- defines commit structure  
- ensures identity and integrity  

## 12.2 Commit Store

- persists local and foreign commits  
- does not define discoverability behavior  

---

# 13. Invariants

- Discoverability must not imply legitimacy  
- Discoverability must not imply freshness  
- Discoverability must not imply completeness  
- Foreign graphs must remain immutable in isolation  
- All integration must occur through reconciliation  
- Provisional and durable references must remain distinct  
- Area references must not imply resolution adoption  
- No automatic synchronization may occur  
- History must never be rewritten  

**Critical Structural Invariant:**

- If a dependency cannot be discovered, it cannot be modeled  
- If it cannot be modeled, it cannot be simulated or aligned  

---

# 14. Mental Model

- Areas are **partial graph holders**  
- Discoverability is **visibility + acquisition**  
- Query precedes fetch  
- Imported graphs are **foreign and isolated**  
- Reconciliation is **the gateway to use**  
- CDS enables **investigation and simulation**  
- References are **provisional until admitted**  
- Structure evolves through **explicit decisions**  

---

# 15. Final Principle

Charter does not assume shared reality.

It enables systems to:

- discover each other  
- observe partial truth  
- investigate and simulate possibilities  
- and reconcile explicitly  

without requiring:

- implicit trust  
- global synchronization  
- or rewritten history