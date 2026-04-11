# Charter Runtime — Foundation Specification

Status: FOUNDATIONAL (DRAFT v3)  
Applies to: All Runtime implementations, CLI orchestration, library integrations  
Depends On: Legitimacy Engine, CQL, CCS, Provenance Model, Versioning & Identity Model, Non-Interpretation Principle  
Does NOT define: legitimacy semantics, alignment computation, identity semantics, or transport protocols  

---

# 1. Purpose

The Charter Runtime is the **host-facing operational gateway** to the Charter platform.

It exists to:

- allocate and manage context workspaces  
- host Areas as operational legitimacy boundaries  
- expose access to Charter substrates and components  
- coordinate workflows and process execution  
- mediate persistence across multiple managed surfaces when persistence is enabled  
- invoke the Legitimacy Engine deterministically  
- invoke the Legitimacy Engine in distinct interaction modes  
- emit durable artifacts through CCS  
- expose unified query access via CQL  

Runtime is where:

> host interaction becomes structured system behavior.

---

# 2. Core Principle

> Runtime orchestrates access, state, and process. It does not create legitimacy or meaning.

Runtime:

- coordinates workflows  
- manages operational state  
- prepares inputs for legitimacy  
- coordinates persistence when enabled  
- integrates engine outputs without reinterpreting them  

It does not:

- create legitimacy  
- interpret meaning  
- infer intent  
- redefine substrate semantics  

---

# 3. Architectural Role

Runtime is the **integration boundary** between the host and the Charter system.

It provides unified access to:

- Legitimacy Engine  
- CDS (Deliberate)  
- CCS / Commit Store  
- derived substrates (CSG, CAS, CIS, CCare)  
- CSP pipelines and feeds  
- CRS untrusted stores  
- audit and metadata systems  

---

## 3.1 Flow Position

Runtime sits at the center of all operational flows:

Host → Runtime → Substrates → Runtime → CCS → Commit Store → Derived Systems

---

## 3.2 Responsibilities

Runtime is responsible for:

- workspace allocation and lifecycle  
- Area management  
- process orchestration  
- persistence coordination when enabled  
- engine invocation and integration  
- artifact emission  
- query exposure  

---

# 4. Workspace Model

## 4.1 Context Workspaces

Runtime allocates **context workspaces** as the physical and logical container for execution.

A workspace defines:

- allocation boundaries  
- process isolation  
- local operational context  
- access to managed surfaces  

---

## 4.2 Workspace Properties

Workspaces are:

- isolated  
- recoverable when persistence is enabled  
- host-scoped  
- non-legitimizing  

---

## 4.3 Workspace Responsibility

Runtime is responsible for:

- workspace creation  
- workspace lifecycle  
- mapping processes and Areas into workspaces  

---

# 5. Area Model

## 5.1 Area as Legitimacy Boundary

An Area represents a **local legitimacy boundary**.

From the Legitimacy Engine perspective:

- an Area may begin as an empty structure with only an identifier  

From the Runtime perspective:

- an Area is an operational container with governance, state, and processes  

---

## 5.2 Area Responsibilities

Runtime manages:

- Area lifecycle  
- governance context (authority + scope)  
- active processes within the Area  
- managed surfaces associated with the Area  

---

## 5.3 Area Bootstrap

A newly created Area begins as an empty legitimacy boundary.

Runtime is responsible for bootstrapping that Area through ordered governance initialization.

### Step 1 — Authority Definition

Authority must be defined first.

Authority provides:

- the decision rule  
- the participant/governance model  
- legitimacy-enabling governance mechanics  

No legitimacy creation may occur until Authority is defined.

### Step 2 — Scope Definition

Scope defines the semantic boundary of the Area.

Scope:

- describes intended domain and limits  
- is used by Runtime and hosts to understand the boundary  
- is not a decision rule in the same sense as Authority  

---

## 5.4 Mutable vs Immutable State

Runtime manages:

- mutable operational state (workspaces, processes, sessions, reviews)  
- coordination of immutable artifact formation (closure, commits)  

Runtime does not redefine artifact semantics.

---

## 5.5 Area Constraints

- governance applies to all Area processes  
- scope may block or invalidate processes  
- authority defines decision rules  

---

## 5.6 Concurrency

Runtime may allow multiple processes per Area.

