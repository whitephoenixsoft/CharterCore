# Charter Core — Incremental Compilation Foundation

Status: FOUNDATIONAL  
Layer: Conceptual / Engine-Adjacent  
Applies to: Engine runtime, import flows, reconstruction, and historical replay  
Does NOT define: storage formats, batching APIs, or persistence mechanics  

---

## Purpose

Incremental compilation defines how Charter reconstructs and validates a legitimacy graph over time from a sequence of inputs.

It exists to ensure that:

- historical legitimacy is preserved exactly as it was created  
- reconstruction is deterministic  
- partial data can be safely processed in stages  
- large histories can be ingested without requiring full recomputation  

Incremental compilation is not decision-making.  
It is **deterministic reconstruction of already-decided truth**.

---

## Core Principle

> Incremental compilation replays legitimacy.  
> It does not reinterpret it.

Given the same inputs in the same order, incremental compilation must always produce the same structural state and legitimacy outcomes.

---

## Mental Model

Incremental compilation is a **ledger replay system**.

- Each input batch adds known facts  
- The engine integrates those facts into the current graph  
- Previously accepted legitimacy is never re-decided  
- The system advances monotonically toward a complete state  

It answers:

> “Given everything that has already happened, what is the current structural truth?”

---

## What Incremental Compilation Does

Incremental compilation:

- Accepts domain objects in batches  
- Integrates them into the existing Area graph  
- Validates structural integrity at each step  
- Reconstructs supersession and lineage relationships  
- Verifies receipts and historical artifacts  
- Maintains deterministic state across partial ingestion  

It allows the system to:

- build state gradually  
- resume from partial progress  
- process large histories safely  

---

## What Incremental Compilation Does Not Do

Incremental compilation does NOT:

- create new legitimacy  
- re-evaluate past acceptance decisions  
- reinterpret votes or authority outcomes  
- infer missing decisions  
- repair invalid historical data  
- depend on timestamps for authority  

If legitimacy is not already present in the inputs,  
incremental compilation must not manufacture it.

---

## Determinism Requirement

Incremental compilation must be fully deterministic.

Given identical:

- input batches  
- ordering of batches  
- domain objects  
- specification set  

the resulting state must be identical.

It must not depend on:

- processing time  
- system clocks  
- insertion order within unordered sets  
- environment-specific behavior  

---

## Ordering Semantics

Incremental compilation depends on **explicit ordering of inputs**.

Rules:

- Batches must be processed in a deterministic order  
- Within a batch, object ordering must not affect outcome  
- Historical causality must be preserved through ordering  

Timestamps are informational only.  
They must not determine legitimacy or precedence.

---

## Partial State Safety

At any point during incremental compilation:

- the system may be in a **partial but valid** state  
- previously validated structure must remain valid  
- incomplete dependencies must be explicitly detectable  

The system must never:

- silently assume missing data  
- produce inferred legitimacy  
- collapse incomplete structures into valid ones  

Partial state is allowed.  
Silent completion is not.

---

## Validation During Compilation

Each incremental step must:

- validate structural integrity  
- validate reference correctness  
- validate receipt consistency  
- detect conflicts deterministically  

Failures must:

- be explicit  
- not mutate prior valid state  
- not produce partial legitimacy  

The system either:

- advances deterministically  
- or reports failure deterministically  

---

## Relationship to Receipts

Receipts are the authoritative record of past legitimacy.

Incremental compilation must:

- treat receipts as fixed historical artifacts  
- verify receipt integrity and consistency  
- reconstruct state based on receipt content  

It must not:

- reinterpret receipt meaning  
- recompute acceptance decisions  
- override recorded outcomes  

Receipts are replayed, not re-judged.

---

## Relationship to Sessions

Sessions represent the process by which legitimacy was created.

During incremental compilation:

- session outcomes are consumed as facts  
- accepted sessions produce structural effects  
- closed sessions produce historical records  

Compilation does not:

- reopen sessions  
- modify session outcomes  
- simulate alternative decisions  

---

## Failure Model

If invalid data is encountered:

- the system must fail deterministically  
- the failure must be explicit and reproducible  
- no hidden repair or correction may occur  

The system must not:

- skip invalid data silently  
- attempt best-effort reconstruction  
- degrade correctness for availability  

Correctness is prioritized over completion.

---

## Idempotency

Incremental compilation must be idempotent.

Re-processing the same batch:

- must not change the resulting state  
- must not duplicate objects  
- must not alter structural relationships  

The system must recognize and safely handle repeated inputs.

---

## Isolation from Runtime Decisions

Incremental compilation is separate from live session evaluation.

It:

- reconstructs past decisions  
- does not participate in current decision-making  

Live sessions:

- create legitimacy  
- operate independently of compilation  

Compilation reflects history.  
Sessions create it.

---

## Conceptual Invariants

- Legitimacy is never created during compilation  
- Historical decisions are never re-evaluated  
- Determinism is absolute  
- Ordering is explicit and controlled  
- Partial states are allowed but never inferred  
- Failures are explicit and non-mutating  
- Receipts are authoritative historical artifacts  
- Compilation is idempotent  

---

## Summary

Incremental compilation is the process by which Charter rebuilds its world from history.

It:

- replays decisions without reinterpretation  
- preserves legitimacy exactly as created  
- enables scalable, staged reconstruction  
- guarantees deterministic outcomes  

It is not a decision engine.

It is the mechanism that ensures that what was decided  
remains exactly what is known.