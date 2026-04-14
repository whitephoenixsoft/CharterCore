# Charter Query Language — Execution Model

Status: STRUCTURAL (DRAFT)  
Applies to: Charter Query Language, Runtime, CLI, VDS, VLS, substrate query integrations  
Depends On: Runtime Foundation, CQL Foundation, Non-Interpretation Principle, Versioning & Identity Model  
Does NOT define: JSON IL schema details, DSL grammar details, storage implementations, or substrate semantics  

---

# 1. Purpose

This document defines the structural execution model of the Charter Query Language.

It exists to clarify:

- how CQL queries are represented  
- how human queries differ from machine queries  
- how queries are validated, planned, and executed  
- how substrates participate in query execution  
- how Runtime acts as the primary host-facing query gateway  

This document describes how CQL works structurally.

It does not redefine what CQL is for.

---

# 2. Core Principle

> CQL has one canonical machine form and one or more human-facing surface forms.

CQL execution is based on:

- a canonical JSON Intermediate Language  
- deterministic validation and planning  
- substrate-owned query adapters  
- Runtime-mediated host execution  

The canonical machine form is JSON Intermediate Language.

Human-facing forms must compile into it.

---

# 3. Architectural Position

CQL is a Charter-native query substrate.

It sits between:

- hosts and users requesting information  
- Runtime providing operational context  
- substrates exposing queryable read surfaces  

Flow:

Host or User → Runtime → CQL Engine → Substrate Query Adapters → Results

CQL is:

- read-only  
- deterministic  
- non-mutating  
- non-authoritative  

---

# 4. Representation Model

## 4.1 Canonical Representation

The canonical representation of every CQL query is:

→ JSON Intermediate Language

This is the authoritative machine form used for:

- Runtime execution  
- host integration  
- library integration  
- VDS and VLS integration  
- automated tooling  

---

## 4.2 Human Representation

Human-facing query syntaxes, including any CQL DSL, are optional surface forms.

Properties:

- intended for human use  
- intended for CLI ergonomics  
- not authoritative  
- must compile deterministically into JSON Intermediate Language  

---

## 4.3 Representation Principle

> JSON Intermediate Language is the authoritative query contract.  
> Surface syntaxes are convenience layers.

---

# 5. Execution Layers

CQL execution is composed of four structural layers.

---

## 5.1 Surface Layer

This layer accepts:

- DSL input  
- direct JSON Intermediate Language input  
- host-generated structured query requests  

Responsibilities:

- parse human syntax where applicable  
- normalize host input into canonical form  

---

## 5.2 Validation and Normalization Layer

This layer validates and normalizes JSON Intermediate Language.

Responsibilities:

- structural validation  
- domain validation  
- normalization of equivalent query forms  
- rejection of ambiguous or invalid query shapes  

Properties:

- deterministic  
- side-effect free  
- independent of storage implementation  

---

## 5.3 Planning and Routing Layer

This layer determines how the query should execute.

Responsibilities:

- determine target domains  
- resolve which substrate adapters are needed  
- determine whether query execution is:
  - single-domain
  - multi-domain
  - derived
  - federated

- establish execution order where needed  

---

## 5.4 Execution Layer

This layer invokes substrate query adapters and assembles results.

Responsibilities:

- invoke adapters deterministically  
- collect domain results  
- preserve source boundaries  
- shape output into standard result envelopes  

---

# 6. JSON Intermediate Language

## 6.1 Role

JSON Intermediate Language is the canonical intermediate representation consumed by the CQL execution engine.

It is not merely a transport format.

It is the stable semantic form of the query.

---

## 6.2 Properties

JSON Intermediate Language must be:

- machine-readable  
- deterministic  
- composable  
- explicit in domain targeting  
- explicit in filtering and scope  

---

## 6.3 Scope

JSON Intermediate Language must support querying across:

- Runtime state  
- Commit and receipt artifacts  
- Deliberate System artifacts  
- structural graph representations  
- identity and lineage structures  
- care signals  
- alignment outputs  
- federated artifacts  

---

# 7. DSL Compilation Model

## 7.1 Principle

Any CQL DSL must compile into JSON Intermediate Language.

---

## 7.2 Constraints

DSL compilation must be:

- deterministic  
- lossless with respect to intended query meaning  
- non-interpreting  
- equivalent in result to directly authored JSON Intermediate Language  

