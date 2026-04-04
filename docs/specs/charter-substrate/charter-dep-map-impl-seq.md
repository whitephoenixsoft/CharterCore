# Charter Platform — Dependency Map & Implementation Sequence

Status: DRAFT (Planning Canon)  
Depends On: Canonical Naming Specification, Module Boundaries & Non-Responsibilities Specification  
Purpose: Define dependency direction, composition rules, and phased implementation order for the Charter platform ecosystem  

---

# 1. Purpose

This document defines:

- the dependency structure of the Charter platform
- which modules may depend on which other modules
- the required implementation order
- how substrate modules compose into higher-level systems

It exists to ensure the platform evolves by:

- layering
- composition
- explicit contracts

and not by:

- hidden coupling
- responsibility collapse
- premature product fusion

---

# 2. Architectural Goal

The Charter platform is a **modular substrate ecosystem**.

It is not a monolith.

Its design goal is to make each major module:

- independently useful
- independently testable
- independently embeddable
- composable into larger systems

The platform is organized into three domains:

1. Charter Substrate  
2. Charter-Derived Meaning Substrates  
3. System-Level Products  

---

# 3. Domain Model

## 3.1 Charter Substrate

The Charter Substrate is the foundation of explicit truth.

It includes:

- Legitimacy Engine  
- Persistence Layer (Runtime Store)  
- Runtime Layer  
- Charter Commit System (CCS)  
- Charter Commit Store  
- Charter Relay System (CRS)  

This domain owns:

- legitimacy  
- runtime operational state  
- append-only artifact storage  
- orchestration  
- commit artifact structure  
- local artifact persistence  
- transport of opaque artifacts  

### Critical Distinction

The Charter Substrate contains **two distinct storage systems**:

#### Runtime Store (Persistence Layer)
- authoritative for live operational legitimacy state  
- mutable within controlled boundaries  
- optimized for active workflows  

#### Commit Store
- authoritative for append-only artifact history  
- immutable  
- optimized for durability, retrieval, and transport  

These stores MUST remain separate.

---

## 3.2 Charter-Derived Meaning Substrates

These modules compute or preserve meaning without creating authority.

They include:

- Charter Structural Graph (CSG)  
- Charter Identity Substrate (CIS)  
- Charter Care Substrate (CCare)  
- Charter Alignment System (CAS)  
- Charter Guidance Layer (CGL)  

This domain owns:

- structural graph reconstruction  
- identity, scope, and version lineage  
- human-first check-ins and requests  
- alignment computation  
- read-only interpretation  

---

## 3.3 System-Level Products

These are software-facing products built on the substrate libraries.

They include:

- system-level VDS  
- system-level VLS  
- telemetry adapters  
- deployment/posture integrations  
- product-facing CLIs, services, or agents  

This domain owns:

- external telemetry integration  
- software-system monitoring workflows  
- product-specific orchestration  
- deployment and environment adapters  

This domain does not redefine substrate semantics.

---

# 4. Dependency Law

## DM-01 — Dependencies Flow Upward

Modules may depend only on modules beneath them in the dependency graph.

Lower modules must never depend on higher modules.

---

## DM-02 — Authority Never Flows Upward

No higher module may modify the legitimacy semantics of a lower module.

No derived module may create legitimacy, identity truth, or obligation.

---

## DM-03 — Transport Is Orthogonal

The Charter Relay System operates orthogonally to local storage and computation.

It may transport artifacts created elsewhere, but it does not participate in their meaning.

---

## DM-04 — Product Layers Must Use Substrate Contracts

System-level products may extend substrate behavior through adapters and integrations.

They must not fork or silently redefine substrate invariants.

---

# 5. Canonical Dependency Map

## 5.1 Logical Order

The platform dependency order is:

Persistence Layer (Runtime Store)  
→ Legitimacy Engine  
→ Runtime Layer  
→ Charter Commit System (CCS)  
→ Charter Commit Store  
→ Charter Structural Graph (CSG)  
→ Charter Identity Substrate (CIS) / Charter Care Substrate (CCare)  
→ Charter Alignment System (CAS)  
→ Charter Guidance Layer (CGL)  

Charter Relay System (CRS) is orthogonal and depends on:

- CCS  
- Persistence primitives  

---

## 5.2 Dependency Table

| Module | May Depend On | Must NOT Depend On |
|--------|----------------|--------------------|
| Persistence Layer | none | all higher modules |
| Legitimacy Engine | none (or abstract persistence contracts only) | Runtime, CCS, CSG, CIS, CCare, CAS, CGL, CRS |
| Runtime Layer | Legitimacy Engine, Persistence Layer | CSG, CIS, CCare, CAS, CGL |
| CCS | Runtime outputs, Persistence abstractions | CSG, CIS, CCare, CAS, CGL |
| Commit Store | CCS, Persistence Layer | Runtime semantics, CAS, CGL |
| CSG | CCS, Commit Store | CIS, CCare, CAS, CGL |
| CIS | CSG, CCS, Commit Store | CCare, CAS, CGL |
| CCare | CCS, Commit Store, Runtime artifacts | CSG mutation, CIS, CAS, CGL |
| CAS | CSG, CCS, Commit Store, CCare, (optional CIS) | CGL, CRS |
| CGL | CAS, CCare, CIS, CSG, CCS, Commit Store | CRS execution semantics |
| CRS | CCS, Persistence primitives | Runtime, CSG, CIS, CCare, CAS, CGL |

