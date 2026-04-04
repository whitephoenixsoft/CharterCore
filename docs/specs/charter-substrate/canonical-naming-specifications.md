# Charter — Canonical Naming Specification (Revised)

Status: FOUNDATIONAL (LOCKED)  
Applies to: All Charter layers, specifications, libraries, and documentation  
Purpose: Establish stable, non-ambiguous naming for Charter architecture independent of versioning  

---

# 1. Purpose

This document defines the canonical naming system for Charter.

It exists to:

- eliminate ambiguity across evolving versions (V1–V7+)
- prevent architectural drift caused by renaming
- decouple version history from system structure
- ensure consistent terminology across:
  - specifications
  - codebases
  - CLI
  - documentation
  - future integrations

---

# 2. Core Rule

## CN-01 — Architecture Names Are Stable

Architecture names describe what the system is.  
They MUST remain stable across versions.

---

## CN-02 — Version Numbers Are Historical

Version numbers (V1, V2, …) describe when capabilities were introduced, not what they are.

They MUST NOT be used as architectural identifiers.

---

## CN-03 — No Cross-Substitution

Architecture names and version numbers MUST NOT be used interchangeably.

Example (invalid):

- “V6 handles commits”

Example (valid):

- “The Charter Commit System handles commits (introduced in V6)”

---

# 3. Canonical Architecture Modules

The following names are authoritative and permanent.

---

## 3.1 Legitimacy Engine

Canonical Name: **Legitimacy Engine**

Responsibility:
- Sessions
- Authority evaluation
- Candidate evaluation
- Resolution creation
- Legitimacy receipts

Key Property:
- Only component that creates legitimacy

---

## 3.2 Persistence Layer

Canonical Name: **Persistence Layer**

Responsibility:
- Object store (immutable)
- Ref store
- Audit store
- Low-level storage primitives (including commit persistence)

Key Property:
- Append-only storage substrate
- Does not define artifact semantics

---

## 3.3 Runtime Layer

Canonical Name: **Runtime Layer**

Responsibility:
- Context isolation
- Session orchestration
- Review orchestration (Baseline, Federation, Archive, etc.)
- Import/export coordination
- Engine invocation

Key Property:
- Orchestrates workflows but does not create legitimacy

---

## 3.4 Charter Commit System (CCS)

Canonical Name: **Charter Commit System (CCS)**

Responsibility:
- Definition of commit artifacts
- Commit taxonomy (Resolution, Receipt, Review, Exploration, Signal, etc.)
- Artifact identity (UUID-based)

Key Property:
- Defines what a commit is, not where it is stored

---

## 3.5 Charter Commit Store

Canonical Name: **Charter Commit Store**

Responsibility:
- Local append-only storage of commits
- Commit indexing and retrieval

Key Property:
- Source of local truth artifacts

---

## 3.6 Charter Structural Graph (CSG)

Canonical Name: **Charter Structural Graph (CSG)**

Responsibility:
- Construction of the local admitted DAG
- Node and edge representation
- Supersession relationships
- Cross-area and cross-resolution references
- Historical and active graph views

Key Property:
- Pure structural representation of commit relationships

Constraints:
- Does NOT define identity
- Does NOT define scope or purpose
- Does NOT interpret meaning
- Does NOT compute alignment

---

## 3.7 Charter Identity Substrate (CIS)

Canonical Name: **Charter Identity Substrate (CIS)**

Responsibility:
- Identity declaration (human-defined)
- Scope binding via resolutions
- Identity versioning
- Deprecation and sunset states
- Identity continuity over time

Key Property:
- Preserves identity and scope evolution over the graph

Notes:
- Consumes CSG
- Does not modify graph structure
- Does not compute alignment

---

## 3.8 Charter Care Substrate (CCare)

Canonical Name: **Charter Care Substrate (CCare)**

Responsibility:
- Check-ins (alignment observations)
- Requests (non-coercive invitations)
- Supportability signals
- Silence as a first-class state

