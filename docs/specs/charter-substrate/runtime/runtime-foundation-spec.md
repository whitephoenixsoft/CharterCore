# Charter Runtime — Foundation Specification

Status: FOUNDATIONAL (DRAFT)  
Applies to: All Runtime implementations, CLI orchestration, library integrations  
Depends On: Legitimacy Engine, CQL, CCS, Provenance Model, Versioning & Identity Model, Non-Interpretation Principle  
Does NOT define: legitimacy semantics, alignment computation, identity semantics, or transport protocols  

---

# 1. Purpose

The Charter Runtime is the **orchestration layer** of the Charter platform.

It exists to:

- coordinate workflows across substrates  
- manage operational state for Areas  
- invoke the Legitimacy Engine deterministically  
- mediate between investigative and legitimate structures  
- coordinate persistence across managed stores  
- expose unified query access via CQL  
- produce durable artifacts through CCS  

Runtime is where **process becomes structure**.

---

# 2. Core Principle

> Runtime orchestrates. It does not decide.

Runtime:

- executes workflows  
- coordinates state transitions  
- prepares inputs for legitimacy  
- persists outcomes  

It does not:

- create legitimacy  
- interpret meaning  
- infer intent  
- redefine substrate semantics  

---

# 3. Architectural Role

Runtime is the **local orchestration boundary** for Charter.

It sits between:

- investigative systems (CDS)  
- legitimacy creation (Legitimacy Engine)  
- durable artifacts (CCS / Commit Store)  
- derived systems (CSG, CAS, CCare, etc.)  
- query interface (CQL)  

---

## 3.1 Flow Position

Runtime participates in the lifecycle as:

Investigation → Runtime → Legitimacy Engine → CCS → Commit Store → Derived Substrates

---

## 3.2 Responsibilities

Runtime is responsible for:

- workflow orchestration  
- process state management  
- persistence coordination  
- engine invocation  
- cross-substrate boundary enforcement  
- artifact emission through CCS  
- exposing query surfaces via CQL  

---

# 4. Process-Oriented Design

## 4.1 Process Engine Model

Runtime operates as a **process-driven system**.

Each workflow is represented as:

- a state machine  
- with explicit states  
- explicit transitions  
- explicit commands  

---

## 4.2 Process Isolation

Each process:

- operates within a scoped workspace  
- maintains its own state  
- does not implicitly affect other processes  

---

## 4.3 Process Types

Examples include:

- Incoming Reconciliation Review  
- Session Orchestration  
- Deliberate (CDS integration)  
- Import / Intake  
- Restore / Rehydration  
- Commit Emission  

Each process must be:

- explicitly defined  
- auditable  
- resumable (if supported)  
- deterministic in transitions  

---

# 5. Area Model

## 5.1 Area as Operational Boundary

Runtime organizes work around **Areas**.

An Area defines:

- authority model  
- scope  
- active processes  
- local operational context  

---

## 5.2 Area Constraints

- governance (authority + scope) applies to all processes  
- processes must respect Area constraints  
- Area changes may invalidate active processes  

---

## 5.3 Concurrency

Runtime may allow multiple processes per Area.

However:

> Interfering processes must be blocked.

Interference includes:

- overlapping structural targets  
- conflicting supersession paths  
- shared dependency mutations  
- competing legitimacy inputs  

---

# 6. Persistence Coordination

## 6.1 Principle

Runtime coordinates persistence across **multiple managed stores**.

There is no single “Runtime Store.”

---

## 6.2 Managed Store Types

Runtime interacts with:

- legitimacy object stores (engine-managed)  
- review store  
- import store  
- CDS workspace stores  
- commit store (via CCS)  
- graph store (via CSG)  
- alignment store (via CAS)  
- audit store  
- CSP pipeline/feed stores  
- CRS untrusted stores  

---

## 6.3 Store Responsibility

Each store:

- owns its data model  
- enforces its invariants  
- exposes a read surface for CQL  

Runtime:

- coordinates writes  
- does not redefine store semantics  

---

## 6.4 Store Update Principle

> State changes must be written to managed stores immediately.

This ensures:

- deterministic querying via CQL  
- no reliance on implicit in-memory state  
- auditability of process evolution  