However:

> Interfering processes must be blocked.

Interference includes:

- overlapping structural targets  
- competing supersession paths  
- dependency conflicts  
- competing legitimacy inputs  

---

# 6. Process-Oriented Execution

## 6.1 Process Model

Runtime operates as a **process engine**.

Each workflow is represented as:

- a state machine  
- explicit states  
- explicit transitions  
- explicit commands  

---

## 6.2 Process Types

Examples:

- Incoming Reconciliation Review  
- Session Orchestration  
- Deliberate (CDS integration)  
- Import / Intake  
- Restore / Rehydration  
- Commit Emission  

---

## 6.3 Process Guarantees

Processes must be:

- explicit  
- auditable  
- deterministic in transitions  
- resumable (if supported)  

---

# 7. Persistence Coordination Model

## 7.1 Principle

Persistence is optional at Runtime.

Runtime may operate:

- entirely in memory  
- with local persisted state  
- with durable artifact emission through CCS  
- in mixed host-selected modes  

Runtime does not own a single persistence layer.

Instead:

> Runtime coordinates multiple managed surfaces across the platform when persistence is present.

---

## 7.2 Managed Surface Categories

### Runtime-Managed Surfaces

- Session Store or in-memory session surface  
- Review Workspace Store or in-memory review surface  
- Deliberate Workspace Store or in-memory deliberate surface  
- Deliberate Artifact Store  
- Ref Store or ref surface  
- Metadata Store (host + local configuration)  
- Audit Store or audit surface  

---

### Substrate-Owned Surfaces

- Commit Store (CCS — immutable artifact truth)  
- Graph Store (CSG)  
- Alignment Store (CAS)  
- Identity Store (CIS)  
- Care Signal Stores (CCare)  
- CSP Pipeline / Feed Stores  
- CRS Untrusted Artifact Stores  

---

## 7.3 Surface Responsibilities

Each managed surface:

- owns its data model  
- enforces its invariants  
- may expose a query surface for CQL  

Runtime:

- coordinates writes and updates where applicable  
- enforces workflow boundaries  
- does not redefine semantics  

---

## 7.4 Mutability Model

Charter operational storage/surfaces follow strict categories:

- mutable workspace state  
- immutable local artifacts  
- immutable committed artifacts (CCS)  
- append-only audit  
- mutable metadata  
- derived indexes  

---

## 7.5 Persistence Rule

When persistence is enabled:

> State changes must be written to managed surfaces immediately.

This ensures:

- deterministic querying via CQL  
- auditability  
- recovery without hidden state  

When persistence is not enabled:

- runtime state must still remain deterministic within the active host context  

---

## 7.6 Cross-Domain Rule

> No implicit cross-domain persistence is allowed.

All cross-domain data movement must occur through:

- explicit workflows  
- explicit review processes  
- explicit engine execution  
- explicit compilation or restore paths  

---

## 7.7 Foreign Artifact Boundary

Foreign artifacts:

- must remain isolated  
- must not be implicitly trusted  
- must pass through reconciliation, compilation, or other explicit workflows  

---

## 7.8 Recovery Model

When persistence is enabled, Runtime should support deterministic recovery:

1. load managed stores  
2. verify integrity  
3. rebuild refs  
4. rebuild indexes  

Derived systems may be rebuilt as needed.

---

# 8. Legitimacy Engine Integration

## 8.1 Invocation Model

Runtime uses the Legitimacy Engine as:

> a deterministic legitimacy computation and compilation system

---

## 8.2 Interaction Modes

Runtime must distinguish engine interaction modes explicitly.

### Execution Mode

Used when new legitimacy is being created through session execution.

### Evaluation Mode

Used when legitimacy-related inputs are being validated or analyzed without creating legitimacy.

### Incremental Compilation Mode

Used when completed legitimacy artifacts created elsewhere are being integrated locally without re-deciding them.

These modes must never be conflated.

---

## 8.3 Responsibilities

Runtime:

- constructs sessions  
- provides candidates  
- invokes the engine  
- receives outputs  
- integrates results according to interaction mode  

---

## 8.4 Constraints

Runtime must not:

- alter engine rules  
- reinterpret engine outcomes  
- bypass session execution when new legitimacy is required  
- treat compilation as execution  
- treat evaluation as legitimacy creation  

