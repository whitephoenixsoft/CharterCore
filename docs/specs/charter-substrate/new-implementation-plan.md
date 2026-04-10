# Charter — Implementation Plan (v1)

Status: ACTIVE  
Intent: Guide phased implementation of Charter from core legitimacy to full substrate ecosystem  
Scope: Runtime, Legitimacy, CCS, CLI, and downstream substrates  
Principle: Build in the order that truth is created → preserved → understood → shared  

---

# 1. Core Architectural Principles (Locked)

## IP-01 — Runtime-First Operation
The system must function locally via:
- Runtime
- Legitimacy Engine
- Review / Reconciliation
- Local storage

CCS/Commit Store are introduced progressively.

---

## IP-02 — Legitimacy is the Only Authority Creator
- All decisions must pass through sessions
- No substrate may bypass this

---

## IP-03 — Dual Representation of Legitimacy

When a resolution is created:
- it exists in Runtime (operational truth)
- it is emitted as a CCS commit (durable artifact)

---

## IP-04 — Reconciliation is the Only Bridge
All movement between:
- CDS ↔ legitimacy
- foreign artifacts ↔ local system

must pass through Reconciliation Review.

---

## IP-05 — Substrates Are Charter-Native Libraries
- implemented as Rust crates
- independently usable
- designed to compose
- orchestrated via Runtime

---

## IP-06 — Commit Store Enables Interoperability, Not Local Thinking
- Runtime = operational system
- Commit Store = durable artifact system

---

# 2. Implementation Phases

---

# Phase 1 — Legitimacy Engine Core

Status: IN PROGRESS

## Goals
- deterministic, stateless engine
- session evaluation
- authority rules
- receipt generation

---

# Phase 2 — Runtime Orchestration (MVP)

## Goals
Define how the system operates locally.

## Capabilities
- session orchestration
- reconciliation review (forward + reverse)
- local storage model (transitional)
- import/export (flat + DAG)

---

# Phase 3 — CLI MVP (Reference Product)

## Goals
Validate system behavior through a real interface.

## Capabilities
- bootstrap (area, authority)
- decision flow (deliberate → review → session)
- reconciliation flow (imports + review)
- audit & inspection
- minimal CGL assistance

---

# Phase 4 — CCS (Commit System Integration)

## Goals
Introduce durable artifact representation.

## Capabilities
- commit creation (resolutions, receipts)
- identity + integrity
- reference structure

---

# Phase 5 — Commit Store (Durable Layer)

## Goals
Establish canonical artifact storage.

## Capabilities
- append-only storage
- indexing
- retrieval
- export bundles

---

# Phase 6 — CRS (Relay / Federation)

## Goals
Enable sharing across systems.

## Capabilities
- push/fetch commits
- isolated relay store

---

# Phase 7 — CSG (Structural Graph)

## Goals
Materialize relationships.

## Capabilities
- DAG construction
- derivation edges
- supersession

---

# Phase 8 — CCare + CSP

## Goals
Introduce signals and feeds.

## Capabilities
- signals (check-ins, supportability)
- subscription triggers

---

# Phase 9 — CAS (Alignment System)

## Goals
Compute system behavior.

## Capabilities
- alignment posture
- volatility
- propagation

---

# Phase 10 — CQL + CGL

## CQL
- unified query layer

## CGL
- explanation
- comparison
- synthesis support

---

# Phase 11 — CIS + CRR Expansion

## CIS
- identity modeling

## CRR
- decomposition
- synthesis
- abstraction tiers

---

# Phase 12 — Product Expansion

## VDS
- care-focused product

## VLS
- identity-focused product

---

# 3. Substrate Readiness Summary

Legitimacy Engine → Phase 1  
Runtime → Phase 2  
CLI → Phase 3  
CCS → Phase 4  
Commit Store → Phase 5  
CRS → Phase 6  
CSG → Phase 7  
CCare/CSP → Phase 8  
CAS → Phase 9  
CQL/CGL → Phase 10  
CIS/CRR → Phase 11  

---

# 4. Transitional vs Canonical

## Transitional
- runtime-local storage
- pre-commit durability

## Canonical
- commit-backed artifacts
- append-only history

---

# 5. Key Risks

- runtime leaking into CCS  
- temporary storage becoming permanent  
- UX bypassing legitimacy  

---

# 6. Final Principle

1. Create truth (Legitimacy)  
2. Orchestrate truth (Runtime)  
3. Use truth (CLI)  
4. Preserve truth (CCS + Store)  
5. Share truth (CRS)  
6. Understand truth (CAS + CGL)