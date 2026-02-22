# Charter Multi-Layer Structural Design (Revised)

**Purpose:** Document the layered architecture of Charter, including engines, libraries, storage layers, and observational layers. This document preserves structural intent, system relationships, boundary enforcement, and auditable flows. It defines architecture only — not implementation details.

---

## 1. Introduction

### 1.1 Purpose
Explain the structural organization and relationships of Charter layers across V1–V7, including legitimacy, storage, orchestration, archival, and transport boundaries.

### 1.2 Scope
Structural overview of:

- Engine (V1/V2)
- Persistence (V1 foundational layer)
- Runtime / Orchestration (V1–V4)
- Guidance (V5)
- Federation (V6)
- Commit Store (V6)
- Commit Relay (V7)

This document does not define engine internals beyond frozen legitimacy semantics.

### 1.3 Audience
Future maintainers, developers, architects, and integrators embedding Charter.

### 1.4 Intended Use
Reference for:

- Dependency management
- Codebase structure validation
- Cross-layer invariant enforcement
- Long-term evolution without legitimacy mutation

---

## 2. Charter Architecture Overview

### 2.1 Layered Model

Charter consists of the following structural layers:

1. Engine Layer (Legitimacy Kernel)
2. Persistence Layer (Truth Ledger)
3. Runtime Layer (Boundary Orchestrator)
4. Observational & Transport Layers:
   - Guidance
   - Federation
   - Commit Store
   - Commit Relay

### 2.2 Authoritative Dependency Direction

Correct dependency direction is:

Engine  
↑  
Persistence  
↑  
Runtime  
↑  
Guidance / Federation / Commit Relay  

Lower layers must never depend on higher layers.

### 2.3 Core Architectural Principles

- Legitimacy is computed only by the Engine.
- Storage is append-only and enforced by Persistence.
- Runtime orchestrates boundaries but does not compute legitimacy rules.
- Audit is descriptive and human-facing.
- Commit Store is archival, not authoritative state.
- Relay transports artifacts but does not interpret them.
- No upstream mutation of legitimacy is permitted.

### 2.4 System Mental Model

- Engine: Legitimacy kernel.
- Persistence: Truth ledger.
- Runtime: Boundary orchestrator.
- Commit Store: Artifact archive.
- Relay: Transport layer.
- Guidance/Federation: Read-only observers.

---

## 3. Engine Layer (V1/V2)

### 3.1 Core Responsibilities

- Deterministic legitimacy creation.
- Session lifecycle management.
- Resolution lifecycle management.
- Authority and scope enforcement.
- Mechanical voting evaluation.
- Domain event emission.

### 3.2 Explicit Non-Responsibilities

Engine does not:

- Know about Context.
- Store data.
- Perform hashing.
- Manage audit storage.
- Interpret commits.
- Perform baseline review.
- Communicate with relay.

### 3.3 Legitimacy Boundary

- Resolution is the smallest legitimacy unit.
- Sessions produce resolutions.
- Authority is explicit and mechanical.
- Scope enforces semantic legitimacy boundaries.
- Legitimacy is never inferred from history or audit.

---

## 4. Persistence Layer (V1 Foundational)

### 4.1 Core Responsibilities

Persistence owns:

- Object Store (hash-based, append-only)
- Ref Store (current pointers)
- Audit Store (append-only event ledger)
- Commit Store (UUID-based artifact archive)
- Hashing and canonicalization
- Envelope validation
- Integrity verification
- fsck validation

### 4.2 Object Store

- Hash-identified objects.
- Immutable and append-only.
- Stores domain objects and receipts.
- Supersession is logical, not physical.

### 4.3 Ref Store

- Stores current pointers (Authority, Scope, etc.).
- Minimal and bounded.
- Does not encode history.

### 4.4 Audit Store

Audit is:

- Append-only.
- Descriptive.
- Human-facing.
- Emitted by Engine and Runtime.
- Not used for legitimacy computation.

Engine must never read audit to determine legitimacy.

### 4.5 Commit Store (V6)

Commit Store is:

- UUID-identified.
- Immutable.
- Append-only.
- Categorized by commit type.
- Independent artifacts.
- Not a state reconstruction mechanism.
- Not legitimacy-computing.

Engine must not read Commit Store to compute legitimacy.

### 4.6 Explicit Non-Responsibilities

Persistence does not:

- Compute authority.
- Compute legitimacy.
- Run baseline review.
- Interpret workflow semantics.
- Derive state from commits.

---

## 5. Runtime Layer (Orchestration & Boundary Control)

### 5.1 Core Responsibilities

Runtime owns:

