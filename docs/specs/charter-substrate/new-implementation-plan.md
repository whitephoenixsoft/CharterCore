# Charter — Implementation Plan (v2)

Status: ACTIVE  
Intent: Guide phased implementation of Charter from core legitimacy to full substrate ecosystem  
Scope: Runtime, Legitimacy, CCS, CLI, and all substrates  
Principle: Build in the order that truth is created → preserved → queried → understood → simulated → shared  

---

# 1. Core Architectural Principles (Locked)

## IP-01 — Runtime-First Operation
The system must function locally via:
- Runtime
- Legitimacy Engine
- Review / Reconciliation
- In-memory or host-provided persistence

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

## IP-06 — Commit-Centric Architecture
- All durable artifacts are CCS commits
- All cross-system exchange flows through commits
- CRS transports commits but does not interpret them

---

## IP-07 — CQL as the Universal Query Interface
- All system querying flows through CQL JSON IL
- CLI, Runtime, VDS, and VLS all use CQL

---

## IP-08 — Persistence is Host-Controlled
- Substrates define logical storage
- Hosts define physical storage
- Default stores are reference implementations only

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
- workspace + area model
- session orchestration
- reconciliation review (incoming)
- process lifecycle (pause, resume, block)
- in-memory execution
- optional persistence hooks (interfaces only)

---

# Phase 3 — CLI MVP (Reference Product)

## Goals
Validate system behavior through a real interface.

## Capabilities
- init (context + area)
- decision flow (deliberate → review → session)
- reconciliation flow
- status commands (via CQL internally)
- audit visibility
- single active process per area enforcement

---

# Phase 4 — CCS (Commit System)

## Goals
Introduce durable artifact representation.

## Capabilities
- commit schema foundation
- commit identity + integrity
- commit emission from:
  - sessions
  - review closure
- commit taxonomy (initial):
  - resolution
  - session receipt
  - review closure

---

# Phase 5 — Commit Store (Logical + Pluggable)

## Goals
Establish canonical artifact storage abstraction.

## Capabilities
- logical commit store interface
- reference implementation (append-only)
- indexing + retrieval
- export/import bundles
- host-provided storage adapter support

---

# Phase 6 — CQL (JSON IL First)

## Goals
Establish unified query interface early.

## Capabilities
- JSON IL definition
- runtime query execution
- CLI uses CQL for:
  - status
  - inspection
- domain query routing

---

# Phase 7 — CRS (Relay / Federation)

## Goals
Enable cross-system exchange.

## Capabilities
- push/fetch commits
- relay storage abstraction
- commit transport (no interpretation)
- basic filtering

---

# Phase 8 — CSG (Structural Graph)

## Goals
Materialize structural relationships.

## Capabilities
- DAG construction from commits
- supersession tracking
- derivation relationships
- partial graph federation support

---

# Phase 9 — CCare + CSP (Signal Foundation)

## Goals
Introduce signal system.

## Capabilities
- signal model (basic types)
- signal emission
- CSP pipelines (basic)
- feeds (subscription model)

---

# Phase 10 — Decision → Measurement Bridge

## Goals
Connect structure to observable behavior.

## Capabilities
- observable model
- relationship bindings:
  - decision → observable
  - observable → threshold
- observation items in CDS

---

# Phase 11 — CAS (Alignment System v1)

## Goals
Compute alignment and structural impact.

## Capabilities
- alignment calculation
- signal interpretation
- basic propagation across graph
- local computation only

---

# Phase 12 — Federation Expansion

## Goals
Strengthen commit-based exchange.

## Capabilities
- expanded commit taxonomy:
  - signal commits
  - CAS summary commits
  - identity commits
- push/fetch policies
- CSP shaping for outbound signals

---

# Phase 13 — CIS + CRR Expansion

## CIS
- identity boundary model
- identity version commits

## CRR
- promotion/demotion
- abstraction tiers
- split/merge operations

---

# Phase 14 — External Disturbance Model

## Goals
Introduce external context awareness.

## Capabilities
- disturbance commit type
- scope (area, identity, graph region)
- time windows:
  - execution
  - observation
- propagation across structure
- VDS interpretation as intentional pause

---

# Phase 15 — CAS Expansion (Simulation Support)

## Goals
Enable structural simulation and replay.

## Capabilities
- cascade computation
- simulation on:
  - modified structures
  - observation conditions
- comparison support

---

# Phase 16 — Simulation & Replay (CDS + CAS Integration)

## Goals
Enable semantic replay for system understanding.

## Capabilities
- replay-capable observation items
- simulation execution:
  - CDS → CSG → apply observations → CAS
- simulated signal emission
- comparison:
  - current vs simulated vs alternative thresholds

---

# Phase 17 — Persistence Expansion

## Goals
Fully decouple storage from implementation.

## Capabilities
- persistence interfaces across all substrates
- host adapter system
- data pooling support
- runtime persistence modes:
  - in-memory
  - partial
  - full

---

# Phase 18 — VDS (Value-Directed Systems)

## Goals
Operational monitoring system.

## Capabilities
- signal generation
- threshold evaluation
- observation item creation
- disturbance interpretation

---

# Phase 19 — VLS (Value Lineage Systems)

## Goals
Structural lineage and system insight.

## Capabilities
- identity timelines
- alignment visualization
- structural change tracking
- disturbance-aware analysis

---

# Phase 20 — CGL (Guidance Layer)

## Goals
System understanding and assistance.

## Capabilities
- explanation
- comparison
- structural insight
- simulation interpretation

---

# 3. Substrate Readiness Summary

Legitimacy Engine → Phase 1  
Runtime → Phase 2  
CLI → Phase 3  
CCS → Phase 4  
Commit Store → Phase 5  
CQL → Phase 6  
CRS → Phase 7  
CSG → Phase 8  
CCare/CSP → Phase 9  
Measurement Bridge → Phase 10  
CAS v1 → Phase 11  
Federation Expansion → Phase 12  
CIS/CRR → Phase 13  
Disturbance Model → Phase 14  
CAS Simulation → Phase 15  
Replay System → Phase 16  
Persistence Expansion → Phase 17  
VDS → Phase 18  
VLS → Phase 19  
CGL → Phase 20  

---

# 4. Transitional vs Canonical

## Transitional
- in-memory runtime state
- partial persistence
- minimal commit taxonomy

## Canonical
- commit-backed durability
- pluggable persistence
- full query via CQL
- simulation and federation

---

# 5. Key Risks

- runtime leaking into commit semantics  
- persistence coupling to specific storage implementations  
- CAS becoming non-deterministic  
- simulation introducing implicit interpretation  
- federation introducing hidden mutation  
- CLI bypassing CQL  

---

# 6. Final Principle

1. Create truth (Legitimacy)  
2. Orchestrate truth (Runtime)  
3. Preserve truth (CCS + Commit Store)  
4. Query truth (CQL)  
5. Understand truth (CAS)  
6. Simulate truth (Replay)  
7. Share truth (CRS)