---

# 6. Composition Rules

## 6.1 Legitimacy Composition

Legitimacy is produced only by:

- Legitimacy Engine  
- under Runtime orchestration  

---

## 6.2 Runtime ↔ Commit Boundary

The Runtime does NOT sync bidirectionally with the Commit Store.

Instead:

### Runtime → Commit Store
- Runtime publishes artifacts via CCS  
- CCS produces commit artifacts  
- Commit Store appends them  

### Commit Store → Runtime
Only through explicit, bounded paths:

- restore  
- review workflows (Baseline, Federation, Archive, etc.)  
- import  
- archive inspection  

No implicit synchronization is allowed.

---

## 6.3 Commit Composition

Commits are:

- defined by CCS  
- stored by Commit Store  
- transported by CRS  

They are:

- immutable  
- append-only  
- not equivalent to runtime state  

---

## 6.4 Archive Composition

Archive is a **Commit Store responsibility**, not Runtime.

Archive:

- changes storage posture, not meaning  
- preserves artifacts durably  
- reduces active runtime surface without deleting truth  

Archive may:

- package artifacts into archive bundles  
- mark artifacts as archived  
- support retrieval without reactivating runtime state  
- be exported or relayed  

Archive must NOT:

- delete legitimacy history  
- rewrite lineage  
- imply semantic removal  

---

## 6.5 Export Composition

Exports are:

- explicit packages of commit artifacts  
- derived from Commit Store  

Exports may:

- be stored as commit artifacts  
- be archived  
- be pushed to Relay  
- be re-imported through review workflows  

Exports must not bypass:

- legitimacy boundaries  
- review boundaries  
- authority validation  

---

## 6.6 Graph Composition (NEW)

CSG produces:

- the admitted local DAG  
- structural relationships  
- supersession paths  
- cross-area references  

It is:

- purely structural  
- independent of identity and alignment  

---

## 6.7 Identity Composition (NEW)

CIS produces:

- identity declarations  
- scope bindings  
- version transitions  
- deprecation and sunset states  

It consumes:

- structural graph (CSG)  
- commit artifacts  

Identity is:

- human-declared  
- structurally grounded  
- independent of runtime behavior  

---

## 6.8 Care Composition

CCare produces:

- check-ins  
- requests  
- supportability signals  
- silence records  

Stored as commit artifacts.

---

## 6.9 Alignment Composition

CAS consumes:

- structural graph (CSG)  
- care signals (CCare)  
- commit artifacts  
- optionally identity partitions (CIS)  

Produces:

- derived alignment state (rebuildable)  

---

## 6.10 Guidance Composition

CGL is read-only.

Produces:

- explanations  
- summaries  
- interpretation  

---

# 7. Storage Model Summary

| Store | Role | Authority |
|------|------|----------|
| Runtime Store | live operational state | legitimacy execution |
| Commit Store | immutable artifact history | artifact truth |
| Relay | remote artifact transport | none |

---

# 8. Implementation Strategy

## 8.1 Principle

Build by **module completion**, not version labels.

---

# 9. Phased Implementation Sequence

## Phase 1 — Legitimacy Core

- Legitimacy Engine  

---

## Phase 2 — Runtime + Persistence

- Runtime Layer  
- Runtime Store  

---

## Phase 3 — Commit Substrate + Archive

- CCS  
- Commit Store  
- Archive model  
- Export model  

Focus:

- commit taxonomy  
- artifact identity  
- append-only storage  
- archive posture  
- export packaging  
- runtime → commit publishing  

---

## Phase 4 — Relay

- CRS  

Includes:

- commit transport  
- archive bundle transport  
- export transport  

---

## Phase 5 — Structural Graph

- Charter Structural Graph (CSG)  

---

## Phase 6 — Identity Substrate

- Charter Identity Substrate (CIS)  

---

## Phase 7 — Care Substrate

- Charter Care Substrate (CCare)  

---

## Phase 8 — Alignment System

- Charter Alignment System (CAS)  

---

## Phase 9 — Guidance Layer

- Charter Guidance Layer (CGL)  

---

# 10. Milestones

## Milestone B — Charter Artifact System Exists

Includes:

- CCS  
- Commit Store  
- Archive capability  
- Export capability  
- CRS  

Outcome:

- full artifact lifecycle exists:
  - creation  
  - storage  
  - archiving  
  - transport  
  - retrieval  

---

# 11. Build Order Guidance

## IM-04 — Do Not Collapse Runtime and Commit Store

Runtime Store and Commit Store must remain:

- physically separable  
- logically distinct  
- independently evolvable  

---

# 12. Final Principle

The platform must be built in the same order that meaning emerges:

- legitimacy  
- orchestration  
- artifact truth  
- structural graph  
- identity  
- care  
- alignment  
- interpretation  
- product integration  

Truth is created once,  
stored immutably,  
interpreted later,  
and never rewritten.