# Charter Core — Determinism Foundation
## Identical Inputs, Identical Outcomes

Status: FOUNDATIONAL  
Layer: Conceptual / Engine Core  
Applies to: All Engine behavior — evaluation, mutation, reporting, receipts, and reconstruction  
Does NOT define: specific algorithms, data structures, or performance strategies  

---

## Purpose

This document defines **determinism** as a primary invariant of Charter Core.

Its purpose is to ensure that:

- legitimacy is reproducible across time and systems  
- evaluation produces stable and predictable results  
- mutation produces consistent and verifiable history  
- audit and receipts can be independently verified  
- no hidden factors influence outcomes  

Determinism is what allows Charter to function as a **legitimacy compiler** rather than a heuristic system.

---

## Core Principle

> The same inputs must always produce the same outputs.

This applies to:

- evaluation  
- mutation  
- reporting  
- reconstruction  

Determinism is not a performance property.  
It is a **correctness requirement**.

---

## Why Determinism Exists

Without determinism, the system would risk:

- conflicting interpretations of legitimacy  
- unverifiable audit trails  
- inconsistent cross-system behavior  
- hidden bias or environmental influence  
- inability to reproduce past outcomes  

Charter avoids these risks by enforcing strict input/output closure.

---

## Deterministic Inputs

For any Engine operation, determinism requires a fully defined input set.

This includes:

- **Domain graph**  
  All objects, relationships, and historical artifacts  

- **Command input**  
  The explicit operation being performed  

- **Specification identity (spec_set_hash)**  
  The exact rule set governing evaluation  

- **Runtime mode**  
  (e.g., NORMAL, DEGRADED_READ_ONLY)  

- **Canonical ordering context**  
  Any required deterministic ordering rules defined by the system  

Determinism requires that no relevant input be implicit.

---

## Deterministic Outputs

Given identical inputs, the Engine must produce identical:

- state transitions  
- EvaluationReports  
- receipts  
- derived state views  
- candidate status evaluations  
- ordering of results  
- error sets and ordering  

Outputs must not vary due to:

- execution timing  
- hardware differences  
- storage layout  
- iteration order of data structures  

---

## Deterministic Evaluation

Evaluation must be:

- pure  
- repeatable  
- side-effect free  

Repeated evaluation with identical inputs must:

- produce identical results  
- produce identical error ordering  
- produce identical candidate and session status  

Evaluation must not depend on:

- prior evaluations  
- cached intermediate results that alter behavior  
- environmental conditions  

---

## Deterministic Mutation

Mutation must be:

- governed by deterministic evaluation  
- applied atomically  
- reproducible given the same pre-state and command  

Mutation must produce:

- identical resulting objects  
- identical structural relationships  
- identical receipts  
- identical post-state  

No mutation path may depend on:

- timing  
- concurrency race conditions  
- storage ordering  
- non-deterministic iteration  

---

## Deterministic Ordering

Whenever ordering is required, it must be:

- explicitly defined  
- stable  
- reproducible  

Typical ordering rules include:

- lexicographic ordering of identifiers  
- canonical ordering defined by ENG-CANON  
- deterministic sorting of error codes and objects  

The Engine must not rely on:

- insertion order  
- memory layout  
- database return order  

If ordering is not defined, results must be treated as **sets**, not sequences.

---

## Prohibited Sources of Non-Determinism

The Engine must not depend on:

- randomness  
- system time for decision logic  
- environment variables  
- thread scheduling  
- parallel execution race conditions  
- non-deterministic hashing or iteration  
- external services  
- network responses  
- hardware-specific behavior  

Timestamps may exist, but:

- they must not influence outcomes  
- they are observational only  

---

## Hidden State Is Forbidden

The Engine must not maintain hidden internal state that affects outcomes.

This includes:

- implicit caches that alter results  
- incremental state that changes behavior across invocations  
- untracked global variables  

All state that affects behavior must be:

- explicit  
- reconstructible  
- part of the defined input set  

---

## Deterministic Reconstruction

Given:

- a complete history  
- a spec_set_hash  

The Engine must be able to:

- reconstruct state deterministically  
- reproduce past outcomes  
- verify receipts and legitimacy decisions  

Reconstruction must not require:

- external data  
- historical environment conditions  
- prior runtime instances  

---

## Determinism Across Systems

Different implementations of the Engine must:

- produce identical results for identical inputs  
- agree on legitimacy outcomes  
- agree on receipt content  
- agree on evaluation reports  

This ensures:

- portability  
- auditability  
- fork transparency  

---

## Determinism vs Performance

Determinism takes precedence over performance.

Optimizations are allowed only if they:

- do not alter observable behavior  
- preserve ordering guarantees  
- do not introduce hidden state dependencies  

Indexes, caches, and acceleration layers must be:

- derivable  
- discardable without changing outcomes  

---

## Failure of Determinism

If identical inputs produce different outputs:

- the system is invalid  
- legitimacy cannot be trusted  
- audit cannot be verified  

This is considered a critical failure of the Engine.

---

## Conceptual Invariants

- identical inputs produce identical outputs  
- evaluation is deterministic and repeatable  
- mutation is deterministic and reproducible  
- ordering is explicit and stable  
- no randomness influences outcomes  
- no hidden state influences behavior  
- reconstruction is deterministic  
- cross-system consistency is required  

---

## Mental Model

The Engine is a pure machine.

It does not guess.  
It does not drift.  
It does not depend on context you cannot see.

Given the same world and the same rules,  
it will always reach the same conclusion.