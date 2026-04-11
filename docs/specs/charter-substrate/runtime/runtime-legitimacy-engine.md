# Charter Runtime — Legitimacy Engine Interaction Model

Status: FOUNDATIONAL (DRAFT v2)  
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
- define proposal-derived session materialization  
- define provisional execution and output holding during batch processing  
- ensure no ambiguity between execution, evaluation, and compilation  

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

Runtime must treat engine usage as distinct modes.

---

## 3.1 Execution Mode (Legitimacy Creation)

### Definition

Execution Mode is used when new legitimacy is being created.

---

### Inputs

Runtime provides:

- Area identifier  
- session definition  
- candidates  
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

Evaluation Mode is used to analyze or validate without creating legitimacy.

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

Compilation Mode is used when legitimacy was created elsewhere and must be incorporated locally.

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

- candidates  
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

## 5.5 Proposal-Derived Session Materialization

### Definition

Proposal-derived session materialization defines how approved workflow output is transformed into engine sessions.

This is especially relevant for incoming reconciliation review.

---

### Core Principle

> Approved review output is mechanically transformed into executable engine sessions.

This transformation:

- is deterministic  
- does not introduce new decisions  
- does not require additional voting  

---

### Derivation Rule

Each accepted proposal produces:

- one single-candidate engine session  

Properties:

- one accepted proposal → one session  
- derived sessions are Runtime-created  
- derived sessions inherit the approved review context  

---

### Inherited Context

Each derived session must copy from the approved review round:

- participants  
- decision rule alignment  
- constraints  
- final voting stances  

---

### Stance Reuse

Approved review stances are reused as session stances.

Implications:

- no second voting phase occurs per proposal  
- session acceptance becomes deterministic from the approved review round  
- Runtime does not ask users to vote again during materialization  

---

### Proposal → Candidate Transformation

Each proposal is transformed into a candidate, including:

- candidate action payload  
- informational internal Area references  
- informational cross-Area references  
- lineage fields where applicable  
- source annotations and provenance  

---

### Structural Preconditions

Before derived sessions may be invoked:

- no two accepted proposals may supersede the same resolution  
- no circular legitimacy dependencies may exist  
- deterministic dependency ordering must be computable  

If these fail:

- materialization fails before execution begins  

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

- atomic at the individual session level  
- complete  
- non-interleaved within its execution unit  

---

## 6.3 Failure Handling

Failures must be:

- returned explicitly  
- non-mutating  
- visible to Runtime  

---

## 6.4 Batch Ordering

When multiple derived sessions are executed as a batch, ordering must be deterministic.

Ordering should prefer:

- dependency-producing sessions first  
- relationship-establishing sessions first  
- sessions that later sessions depend on first  

---

## 6.5 Sandboxed Batch Execution

When Runtime executes a derived batch of sessions:

- execution must occur in an isolated sandbox  
- engine state must remain isolated from live Area state  
- outputs must remain provisional during execution  
- no legitimacy-bearing output becomes externally visible during provisional execution  

---

## 6.6 Provisional Output Holding

During sandboxed batch execution, Runtime must hold provisionally:

- resulting resolutions  
- resulting session receipts  
- commit-ready artifacts  
- batch-level aggregation data  

These outputs must not be:

- committed  
- published  
- integrated into live Area state  

until the full batch succeeds.

---

## 6.7 Batch Atomicity

When derived sessions are executed as an accepted batch:

- all sessions must succeed  
- or none of their outputs may take effect  

If any session fails:

- the full batch fails  
- all provisional outputs are discarded  
- no partial legitimacy effect is allowed  
- no commit artifacts are emitted  
- Runtime returns control to the originating process for block/resume handling  

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
- integrate results into Area state only after success in the applicable mode  

---

## 7.3 Execution Finalization

In Execution Mode, once session execution succeeds:

- resulting legitimacy artifacts become valid engine outputs  
- Runtime may emit durable artifacts through CCS  
- Runtime may integrate the results into live Area state  

For batch-derived execution, this occurs only after the full batch succeeds.

---

## 7.4 Compilation Finalization

In Compilation Mode:

- integrated legitimacy remains historical/local reconstruction  
- Runtime must not treat compilation as new legitimacy creation  
- Runtime may persist reconstructed state according to host configuration  

---

## 7.5 No Reinterpretation

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

For batch-derived execution:

- provisional outputs must be held until the full batch succeeds  

---

## 9.3 Compilation Mode

- may integrate directly into local state  
- may persist artifacts depending on host configuration  

---

## 9.4 Evaluation Mode

- must not emit durable legitimacy artifacts  
- must not mutate live legitimacy state  

---

## 9.5 Determinism Requirement

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
- approved workflow stances may be reused only through explicit materialization rules  
- proposal-derived sessions must be deterministic  
- batch execution must be sandboxed when provisional aggregation is required  
- provisional outputs must not escape before full batch success  
- outputs must not be reinterpreted  
- rule identity must be preserved  
- provenance must be preserved  

---

# 11. Mental Model

Runtime is:

- the caller  
- the assembler  
- the materializer  
- the integrator  

The Engine is:

- the evaluator  
- the arbiter  
- the legitimacy calculator and compiler  

---

# 12. Final Principle

All legitimacy flows through the engine.

Runtime ensures that:

- inputs are explicit  
- execution is controlled  
- batch execution is isolated when needed  
- outputs are preserved without reinterpretation  

Legitimacy is never:

- implied  
- reconstructed incorrectly  
- or created outside defined modes