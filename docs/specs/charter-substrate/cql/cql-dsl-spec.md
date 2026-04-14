# Charter Query Language (CQL) — DSL Specification

Status: STRUCTURAL (DRAFT)  
Applies to: CLI, host interfaces, human-authored queries  
Depends On: CQL Foundation Specification, CQL JSON Intermediate Language Specification, Determinism, Non-Interpretation Principle  
Does NOT define: canonical query structure, storage implementation, result schema, or substrate semantics  

---

# 1. Purpose

This document defines the human-facing Domain Specific Language for CQL.

It exists to:

- provide an ergonomic query syntax for humans  
- support CLI interaction and manual query construction  
- allow structured queries without requiring JSON authoring  
- compile deterministically into the canonical JSON Intermediate Language  

The DSL is not the canonical representation of a query.

All DSL queries must compile into JSON Intermediate Language.

---

# 2. Core Principle

> The DSL is a convenience layer over the canonical JSON Intermediate Language.

The DSL must:

- compile deterministically into JSON Intermediate Language  
- preserve full query intent without loss  
- introduce no additional semantics beyond JSON Intermediate Language  
- remain explicit and non-interpreting  

---

# 3. Relationship to JSON Intermediate Language

The JSON Intermediate Language defines:

- the canonical query structure  
- all valid query components  
- determinism rules  
- non-interpretation rules  

The DSL:

- maps directly to the JSON Intermediate Language  
- may omit optional fields for convenience  
- must not redefine or extend canonical semantics  

---

## 3.1 Compilation Requirements

DSL compilation must be:

- deterministic  
- lossless  
- explicit  
- non-inferential  

Compilation must not:

- infer missing structure  
- introduce implicit joins  
- reinterpret domain semantics  
- alter query meaning  

---

## 3.2 Canonical Mapping

All DSL queries must map to the following JSON Intermediate Language components:

- domain  
- subject  
- target  
- scope  
- filters  
- output  
- context (when applicable)  
- metadata (optional)  

---

# 4. DSL Query Model

The DSL expresses the canonical query model in a simplified linear form.

Canonical structure:

domain subject on target [scope] [filters] [output]

---

## 4.1 Domain

Specifies the domain or domains being queried.

Examples:

- domain:cas  
- domain:csg  
- domain:runtime  

Multi-domain queries may be supported if explicitly declared.

---

## 4.2 Subject

Defines what is being requested.

Syntax:

- view:name  
- raw:name  

Examples:

- view:posture  
- view:items  
- raw:fields  

Optional arguments may be provided:

view:posture(window=7d, aggregation=rolling)

Arguments must map directly to JSON Intermediate Language subject arguments.

---

## 4.3 Target

Defines the object or collection the query applies to.

Syntax:

- on kind:id  
- on kind:id1,id2,id3  
- on pair(kind:id, kind:id)  

Examples:

- on area:payments  
- on resolution:res_123  
- on item:item_1,item_2  
- on pair(resolution:r1, resolution:r2)  

---

## 4.4 Scope

Defines visibility constraints.

Examples:

- active  
- historical  
- since=2024-01-01  
- projection=resolution  
- projection=item  

---

### Scope Rule

Scope defines where data is visible.

Scope must not:

- filter by value  
- constrain based on conditions  

---

## 4.5 Filters

Defines constraints within visible scope.

Examples:

- state=accepted  
- volatility=high  
- relationship_type=supersedes  
- provenance=external  

---

### Filter Rule

Filters operate only on visible data.

Filters must not:

- expand scope  
- cross domains implicitly  
- introduce inference  

---

## 4.6 Output

Defines result structure.

Examples:

- summary  
- structured  
- detailed  

---

## 4.7 Context

Context may be provided explicitly.

Syntax example:

context:mode=investigation

Context must:

- map to JSON Intermediate Language context  
- remain explicit  
- not introduce hidden semantics  

---

## 4.8 Metadata

Metadata may be optionally provided.

Metadata is not typically exposed in CLI syntax but may be included in structured host usage.

---

# 5. Examples

- domain:cas view:posture on area:payments active projection=resolution  
- domain:cds view:items on deliberate:delib_123 state=locked  
- domain:audit view:provenance on resolution:res_123 detailed  
- domain:csg view:graph on resolution:r1 projection=item  
- domain:cis view:identity on identity:id_42 structured  

---

# 6. Extension Model

## 6.1 Purpose

The DSL supports host-defined extension views.

---

## 6.2 Naming

Extension views must be namespaced.

Syntax:

view:x.<domain>.<host>.<view>

Examples:

- view:x.cas.myhost.ops_surface  
- view:x.cds.myhost.investigation_panel  

---

## 6.3 Execution Model

When an extension view is invoked:

1. DSL is parsed  
2. Query is compiled into JSON Intermediate Language  
3. JSON Intermediate Language is validated  
4. Registered extension handler is resolved  
5. Handler is invoked with the normalized query representation  

---

## 6.4 Handler Contract

Handlers receive:

- validated and normalized JSON Intermediate Language representation  
- resolved target  
- scope  
- filters  
- output mode  

Handlers may access:

- Charter read interfaces  
- host systems  

Handlers must return:

- deterministic  
- read-only  
- structured results  

---

## 6.5 Constraints

Extension views must:

- be read-only  
- be deterministic  
- not redefine canonical views  
- not modify DSL grammar  
- not introduce implicit semantics  

---

## 6.6 Non-Goals

The DSL does not support:

- joins  
- query composition syntax  
- computed expressions  
- user-defined operators  
- host-defined domains  

---

# 7. Determinism

Given identical DSL input and execution context, the resulting JSON Intermediate Language must be identical.

Execution must not depend on:

- hidden defaults  
- runtime timing  
- implicit ordering  

---

# 8. Ordering

Result ordering must be:

- explicitly requested  
- or defined by the domain  

No implicit ordering is guaranteed.

---

# 9. Relationship to Runtime

The DSL is typically executed through Runtime.

Runtime:

- resolves context  
- invokes CQL execution  
- routes to appropriate adapters  

Runtime does not:

- alter query semantics  
- reinterpret query meaning  

---

# 10. Mental Model

The DSL allows users to express:

- what domain to query  
- what to retrieve  
- where to look  
- what constraints apply  
- how results should be shaped  

It is a human-friendly way to express a canonical query.

---

# 11. Final Principle

The DSL provides an ergonomic interface for querying Charter.

It ensures:

- simplicity for humans  
- consistency with JSON Intermediate Language  
- deterministic compilation  
- no semantic drift  

The DSL is not the source of truth.

The JSON Intermediate Language remains the canonical query representation.