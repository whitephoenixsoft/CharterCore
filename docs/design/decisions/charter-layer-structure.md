# Charter Stack — Unified Architecture (v2, Alignment-Aware)

Status: FOUNDATIONAL  
Replaces: Prior Stack & Multi-Layer Structural Documents  
Scope: Full Charter system architecture including legitimacy, storage, orchestration, alignment, and transport  
Does NOT define: implementation details, UI, or infrastructure policy  

---

# I. Purpose

This document defines the complete structural architecture of Charter after the introduction of:

- Commit-based storage (V6)
- Alignment Engine (new)
- VDS / VLS separation
- Relay (V7)

It establishes:

- Correct dependency direction
- Clear separation of concerns
- Canonical vs derived boundaries
- Transport vs computation separation

---

# II. Architectural Model

Charter is a layered epistemic system composed of three independent axes:

---

## 1. Canonical Truth Axis

Defines what is legitimate and durable

Engine  
↑  
Persistence  
↑  
Runtime  

---

## 2. Epistemic (Understanding) Axis

Defines how the system is observed, computed, and interpreted

Federation (structure)  
↑  
VDS (signals)  
↑  
Alignment Engine (computation)  
↑  
Guidance (interpretation)  

---

## 3. Transport Axis (Parallel)

Defines how artifacts move across systems

Relay (append-only, external, non-interpreting)

---

## Core Principle

Truth is created locally.  
Understanding is computed locally.  
Transport is external and neutral.

---

# III. Canonical Truth Layers

## 1. Engine Layer (Legitimacy Kernel)

### Responsibilities

- Deterministic legitimacy computation
- Session lifecycle
- Resolution creation
- Authority and scope enforcement

### Guarantees

- Resolution is the smallest legitimacy unit
- Legitimacy is explicit and mechanical
- No inference from history

### Explicit Non-Responsibilities

- No storage
- No context awareness
- No signal interpretation
- No alignment computation

---

## 2. Persistence Layer (Truth Ledger)

### Responsibilities

- Append-only object storage
- Hash-based integrity
- Audit logging (descriptive only)
- Commit storage (V6)

### Stores

- Object Store (immutable objects)
- Ref Store (current pointers)
- Audit Store (append-only events)
- Commit Store (UUID artifacts)

---

## 3. Commit Store (Canonical Input Layer)

### Properties

- Append-only
- Immutable
- UUID-identified
- Multi-type (resolutions, signals, receipts, artifacts)

### Role

Primary input source for:

- signals (VDS)
- artifacts
- cross-context data

### Constraints

- Not authoritative state
- Not used to compute legitimacy directly
- Not a state machine

---

## 4. Runtime Layer (Boundary Orchestrator)

### Responsibilities

- Context isolation
- Session orchestration
- Baseline review (integration path)
- Restore (authoritative rehydration)
- Engine invocation

---

### Integration Paths

#### Baseline Review (Artifact Path)

Artifact → Baseline Review → Session → Resolution  

- Required for foreign or exploratory inputs
- Does not create legitimacy

---

#### Restore (Authoritative Path)

CCE → Spec Verify → Restore → Persistence  

- No baseline review
- No merging
- Strict compatibility enforcement

---

### Constraints

- Cannot compute legitimacy
- Cannot infer authority
- Cannot bypass engine

---

# IV. Structure Layer

## 5. Federation Layer (VLS-Aligned)

### Responsibilities

- Reconstruct multi-area DAG
- Preserve identity boundaries
- Enable cross-context visibility

### Properties

- Graph-based, not hierarchical
- No authority transfer
- No enforcement

---

### Constraints

- Does not compute alignment
- Does not interpret signals
- Does not mutate identity

---

# V. Observation Layer

## 6. VDS Layer (Signal Production)

### Responsibilities

- Generate alignment signals (check-ins)
- Observe behavior relative to decisions
- Produce care metrics

### Properties

- Observational only
- Append-only signals
- Non-coercive

---

### Constraints

- Does not compute alignment dynamics
- Does not mutate identity
- Does not create legitimacy

---

# VI. Computation Layer

## 7. Alignment Engine

### Responsibilities

- Compute drift, variance, density
- Compute renewal (freshness vs stagnation)
- Maintain derived alignment state
- Perform deterministic analysis over DAG

---

### Inputs

- VLS reconstructed DAG (via Federation)
- VDS signals (from Commit Store)
- Context (time, windows)

---

### Outputs

- Alignment State Store
- Queryable metrics

---

### Constraints

- Deterministic and rebuildable
- No authority
- No legitimacy mutation
- No direct relay access

---

## 8. Alignment State Store (Derived)

### Properties

- Rebuildable
- Non-authoritative
- Cacheable
- Discardable

### Contains

- Drift metrics
- Variance
- Signal density
- Renewal state
- Aggregated area metrics

---

### Invariant

If deleted, nothing is lost.

---

# VII. Interpretation Layer

## 9. Guidance / Exegesis (V5 Repositioned)

### Responsibilities

- Interpret alignment state
- Summarize patterns
- Highlight drift, tension, renewal risk

---

### Inputs

- Alignment Engine outputs
- Audit
- Commit history

---

### Constraints

- Read-only
- Stateless
- Non-legitimizing
- Non-coercive

---

### Principle

Guidance explains.  
It does not decide.

---

# VIII. Transport Layer

## 10. Commit Relay (V7)

### Nature

- Parallel to the system
- Not part of the computation stack

---

### Responsibilities

- Append-only commit storage
- Transport between systems
- Archival preservation

---

### Properties

- Opaque
- Immutable
- UUID-based
- Idempotent

---

### Constraints

Relay does NOT:

- Interpret commits
- Compute legitimacy
- Reconstruct state
- Enforce ordering
- Validate references

---

## Foreignness Boundary

All relay-fetched commits are:

Foreign until locally integrated

They cannot affect:

- legitimacy
- alignment
- identity

Until:

- baseline review, or
- restore

---

## Critical Rule

Alignment Engine and VDS operate only on local commits.  
Relay is never a direct computation source.

---

# IX. Dependency Direction

## Canonical Flow

Engine  
↑  
Persistence  
↑  
Runtime  

---

## Epistemic Flow

Runtime  
↑  
Federation  
↑  
VDS  
↑  
Alignment Engine  
↑  
Guidance  

---

## Transport

Relay operates independently and interfaces only with commit stores.

---

# X. Cross-Layer Invariants

- Resolution is the smallest legitimacy unit
- Only Engine computes legitimacy
- Commit Store is canonical input, not state
- Alignment State is derived, not authoritative
- Guidance never mutates state
- Relay never interprets data
- Federation never creates authority
- VDS never enforces action
- Alignment Engine never creates legitimacy

---

# XI. System Mental Model

| Layer | Question Answered |
|------|------------------|
| Engine | Was this legitimate? |
| Persistence | What was recorded? |
| Runtime | How is it integrated? |
| Federation | How is it structured? |
| VDS | What is happening? |
| Alignment Engine | What patterns exist? |
| Guidance | What does it mean? |
| Relay | How is it transported? |

---

# XII. Final Principle

Charter is not a tool.

It is a structured system for:

- preserving legitimacy
- observing reality
- computing alignment
- interpreting patterns
- transporting truth

It evolves by adding layers of visibility and computation —

never by mutating legitimacy.