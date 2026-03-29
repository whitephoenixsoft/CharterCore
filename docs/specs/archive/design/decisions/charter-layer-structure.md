# Charter Stack — Unified Architecture (v3, VLS-Aligned)

Status: FOUNDATIONAL  
Replaces: Prior Stack & Multi-Layer Structural Documents  
Scope: Full Charter system architecture including legitimacy, storage, structure, alignment, and transport  
Does NOT define: implementation details, UI, or infrastructure policy  

---

# I. Purpose

This document defines the complete structural architecture of Charter with:

- Explicit VLS layer (structure + identity)
- Alignment Engine as computation layer
- Guidance as interpretation layer
- Strict separation of truth, structure, computation, and meaning

---

# II. Architectural Model

Charter operates across three independent axes:

---

## 1. Canonical Truth Axis

Defines what is legitimate and durable

Engine  
↑  
Persistence  
↑  
Runtime  

---

## 2. Epistemic Axis (Understanding Pipeline)

Defines how the system becomes understandable

VLS (structure & identity)  
↑  
VDS (signals / observations)  
↑  
Alignment Engine (computation)  
↑  
Guidance (interpretation)  

---

## 3. Transport Axis (Parallel)

Relay (append-only, external, non-interpreting)

---

## Core Principle

Truth is created locally.  
Structure is reconstructed explicitly.  
Alignment is computed deterministically.  
Meaning is interpreted separately.  
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

### Explicit Non-Responsibilities

- No storage
- No context awareness
- No signal interpretation
- No alignment computation

---

## 2. Persistence Layer (Truth Ledger)

### Responsibilities

- Append-only storage
- Object integrity
- Audit logging
- Commit storage

### Stores

- Object Store
- Ref Store
- Audit Store
- Commit Store

---

## 3. Commit Store (Canonical Input Layer)

### Properties

- Append-only
- Immutable
- UUID-based
- Multi-type (resolutions, signals, receipts, artifacts)

### Role

Primary source for:

- signals (VDS)
- artifacts
- cross-context inputs

### Constraint

Not authoritative state.

---

## 4. Runtime Layer (Boundary Orchestrator)

### Responsibilities

- Context isolation
- Session orchestration
- Baseline review
- Restore
- Engine invocation

---

### Integration Paths

Baseline Review:
Artifact → Review → Session → Resolution  

Restore:
CCE → Verify → Restore → Persistence  

---

# IV. Structure Layer (VLS)

## 5. Value Lineage System (VLS)

### Responsibilities

- Reconstruct global DAG from areas
- Preserve identity boundaries
- Track identity versions and scope
- Maintain lineage continuity

---

### Outputs

- Federated DAG (structure only)
- Identity-scoped graph partitions

---

### Constraints

VLS does NOT:

- Observe signals
- Compute alignment
- Interpret behavior
- Enforce action

---

### Principle

VLS defines:

“Who we are structurally, and how that evolved.”

---

# V. Observation Layer (VDS)

## 6. Value Directed System (VDS)

### Responsibilities

- Emit signals (check-ins)
- Observe behavior relative to decisions
- Record supportability

---

### Properties

- Append-only
- Non-authoritative
- Descriptive

---

### Constraints

VDS does NOT:

- Compute alignment
- Modify identity
- Create legitimacy

---

### Principle

VDS defines:

“What appears to be happening.”

---

# VI. Computation Layer

## 7. Alignment Engine

### Responsibilities

- Compute drift, variance, density
- Compute renewal (freshness vs stagnation)
- Compute trends and instability
- Aggregate metrics over DAG

---

### Inputs

- VLS DAG (structure)
- VDS signals (observations)
- Context (time, windows)

---

### Outputs

- Alignment State Store
- Derived metrics

---

### Constraints

- Deterministic
- Rebuildable
- Non-authoritative
- No legitimacy mutation

---

### Principle

Alignment Engine defines:

“What patterns exist in behavior relative to intent.”

---

## 8. Alignment State Store (Derived)

### Properties

- Rebuildable
- Cacheable
- Discardable
- Non-authoritative

---

### Contains

- Drift
- Variance
- Density
- Renewal
- Aggregations

---

### Invariant

Loss of this store does not affect truth.

---

# VII. Interpretation Layer

## 9. Guidance / Exegesis (V5)

### Responsibilities

- Interpret alignment state
- Surface patterns
- Highlight drift, tension, stagnation

---

### Inputs

- Alignment state
- Commit history
- Audit

---

### Constraints

- Read-only
- Stateless
- Non-coercive
- Non-legitimizing

---

### Principle

Guidance answers:

“What does this appear to mean?”

---

# VIII. Transport Layer

## 10. Commit Relay (V7)

### Responsibilities

- Store commits immutably
- Transport commits between systems
- Preserve history

---

### Properties

- Opaque
- Append-only
- Idempotent
- Stateless

---

### Constraints

Relay does NOT:

- Interpret commits
- Compute legitimacy
- Reconstruct state
- Validate references

---

## Foreignness Rule

All relay data is:

Foreign until locally integrated

---

## Critical Constraint

Alignment Engine and VDS operate only on:

Local commit store

Never directly on relay.

---

# IX. Dependency Direction

## Truth Axis

Engine  
↑  
Persistence  
↑  
Runtime  

---

## Epistemic Axis

Runtime  
↑  
VLS  
↑  
VDS  
↑  
Alignment Engine  
↑  
Guidance  

---

## Transport

Relay is parallel and isolated.

---

# X. Cross-Layer Invariants

- Resolution is the smallest legitimacy unit
- Only Engine creates legitimacy
- VLS defines structure, not behavior
- VDS observes behavior, not identity
- Alignment Engine computes, not decides
- Guidance interprets, not enforces
- Commit Store is input, not state
- Alignment State is derived, not truth
- Relay is transport only

---

# XI. System Mental Model

| Layer            | Question Answered        |
| ---------------- | ------------------------ |
| Engine           | Was this legitimate?     |
| Persistence      | What was recorded?       |
| Runtime          | How is it integrated?    |
| VLS              | Who are we structurally? |
| VDS              | What is happening?       |
| Alignment Engine | What patterns exist?     |
| Guidance         | What does it mean?       |
| Relay            | How is it transported?   |

---

# XII. Final Principle

Charter is a layered system of:

- legitimacy (truth)
- lineage (structure)
- observation (signals)
- computation (alignment)
- interpretation (guidance)
- transport (relay)

Each layer preserves a different dimension of reality.

None may collapse into another.