# Charter Platform — Dependency Map & Implementation Sequence (Revised vNext)

Status: FOUNDATIONAL (Planning Canon)  
Depends On: Canonical Naming Specification  
Purpose: Define dependency direction, composition rules for the Charter platform ecosystem  

---

# 1. Purpose

This document defines:

- dependency structure across Charter modules  
- allowable dependency directions  
- implementation sequencing  
- composition rules across substrates  

It ensures the platform evolves through:

- layering  
- composition  
- explicit contracts  

and not:

- hidden coupling  
- responsibility collapse  
- implicit synchronization  

---

# 2. Architectural Goal

Charter is a **modular, composable substrate ecosystem**.

It is not a monolith or strict stack.

Each module must be:

- independently useful  
- independently testable  
- independently deployable  
- composable into larger systems  

---

# 3. Domain Model

---

## 3.1 Core Substrate Domain (Authority + Artifacts)

Includes:

- Runtime Layer  
- Review Layer  
- Legitimacy Engine  
- Charter Commit System (CCS)  
- Charter Commit Store  
- Charter Relay System (CRS)  

Owns:

- legitimacy creation  
- orchestration  
- artifact definition  
- artifact preservation  
- transport  

---

## 3.2 Investigation & Signal Domain

Includes:

- Charter Deliberate Substrate (CDS)  
- Charter Care Substrate (CCare)  
- Charter Signal Processing Substrate (CSP)  

Owns:

- thinking and simulation  
- observation  
- signal shaping  

---

## 3.3 Structural & Analytical Domain

Includes:

- Charter Structural Graph (CSG)  
- Charter Identity Substrate (CIS)  
- Charter Alignment System (CAS)  
- Charter Query Language (CQL)  
- Charter Guidance Layer (CGL)  

Owns:

- structure  
- identity  
- computation  
- query  
- interpretation  

---

## 3.4 Structural Evolution Model

Includes:

- Charter Resolution Recontextualization (CRR)  

Owns:

- promotion / demotion patterns  
- decomposition / synthesis lineage  
- abstraction transitions  

CRR is cross-cutting (not a standalone substrate).

---

# 4. Dependency Law

---

## DM-01 — Directional, Not Strictly Vertical

Dependencies must follow allowed directions:

- forward (creation → preservation → computation)  
- reverse (reconciliation)  
- lateral (federation)  

No cyclic authority dependencies are allowed.

---

## DM-02 — Authority Is Isolated

Only the **Legitimacy Engine** may create legitimacy.

No other module may:

- create legitimacy  
- reinterpret authority  
- enforce decisions  

---

## DM-03 — Structure Is Explicit

CSG depends only on:

- CCS  
- Commit Store  

It must never depend on:

- CIS  
- CAS  
- CDS  

---

## DM-04 — Query Is Read-Only

CQL may depend on:

- CSG  
- CDS  
- CAS  
- CIS  

It must not:

- mutate any substrate  
- introduce semantics  

---

## DM-05 — Transport Is Orthogonal

CRS:

- depends on CCS  
- transports commits  

It does not:

- interpret  
- compute  
- integrate  

---

## DM-06 — Runtime Is the Integration Point

Runtime may depend on:

- Legitimacy Engine  
- CDS  
- Review Layer  
- CCS  

Runtime orchestrates:

- federation  
- reconciliation  
- review  

---

# 5. Canonical Dependency Map

---

## 5.1 Core Dependencies

| Module | Depends On |
|--------|------------|
| Legitimacy Engine | none |
| Runtime Layer | Legitimacy Engine |
| Review Layer | Runtime |
| CCS | Runtime outputs |
| Commit Store | CCS |
| CRS | CCS |

---

## 5.2 Investigation & Signal

| Module | Depends On |
|--------|------------|
| CDS | Runtime |
| CCare | CCS |
| CSP | CCare |

---

## 5.3 Structure & Analysis

| Module | Depends On |
|--------|------------|
| CSG | CCS + Commit Store |
| CIS | CSG |
| CAS | CSG + CCare (+ optional CIS, CSP-shaped signals) |
| CQL | CSG + CDS + CAS + CIS |
| CGL | CQL (or CAS directly) |

---

## 5.4 CRR (Cross-Cutting)

CRR spans:

- Runtime (orchestration)  
- CCS (`derives_from`)  
- CSG (derivation edges)  
- CDS (simulation)  

---

# 6. Composition Rules

---

## 6.1 Legitimacy Composition

Legitimacy is produced only by:

- Legitimacy Engine  
- under Runtime orchestration  

---

## 6.2 CDS ↔ Legitimacy Boundary

Flow to legitimacy:

CDS → Review → Session → Resolution  

Reverse flow:

Resolution → Reconciliation → CDS Item  

No implicit synchronization.

---

## 6.3 Commit Composition

Commits are:

- defined by CCS  
- stored by Commit Store  
- transported by CRS  

---

## 6.4 Graph Composition

CSG produces:

- resolution nodes  
- item nodes  
- explicit edges  
- projection views  

---

## 6.5 Identity Composition

CIS:

- overlays identity onto structure  
- does not modify structure  

---

## 6.6 Signal Composition

CCare:

- produces signals  

CSP:

- shapes signal flow  

---

## 6.7 Alignment Composition

CAS:

- computes alignment over:
  - structure  
  - signals  
  - identity (optional)  

---

## 6.8 Query Composition

CQL:

- composes cross-substrate queries  
- enables projection-aware access  

---

## 6.9 Interpretation Composition

CGL:

- interprets query results  
- produces narratives  

---

# 7. Storage Model Summary

| Store | Role |
|------|------|
| Runtime Store | working state |
| CDS Workspace | investigative state |
| Commit Store | immutable artifacts |
| Relay | transport |

---

# 8. Implementation Strategy

---

## 8.1 Principle

Build by **capability layers**, not versions.

---

# 9. Final Principle

Charter must be built in the order that **understanding emerges**:

- thinking  
- deciding  
- recording  
- structuring  
- observing  
- computing  
- querying  
- explaining  

> Truth is created once,  
> explored continuously,  
> and never rewritten.