Key Property:
- Observational, descriptive, non-authoritative

---

## 3.9 Charter Alignment System (CAS)

Canonical Name: **Charter Alignment System (CAS)**

Responsibility:
- Derived alignment computation over:
  - structural graph (CSG)
  - signals (CCare)
  - optionally identity partitions (CIS)
- Alignment state modeling
- Drift, tension, and capacity analysis
- Semantic lattice evaluation

Key Property:
- Query-based, descriptive, non-authoritative
- Produces derived state only

Notes:
- Does not create legitimacy
- Does not enforce behavior
- Identity is optional for core computation

---

## 3.10 Charter Guidance Layer (CGL)

Canonical Name: **Charter Guidance Layer (CGL)**

Responsibility:
- Interpretation (exegesis)
- Summaries and narratives
- Alignment explanation
- Structural and semantic translation for humans

Key Property:
- Read-only, non-authoritative

---

## 3.11 Charter Relay System (CRS)

Canonical Name: **Charter Relay System (CRS)**

Responsibility:
- Transport of commit artifacts
- Append-only archival endpoints
- Push/fetch operations

Key Property:
- Opaque transport layer
- No interpretation, no legitimacy, no reconstruction

---

# 4. Cross-Layer Relationships

## CN-04 — Directionality

Dependencies flow upward:

Legitimacy Engine  
↑  
Persistence Layer  
↑  
Runtime Layer  
↑  
CCS  
↑  
Commit Store  
↑  
CSG  
↑  
CIS / CCare  
↑  
CAS  
↑  
CGL  

CRS operates orthogonally as a transport layer.

---

## CN-05 — Separation of Concerns

- Legitimacy Engine → creates legitimacy  
- Runtime → orchestrates workflows  
- CCS → defines commit structure  
- Commit Store → stores commits  
- CSG → defines structural relationships  
- CIS → defines identity and scope  
- CCare → records observational signals  
- CAS → computes alignment  
- CGL → interprets  
- CRS → transports  

No layer may assume another layer’s responsibility.

---

# 5. Mapping Legacy Concepts

| Legacy Concept | Canonical Module |
|----------------|------------------|
| CLL (Lineage Substrate) | CSG + CIS |
| VLS concepts | CIS |
| VDS concepts | CCare + CAS |
| CAE | CAS |

This table is informational only.  
Future documents MUST use canonical names.

---

# 6. Naming Rules

## CN-06 — Use Canonical Names in All Specs

All new documents MUST use:

- Legitimacy Engine  
- Runtime Layer  
- Persistence Layer  
- Charter Commit System (CCS)  
- Charter Commit Store  
- Charter Structural Graph (CSG)  
- Charter Identity Substrate (CIS)  
- Charter Care Substrate (CCare)  
- Charter Alignment System (CAS)  
- Charter Guidance Layer (CGL)  
- Charter Relay System (CRS)  

---

## CN-07 — Avoid Abbreviations in First Use

First reference must use full name.

---

## CN-08 — Do Not Reintroduce Version Labels

Prohibited patterns:

- “V6 commits”
- “V5 alignment”
- “V7 relay layer”

Allowed pattern:

- “CCS (introduced in V6)”

---

# 7. Mental Model

Charter is a layered system composed of:

- a legitimacy kernel  
- a storage substrate  
- a workflow orchestrator  
- a commit definition layer  
- a local truth store  
- a structural graph layer  
- an identity substrate  
- a care substrate  
- a derived alignment system  
- a read-only interpretation layer  
- an external transport system  

Each module is:

- independent  
- composable  
- auditable  

Version numbers describe evolution.  
Canonical names describe structure.

---

# 8. Final Principle

If naming becomes ambiguous, architecture will drift.

This document exists to ensure:

- clarity across time  
- consistency across implementations  
- survivability across forks  

Canonical names are part of Charter’s long-term integrity.