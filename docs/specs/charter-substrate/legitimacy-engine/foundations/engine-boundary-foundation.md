# Charter Core — Engine Boundary Specification
Status: FROZEN  
Layer: Charter Core Engine  
Audience: Engine implementers, CLI maintainers, future contributors  
Purpose: Define the permanent structural boundary of the Charter Core engine to prevent architectural drift.

---

## 1. Intent

This document defines the immutable boundary of the Charter Core Engine.

It specifies:

- What responsibilities belong to the Engine  
- What responsibilities are explicitly excluded  
- The abstract storage contract  
- The deterministic execution and output model  
- The minimal query surface  
- Embedding invariants for external consumers  

This boundary ensures that:

- The Engine remains lean and stable  
- Storage and deployment concerns remain application-level  
- Legitimacy mechanics remain mechanically correct  
- The Engine does not expand beyond its core responsibility  

---

## 2. Core Design Principle

The Engine is a deterministic legitimacy processor operating on a single Area namespace at a time.

The Engine:

- Enforces legitimacy rules  
- Validates invariants  
- Transitions state  
- Produces deterministic outputs  

The Engine does NOT:

- Own persistence  
- Manage deployments  
- Implement federation  
- Perform guidance  
- Emit or manage audit systems  
- Interpret meaning  

The Engine is a legitimacy kernel — not an application framework.

---

## 3. Legitimacy Area Model

### 3.1 Area as Legitimacy Namespace

An Area is a legitimacy boundary.

An Area defines:

- Authority context  
- Scope governance context  
- Session concurrency domain  
- Resolution namespace  
- Reference namespace  

The Engine operates on exactly one Area context per invocation.

The Engine does not:

- Enumerate Areas  
- Coordinate multiple Areas  
- Perform cross-Area operations  

Area selection and orchestration are application responsibilities.

---

## 4. What Lives in the Engine

### 4.1 Domain Primitives

The Engine defines and owns:

- Resolution  
- Session  
- Candidate  
- Authority rule definitions (via governance artifacts)  
- Scope governance semantics  
- Session constraints  
- Status enums (ACTIVE, ON_HOLD, SUPERSEDED, RETIRED, etc.)  
- Supersession semantics  
- Informational metadata fields (e.g., annotation, reversibility_intent)

These define mechanical legitimacy.

Informational fields:

- must be preserved  
- must not influence legitimacy or structural evaluation  

---

### 4.2 State Transition Logic

The Engine must enforce:

- Authority evaluation rules  
- Explicit acceptance requirement  
- Session phase separation (evaluation vs commitment)  
- Deterministic round behavior  
- Concurrent session conflict resolution  
- ON_HOLD usability blocking rules  
- Supersession directionality  
- Explicit transition requirements  
- Immutable object guarantees  

Reversibility intent:

- is informational only  
- must not affect legitimacy or transitions  

All legitimacy-creating transitions must be validated here.

---

### 4.3 Deterministic Query Surface

The Engine exposes only queries required to:

- Evaluate legitimacy  
- Confirm invariant compliance  
- Retrieve canonical state  

Engine queries must:

- Be pure  
- Be deterministic  
- Never mutate state  
- Never infer intent  
- Never create legitimacy  

Evaluation outputs must be consistent with the deterministic reporting model (e.g., EvaluationReport).

Engine queries answer only:

“What is mechanically true?”

They do not summarize, interpret, or advise.

---

### 4.4 Execution Output Model

The Engine produces deterministic outputs, including:

- Evaluation results  
- Accepted artifacts (e.g., Resolutions)  
- Receipts (terminal artifacts of sessions)  

The Engine does not:

- Emit audit events  
- Persist outputs  
- Guarantee external observation  

External systems may observe Engine operations and record audit data, but:

- audit is not required for correctness  
- audit must not influence execution  
- audit must not be used for reconstruction of legitimacy  

---

## 5. Abstract Storage Interface

The Engine operates over an abstract storage adapter supplied by the embedding application.

The storage interface must support:

- Retrieving immutable objects by ID  
- Storing immutable objects  
- Reading references (named pointers)  
- Updating references atomically  

Optional capabilities (if provided):

- Deterministic listing of objects  

Storage must guarantee:

- Atomicity per Engine invocation  
- Deterministic reads  
- No hidden mutation  
- Append-only object semantics  

The Engine must not:

- Open files  
- Connect to databases  
- Assume filesystem layout  
- Assume network topology  

Persistence lifecycle belongs entirely to the application layer.

---

## 6. Indexing

Indexes are performance optimizations.

Indexes:

- May be used internally  
- Must be derivable from objects and references  
- Must never be a source of truth  

Loss of indexes must not alter legitimacy or evaluation outcomes.

Index persistence format is not part of Engine guarantees.

---

## 7. What Must NOT Live in the Engine

The Engine must never include:

- Commit abstraction (higher-layer concept)  
- Relay or synchronization logic  
- Federation or multi-Area coordination  
- Guidance or interpretation  
- UX-level queries  
- Deployment topology awareness  
- External authority inference  
- Meaning interpretation  
- Audit emission or audit persistence  

If a feature does not affect mechanical legitimacy within a single Area, it does not belong in the Engine.

---

## 8. Embedding Invariants

Any application embedding the Engine must guarantee:

- Only one Area context is provided per invocation  
- Storage adapter enforces atomic updates  
- No partial state is exposed to queries  
- Engine calls are treated as atomic operations  

Applications may:

- Maintain multiple Areas  
- Coordinate multiple Engine instances  
- Implement relay or federation  
- Wrap resolutions into higher-level constructs  
- Implement audit systems  
- Provide guidance or UX layers  

These must remain outside the Engine boundary.

---

## 9. Stability Guarantee

The Engine boundary defined in this document is stable.

Future changes may:

- Add new domain primitives required for legitimacy  
- Add new transitions consistent with invariants  
- Add new deterministic queries  

Future changes must not:

- Introduce storage coupling  
- Introduce federation logic  
- Introduce transport concerns  
- Introduce guidance logic  
- Blur Area boundaries  
- Blur legitimacy vs application concerns  

If a change affects application behavior but not mechanical legitimacy, it must occur outside the Engine.

---

## 10. Mental Model

The Engine is a deterministic legitimacy processor.

It:

- Operates within a single Area namespace  
- Enforces legitimacy rules  
- Evaluates and transitions state  
- Produces deterministic outputs  

Everything else — storage, transport, federation, audit, guidance, and user experience — belongs to higher layers.

This separation preserves:

- Determinism  
- Architectural clarity  
- Long-term stability  
- Mechanical correctness  

Nothing beyond mechanical legitimacy lives here.