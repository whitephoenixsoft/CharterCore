# Charter Runtime — Legitimacy Engine Interaction Model

Status: FOUNDATIONAL (DRAFT)  
Applies to: Runtime Layer, Legitimacy Engine integration, Federation workflows  
Depends On: Runtime Foundation, Legitimacy Engine Specification, Provenance Model, Versioning & Identity Model  
Does NOT define: legitimacy semantics, decision rules, or session internal mechanics  

---

# 1. Purpose

This document defines how the Charter Runtime interacts with the Legitimacy Engine.

It exists to:

- define all valid engine interaction modes  
- separate legitimacy creation from legitimacy reconstruction  
- establish deterministic invocation boundaries  
- define Area bootstrap behavior  
- ensure no ambiguity between execution and compilation  

---

# 2. Core Principle

> The Legitimacy Engine creates or validates legitimacy.  
> Runtime prepares, invokes, and integrates.

Runtime:

- constructs inputs  
- invokes the engine  
- receives outputs  
- coordinates persistence and propagation  

It does not:

- evaluate legitimacy itself  
- reinterpret engine outcomes  

---

# 3. Engine Interaction Modes

Runtime must treat engine usage as **distinct modes**.

---

## 3.1 Execution Mode (Legitimacy Creation)

### Definition

Execution Mode is used when **new legitimacy is being created**.

---

### Inputs

Runtime provides:

- Area identifier  
- session definition  
- candidates (proposals)  
- participants  
- decision rules (authority)  
- constraints  
- structural references  

---

### Behavior

- engine evaluates session  
- applies legitimacy rules  
- determines acceptance or failure  

---

### Outputs

- resolution artifacts  
- session receipt  
- legitimacy result  

---

### Properties

- creates new legitimacy  
- requires governance alignment  
- must follow full session mechanics  

---

## 3.2 Evaluation Mode (Non-Legitimizing)

### Definition

Evaluation Mode is used to **analyze or validate** without creating legitimacy.

---

### Inputs

Same as Execution Mode.

---

### Behavior

- engine evaluates session deterministically  
- does not commit results  
- does not produce legitimate artifacts  

---

### Outputs

- validation results  
- blocking conditions  
- eligibility outcomes  

---

### Properties

- read-only  
- deterministic  
- non-legitimizing  

---

## 3.3 Incremental Compilation Mode (Federation / Reconstruction)

### Definition

Compilation Mode is used when **legitimacy was created elsewhere** and must be incorporated locally.

---

### Inputs

Runtime provides:

- completed session data  
- session receipt  
- resulting resolution artifacts  
- provenance metadata  
- rule identity  

---

### Behavior

- engine validates artifact consistency  
- reconstructs legitimacy state  
- integrates results into local Area  

---

### Outputs

- reconstructed legitimacy state  
- validation outcome  

---

### Properties

- does not create new legitimacy  
- does not re-evaluate decisions  
- preserves historical meaning  
- enforces anti-reinterpretation  

---

### Principle

> Compilation integrates history. It does not recreate it.

---

## 3.4 Mode Separation Invariant

- Execution Mode creates legitimacy  
- Compilation Mode reconstructs legitimacy  
- Evaluation Mode analyzes without creating legitimacy  

These must never be conflated.

---

# 4. Area Bootstrap

## 4.1 Definition

Area bootstrap defines how a new Area becomes usable.

---

## 4.2 Initial State

A new Area begins as:

- an identifier  
- an empty legitimacy structure  

---

## 4.3 Step 1 — Authority Definition

Authority must be defined first.

This includes:

- decision rule  
- participant model  
- governance constraints  

Properties:

- required for any session execution  
- consumed directly by the engine  

---

## 4.4 Step 2 — Scope Definition

Scope defines the semantic boundary of the Area.

Properties:

- describes intended domain or limits  
- not directly used as a decision rule by the engine  
- may be used by Runtime or host systems  

---

## 4.5 Bootstrap Constraint

No legitimacy creation may occur until:

- authority is defined  

---

# 5. Session Construction

## 5.1 Responsibility

Runtime is responsible for constructing valid session inputs.

---

## 5.2 Inputs

Runtime must assemble:

- candidates (proposals → candidates)  
- participants  
- decision rule  
- constraints  
- structural references  
- Area context  

---

## 5.3 Determinism

Session construction must be:

- explicit  
- reproducible  
- independent of execution timing  

---

## 5.4 Boundary

Runtime constructs sessions.

Engine evaluates sessions.

---

# 6. Invocation Model

## 6.1 Deterministic Invocation

Runtime must invoke the engine with:

- complete input  
- explicit parameters  
- no hidden state  

---

## 6.2 No Partial Execution

Engine invocation must be:

- atomic  
- complete  
- non-interleaved  

---

## 6.3 Failure Handling

Failures must be:

- returned explicitly  
- non-mutating  
- visible to Runtime  

---

# 7. Output Handling

## 7.1 Output Types

Engine outputs may include:

- resolution artifacts  
- session receipts  
- validation results  

---

## 7.2 Runtime Responsibilities

Runtime must:

- persist outputs when applicable  
- emit commit artifacts via CCS  
- maintain provenance and rule identity  
- integrate results into Area state  

---

## 7.3 No Reinterpretation

Runtime must not:

- reinterpret results  
- modify engine outputs  
- infer additional meaning  

---

# 8. Provenance & Rule Identity

## 8.1 Separation

- Engine outputs carry Engine Rule Identity  
- Runtime artifacts carry Runtime Rule Identity  

---

## 8.2 Compilation Requirements

Compilation Mode must validate:

- rule identity compatibility  
- artifact provenance  
- structural integrity  

---

## 8.3 Anti-Reinterpretation

If rule identity is unknown:

- artifacts may be stored  
- artifacts must not be reinterpreted  

---

# 9. Persistence Interaction

## 9.1 Optional Persistence

Runtime may operate:

- in-memory  
- with persistence  
- with mixed modes  

---

## 9.2 Execution Mode

- may produce immediate in-memory results  
- may emit durable artifacts via CCS  

---

## 9.3 Compilation Mode

- may integrate directly into local state  
- may persist artifacts depending on host configuration  

---

## 9.4 Determinism Requirement

Regardless of persistence:

- engine interaction must remain deterministic  

---

# 10. Invariants

- Runtime never creates legitimacy directly  
- Engine is the sole creator of legitimacy  
- Execution Mode creates legitimacy  
- Compilation Mode reconstructs legitimacy  
- Evaluation Mode does not create legitimacy  
- modes must never be conflated  
- session construction must be explicit  
- engine invocation must be deterministic  
- outputs must not be reinterpreted  
- rule identity must be preserved  
- provenance must be preserved  

---

# 11. Mental Model

Runtime is:

- the caller  
- the assembler  
- the integrator  

The Engine is:

- the evaluator  
- the arbiter  
- the legitimacy calculator  

---

# 12. Final Principle

All legitimacy flows through the engine.

Runtime ensures that:

- inputs are explicit  
- execution is controlled  
- outputs are preserved  

Legitimacy is never:

- implied  
- reconstructed incorrectly  
- or created outside defined modes