---

# 7. Legitimacy Engine Integration

## 7.1 Invocation Model

Runtime uses the Legitimacy Engine as:

> a deterministic computation engine

Runtime:

- prepares inputs (sessions, candidates)  
- invokes the engine  
- receives outputs (resolutions, receipts)  

---

## 7.2 Boundary Rules

Runtime must not:

- alter engine rules  
- reinterpret engine outcomes  
- bypass session execution  

---

## 7.3 Session Discipline

All legitimacy creation must occur through:

- session execution  
- authority-aligned decision rules  
- explicit candidate evaluation  

---

# 8. Commit Integration (CCS)

## 8.1 Principle

All durable artifacts must be emitted through CCS.

---

## 8.2 Commit-Eligible Events

Examples include:

- resolution creation  
- session receipts  
- review closure  
- deliberate closure  
- signal creation (CCare)  
- lineage artifacts (CIS)  

---

## 8.3 Closure Rule

> Artifacts that become part of durable history must be committed.

---

## 8.4 Runtime Responsibility

Runtime:

- constructs commit artifacts  
- passes them through CCS  
- ensures identity and lineage are preserved  

---

# 9. Query Integration (CQL)

## 9.1 Role

Runtime exposes query access through CQL.

---

## 9.2 Query Model

CQL:

- queries managed read surfaces  
- operates on store-backed data  
- is deterministic and read-only  

---

## 9.3 Runtime Responsibility

Runtime must:

- expose queryable surfaces for processes  
- ensure process state is persisted for queryability  
- support domain-based querying  

---

# 10. Governance Enforcement

## 10.1 Authority

Runtime enforces that:

- all legitimacy-related actions align with Area Authority  
- decision rules match governance requirements  

---

## 10.2 Scope

Runtime enforces:

- Area Scope constraints  
- blocking when scope is ON_HOLD  
- invalidation when scope changes materially  

---

## 10.3 Blocking Behavior

Processes must be blocked when:

- governance constraints are not met  
- structural conditions are invalid  
- dependencies are unresolved  

Blocked processes must:

- remain auditable  
- require explicit resume  

---

# 11. Provenance & Rule Identity

## 11.1 Runtime Rule Identity

Runtime must expose a deterministic **rule identity**.

---

## 11.2 Artifact Stamping

Runtime-generated artifacts (e.g., review receipts) must include:

- runtime rule identity  
- provenance metadata  

---

## 11.3 Separation of Identities

- Runtime artifacts carry Runtime Rule Identity  
- Engine artifacts carry Engine Rule Identity  

These must remain distinct.

---

# 12. Non-Interpretation

Runtime must adhere to the Non-Interpretation Principle.

Runtime must not:

- infer meaning  
- infer intent  
- resolve ambiguity implicitly  
- create relationships automatically  

All structure must be:

- explicitly declared  
- explicitly accepted  
- explicitly recorded  

---

# 13. Determinism

Runtime behavior must be deterministic given:

- identical inputs  
- identical rule identities  
- identical process states  

Runtime must not depend on:

- execution timing  
- implicit ordering  
- hidden mutable state  

---

# 14. Auditability

All Runtime processes must be:

- fully traceable  
- reconstructible from stored state  
- queryable via CQL  

---

# 15. Invariants

- Runtime does not create legitimacy  
- Runtime orchestrates all workflows  
- all legitimacy flows through the engine  
- all durable artifacts flow through CCS  
- all query access flows through CQL  
- process state must be persisted  
- structural ambiguity must be explicit  
- governance constraints must be enforced  
- blocking must be explicit  
- no implicit mutation of structure  
- rule identity must be preserved  
- provenance must not be lost  

---

# 16. Mental Model

Runtime is:

- the conductor of the system  
- the coordinator of processes  
- the boundary between thinking and decision  
- the manager of operational truth  

It is not:

- a decision-maker  
- an interpreter  
- a source of authority  

---

# 17. Final Principle

Runtime ensures that:

- nothing becomes legitimate without process  
- nothing is persisted without structure  
- nothing is interpreted without being recorded  

It is the system that turns **workflow into durable truth**  
without ever becoming the source of that truth.