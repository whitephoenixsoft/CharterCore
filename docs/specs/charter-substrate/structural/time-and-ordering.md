# Charter Core — Time & Ordering Foundation
## Non-Authoritative Time, Deterministic Order

Status: FOUNDATIONAL  
Layer: Conceptual / Engine Core  
Applies to: Engine evaluation, mutation, receipts, audit, and reconstruction  
Does NOT define: timestamp formats, clock synchronization, or storage implementations  

---

## Purpose

This document defines how **time** and **ordering** are treated in Charter Core.

Its purpose is to ensure that:

- legitimacy is never derived from timestamps  
- ordering is deterministic and reproducible  
- historical records remain stable across systems  
- reconstruction does not depend on temporal interpretation  
- the system avoids ambiguity caused by time-based reasoning  

Charter separates **when something happened** from **what matters structurally**.

---

## Core Principle

> Time records events. Structure determines order.

Time answers:

> “When was this recorded?”

Structure answers:

> “How does this relate to legitimacy and precedence?”

Only structure is authoritative.

---

## Why This Separation Exists

Time is inherently unreliable for authority:

- clocks can drift  
- systems can disagree  
- events can be recorded out of order  
- imports can introduce older data later  
- distributed systems cannot guarantee global time consistency  

If time were used to determine legitimacy:

- outcomes would vary across systems  
- audit would become non-deterministic  
- reconstruction would become ambiguous  

Charter avoids this by:

- treating time as observational only  
- deriving all meaningful ordering from structure and rules  

---

## Time

## Definition

Time refers to timestamps or temporal metadata associated with events or artifacts.

---

## Role of Time

Time may be used to:

- record when an event was observed or recorded  
- support human understanding and audit readability  
- assist external systems (UI, logs, analytics)  

Time provides context, not authority.

---

## Properties of Time

Timestamps must be:

- non-authoritative  
- non-deterministic across systems (allowed)  
- excluded from decision logic  
- preserved as recorded facts  

Timestamps may differ across:

- systems  
- environments  
- rehydrations  

and still be valid.

---

## What Time Must Not Do

Time must not:

- determine legitimacy  
- determine acceptance outcomes  
- resolve conflicts between candidates or resolutions  
- define precedence between artifacts  
- influence authority evaluation  
- influence constraint evaluation  
- influence structural relationships  
- influence ordering in deterministic outputs  

No rule in the Engine may depend on timestamp comparison.

---

## Ordering

## Definition

Ordering is the deterministic arrangement of elements required for:

- evaluation  
- reporting  
- canonical representation  
- reconstruction  

Ordering is meaningful only when explicitly defined.

---

## Types of Ordering

### 1. Structural Ordering

Derived from:

- supersession relationships  
- session lifecycle  
- round indices  
- explicit structural rules  

This is authoritative.

---

### 2. Canonical Ordering

Defined for:

- serialization  
- hashing  
- deterministic output  

Examples include:

- lexicographic ordering of identifiers  
- defined ordering of fields  
- ordered error lists  

This ensures reproducibility.

---

### 3. Presentation Ordering

Used for:

- UI display  
- human readability  

This is non-authoritative and must not affect Engine behavior.

---

## Ordering Requirements

Whenever ordering is required, it must be:

- explicitly defined  
- deterministic  
- stable across executions  
- independent of storage or runtime artifacts  

If ordering is not explicitly defined:

- the result must be treated as a **set**, not a sequence  

---

## Prohibited Ordering Sources

The Engine must not derive ordering from:

- timestamps  
- insertion order  
- database return order  
- memory layout  
- iteration order of unordered structures  
- concurrency timing  

Any such dependence would break determinism.

---

## Relationship to Determinism

Deterministic ordering is required to ensure:

- identical outputs for identical inputs  
- stable hashing and canonical representation  
- reproducible EvaluationReports  
- consistent receipt generation  

Ordering is part of the deterministic contract of the system.

---

## Relationship to History

History records events in an append-only manner, but:

- historical sequence is not defined by timestamps  
- historical precedence is defined by structure (e.g., supersession, session closure)  

Receipts and structural relationships define what came “before” in a meaningful sense.

---

## Relationship to Audit

Audit may include timestamps and event sequences for:

- human inspection  
- debugging  
- traceability  

However:

- audit ordering must not define legitimacy  
- audit timestamps must not be used to resolve conflicts  
- audit is observational, not authoritative  

---

## Relationship to Evaluation

Evaluation must:

- rely only on structural and rule-defined ordering  
- ignore timestamps in decision-making  
- produce deterministically ordered outputs where required  

Evaluation must behave identically regardless of timestamp differences.

---

## Relationship to Mutation

Mutation may:

- record timestamps as metadata  
- append events to history  

But must not:

- use timestamps to determine behavior  
- rely on “latest by time” semantics  

All mutation decisions must be rule-based, not time-based.

---

## Reconstruction Independence

Reconstruction of state must be possible without:

- relying on timestamps  
- relying on original execution timing  
- relying on external clock systems  

Given history and rules, the same state must be derived regardless of time metadata.

---

## Conceptual Invariants

- timestamps are observational only  
- timestamps do not influence legitimacy  
- ordering is explicitly defined where required  
- ordering is deterministic and reproducible  
- structural relationships define precedence  
- time must not resolve conflicts  
- time must not influence evaluation or mutation  
- unordered results are treated as sets  

---

## Mental Model

Time tells you when something was recorded.

Structure tells you what it means.

If time disappeared entirely,  
the Engine must still produce the same outcomes.

Only structure carries authority.