# Usage Pattern — Charter Engine as a Consensus Primitive

**Status:** Conceptual  
**Audience:** AI Engineers, Distributed Systems Developers  
**Scope:** Conceptual integration with external systems  

---

## Purpose

This document explains how the **Charter Core Engine** can serve as a **deterministic, memory-based consensus layer** for multi-agent or AI-driven systems. It is not a workflow specification, but a conceptual guide for integrating authoritative decision-making into an application or distributed environment.

Charter provides a **legitimacy-first primitive**:

- Accepts resolutions from multiple participants (human or AI)
- Evaluates authority deterministically
- Maintains an immutable, auditable history
- Supports branching and incremental updates

---

## Conceptual Model

The engine consists of two core abstractions:

1. **Compiler (DAG Builder)**  
   - Transforms proposals, candidate resolutions, and stances into a directed acyclic graph (DAG)  
   - Encodes dependency and supersession relationships  
   - Ensures determinism: given the same inputs, the DAG is identical  

2. **Runtime (Evaluator)**  
   - Processes the DAG to evaluate legitimacy under the current authority rules  
   - Only recomputes incremental changes when new inputs arrive  
   - Exposes queries for resolutions, acceptance status, and valid supersessions  

**Analogy to blockchain:**  
- Resolutions are like blocks  
- Sessions are mini-consensus events (analogous to a block being validated)  
- Authority participants are validators  
- DAG structure encodes dependency, supersession, and finality  

---

## Usage Patterns

### 1. Distributed Governance

- Each external agent (human or AI) submits a **resolution** to the engine.  
- Votes or stances are collected according to the **V2 authority model**.  
- The engine deterministically evaluates whether authority is satisfied.  
- Accepted resolutions are appended as **immutable events** in the DAG.  

**Outcome:** Every node or agent can independently reconstruct the same legitimacy graph without sharing storage or trusting a central server.

---

### 2. Incremental Updates

- New proposals or candidate changes are **incrementally compiled** into the existing DAG.  
- Only affected paths are re-evaluated; the rest of the DAG remains untouched.  
- Applications can query the runtime to see which resolutions became legitimate after the change.  

**Outcome:** High throughput and efficiency for dynamic environments with frequent updates.

---

### 3. Fork Detection and Divergence Management

- Divergent paths in the DAG represent conflicting or alternative resolutions.  
- Engine queries can detect **non-shared paths**, providing insights into unresolved conflicts.  
- The runtime allows multiple branches to coexist until explicit resolution (acceptance) occurs.  

**Outcome:** Supports safe experimentation and parallel evaluation without violating legitimacy.

---

### 4. AI Integration

- AI agents can submit candidate resolutions representing suggested decisions or state updates.  
- Charter enforces **rules of legitimacy**, preventing accidental or premature commitment.  
- Accepted resolutions can be exported to external systems as deterministic, auditable commands.  

**Outcome:** AI systems gain a **mechanically enforced governance layer** for decision-making.

---

## Key Advantages

1. **Deterministic Evaluation:**  
   - All nodes see the same results given the same inputs.
2. **Auditability:**  
   - Every accepted resolution is traceable; every DAG mutation is append-only.  
3. **Incremental Compilation:**  
   - Only recomputes what changes, making it efficient for large datasets.  
4. **Branching Support:**  
   - Divergent proposals coexist safely; finality is explicit.  
5. **Composable Primitive:**  
   - Can be embedded into applications, used by AI agents, or integrated into distributed networks.  

---

## Integration Notes

- The engine **does not store persistence**; external systems may snapshot DAGs if needed.  
- For multi-agent or federated environments, each agent can maintain its own runtime instance.  
- Accepted resolutions can be used as triggers for downstream workflows (business rules, API updates, state changes).  
- CLI or Python bindings can be built on top for orchestration, incremental review, or baseline evaluations.

---

## Mental Model

Charter’s engine is a **consensus compiler**:

- Input: Candidate resolutions + participant stances  
- Transformation: DAG compilation (dependencies, supersession)  
- Output: Deterministic legitimacy evaluation  

Think of it as **mechanical governance for distributed agents**, where **finality, authority, and auditability** are enforced by computation rather than trust.

