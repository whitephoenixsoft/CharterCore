# Charter Query Language (CQL)

Status: EVOLVING
Type: Read-Only Query SDK and Query Contract
Primary Language: Rust
Scope: Domain-Neutral
Reference Ecosystem: Charter

---

# Executive Summary

CQL is a read-only query SDK and query contract that provides a unified,
capability-driven way to access data across independent systems.

It allows substrates and hosts to expose queryable surfaces without forcing
shared storage, shared schemas, or shared business semantics.

CQL provides:

- Capability discovery
- Validation before execution
- Canonical query representation
- Human-friendly query authoring
- Bounded execution
- Explicit result contracts

CQL does not own domain meaning.

Domains own meaning.

CQL owns query structure.

---

# Why CQL Exists

Modern systems often suffer from one or more of the following:

- Every service invents its own query API
- Query capabilities are poorly discoverable
- Consumers cannot validate requests ahead of execution
- Cross-system tooling becomes expensive
- Read contracts drift over time
- Result semantics become inconsistent

CQL exists to establish a common read contract while preserving ownership and autonomy of individual systems.

---

# Design Philosophy

The following principles drive all CQL design decisions.

## Domain Ownership

CQL never defines business meaning.

Domain owners define:

- domains
- views
- fields
- output modes
- semantics

CQL only defines how those things are queried.

---

## Read-Only First

CQL is intentionally read-only.

Mutation remains owned by substrates and hosts.

This keeps:

- authorization simpler
- execution safer
- contracts more stable
- portability higher

---

## Capability-Driven

Nothing is queryable unless explicitly declared.

Capability declarations are authoritative.

CQL never guesses.

---

## Explicit Over Implicit

CQL prefers explicit outcomes over silent behavior.

The system distinguishes:

- success
- empty
- hidden
- unauthorized
- nonexistent
- unsupported
- partial
- error

Trust is preserved by making outcomes visible.

---

## Bounded Execution

All execution must remain bounded.

Unbounded traversal, arbitrary joins, and uncontrolled recursion are intentionally excluded.

---

# Design Lineage

CQL borrows proven ideas from existing systems while remaining its own contract.

| Concern | Inspiration |
|----------|-------------|
| Canonical query representation | Elasticsearch / OpenSearch DSL |
| Capability discovery | GraphQL |
| Read option separation | OData |
| Human query syntax | KQL |
| Temporal scope | PromQL |
| Graph read-surface lessons | Cypher / SPARQL |

CQL is not a clone of any of these systems.

These systems provide design references only.

---

# High-Level Architecture

Authoring Surfaces
```
    Human DSL
         |
         v
    Query Builder APIs
         |
         v
      JSON IL
         |
         v
      Validator
         |
         v
 Capability Model
         |
         v
 Execution Context
         |
         v
 Adapter Layer
         |
         v
 Result Envelope
```
JSON IL is the canonical representation.

Everything ultimately becomes JSON IL.

---

# Core Components

## DSL

Human-friendly authoring surface.

Purpose:

- easy manual usage
- CLI support
- documentation examples

The DSL introduces no semantics of its own.

---

## JSON Intermediate Language

Canonical machine representation.

All execution originates from JSON IL.

All authoring surfaces compile into JSON IL.

---

## Capability Declarations

Published contract describing:

- domains
- views
- fields
- output modes
- constraints
- dependencies
- versions

Capability declarations are the basis for validation.

---

## Execution Context

Provides execution-scoped information.

Examples:

- authorization context
- visibility rules
- recursion limits
- execution budgets
- dependency policies

---

## Adapter Layer

Substrate-owned execution boundary.

Adapters:

- receive validated queries
- execute against local systems
- return standardized outcomes

Adapters own implementation.

CQL owns contracts.

---

## Result Envelope

Standard response structure.

Provides:

- results
- diagnostics
- warnings
- metadata
- version information
- query identity

---

# Scope Boundaries

## CQL Does

- Query validation
- Capability discovery
- Query normalization
- Query execution routing
- Result normalization

## CQL Does Not

- Store data
- Own authorization systems
- Define domain meaning
- Define transport protocols
- Perform mutations
- Own business logic

---

# Current Architecture Decisions

## AD-001

JSON IL is canonical.

---

## AD-002

DSL compiles to JSON IL.

---

## AD-003

Validation occurs before dispatch.

---

## AD-004

Capability declarations are authoritative.

---

## AD-005

Host-defined domains are supported.

---

## AD-006

Managed composition is allowed but must remain bounded and traceable.

---

## AD-007

Versioning is mandatory.

Domains, views, capabilities, and results all carry version information.

---

## AD-008

Output modes are domain-owned.

Detailed is the default mode.

Summary is optional.

---

## AD-009

Result conditions remain explicit.

No silent collapsing of outcomes.

---

# Current Status

Architecture Status:
- Defined

Specification Status:
- Evolving

Implementation Status:
- Not Started

Target Platform:
- Rust SDK

---

# Roadmap

Near-Term

- Capability schema finalization
- Result envelope finalization
- Versioning refinement
- Dependency declaration refinement

Mid-Term

- Rust SDK
- Validation engine
- Query builders
- DSL parser

Long-Term

- Tooling ecosystem
- IDE support
- AI-assisted query generation
- Broader host adoption

---

# Mental Model

Think of CQL as:

"A capability-driven read contract that sits between consumers and data-producing systems."

Or:

"GraphQL-style discovery combined with a canonical query model and substrate-neutral execution."

---

# Related Specifications

Foundations

- JSON IL Foundation
- Shared Invariants
- DSL Specification

Execution

- Capability Declaration
- Execution Context
- Result Envelope
- Adapter Outcome

Support

- Canonical Naming
- Module Boundaries