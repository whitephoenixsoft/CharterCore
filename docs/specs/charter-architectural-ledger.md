# Charter — Architectural Ledger

Status: ACTIVE  
Purpose: Maintain system-wide architectural coherence across all substrates and products  

---

# 1. Overview

This document tracks:

- system-wide architectural decisions  
- cross-substrate invariants  
- integration agreements  
- major changes over time  

It is the authoritative reference for:

- architectural intent  
- system boundaries  
- cross-domain consistency  

---

# 2. Core Principles (Locked)

These principles apply across all Charter substrates.

- Non-Interpretation Principle  
- Determinism  
- Explicit Structure  
- Legitimacy as the only authority creator  
- Reconciliation as the only structural bridge  
- Commit System as the only durable exchange protocol  
- CQL as the unified read interface  

---

# 3. System Invariants

These invariants must hold across all substrates.

- No legitimacy is created outside the Legitimacy Engine  
- All durable artifacts are emitted through CCS  
- All cross-system exchange occurs through commits  
- All queries must be deterministic  
- No implicit relationships are created  
- All structure must be explicitly defined  
- Provenance must never be lost  
- Rule identity must be preserved  

---

# 4. Substrate Registry

Tracks all known substrates and their roles.

| Substrate | Role | Status | Integration Contract |
|----------|------|--------|----------------------|
| Runtime | Orchestration | ACTIVE | <link/ref> |
| Legitimacy Engine | Authority | ACTIVE | <link/ref> |
| CCS | Commit Protocol | ACTIVE | <link/ref> |
| CSG | Structural Graph | ACTIVE | <link/ref> |
| CIS | Identity | ACTIVE | <link/ref> |
| CAS | Alignment | ACTIVE | <link/ref> |
| CCare | Signals | ACTIVE | <link/ref> |
| CDS | Deliberate | ACTIVE | <link/ref> |
| CRS | Relay | ACTIVE | <link/ref> |
| CQL | Query Layer | ACTIVE | <link/ref> |

---

# 5. Integration Agreements

Defines cross-substrate contracts that must remain consistent.

## 5.1 Commit-Based Exchange

- All cross-substrate and cross-system exchange uses CCS commits  
- No direct data sharing bypassing CCS  

---

## 5.2 Query Model

- All queries use CQL JSON Intermediate Language  
- DSL compiles into JSON IL  
- Substrates expose query adapters  

---

## 5.3 Runtime Orchestration

- Runtime orchestrates but does not reinterpret  
- Substrates own their semantics  
- Runtime consumes integration contracts  

---

## 5.4 Federation Model

- CRS transports commits only  
- CSP may shape outbound flows  
- Federation preserves provenance and autonomy  

---

# 6. Change Log

Track major architectural changes.

---

## Entry Template

Date: YYYY-MM-DD  
Substrate(s): <affected substrates>  

### Change
<what changed>

### Reason
<why it changed>

### Impact
- Runtime  
- CQL  
- CCS  
- Other substrates  

### Action Required
<what must be updated>

---

# 7. Active Architectural Decisions

Tracks decisions that are currently shaping the system.

- <decision 1>  
- <decision 2>  

---

# 8. Open Architectural Questions

Cross-cutting questions requiring resolution.

- <question 1>  
- <question 2>  

---

# 9. Gap Signals (System-Level)

Tracks gaps that affect multiple substrates.

- <gap 1>  
- <gap 2>  

---

# 10. Runtime Integration View

Summarizes how Runtime interacts with all substrates.

Runtime is responsible for:

- process orchestration  
- workspace and Area management  
- persistence coordination  
- CQL query routing  
- commit emission coordination  

Runtime consumes:

- substrate integration contracts  

Runtime must not:

- redefine substrate semantics  
- bypass legitimacy  
- reinterpret outputs  

---

# 11. Product Layer Alignment

Tracks how products interact with the system.

## CLI

- primary human interface  
- uses Runtime  
- executes CQL queries  
- manages local context  

---

## VDS

- originates signals  
- uses CDS for observation  
- emits signals via CSP  

---

## VLS

- consumes CSG, CIS, CAS  
- provides lineage views  
- supports deployment and transition modeling  

---

# 12. Mental Model

This ledger represents:

- the system’s shared memory  
- the alignment point across all design efforts  

It ensures that:

- no substrate evolves in isolation  
- all integrations remain coherent  
- architectural intent is preserved  

---

# 13. Final Principle

Architecture is not owned by any single substrate.

It is maintained through:

- explicit decisions  
- shared contracts  
- continuous reconciliation  

This document is the anchor that keeps the system coherent as it evolves.