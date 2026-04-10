# Charter Core — Rehydration & Degraded Mode Foundation  
## Deterministic Runtime Entry & Safety Boundaries

Status: FOUNDATIONAL  
Layer: Conceptual / Engine Runtime Boundary  
Applies to: Engine Core, initialization flows, runtime safety  
Does NOT define: structural validation rules, persistence mechanics, or canonical encoding  

---

## 1. Purpose

This document defines how the Engine:

- Enters runtime from a provided domain graph (**rehydration**)  
- Handles invalid or unsafe states (**degraded mode**)  
- Preserves determinism and safety at all times  

The goal is to ensure that:

- runtime entry is deterministic  
- invalid state is never silently accepted  
- legitimacy is never computed on unsafe foundations  
- failures are explicit, not partial  

Rehydration is the gateway to truth.  
Degraded mode is the guardrail against corruption.

---

## 2. Core Principles

### 2.1 Deterministic Runtime Entry

Given identical inputs:

- the same domain graph  
- the same specification set  
- the same initialization command  

The Engine must produce:

- the same runtime mode  
- the same validation outcome  
- the same observable state  

There is no partial success.

---

### 2.2 No Implicit Repair

The Engine must not:

- fix missing data  
- infer intended structure  
- resolve conflicts automatically  
- patch inconsistencies  

Invalid input must remain invalid.

---

### 2.3 Safety Over Availability

If the Engine cannot guarantee correctness:

- it must restrict behavior  
- not approximate correctness  

It is better to block legitimacy than to risk incorrect legitimacy.

---

## 3. Rehydration

### 3.1 Definition

Rehydration is the process by which the Engine:

- consumes a domain graph  
- validates structural and historical integrity  
- establishes a runtime state for evaluation and mutation  

Rehydration is the only valid entry point into runtime.

---

### 3.2 Input Requirements

The input graph must be:

- structurally complete  
- internally consistent  
- confined to a single Area  
- self-sufficient for validation  

The Engine must not depend on:

- external systems  
- remote references  
- implicit context  

---

### 3.3 Rehydration Outcomes

Rehydration produces exactly one of the following:

#### NORMAL_RUNTIME

- All required invariants are satisfied  
- Full Engine functionality is available  
- Legitimacy evaluation and mutation are permitted  

#### DEGRADED_READ_ONLY

- Structural integrity is partially compromised  
- Some invariants cannot be guaranteed  
- Read-only operations may be permitted  
- Mutation and legitimacy creation are disallowed  

#### INITIALIZATION_FAILURE

- Critical invariants are violated  
- Runtime cannot be safely established  
- No Engine operations are permitted  

---

### 3.4 No Partial Runtime

The Engine must not:

- partially load a graph  
- expose incomplete state  
- allow selective evaluation  

Runtime is either:

- fully valid  
- safely restricted  
- or not established at all  

---

## 4. Degraded Mode

### 4.1 Definition

Degraded mode is a restricted runtime state in which:

- the Engine acknowledges structural or historical inconsistency  
- correctness cannot be fully guaranteed  
- mutation and legitimacy are disabled  

It exists to allow **inspection without risk**.

---

### 4.2 Allowed Operations

In degraded mode, the Engine may allow:

- read-only queries  
- inspection of objects and history  
- retrieval of receipts and audit data  

All allowed operations must be:

- deterministic  
- side-effect free  
- explicitly safe under incomplete guarantees  

---

### 4.3 Forbidden Operations

In degraded mode, the Engine must reject:

- all mutating commands  
- all session progression  
- all acceptance attempts  
- all legitimacy creation  

No state transition may occur.

---

### 4.4 Explicit Signaling

Degraded mode must be:

- explicitly indicated in runtime state  
- visible to all API consumers  
- reflected in EvaluationReports  

There must be no ambiguity about runtime safety.

---

## 5. Failure Conditions

### 5.1 Initialization Failure

Initialization must fail when:

- structural integrity cannot be established  
- required references are missing  
- canonical validation fails  
- receipt integrity cannot be verified  

In this state:

- no runtime exists  
- no queries are allowed  
- no partial inspection is permitted  

---

### 5.2 Degraded Entry Conditions

Degraded mode may be entered when:

- non-critical inconsistencies are detected  
- historical verification is incomplete but partially intact  
- structural truth cannot be fully guaranteed but is inspectable  

The distinction between degraded and failure must be:

- deterministic  
- rule-defined  
- consistent across implementations  

---

## 6. Relationship to Legitimacy

Legitimacy requires:

- a fully valid runtime  
- verified structural integrity  
- deterministic evaluation conditions  

Therefore:

- legitimacy must not be evaluated in degraded mode  
- legitimacy must not be created during or after failed initialization  

Rehydration is a prerequisite to legitimacy.

---

## 7. Relationship to Receipts

Receipts are:

- inputs to rehydration  
- sources of historical truth  

During rehydration:

- receipts must be validated  
- canonical integrity must be confirmed  

If receipt integrity fails:

- degraded mode or failure must result  
- legitimacy must not proceed  

Receipts are never repaired.

---

## 8. Relationship to Determinism

Rehydration and degraded mode must be fully deterministic.

Given identical inputs:

- the same inconsistencies must be detected  
- the same runtime mode must be selected  
- the same operations must be allowed or rejected  

No randomness or environment-dependent behavior is allowed.

---

## 9. Conceptual Invariants

- Rehydration is the only entry into runtime  
- Runtime mode is deterministic  
- No implicit repair is allowed  
- Degraded mode is read-only  
- Initialization failure produces no runtime  
- Legitimacy requires a fully valid runtime  
- Receipts must be validated, not trusted blindly  
- The Engine never operates on uncertain truth  

---

## 10. Mental Model

Rehydration is loading the world.  

- If the world is valid → you may act  
- If the world is questionable → you may observe  
- If the world is broken → you may not proceed  

The Engine does not guess.  
It either knows, or it refuses.