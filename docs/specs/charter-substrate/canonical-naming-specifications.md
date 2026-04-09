# Charter — Canonical Naming Specification (Revised vNext)

Status: FOUNDATIONAL (LOCKED)  
Applies to: All Charter layers, specifications, libraries, and documentation  
Purpose: Establish stable, non-ambiguous naming for Charter architecture independent of versioning  

---

# 1. Purpose

This document defines the canonical naming system for Charter.

It exists to:

- eliminate ambiguity across evolving versions  
- prevent architectural drift caused by renaming  
- decouple version history from system structure  
- ensure consistent terminology across:
  - specifications  
  - codebases  
  - CLI  
  - documentation  
  - integrations  

---

# 2. Core Rules

## CN-01 — Architecture Names Are Stable

Architecture names describe what the system *is*.  
They MUST remain stable across versions.

---

## CN-02 — Versions Are Historical Only

Version numbers describe *when* capabilities were introduced, not *what they are*.

They MUST NOT be used as architectural identifiers.

---

## CN-03 — No Cross-Substitution

Architecture names and versions MUST NOT be interchangeable.

---

# 3. Canonical Architecture Modules

---

## 3.1 Runtime Layer

Canonical Name: **Runtime Layer**

Responsibility:

- orchestration of all workflows  
- CDS lifecycle management  
- Review Layer orchestration  
- federation (discover, acquire, integrate, emit)  
- session invocation  

Key Property:

> Orchestrates everything. Creates nothing authoritative.

---

## 3.2 Review Layer (Runtime Subsystem)

Canonical Name: **Review Layer**

Variants:

- **Foreign Integration Review**  
- **Reconciliation Review**

Responsibility:

- integration boundary to legitimacy  
- synchronization between CDS and legitimacy  
- admission of structure  

Key Property:

> Legitimacy firewall and synchronization layer.

---

## 3.3 Legitimacy Engine

Canonical Name: **Legitimacy Engine**

Responsibility:

- sessions  
- authority evaluation  
- candidate evaluation  
- resolution creation  
- legitimacy receipts  

Key Property:

> The only component that creates legitimacy.

---

## 3.4 Charter Commit System (CCS)

Canonical Name: **Charter Commit System (CCS)**

Responsibility:

- commit envelope definition  
- commit taxonomy (resolution, receipt, signal, etc.)  
- identity + integrity  

Key Property:

> Defines artifact structure, not meaning.

---

## 3.5 Charter Commit Store

Canonical Name: **Charter Commit Store**

Responsibility:

- append-only commit storage  
- indexing and retrieval  

Key Property:

> Durable artifact preservation.

---

## 3.6 Charter Structural Graph (CSG)

Canonical Name: **Charter Structural Graph (CSG)**

Responsibility:

- structural DAG construction  
- node types:
  - resolution nodes  
  - item nodes  
- edges:
  - supersession  
  - reference  
  - derivation  
- projection system:
  - resolution  
  - item  
  - mixed  

Key Property:

> Explicit structure only. No inference.

---

## 3.7 Charter Identity Substrate (CIS)

Canonical Name: **Charter Identity Substrate (CIS)**

Responsibility:

- identity declaration  
- scope definition  
- bounded membership over CSG  
- identity versioning  

Key Property:

> Identity is declared and bounded.

---

## 3.8 Charter Deliberate Substrate (CDS)

Canonical Name: **Charter Deliberate Substrate (CDS)**

Responsibility:

- investigation and thinking  
- Items (universal unit of thought)  
- observations and synthesis  
- simulation over structure  
- subscriptions to signals  

Key Property:

> Thinking without authority.

---

## 3.9 Charter Care Substrate (CCare)

Canonical Name: **Charter Care Substrate (CCare)**

Responsibility:

- signals (state, confidence, time)  
- requests  
- supportability  
- silence  

Key Property:

> Observational, non-authoritative.

---

## 3.10 Charter Signal Processing Substrate (CSP)

Canonical Name: **Charter Signal Processing Substrate (CSP)**

Responsibility:

- signal shaping  
- clustering and aggregation  
- feed generation  
- emission policy control  

Key Property:

> Shapes signal flow without creating meaning.

---

## 3.11 Charter Alignment System (CAS)

Canonical Name: **Charter Alignment System (CAS)**

Responsibility:

- alignment computation over:
  - CSG projections  
  - CCare signals (optionally CSP-shaped)  
  - CIS scopes (optional)  
- drift, tension, propagation  
- semantic lattice  

Key Property:

> Computes alignment. Does not interpret or enforce.

---

## 3.12 Charter Query Language (CQL)

Canonical Name: **Charter Query Language (CQL)**

Responsibility:

- unified query interface across:
  - CSG  
  - CDS  
  - CAS  
  - CIS  
- projection-aware queries  
- cross-substrate composition  

Key Property:

> Reveals system state without modifying it.

---

## 3.13 Charter Guidance Layer (CGL)

Canonical Name: **Charter Guidance Layer (CGL)**

Responsibility:

- interpretation  
- explanation  
- narrative generation  

Key Property:

> Explains. Does not decide.

---

## 3.14 Charter Relay System (CRS)

Canonical Name: **Charter Relay System (CRS)**

Responsibility:

- transport of commits  
- push/fetch operations  

Key Property:

> Opaque transport only.

## 3.15 Charter Resolution Recontextualization (CRR)

Canonical Name: **Charter Resolution Recontextualization (CRR)**

Responsibility:

- defines structural recontextualization patterns:
  - promotion (abstract → broader scope)  
  - demotion (abstract → more specific decomposition)  
  - copy (parallel reuse)  
  - move (recontextualized replacement)  

- ensures all recontextualization is expressed as:
  - new resolution artifacts  
  - explicit `derives_from` relationships  

Key Property:

> Recontextualization creates new structure without mutating existing artifacts.

---

## CRR Positioning

CRR is not a standalone substrate.

It operates across:

- Runtime (orchestration)  
- CCS (derivation recording)  
- CSG (structural lineage)  
- CDS (simulation of decomposition and restructuring)  

CRR defines how structure evolves across abstraction levels without rewriting history.

---

# 4. Cross-Layer Relationships

## CN-04 — Non-Linear Architecture

Charter is NOT a strict vertical stack.

It consists of:

- forward flows (creation → preservation → analysis)  
- reverse flows (reconciliation)  
- lateral flows (federation)  
- observational loops (signals → CDS → decisions → signals)  

---

## CN-05 — Separation of Responsibilities

- Runtime → orchestration  
- Review → boundary + synchronization  
- Engine → legitimacy creation  
- CCS → artifact definition  
- Store → persistence  
- CSG → structure  
- CIS → identity  
- CDS → thinking  
- CCare → observation  
- CSP → signal shaping  
- CAS → computation  
- CQL → query  
- CGL → interpretation  
- CRS → transport  

No module may assume another’s responsibility.

---

# 5. Legacy Mapping

| Legacy Concept | Canonical Module |
|----------------|------------------|
| CLL | CSG + CIS |
| CAE | CAS |
| VLS | CIS |
| VDS | CCare + CAS |
| Baseline Review | Foreign Integration Review |

---

# 6. Naming Rules

## CN-06 — Canonical Names Required

All specs MUST use canonical names.

---

## CN-07 — First Use Must Be Expanded

Example:

Charter Structural Graph (CSG)

---

## CN-08 — No Version-Based Naming

Prohibited:

- “V6 commits”  
- “V7 alignment”  

---

# 7. Mental Model

Charter is a system of:

- thinking (CDS)  
- deciding (Legitimacy Engine)  
- recording (CCS + Store)  
- structuring (CSG)  
- scoping (CIS)  
- observing (CCare + CSP)  
- computing (CAS)  
- querying (CQL)  
- explaining (CGL)  
- exchanging (CRS)  

---

# 8. Final Principle

If naming drifts, architecture collapses.

Canonical names ensure:

- clarity across time  
- consistency across systems  
- survivability across evolution  

> Names are part of the system’s integrity.