---

## 7.3 Boundary

The DSL compiler must not:

- infer hidden domain semantics  
- rewrite user intent  
- introduce implicit cross-domain meaning  

---

# 8. Runtime Role

## 8.1 Primary Host Gateway

Runtime is the primary host-facing query gateway for CQL execution.

Hosts should interact with query execution primarily through Runtime.

---

## 8.2 Responsibilities

Runtime is responsible for:

- resolving execution context  
- resolving current workspace and Area context where applicable  
- exposing available query adapters  
- providing access to live operational read surfaces  
- invoking CQL execution in a deterministic environment  

---

## 8.3 Constraint

Runtime does not own all query semantics.

Runtime routes and hosts execution.

Substrates own domain-specific query meaning.

---

# 9. Substrate Query Adapter Model

## 9.1 Principle

Each substrate participates in CQL through its own query adapter module.

---

## 9.2 Adapter Responsibilities

A substrate query adapter is responsible for:

- registering its domain  
- declaring supported query targets  
- validating domain-specific query fragments  
- executing against its own logical read surfaces  
- shaping results into the standard CQL result model  

---

## 9.3 Adapter Boundaries

A substrate adapter must not:

- assume the semantics of another substrate  
- mutate state  
- reinterpret foreign domain meaning  
- bypass CQL result structure  

---

## 9.4 Examples

Adapters may exist for:

- Runtime  
- Charter Commit System and Commit Store  
- Deliberate System  
- Charter Structure Graph  
- Charter Identity System  
- Charter Care  
- Charter Alignment System  
- Value-Directed Systems  
- Value Lineage Systems  

---

# 10. Query Domains and Composition

## 10.1 Single-Domain Queries

A query may target one domain only.

In this case:

- one adapter executes  
- one result domain is returned  

---

## 10.2 Multi-Domain Queries

A query may target multiple domains.

In this case:

- execution must preserve explicit domain boundaries  
- composition must remain deterministic  
- no implicit semantic merging is allowed  

---

## 10.3 Derived Queries

Some queries may rely on derived systems such as:

- graph projections  
- alignment outputs  
- lineage representations  

Derived queries remain read-only and non-authoritative.

---

## 10.4 Federated Queries

Some queries may operate on federated artifacts.

Federated querying must preserve:

- provenance  
- source identity  
- local versus foreign distinction  

---

# 11. Result Model

## 11.1 Standard Envelope

All query execution must produce a standard result envelope.

This envelope must preserve:

- execution status  
- domain origin  
- result payload  
- provenance where applicable  
- rule identity where applicable  
- query errors or warnings  

---

## 11.2 Composition Rule

When multiple domains contribute results:

- results must remain attributable to origin domains  
- composition must not erase source boundaries  
- ambiguity must remain visible  

---

# 12. Error Model

## 12.1 Principle

CQL failures must be explicit.

---

## 12.2 Failure Types

Failures may include:

- invalid syntax  
- invalid JSON Intermediate Language structure  
- unknown domain  
- unsupported query target  
- unavailable adapter  
- inaccessible query scope  
- deterministic execution failure  

---

## 12.3 Constraint

Errors must not be hidden behind partial interpretation.

---

# 13. Determinism

CQL execution must be deterministic given:

- identical JSON Intermediate Language input  
- identical Runtime context  
- identical registered adapters  
- identical underlying read surfaces  
- identical rule identities where relevant  

---

# 14. Non-Interpretation

CQL must not:

- infer intent  
- create relationships  
- merge domain semantics implicitly  
- reinterpret historical artifacts  

It may only query and assemble explicitly available information.

---

# 15. Mental Model

CQL is:

- a canonical query substrate  
- a structured execution engine for read-only system access  
- a bridge between hosts and substrate-specific query surfaces  

It is not:

- a storage engine  
- a mutation layer  
- a semantic authority over all substrates  

---

# 16. Final Principle

CQL works by:

- accepting human or machine query forms  
- compiling surface syntax into JSON Intermediate Language where needed  
- validating and routing canonical queries  
- executing them through substrate-owned adapters  
- returning explicit, deterministic, domain-preserving results  

It provides one way to ask questions across Charter:

> explicit in structure,  
> deterministic in execution,  
> and honest about boundaries.