- Context isolation (hard storage boundary).
- Session orchestration.
- Baseline review (integration gateway).
- Restore operations.
- Engine invocation.
- Applying mutation plans atomically.
- Emitting workflow-level audit events.
- Enforcing spec verification at legitimacy boundaries.

### 5.2 Context

- Context is a storage isolation boundary.
- Engine is unaware of Context.
- Commit stores are scoped per Context.
- Cross-context movement is explicit and auditable.

### 5.3 Baseline Review (Integration Path)

Baseline Review is required for:

- Artifact integration.
- Deliberate outputs.
- Synthesis artifacts.
- External non-CCE commits.
- Cross-boundary workflow artifacts.

Flow:

Artifact → Baseline Review → Session → Resolution

Baseline review does not create legitimacy.
Sessions do.

### 5.4 Restore (Authoritative Rehydration Path)

Restore is used for:

- CCE JSON payloads.
- Snapshot restoration.
- Relay commits containing CCE exports.

Flow:

CCE Payload → Spec Verify → Restore → Persistence Load

Restore:

- Does not use baseline review.
- Must not merge with existing legitimacy state.
- Must enforce strict spec compatibility.

### 5.5 Explicit Non-Responsibilities

Runtime does not:

- Compute legitimacy rules.
- Infer authority.
- Interpret commit archive as state.
- Bypass engine.

---

## 6. Guidance Layer (V5)

### 6.1 Responsibilities

- Observes engine and runtime state.
- Consumes audit.
- Generates summaries and drift detection.
- Produces read-only outputs.
- May support AI-assisted reflection.

### 6.2 Explicit Constraints

Guidance:

- Does not mutate engine state.
- Does not create legitimacy.
- Does not infer authority.

---

## 7. Federation Layer (V6)

### 7.1 Responsibilities

- Observes multiple contexts or areas.
- Aggregates signals and check-ins.
- Provides reflective summaries.
- Fully auditable.

### 7.2 Explicit Constraints

Federation:

- Does not compute authority.
- Does not synthesize legitimacy.
- Does not override sessions.

---

## 8. Commit Relay Layer (V7)

### 8.1 Responsibilities

Relay:

- Stores commits immutably.
- Applies structural filtering.
- Preserves timestamps.
- Supports manual push/fetch.
- Does not interpret payloads.

### 8.2 Restore and Integration from Relay

Two cases exist:

1. CCE Commit:
   - Treated as restore input.
   - No baseline review.
   - Must pass spec verification.

2. Non-CCE Commit:
   - Treated as artifact.
   - Requires baseline review before legitimacy impact.

### 8.3 Explicit Non-Responsibilities

Relay:

- Does not compute authority.
- Does not reconstruct canonical state.
- Does not run baseline review.
- Does not derive legitimacy.

---

## 9. Spec Verification (Cross-Layer Contract)

### 9.1 Purpose

Spec Verification ensures semantic compatibility across:

- Engine implementations
- Persistence implementations
- Runtime implementations
- Cross-system integrations

### 9.2 Properties

Each library exposes:

- Spec identity
- Invariant validation capability

Spec verification:

- Blocks legitimacy-affecting imports if incompatible.
- Blocks restore if incompatible.
- Is required for authoritative integration.
- Is not required for read-only observation.

### 9.3 Structural Rule

Spec verification protects legitimacy semantics across forks and external integrations.

---

## 10. Cross-Layer Invariants

- Resolution remains the smallest legitimacy unit.
- Only Engine computes legitimacy.
- Audit is descriptive, never generative.
- Commit Store is archival, not authoritative.
- Relay is transport-only.
- Baseline Review is the only integration gateway for artifacts.
- Restore is the only authoritative rehydration path.
- No layer may collapse Context into legitimacy.
- No layer may collapse transport into authority.

---

## 11. Deployment & Validation Considerations

### 11.1 Library Independence

Each layer may be versioned independently but must expose spec identity.

### 11.2 Multi-Instance Environments

Multiple runtimes may coexist.
Federation and relay may aggregate artifacts.
Legitimacy remains local.

### 11.3 Code Structure Validation Rules

The following must always hold:

- Engine contains no persistence logic.
- Persistence contains no legitimacy logic.
- Runtime does not compute authority rules.
- Engine does not read audit or commit store.
- Relay does not depend on Engine.
- Restore must enforce spec verification.
- Baseline review must precede artifact-based legitimacy changes.

---

## 12. Conclusion

Charter is a layered legitimacy platform built on strict separation of concerns:

- Engine as legitimacy kernel.
- Persistence as truth ledger.
- Runtime as boundary orchestrator.
- Commit Store as artifact archive.
- Relay as transport.
- Guidance and Federation as read-only observers.

Charter evolves by layering visibility, transport, and orchestration —
never by mutating legitimacy semantics.