---

## 8.5 Session Materialization

When Runtime transforms approved workflow output into engine sessions:

- session construction must be explicit  
- proposal/candidate translation must be deterministic  
- governance context must remain aligned  
- structural ordering requirements must be respected  

---

## 8.6 Sandboxed Batch Execution

When Runtime executes a derived batch of sessions:

- execution must occur in an isolated sandbox  
- outputs remain provisional until the full batch succeeds  
- no commits are emitted during provisional execution  
- no legitimacy-bearing outputs become externally visible before finalization  

If any session in the batch fails:

- the batch fails as a whole  
- provisional outputs are discarded  
- no partial legitimacy effect is allowed  

---

# 9. Commit Integration (CCS)

## 9.1 Principle

All durable artifacts must be emitted through CCS.

---

## 9.2 Closure Rule

> Artifacts that become part of durable history must be committed.

Examples:

- resolutions  
- session receipts  
- review closure  
- deliberate closure  
- signals  
- lineage artifacts  

---

## 9.3 Runtime Responsibility

Runtime:

- constructs commit artifacts  
- passes them through CCS  
- preserves identity and lineage  
- may hold commit-ready artifacts provisionally until workflow or batch finalization succeeds  

---

# 10. Query Integration (CQL)

## 10.1 Role

Runtime exposes queryable system state through CQL.

---

## 10.2 Query Model

CQL:

- queries domain-owned read surfaces  
- is deterministic  
- is read-only  
- is not limited to persisted storage  

---

## 10.3 Runtime Responsibility

Runtime must:

- expose process and Area state through domain query surfaces  
- support domain-based query resolution  
- preserve deterministic query behavior whether data is in memory, persisted, derived, isolated, or untrusted  

---

# 11. Governance Enforcement

Runtime enforces:

- Area Authority  
- Area Scope  
- decision rule alignment  

---

## 11.1 Blocking

Processes must be blocked when:

- scope is ON_HOLD  
- authority constraints are unmet  
- structural conditions are invalid  
- governance context changes materially  
- sandboxed batch execution fails and requires resume/recovery  

Blocked processes must:

- remain visible  
- require explicit resume  

---

# 12. Provenance & Rule Identity

## 12.1 Runtime Rule Identity

Runtime must expose a deterministic rule identity.

---

## 12.2 Artifact Stamping

Runtime artifacts must include:

- runtime rule identity  
- provenance metadata  

---

## 12.3 Separation

- Runtime artifacts → Runtime Rule Identity  
- Engine artifacts → Engine Rule Identity  

---

# 13. Non-Interpretation

Runtime must not:

- infer intent  
- infer meaning  
- create relationships implicitly  

All structure must be explicit.

---

# 14. Determinism

Runtime must be deterministic given:

- identical inputs  
- identical rule identities  
- identical operational state  
- identical interaction mode selection  

---

# 15. Auditability

All Runtime behavior must be:

- traceable  
- reconstructible when persistence is enabled  
- queryable  

---

# 16. Invariants

- Runtime is the host gateway to Charter  
- Runtime orchestrates all workflows  
- Runtime does not create legitimacy  
- all new legitimacy flows through engine execution  
- legitimacy created elsewhere is integrated through explicit compilation paths  
- evaluation does not create legitimacy  
- engine interaction modes must remain distinct  
- all durable artifacts flow through CCS  
- all queries flow through CQL  
- persistence is optional  
- when persistence is enabled, state changes must be written explicitly  
- no implicit cross-domain writes  
- structural ambiguity must be explicit  
- governance must be enforced  
- sandboxed batch execution must not leak partial outputs  
- rule identity must be preserved  
- provenance must not be lost  

---

# 17. Mental Model

Runtime is:

- the system boundary  
- the orchestrator of processes  
- the manager of operational state  
- the gateway to all substrates  
- the caller and integrator of the Legitimacy Engine  

It is not:

- a source of authority  
- a decision-maker  
- a meaning engine  

---

# 18. Final Principle

Runtime ensures that:

- hosts interact with Charter safely  
- workflows are explicit and auditable  
- legitimacy is created only through the engine  
- previously created legitimacy is integrated without reinterpretation  
- structure is preserved across all layers  

It is the system that turns:

> interaction → process → structure → durable truth  

without ever becoming the source of that truth.