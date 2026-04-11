# Charter CLI — Foundation Specification

Status: FOUNDATIONAL (DRAFT)  
Applies to: Charter CLI (Primary Host Interface)  
Depends On: Runtime Foundation, CQL, CCS, Provenance Model, Versioning & Identity Model, Non-Interpretation Principle  
Does NOT define: command syntax, legitimacy semantics, or workflow-specific rules  

---

# 1. Purpose

The Charter CLI is the **primary host implementation** of the Charter Runtime.

It exists to:

- provide a human interface to Charter systems  
- manage local Charter state and context  
- coordinate Runtime execution cycles  
- expose workflows and processes  
- provide query access through CQL  
- ensure safe and explicit interaction with all substrates  

---

# 2. Core Principle

> The CLI is a context-aware host that loads, operates, and persists deterministically.

The CLI:

- loads state  
- executes an operation  
- persists results  
- exits  

It is not a long-lived in-memory system.

Durability exists in stored state, not process memory.

---

# 3. Architectural Role

The CLI is:

- a **host of Runtime**  
- a **coordinator of persistence surfaces**  
- a **human-facing interface layer**  

It is not:

- a source of truth  
- a legitimacy system  
- a persistence engine  

---

# 4. Storage Topology

## 4.1 User Profile Root

Charter state is stored under a user-scoped root:
```
~/.charter/
```
This root contains:

- contexts  
- default context reference  
- global metadata  

---

## 4.2 Context Storage

Each context is physically isolated:
```
~/.charter/contexts/<context_id>
```
A context contains:

- Area state  
- Runtime-managed surfaces  
- local artifacts  
- metadata  
- ref store  

Contexts must not share data implicitly.

---

## 4.3 Folder Binding

A project folder may contain:
```
./.charter/
```
This folder stores:

- context reference  
- Area reference  

### Resolution Rule

- CLI searches upward from current directory  
- first `.charter` found is used  
- if none found → fallback to user profile defaults  

---

## 4.4 Defaults

The CLI maintains:

- default context (user profile)  
- default Area (per context)  

If a binding is invalid:

- fallback to defaults  
- if defaults are invalid → require initialization  

---

# 5. Context & Area Resolution

## 5.1 Resolution Order

For each invocation:

1. folder-local binding (if present)  
2. user profile default context  
3. context default Area  

---

## 5.2 Failure Handling

If resolution fails:

- CLI enters initialization or recovery flow  
- user must explicitly restore or create context/Area  

---

## 5.3 Explicit Override Principle

- context is a physical boundary and must not be implicitly overridden  
- Area override may be allowed only through explicit commands  
- no silent cross-context execution  

---

# 6. Ref Store Model

## 6.1 Purpose

The ref store tracks **current operational state**.

---

## 6.2 Tracked References

Ref store may track:

- active Area per context  
- active process per Area  
- runtime pointers  
- host-level metadata  

---

## 6.3 Properties

Ref store entries are:

- mutable  
- non-authoritative  
- recoverable  
- explicit  

If inconsistent:

- CLI must reconcile or require user intervention  

---

# 7. Process Model

## 7.1 Active Process Constraint

Within a single Area:

> Only one active mutable process may exist.

---

## 7.2 Enforcement

If a process is active:

- no new mutable process may start  
- no other process may resume  

Until:

- the active process is paused or completed  

---

## 7.3 Process Continuity

Processes are:

- persisted in managed surfaces  
- reloadable across CLI invocations  

CLI must:

- detect active process on load  
- surface it to the user  
- allow resume or pause  

---

## 7.4 Process States

Processes may be:

- active  
- paused  
- blocked  
- completed  

Blocked processes:

- still count as active  
- must be resumed or resolved  

---

# 8. Runtime Interaction Model

## 8.1 Execution Cycle

Each CLI invocation:

1. resolves context and Area  
2. loads Runtime state  
3. executes operation  
4. updates state  
5. persists changes  
6. exits  

---

## 8.2 Determinism

CLI must ensure:

- identical inputs produce identical results  
- no hidden state exists outside managed surfaces  

---

## 8.3 Failure Handling

If Runtime fails:

- no partial persistence may occur  
- CLI must preserve prior consistent state  
- user must be informed explicitly  

---

## 8.4 Degraded Mode

If legitimacy state cannot be safely loaded:

- CLI enters degraded mode  

Properties:

- read-only legitimacy surfaces  
- limited mutation allowed  
- export and recovery operations available  

---

# 9. Persistence Model

## 9.1 Principle

Persistence is managed by Runtime.

CLI:

- triggers persistence  
- does not define storage semantics  

---

## 9.2 Persistence Behavior

At the end of execution:

- all state changes must be persisted  
- no in-memory state may be lost silently  

---

## 9.3 Atomicity

CLI operations must be:

- atomic from user perspective  
- either fully applied or not applied  

---

# 10. Query Model (CQL Integration)

## 10.1 Principle

All read operations should be expressed through CQL.

---

## 10.2 CLI Role

CLI provides:

- human-readable views  
- structured outputs  

Backed by:

- CQL queries  

---

## 10.3 Scope

CQL may query:

- runtime surfaces  
- substrate-owned data  
- derived views  

---

## 10.4 Properties

CQL queries are:

- read-only  
- deterministic  
- side-effect free  

---

# 11. Status & History Model

## 11.1 Status (Present State)

Status commands expose:

- current context and Area  
- active processes  
- pending changes  
- relevant signals  

---

## 11.2 History (Past State)

History commands expose:

- receipts  
- audit logs  
- lineage  
- prior decisions  

---

## 11.3 Separation Principle

- status reflects present  
- history reflects past  
- both must remain distinct  

---

# 12. Guidance Integration

## 12.1 Role

Guidance may assist users by:

- surfacing conflicts  
- highlighting ambiguity  
- suggesting structural relationships  

---

## 12.2 Constraints

Guidance must be:

- non-authoritative  
- non-mutating  
- optional beyond basic surfacing  

---

## 12.3 Invocation

- passive surfacing allowed  
- deeper assistance must be user-invoked  

---

# 13. Export & External Collaboration

## 13.1 Principle

Before relay systems exist:

- artifacts may be exported as structured formats (e.g., CCE JSON)  

---

## 13.2 Use Cases

- version control (e.g., git)  
- sharing between systems  
- backup and recovery  

---

## 13.3 Import Boundary

Imported artifacts:

- must not be trusted implicitly  
- must pass through reconciliation workflows  

---

# 14. Non-Interpretation

CLI must not:

- infer user intent  
- create implicit relationships  
- reinterpret data  

All actions must be:

- explicit  
- user-driven  

---

# 15. Invariants

- CLI is a host, not a source of truth  
- context is a hard isolation boundary  
- no implicit cross-context operations  
- one active mutable process per Area  
- all state changes must be persisted  
- all reads must be queryable through CQL  
- history must not be rewritten  
- failures must not corrupt prior state  
- degraded mode must preserve visibility  
- guidance must not alter system state  

---

# 16. Mental Model

The CLI is:

- a structured environment for thinking and deciding  
- a gateway to all Charter systems  
- a coordinator of workflows and state  

It is not:

- a scripting tool  
- a hidden automation layer  
- a decision engine  

---

# 17. Final Principle

The Charter CLI ensures that:

- users always know where they are  
- changes are always explicit  
- state is always recoverable  
- and structure is never hidden  

It turns:

> human interaction → structured state → durable understanding  

without ever becoming the source of truth itself.