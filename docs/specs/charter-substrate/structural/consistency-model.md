# Charter Core — Consistency Model Foundation
## Deterministic, Closed-State Evaluation

Status: FOUNDATIONAL  
Layer: Conceptual / Engine Core  
Applies to: All Engine operations — evaluation, mutation, rehydration, and query execution  
Does NOT define: storage implementation, database isolation levels, or distributed coordination mechanisms  

---

## Purpose

This document defines the **consistency model** of Charter Core.

Its purpose is to ensure that:

- all evaluation is performed over a complete and valid state  
- legitimacy decisions are never made on partial or corrupted data  
- commands observe a stable and deterministic view of the system  
- no operation depends on in-flight or intermediate changes  
- correctness is preserved regardless of execution environment  

Consistency is required for **trustworthiness**.  
Without it, legitimacy cannot be reliably determined.

---

## Core Principle

> The Engine operates only on fully consistent state.

The Engine must never:

- evaluate incomplete data  
- mix states from different points in time  
- proceed under uncertainty about structural validity  

It must either:

- see a complete, valid world  
- or refuse to operate  

---

## Why This Model Exists

Without strict consistency, the system would risk:

- accepting invalid legitimacy due to missing data  
- conflicting evaluation results across invocations  
- non-deterministic behavior  
- race-condition-driven outcomes  
- silent corruption of state  

Charter prevents this by enforcing:

- closed-state evaluation  
- strict isolation  
- validation before use  

---

## Consistency Model

### Strong Consistency per Invocation

Each Engine invocation (evaluation or mutation) operates on:

- a single, internally consistent snapshot of state  

This snapshot must:

- include all required objects  
- satisfy structural integrity  
- be complete for the scope of the operation  

There is no concept of “eventual consistency” inside the Engine.

---

### No Partial Reads

The Engine must not:

- read partially written objects  
- observe intermediate mutation states  
- operate on incomplete graphs  
- tolerate missing required dependencies  

If required data is missing:

- evaluation must fail  
- mutation must not proceed  

---

### No Mixed-State Evaluation

The Engine must not:

- combine data from different logical snapshots  
- observe pre- and post-mutation state simultaneously  
- evaluate with partially updated references  

All reads within an invocation must come from the same consistent view.

---

## Evaluation Requirements

Evaluation requires:

- a complete domain graph for the relevant Area  
- validated structural integrity (via ENG-INTEGRITY)  
- deterministic inputs (see Determinism Foundation)  
- consistent governance references  
- consistent session state  

Evaluation must not proceed if:

- structural validation fails  
- required references are missing  
- receipts are invalid or inconsistent  
- runtime is not in a valid mode  

---

## Isolation Model

### Snapshot Isolation (Conceptual)

Each command operates on a **closed snapshot** of the system.

This means:

- no concurrent mutation is visible during evaluation  
- no partial updates are observed  
- the state does not change during execution  

Even if the underlying storage is concurrent, the Engine must behave as if:

> it is operating on a frozen world

---

### Mutation Isolation

Mutation must:

- evaluate against a consistent snapshot  
- apply changes atomically  
- produce a new consistent state  

No other operation may observe:

- intermediate mutation steps  
- partially committed state  

---

## Relationship to Rehydration

Before entering runtime:

- the Engine must validate that the provided graph is consistent  

If consistency cannot be established:

- runtime must not proceed normally  
- degraded or failure modes must be entered  

The Engine must not “fix” inconsistent input.

---

## Relationship to Evaluation

Evaluation depends entirely on consistency.

If the state is inconsistent:

- evaluation results are invalid  
- legitimacy decisions cannot be trusted  

Therefore:

- evaluation must fail rather than proceed on uncertain data  

---

## Relationship to Mutation

Mutation depends on consistent pre-state and must produce consistent post-state.

Mutation must not:

- apply partial changes  
- leave dangling references  
- violate structural integrity  

If mutation cannot maintain consistency:

- it must fail entirely  
- no state changes may occur  

---

## Relationship to Determinism

Consistency is a prerequisite for determinism.

Without consistency:

- identical inputs cannot be guaranteed  
- outputs may vary  
- reconstruction becomes unreliable  

Determinism assumes a closed, valid input state.

---

## Relationship to State vs History

Consistency ensures that:

- derived state accurately reflects history  
- no partial history is interpreted as complete  

If history is incomplete or corrupted:

- state must not be derived  
- the system must signal failure or degraded mode  

---

## Relationship to Runtime Modes

### Normal Runtime

- full consistency is required  
- all operations may proceed  

### Degraded Read-Only Mode

- consistency may be partially compromised  
- mutation is forbidden  
- evaluation may be limited to safe operations  

### Initialization Failure

- consistency could not be established  
- runtime must not proceed  

---

## No Implicit Repair

The Engine must not:

- auto-correct missing references  
- fill in missing data  
- infer intended structure  
- silently ignore inconsistencies  

All inconsistencies must be:

- surfaced  
- explicit  
- blocking where required  

Repair, if any, must occur through explicit external processes.

---

## Failure Semantics

If consistency cannot be guaranteed:

- evaluation must fail or be restricted  
- mutation must not occur  
- no legitimacy may be created  

Proceeding under inconsistent state is considered a critical violation.

---

## Conceptual Invariants

- the Engine operates on fully consistent state  
- no partial reads are allowed  
- no mixed-state evaluation is allowed  
- each command sees a closed snapshot  
- mutation is atomic and isolated  
- inconsistent state must not be used  
- no implicit repair is allowed  
- consistency is required for legitimacy  

---

## Mental Model

The Engine sees a complete world or no world at all.

It does not guess.  
It does not interpolate.  
It does not proceed on partial truth.

If the world is not whole,  
the Engine refuses to act.
