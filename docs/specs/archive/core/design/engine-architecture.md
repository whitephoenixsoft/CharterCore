# Charter Core — Engine Architecture Decision Record
Document ID: ENG-ARCH-01  
Status: FROZEN (v1)  
Audience: Charter Core engine implementers  
Scope: Internal engine architecture

---

## 1. Purpose

This document formalizes the internal architectural separation of the Charter Core engine into two conceptual subsystems:

- **Compiler**
- **Runtime**

This separation exists to provide:

- Clear mental models for engine behavior
- Deterministic reconstruction of governance state
- Efficient incremental evaluation of new events
- Stronger testability and property-based verification
- Long-term maintainability

The engine functions as a **deterministic governance computation system** that transforms historical objects and new events into legitimacy outcomes.

---

## 2. Core Principle

ENG-ARCH-01 — Engine Is Both Compiler and Runtime

The Charter engine operates in two distinct modes:

1. **Compilation Mode**
   - Reconstructs governance state from historical objects.

2. **Runtime Mode**
   - Evaluates new governance events against compiled state.

These modes MUST remain logically separated.

Fail if:

- Runtime performs full historical recomputation
- Compiler performs forward event evaluation
- Responsibilities overlap or become ambiguous

---

## 3. Conceptual Model

The engine follows a two-stage computation model.

Past → Compiler → Compiled State

Compiled State + New Event → Runtime → Updated State

The compiler processes historical facts.  
The runtime processes new decisions.

---

## 4. Compiler Responsibilities

The compiler reconstructs the canonical governance state from persisted objects.

It operates during:

- Engine rehydration
- Import validation
- fsck verification
- State reconstruction after failure

Compiler responsibilities include:

- Building the governance DAG
- Validating object integrity
- Constructing supersession graphs
- Determining resolution lifecycle states
- Building all runtime indexes
- Detecting structural violations (cycles, invalid references)

The compiler MUST treat objects as immutable facts.

Compiler outputs a deterministic **CompiledState** structure.

---

## 5. Compiler Output

The compiler produces a complete governance state representation.

CompiledState contains:

- Resolution State Index
- Supersession Graph
- Area Membership Index
- Authority Resolution Index
- Scope Resolution Index
- Session Outcome Index

This state is considered canonical until new events are applied.

---

## 6. Runtime Responsibilities

The runtime processes new governance events against compiled state.

Examples of runtime events:

- Starting sessions
- Recording stances
- Evaluating authority
- Accepting sessions
- Producing new resolutions

Runtime responsibilities include:

- Domain validation
- Authority evaluation
- Session acceptance logic
- Candidate freeze enforcement
- Incremental index updates
- Emission of new objects and audit events

Runtime operates only on **current compiled state plus new events**.

---

## 7. Runtime Constraints

ENG-ARCH-02 — Runtime Must Not Perform Global Rebuilds

Runtime MUST NOT:

- Rebuild the supersession graph
- Scan historical objects
- Recompute resolution states globally
- Recompile indexes from scratch

Runtime may only perform **incremental updates** to compiled state.

Fail if:

- Runtime performs full DAG reconstruction
- Runtime scans entire object history

---

## 8. Deterministic Rehydration

The engine must always be capable of deterministic state reconstruction.

Objects → Compiler → CompiledState

This guarantees:

- Crash recovery
- Import validation
- fsck correctness
- Deterministic replay

Compiled state must always be derivable solely from stored objects.

---

## 9. Engine Integration Flow

External storage provides objects and refs.

The engine workflow is:

1. Load persisted objects
2. Invoke compiler
3. Produce compiled governance state
4. Process runtime events
5. Emit new objects and audit events
6. Persist externally

The engine itself does not manage persistence.

---

## 10. Testing Strategy

The compiler/runtime separation enables two independent testing domains.

Compiler tests:

- Random DAG generation
- Structural validation
- Supersession correctness
- Index construction
- Deterministic rebuilds

Runtime tests:

- Session evaluation
- Authority checks
- Candidate freeze rules
- Acceptance behavior
- Incremental index updates

Property testing tools (e.g., proptest) should target both layers independently.

---

## 11. Recommended Internal Structure

The internal engine structure should reflect the conceptual split.

Suggested module layout:

charter_core/
    engine/
        compiler/
            dag_builder.rs
            index_builder.rs
            validation.rs

        runtime/
            session_eval.rs
            authority_check.rs
            acceptance.rs
            state_update.rs

        state/
            compiled_state.rs
            indexes.rs

        api/
            engine.rs

External callers interact only with the API layer.

---

## 12. Public Engine API

The public interface should expose minimal entry points.

Required operations include:

- rehydrate()
- evaluate_session()
- accept_session()

Internally:

- rehydrate() invokes the compiler
- evaluation and acceptance invoke runtime logic

---

## 13. Design Guarantees

This architecture guarantees:

- Deterministic governance state reconstruction
- Separation of historical computation and forward evaluation
- Efficient runtime operations
- Strong testability
- Clear mental models for developers

---

## 14. Mental Model (Locked)

Compiler:
Past → State

Runtime:
State + Event → New State

Objects are historical facts.  
The compiler reconstructs governance state.  
The runtime evaluates new legitimacy events.

---

## 15. Architectural Intent

The Charter engine is designed as a **deterministic governance computation engine**.

It behaves similarly to:

- a compiler (reconstructing state from immutable inputs)
- a runtime (evaluating new decisions)

This dual structure ensures the engine remains:

- deterministic
- auditable
- verifiable
- scalable
- maintainable

---

## 16. Architecture Invariants 

- Objects are immutable facts.
- Compilation is deterministic and reproducible.
- Runtime only performs incremental state transitions.
- Indexes are always derivable and disposable.
- Invalid transitions are rejected, never repaired.
- State can always be rebuilt from history alone.
