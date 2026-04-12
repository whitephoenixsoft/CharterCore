# Charter — Implementation Plan (v3)

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
- Review and Reconciliation
- In-memory or host-provided persistence

---

## IP-02 — Legitimacy is the Only Authority Creator
- All decisions must pass through sessions
- No substrate may bypass this

---

## IP-03 — Dual Representation of Legitimacy

When a resolution is created:
- it exists in Runtime as operational truth
- it is emitted as a Charter Commit System commit as a durable artifact

---

## IP-04 — Reconciliation is the Only Bridge
All movement between:
- Deliberate System and legitimacy
- foreign artifacts and local system

must pass through Reconciliation Review.

---

## IP-05 — Substrates Are Charter-Native Libraries
- implemented as Rust crates
- independently usable
- designed to compose
- orchestrated through Runtime

---

## IP-06 — Commit-Centric Architecture
- All durable artifacts are Charter Commit System commits
- All cross-system exchange flows through commits
- Charter Relay System transports commits but does not interpret them

---

## IP-07 — CQL as the Universal Query Interface
- All system querying flows through Charter Query Language JSON Intermediate Language
- CLI, Runtime, Value-Directed Systems, and Value Lineage Systems all use Charter Query Language

---

## IP-08 — Persistence is Host-Controlled
- Substrates define logical storage contracts
- Hosts define physical storage implementations
- Default object stores and commit stores are reference implementations only

---

## IP-09 — Persistence Interfaces Must Exist with Runtime
- Persistence interfaces must be standardized at the same time as Runtime
- Persistence implementations may mature incrementally
- CLI development depends on Runtime load and flush behavior from the beginning

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

# Phase 2 — Runtime and Persistence Foundation

## Goals
Define how the system operates locally and how state is loaded and persisted.

## Capabilities
- workspace and area model
- session orchestration
- incoming reconciliation review
- process lifecycle:
  - pause
  - resume
  - block

- logical store contracts for:
  - commit store
  - reference store
  - metadata store
  - audit store
  - review workspace store
  - deliberate workspace store

- persistence interface contracts
- host adapter boundary for persistence
- minimal reference implementations
- in-memory runtime operation
- load and flush lifecycle required by CLI

---

# Phase 3 — CLI MVP (Reference Product)

## Goals
Validate system behavior through a real interface.

## Capabilities
- initialization of context and area
- decision flow:
  - deliberate
  - review
  - session

- reconciliation flow
- status commands using Charter Query Language internally
- audit visibility
- single active mutable process per area enforcement
- persistent load / operate / flush lifecycle

---

# Phase 4 — Charter Commit System

## Goals
Introduce durable artifact representation.

## Capabilities
- commit schema foundation
- commit identity and integrity
- commit emission from:
  - sessions
  - review closure

- initial commit taxonomy:
  - resolution
  - session receipt
  - review closure

---

# Phase 5 — Commit Store

## Goals
Establish canonical artifact storage abstraction.

## Capabilities
- logical commit store interface
- reference append-only implementation
- indexing and retrieval
- export and import bundles
- host-provided storage adapter support

---

# Phase 6 — Charter Query Language

## Goals
Establish the unified query interface early.

## Capabilities
- JSON Intermediate Language definition
- runtime query execution
- CLI uses Charter Query Language for:
  - status
  - inspection
  - history visibility

- domain query routing
- human DSL as a compiler target into JSON Intermediate Language

---

# Phase 7 — Charter Relay System

## Goals
Enable cross-system exchange.

## Capabilities
- push and fetch of commit artifacts
- relay storage abstraction
- commit transport without interpretation
- basic filtering

---

# Phase 8 — Charter Structure Graph

## Goals
Materialize structural relationships.

## Capabilities
- directed acyclic graph construction from commits
- supersession tracking
- derivation relationships
- partial graph federation support

---

# Phase 9 — Charter Care and Charter Signal Pipeline

## Goals
Introduce the signal system foundation.

## Capabilities
- signal model with basic types
- signal emission
- Charter Signal Pipeline pipelines
- feeds and subscription model

---

# Phase 10 — Decision to Measurement Bridge

## Goals
Connect structure to observable behavior.

## Capabilities
- observable model
- relationship bindings:
  - decision to observable
  - observable to metric source
  - observable to threshold

- observation items in Deliberate System

---

# Phase 11 — Charter Alignment System Version 1

