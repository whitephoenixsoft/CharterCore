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

Notes:
- Derived from V1/V2 concepts
- Fully deterministic
- Storage-agnostic

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

Canonical Name: **Runtime Layer (CLI Runtime)**

Responsibility:
- Context isolation
- Session orchestration
- Baseline review
- Import/export coordination
- Engine invocation

Key Property:
- Orchestrates workflow but does not create legitimacy

---

## 3.4 Charter Commit System (CCS)

Canonical Name: **Charter Commit System (CCS)**

Responsibility:
- Definition of commit artifacts
- Commit taxonomy (Resolution, Receipt, Review, Exploration, Signal, etc.)
- Artifact identity (UUID-based)
- Commit-level lineage rules

Key Property:
- Defines *what a commit is*, not where it is stored

Notes:
- Logical layer, not a storage system
- Previously referred to as “V6”

---

## 3.5 Charter Commit Store

Canonical Name: **Charter Commit Store**

Responsibility:
- Local append-only storage of commits
- Commit indexing and retrieval
- Local lineage materialization inputs

Key Property:
- Source of local truth artifacts
- Storage implementation of CCS artifacts

Notes:
- Distinct from CCS (which defines structure)
- Distinct from Relay (which transports)

---

## 3.6 Charter Care Substrate (CCare)

Canonical Name: **Charter Care Substrate**

Responsibility:
- Check-ins (alignment observations)
- Requests (non-coercive invitations)
- Supportability signals
- Silence as a first-class state

Key Property:
- Observational, descriptive, non-authoritative

Notes:
- Human-first caregiving model
- Foundational layer for future VDS systems
- Previously referred to as “Charter VDS”

---

## 3.7 Charter Lineage Substrate (CLL)

Canonical Name: **Charter Lineage Substrate**

Responsibility:
- Identity definition
- Scope and purpose tracking
- Versioning (mechanically derived)
- Deprecation and sunset states
- Lineage continuity

Key Property:
- Preserves identity and intent over time
- Does not observe behavior

Notes:
- Foundational layer for future VLS systems
- Previously referred to as “Charter VLS”

---

## 3.8 Charter Alignment Engine (CAE)

Canonical Name: **Charter Alignment Engine (CAE)**

Responsibility:
- Derived computation over:
  - commit lineage
  - identity structure (CLL)
  - signals/check-ins (CCare)
- Drift, variance, and propagation analysis
- Structural and predictive alignment state

Key Property:
- Deterministic and rebuildable
- Produces derived state only

Notes:
- Consumes CCS + Commit Store + CCare + CLL
- Does not create legitimacy or authority

---

## 3.9 Charter Guidance Layer (CGL)

Canonical Name: **Charter Guidance Layer (CGL)**

Responsibility:
- Interpretation (exegesis)
- Summaries and narratives
- Alignment explanation
- Drift and tension description

Key Property:
- Read-only, non-authoritative

Notes:
- Consumes CAE and all underlying artifacts
- Does not compute alignment itself

---

## 3.10 Charter Relay System (CRS)

Canonical Name: **Charter Relay System (CRS)**

Responsibility:
- Transport of commit artifacts
- Append-only archival endpoints
- Push/fetch operations

Key Property:
- Opaque transport layer
- No interpretation, no legitimacy, no state reconstruction

Notes:
- Stores commits without meaning
- External to local Commit Store

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
CCare / CLL  
↑  
CAE  
↑  
CGL  

CRS operates orthogonally as a transport layer.

---

## CN-05 — Separation of Concerns

- Legitimacy Engine → creates legitimacy  
- Runtime → orchestrates workflows  
- CCS → defines commit structure  
- Commit Store → stores local commits  
- CCare → records observational signals  
- CLL → preserves identity and intent  
- CAE → computes derived alignment  
- CGL → interprets  
- CRS → transports  

No layer may assume another layer’s responsibility.

---

# 5. Mapping Legacy Version Terms

For historical reference only:

| Legacy Version | Canonical Module |
|----------------|------------------|
| V1–V2 | Legitimacy Engine |
| V1–V4 | Runtime Layer |
| V5 (alignment substrate emergence) | CAE + CCare + CLL |
| V6 (commits) | CCS + Commit Store |
| V6+ (guidance) | CGL |
| V7 | CRS |

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
- Charter Care Substrate (CCare)  
- Charter Lineage Substrate (CLL)  
- Charter Alignment Engine (CAE)  
- Charter Guidance Layer (CGL)  
- Charter Relay System (CRS)  

---

## CN-07 — Avoid Abbreviations in First Use

First reference in any document must use full name:

Example:

“Charter Commit System (CCS)”

Subsequent references may use abbreviation.

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
- a care substrate  
- a lineage substrate  
- a derived alignment engine  
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