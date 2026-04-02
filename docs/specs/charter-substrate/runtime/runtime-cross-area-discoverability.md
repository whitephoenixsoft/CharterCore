# Charter Runtime — Cross-Area Discoverability Specification

Status: FOUNDATIONAL  
Applies to: Runtime Layer, Charter Commit System (CCS), Commit Store, Baseline Review, Federation  
Does NOT define: transport protocols (CRS), legitimacy semantics, alignment computation, or guidance behavior  

---

# 1. Purpose

This document defines how **external Areas and their associated graphs become discoverable, accessible, and referenceable** within the Charter Runtime.

Discoverability exists to:

- make external Areas **identifiable**
- enable acquisition of **foreign partial graphs**
- support **review and integration workflows**
- allow **safe cross-area referencing**

Discoverability does not imply:

- trust  
- freshness  
- completeness  
- legitimacy  

---

# 2. Core Principle

> Discoverability provides access to foreign structure.  
> It does not grant authority, correctness, or integration.

A discovered Area is:

- visible  
- fetchable  
- inspectable  

It is not:

- locally valid  
- up-to-date  
- structurally integrated  

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

- different runtimes may hold different subsets of the graph  
- histories may diverge  
- snapshots may be stale  

> Global consistency is not required.  
> Cohesion is achieved through explicit review and integration.

---

# 4. Discoverability Capabilities

## 4.1 Identification

The Runtime must support identifying external Areas via:

- relay sources (CRS)  
- local registries  
- previously imported artifacts  
- manual references  

---

## 4.2 Acquisition (Fetch / Pull)

The Runtime must support acquiring a **foreign partial graph**:

- full Area snapshot  
- filtered subset  
- commit bundle  

Acquisition produces a **foreign, read-only graph representation**.

---

## 4.3 Local Availability

Once acquired, the foreign graph becomes:

- locally accessible  
- referenceable within controlled contexts  
- available for review workflows  

---

## 4.4 No Implicit Synchronization

Discoverability does NOT:

- synchronize Areas automatically  
- maintain continuous consistency  
- update local copies without explicit action  

All updates require explicit acquisition.

---

# 5. Isolation of Foreign Graphs

## 5.1 Isolated Storage

Acquired foreign graphs SHOULD be stored in an **isolated, read-only environment**.

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

# 6. Relationship to Review

## 6.1 Review as Integration Boundary

All integration of foreign graph data MUST occur through a **review construct**.

This includes:

- Baseline Review  
- Federation Review  
- Archive Review  
- Deliberate Review  

---

## 6.2 Unified Review Model

All reviews share:

- a **workflow decision rule**  
- a **workflow scope**  

Examples:

| Review Type | Governing Rule | Governing Scope |
|-------------|--------------|----------------|
| Baseline Review | Area authority | Area scope |
| Federation Review | Federation-defined | Cross-area scope |
| Deliberate Review | Deliberate rules | Epic boundary |

---

## 6.3 Discoverability Does Not Bypass Review

Discovered or fetched artifacts:

- must not become locally integrated automatically  
- must pass through review before structural adoption  

---

# 7. Reference Model

## 7.1 Reference Targets

Cross-area references may target:

- **Areas** (boundary-level reference)  
- **Resolutions** (artifact-level reference)  

These represent fundamentally different types of relationships.

---

## 7.2 Area References (Boundary-Level)

An Area reference indicates:

- structural relevance of another Area  
- dependency, coordination, or relationship at the boundary level  
- awareness of another graph domain  

Area references:

- do NOT reference any specific resolution  
- do NOT imply adoption of decisions  
- do NOT imply authority or trust  
- do NOT imply completeness or freshness  

> Area references establish that a boundary matters,  
> not what inside that boundary is accepted.

---

## 7.3 Resolution References (Artifact-Level)

A Resolution reference indicates:

- relevance of a specific accepted artifact  
- a concrete decision anchor  
- a precise dependency or alignment point  

Resolution references are:

- more specific  
- stronger than area references  
- suitable for structural integration after review  

---

## 7.4 Provisional vs Integrated References

### A. Provisional References

- created against isolated foreign graphs  
- may target Areas or Resolutions  
- used during review and inspection  
- non-authoritative  
- non-durable  

---

### B. Integrated References

- created after successful review  
- may target Areas or Resolutions  
- durable within local graph  
- visible to lineage and alignment systems  

---

## 7.5 Core Rule

> Unreviewed foreign artifacts may be referenced provisionally,  
> but must not become durable structural anchors.

---

## 7.6 Strength of References

- Area references = structural / boundary-level (weaker)  
- Resolution references = artifact / decision-level (stronger)  

Systems must not treat Area references as implicit Resolution references.

---

# 8. Staleness and Supersession

## 8.1 No Freshness Guarantee

Discovered or fetched Areas may be:

- outdated  
- incomplete  
- superseded  

The system must not assume freshness.

---

## 8.2 Stale Reference Handling

Staleness is resolved through:

- re-fetching / updating  
- review workflows  
- explicit reconciliation  

No automatic rebasing or mutation occurs.

---

## 8.3 Convergence Model

Different runtimes may hold different histories.

> Convergence occurs through explicit review and integration,  
> not through enforced synchronization.

---

# 9. Relationship to Federation

## 9.1 Federation as Extension of Review

Federation is:

- repeated or structured acquisition of foreign graphs  
- governed by the same review principles  

It is not:

- automatic synchronization  
- shared authority  
- implicit trust  

---

## 9.2 Federation References

Federation may:

- create provisional references during review  
- create durable references only after review  

---

# 10. Relationship to Commit Store and CCS

## 10.1 CCS

- defines commit structure  
- preserves identity and integrity  

## 10.2 Commit Store

- stores both local and imported commits  
- maintains immutability  
- does not enforce discoverability  

---

# 11. Invariants

- Discoverability must not imply legitimacy  
- Discoverability must not imply freshness  
- Foreign graphs must remain immutable in isolation  
- Integration must occur only through review  
- Provisional and durable references must remain distinct  
- Area references must not imply resolution adoption  
- No automatic synchronization may occur  
- History must never be rewritten  

---

# 12. Mental Model

- Areas are **partial graph holders**  
- Discoverability is **finding and fetching graphs**  
- Imported graphs are **foreign and isolated**  
- Review is **the gateway to integration**  
- References are **provisional until accepted**  
- Area references express **structural awareness**  
- Resolution references express **decision alignment**  
- Cohesion is **achieved explicitly, not assumed**  

---

# 13. Final Principle

Charter does not assume shared reality.

It enables systems to:

- discover each other  
- exchange partial truth  
- and reconcile explicitly  

without ever requiring:

- implicit trust  
- global synchronization  
- or rewritten history