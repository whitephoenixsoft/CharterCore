# Charter Core — Transaction & Atomicity Foundation
## All-or-Nothing State Transitions

Status: FOUNDATIONAL  
Layer: Conceptual / Engine Core  
Applies to: All mutating Engine commands, acceptance flows, session lifecycle changes, and persistence boundaries  
Does NOT define: storage engines, database transactions, or physical commit mechanisms  

---

## Purpose

This document defines how Charter Core performs **state transitions** safely and deterministically through transactions.

Its purpose is to ensure that:

- no partial legitimacy is ever created  
- all state changes are atomic and auditable  
- evaluation and mutation are cleanly separated  
- failures do not corrupt state  
- the system remains consistent under all conditions  

Transactions are the mechanism by which Charter turns **valid intent into committed history**.

---

## Core Principle

> Every command is atomic: it fully succeeds or has no effect.

There is no:

- partial success  
- partial legitimacy  
- partially applied mutation  

A command either:

- commits completely  
- or leaves the system unchanged  

---

## What a Transaction Is

A transaction is a **single Engine command execution** that may result in state mutation.

A transaction:

- begins with a consistent snapshot of state  
- performs full evaluation  
- determines whether mutation is allowed  
- commits all changes atomically if allowed  

Each transaction produces exactly one outcome:

- SUCCESS  
- REJECTED  
- BLOCKED  
- NO_OP  

(Outcome semantics are defined in ENG-ERROR.)

---

## Transaction Phases (Conceptual)

Every transaction follows a strict conceptual sequence:

### 1. Snapshot Acquisition

- A consistent, closed view of the current state is obtained  
- No partial or mixed state is allowed  

---

### 2. Evaluation

- All governing rules are applied  
- All violations are accumulated  
- Eligibility and blocking are determined  

No mutation occurs in this phase.

---

### 3. Outcome Determination

- The Engine determines whether mutation is allowed  
- If not allowed → transaction ends with no mutation  
- If allowed → proceed to commit  

---

### 4. Atomic Commit

- All state changes are applied together  
- All artifacts (e.g., receipts) are created  
- All invariants must hold  

This phase must be indivisible.

---

### 5. Visibility

- The new state becomes fully visible  
- No intermediate state is exposed  

---

## Atomicity Guarantees

A transaction must guarantee:

### All-or-Nothing Mutation

- Either all changes are applied  
- Or none are applied  

---

### Artifact Integrity

If a transaction succeeds:

- all required artifacts exist (e.g., receipts)  
- all structural relationships are valid  

If a transaction fails:

- no artifacts are created  

---

### No Partial State Exposure

At no point may the system expose:

- partially updated sessions  
- partially created resolutions  
- partially emitted receipts  
- incomplete structural relationships  

---

### Deterministic Commit Boundary

The boundary between:

- pre-state  
- post-state  

must be:

- clear  
- atomic  
- reproducible  

---

## Failure Behavior

If a transaction cannot complete successfully:

- all changes must be discarded  
- the system must revert to the pre-transaction state  

Failure may occur due to:

- evaluation failure (rules not satisfied)  
- integrity constraints  
- runtime mode restrictions  
- inability to guarantee atomic commit  

In all cases:

- no mutation may persist  
- no legitimacy may be created  

---

## No Partial Legitimacy Rule

This is critical:

> Legitimacy cannot exist in a partial form.

For example:

- a Resolution must not exist without its receipt  
- a session must not be marked ACCEPTED without full commit  
- supersession must not partially apply  

Either the entire legitimacy artifact set exists, or none of it does.

---

## Visibility Model

State visibility must follow:

- **Before commit:** changes are invisible  
- **After commit:** changes are fully visible  

There is no:

- “in-progress” state  
- “eventually visible” mutation  
- partially visible results  

All observers must see:

- either the old state  
- or the fully committed new state  

---

## Relationship to Evaluation

Evaluation and transaction are distinct:

- evaluation determines whether mutation is allowed  
- transaction performs mutation if allowed  

Transactions must not:

- bypass evaluation  
- mutate during evaluation  
- commit based on partial evaluation  

---

## Relationship to Consistency

Transactions require:

- consistent pre-state  
- consistent post-state  

A transaction must not:

- begin on inconsistent state  
- end in inconsistent state  

If consistency cannot be maintained:

- the transaction must fail  

---

## Relationship to Determinism

Transactions must be deterministic:

Given identical:

- pre-state  
- command input  
- spec_set_hash  

the transaction must produce identical:

- outcome  
- post-state  
- artifacts  

---

## Relationship to Persistence

This document defines **logical atomicity**, not physical implementation.

Persistence systems must ensure:

- atomic durability of committed state  
- no partial writes are externally visible  

The Engine must behave as if:

> commit is instantaneous and indivisible

---

## Relationship to Receipts

For terminal operations:

- receipt creation is part of the transaction  

A valid transaction must not produce:

- a receipt without state  
- state without a receipt (where required)  

Receipts and state transitions are bound together atomically.

---

## Relationship to Audit

Audit events (if implemented) must:

- correspond to committed transactions  
- not exist for failed transactions  

Audit must not reflect:

- partial attempts  
- failed mutations as completed actions  

---

## Concurrency (Conceptual)

The Engine assumes:

- one transaction operates on a given Area snapshot at a time  

If concurrency exists at the system level:

- it must not violate atomicity  
- it must not introduce race conditions into outcomes  

From the Engine’s perspective:

> each transaction is isolated and sequential

---

## Prohibited Behaviors

The Engine must never:

- partially commit changes  
- expose intermediate state  
- create partial legitimacy artifacts  
- retry partial commits silently  
- merge results from multiple incomplete transactions  
- allow overlapping commits to produce inconsistent state  

---

## Conceptual Invariants

- every transaction is atomic  
- no partial mutation is allowed  
- no partial legitimacy is allowed  
- evaluation precedes mutation  
- commit boundary is deterministic  
- state is either pre- or post-transaction  
- failure leaves state unchanged  
- artifacts and state changes are inseparable  

---

## Mental Model

A transaction is a sealed decision.

It is evaluated completely.  
It is committed completely.  
Or it does not exist at all.

There is no “almost.”