## Goals
Compute alignment and structural impact.

## Capabilities
- alignment calculation
- signal interpretation
- basic propagation across structure
- local computation only

---

# Phase 12 — Federation Expansion

## Goals
Strengthen commit-based exchange.

## Capabilities
- expanded commit taxonomy:
  - signal commits
  - Charter Alignment System summary commits
  - identity commits

- push and fetch policies
- Charter Signal Pipeline shaping for outbound signals

---

# Phase 13 — Charter Identity System and Cross-Resolution Reconciliation Expansion

## Charter Identity System
- identity boundary model
- identity version commits
- deterministic version derivation

## Cross-Resolution Reconciliation
- promotion and demotion
- abstraction tiers
- split and merge operations

---

# Phase 14 — External Disturbance Model

## Goals
Introduce external context awareness through Value-Directed Systems.

## Capabilities
- disturbance commit type
- scope:
  - area
  - identity
  - graph region

- time windows:
  - execution
  - observation

- propagation across structural relationships
- Value-Directed System interpretation as external intentional pause

---

# Phase 15 — Charter Alignment System Expansion

## Goals
Enable structural simulation and replay.

## Capabilities
- cascade computation
- simulation on:
  - modified structures
  - replayed observation conditions

- comparison support across configurations

---

# Phase 16 — Simulation and Replay

## Goals
Enable semantic replay for system understanding.

## Capabilities
- replay-capable observation items
- simulation execution:
  - Deliberate System
  - Charter Structure Graph
  - apply observations
  - Charter Alignment System

- simulated signal emission
- comparison between:
  - current state
  - simulated structure
  - alternative thresholds

---

# Phase 17 — Persistence Expansion

## Goals
Mature persistence breadth without changing the early interface contracts.

## Capabilities
- broader logical store coverage across all substrates
- additional host adapter implementations
- centralized and pooled host storage options
- runtime persistence modes:
  - in-memory
  - partial
  - full

- Value-Directed System and Value Lineage System persistence support
- multi-substrate storage access patterns

---

# Phase 18 — Value-Directed Systems

## Goals
Operational monitoring and caregiving system.

## Capabilities
- signal generation
- threshold evaluation
- observation item creation
- disturbance interpretation

---

# Phase 19 — Value Lineage Systems

## Goals
Structural lineage and system insight.

## Capabilities
- identity timelines
- alignment visibility
- structural change tracking
- disturbance-aware analysis

---

# Phase 20 — Charter Guidance Layer

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
Runtime and Persistence Foundation → Phase 2  
CLI → Phase 3  
Charter Commit System → Phase 4  
Commit Store → Phase 5  
Charter Query Language → Phase 6  
Charter Relay System → Phase 7  
Charter Structure Graph → Phase 8  
Charter Care and Charter Signal Pipeline → Phase 9  
Decision to Measurement Bridge → Phase 10  
Charter Alignment System Version 1 → Phase 11  
Federation Expansion → Phase 12  
Charter Identity System and Cross-Resolution Reconciliation → Phase 13  
External Disturbance Model → Phase 14  
Charter Alignment System Expansion → Phase 15  
Simulation and Replay → Phase 16  
Persistence Expansion → Phase 17  
Value-Directed Systems → Phase 18  
Value Lineage Systems → Phase 19  
Charter Guidance Layer → Phase 20  

---

# 4. Transitional vs Canonical

## Transitional
- in-memory runtime state
- minimal reference persistence implementations
- partial persistence coverage
- minimal commit taxonomy

## Canonical
- commit-backed durability
- pluggable persistence
- full query through Charter Query Language
- simulation and federation across systems

---

# 5. Key Risks

- runtime leaking into commit semantics  
- persistence coupling to specific storage implementations  
- Charter Alignment System becoming non-deterministic  
- simulation introducing implicit interpretation  
- federation introducing hidden mutation  
- CLI bypassing Charter Query Language  
- persistence interfaces being defined too late or too narrowly  

---

# 6. Final Principle

1. Create truth through legitimacy  
2. Orchestrate truth through Runtime  
3. Persist truth through standardized store contracts  
4. Preserve truth through Charter Commit System and commit storage  
5. Query truth through Charter Query Language  
6. Understand truth through Charter Alignment System  
7. Simulate truth through replay and structural experimentation  
8. Share truth through Charter Relay System