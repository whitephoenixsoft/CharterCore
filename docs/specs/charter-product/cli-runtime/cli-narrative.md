# Charter CLI — Narrative

Status: ARCHITECTURAL (BUILD REFERENCE)  
Purpose: Provide a clear, end-to-end mental model of how the CLI, Runtime, Engine, and Substrates work together  

---

# 1. The Starting Point

A user opens the Charter CLI.

They are not interacting with an engine.

They are entering a **structured environment for thinking and deciding**.

Before anything happens, the CLI determines:

- where the user is (context)  
- what they are working in (Area)  
- whether something is already in progress (active process)  

This resolution is explicit, stable, and deterministic.

---

# 2. Context Resolution

The CLI resolves location in this order:

1. folder binding (`.charter` in current or parent directory)  
2. user profile default context  
3. default Area within that context  

If resolution fails:

- the CLI enters initialization or recovery flow  

The user is never allowed to operate in an undefined state.

---

# 3. The Execution Cycle

Every CLI command follows the same pattern:

1. resolve context and Area  
2. load Runtime state  
3. execute operation  
4. persist results  
5. exit  

There is no hidden state between commands.

Everything meaningful is stored.

---

# 4. Runtime as the Orchestrator

Once loaded, the Runtime becomes the coordinator.

It:

- manages Areas  
- tracks processes  
- coordinates workflows  
- invokes the Legitimacy Engine  
- interacts with storage surfaces  
- exposes query interfaces  

It does not:

- create legitimacy  
- interpret meaning  
- infer intent  

---

# 5. The Two Modes of Work

From the user’s perspective, work falls into two broad categories:

## 5.1 Exploration (Non-Legitimizing)

The user:

- writes ideas  
- investigates  
- connects concepts  
- imports data  

This happens in:

- deliberate spaces  
- review workspaces  
- non-legitimate structures  

Nothing here becomes permanent truth.

---

## 5.2 Formalization (Legitimizing)

When the user chooses to make something real:

- it must go through structured processes  
- it must respect Area governance  
- it must pass through the Legitimacy Engine  

This is where decisions become durable.

---

# 6. The Role of Reconciliation

Not everything starts clean.

When something is:

- imported  
- diverged  
- externally created  
- structurally unclear  

It enters **Incoming Reconciliation Review**.

---

## 6.1 What Review Does

Review:

- stages proposals  
- compares them to existing structure  
- surfaces conflicts and relationships  
- allows user alignment  
- requires explicit approval  

It does not create legitimacy.

---

## 6.2 The Outcome of Review

At completion:

- accepted proposals form a batch  
- rejected proposals are recorded  
- unresolved proposals become abandoned  

The batch is then prepared for legitimacy.

---

# 7. Session Materialization

Accepted proposals are transformed into **sessions**.

Each proposal:

- becomes a candidate  
- carries relationships  
- carries provenance  
- carries governance context  

Sessions are:

- ordered deterministically  
- constructed explicitly  
- prepared for engine execution  

---

# 8. Sandbox Execution

Sessions do not immediately affect the system.

They are executed in isolation:

- all sessions run  
- outputs are collected  
- no changes are persisted yet  

If any session fails:

- the entire batch fails  
- nothing becomes legitimate  
- the review is blocked and must resume  

---

# 9. Legitimacy Creation

If all sessions succeed:

- resolutions are created  
- session receipts are produced  

These are the only source of legitimacy.

---

# 10. Commit and Durability

Once legitimacy is confirmed:

- all outputs are emitted through CCS  
- artifacts are written to the Commit Store  

This includes:

- resolutions  
- session receipts  
- review closure artifacts  

Rule:

> Any durable closure becomes a commit.

---

# 11. Querying the System

At any time, the user can query the system.

All queries flow through **CQL**.

CQL allows access to:

- current state (status)  
- process state  
- relationships  
- history  
- lineage  
- provenance  

CLI commands are human-friendly layers over CQL.

---

# 12. Present vs Past

The system is divided into:

## Present (Status)

- current Area  
- active processes  
- pending changes  
- current structure  

## Past (History)

- receipts  
- commits  
- prior decisions  
- evolution over time  

These remain separate.

---

# 13. Active Process Constraint

Within an Area:

- only one active mutable process may exist  

If another process is needed:

- the current one must be paused  

This ensures:

- no conflicting changes  
- no hidden concurrency  
- clear system state  

---

# 14. Ref Store

The Runtime maintains a **ref store** for:

- current Area  
- active process  
- runtime pointers  

Refs are:

- mutable  
- recoverable  
- non-authoritative  

They provide convenience, not truth.

---

# 15. Import and Export

## Export

The user may export:

- structured artifacts (e.g., CCE JSON)  

For:

- version control  
- sharing  
- backup  

---

## Import

Imported artifacts:

- are never trusted automatically  
- must go through reconciliation  

---

# 16. Degraded Mode

If legitimacy state cannot be safely loaded:

- the system enters degraded mode  

In this mode:

- legitimacy is read-only  
- history remains visible  
- recovery paths are available  

The system does not fail silently.

---

# 17. Persistence Model

Runtime may operate in-memory.

The CLI:

- always loads from storage  
- always persists at the end  

This ensures:

- durability  
- recoverability  
- deterministic behavior  

---

# 18. Guidance

Guidance may:

- highlight conflicts  
- suggest relationships  
- surface ambiguity  

It:

- never changes state  
- never overrides decisions  
- remains optional  

---

# 19. System Boundaries

The system enforces:

- context isolation (no cross-context leakage)  
- Area governance boundaries  
- explicit structure only  
- no implicit meaning  

---

# 20. The Loop

The full system loop is:

1. user observes and thinks  
2. user explores ideas  
3. user aligns structure (review if needed)  
4. user approves change  
5. runtime executes sessions  
6. engine determines legitimacy  
7. results are committed  
8. system becomes queryable  
9. user reflects and continues  

---

# 21. Final Mental Model

The Charter CLI is:

- a journal for decisions  
- a gateway to structured systems  
- a coordinator of workflows  
- a bridge between thinking and legitimacy  

It ensures that:

- nothing becomes real silently  
- nothing is lost  
- everything remains understandable  

Over time, this creates:

> a system where decisions are not just stored,  
> but continuously understood.