# Charter Query Layer (CQL) — Foundation Specification

Status: FOUNDATIONAL  
Applies to: All Charter Substrates (Runtime, Legitimacy, CCS, CSG, CIS, CAS, CCare, CDS), Built-In Views, Raw Field Access, Host Extensions  
Depends On: Charter Substrate Specifications (CCS, CSG, CIS, CAS, CCare, CDS, Runtime, Legitimacy Engine)  
Does NOT define: transport syntax, UI rendering, storage backends, computation semantics, or host language bindings  

---

# 1. Purpose

This document defines a minimal, stable query model for accessing **all Charter-readable state**.

CQL exists to:

- query raw substrate data
- query built-in and derived views
- uniformly access multiple substrates
- scope queries across structure, identity, runtime, and alignment
- support host-defined extensions

CQL is:

- read-only  
- deterministic  
- substrate-agnostic  
- composable  
- selector-oriented  

It is not:

- a programming language  
- a computation engine  
- an interpretation layer  

---

# 2. Core Principle

> A CQL query selects a view or field set over a target, within a domain, under a scope, with optional filters and context.

CQL must remain:

- simple enough for CLI and JSON use  
- structured enough for library embedding  
- stable enough for cross-substrate use  
- extensible without breaking core semantics  

---

# 3. Query Model

A CQL query consists of six conceptual parts:

1. domain  
2. subject  
3. target  
4. scope  
5. filters  
6. output  

Not all parts are required in every query.

---

## 3.1 Domain

The domain defines which substrate is being queried.

Supported domains include:

- `runtime`
- `legitimacy`
- `ccs`
- `csg`
- `cis`
- `cas`
- `ccare`
- `cds`

Domains define:

- available targets  
- available views  
- available filters  

Examples:

- `domain:cas`
- `domain:cds`
- `domain:runtime`

---

## 3.2 Subject

The subject defines what is being requested.

Two subject classes exist:

- `view`
- `raw`

Examples:

- `view:posture`
- `view:trend`
- `view:items`
- `view:signals`
- `raw:fields`

Subjects are interpreted within the selected domain.

---

## 3.3 Target

The target defines where the query applies.

Target kinds vary by domain but may include:

- `resolution`
- `area`
- `identity`
- `global`
- `pair`
- `deliberate`
- `item`
- `session`
- `review`
- `signal`

Examples:

- `resolution:res_123`
- `area:payments`
- `identity:platform`
- `deliberate:delib_456`
- `session:sess_789`
- `global`

---

## 3.4 Scope

Scope constrains the slice of data.

Common scope dimensions include:

- activity (`active`, `historical`)  
- time (`since`, `until`)  
- posture (contextual interpretation)  
- structural mode (`local`, `global`)  
- lifecycle state  

Examples:

- `active`
- `historical`
- `since=2026-01-01`
- `until=2026-03-31`
- `posture=transition_window`
- `mode=global`

Scope is additive.

---

## 3.5 Filters

Filters narrow results.

Filter dimensions are domain-specific but may include:

- semantic state  
- lifecycle state  
- volatility  
- confidence  
- category  
- relationship type  

Examples:

- `state=misaligned`
- `state=locked`
- `volatility=increasing`
- `confidence>=medium`

---

## 3.6 Output

Output controls result form.

Supported modes:

- `summary`
- `structured`
- `detailed`

Default output is host-defined but must be deterministic.

---

# 4. Canonical Query Shape

The canonical logical shape is:

domain subject on target [scope] [filters] [output]

Examples:

- `domain:cas view:posture on area:payments active`
- `domain:cas view:trend on identity:platform since=2026-01-01`
- `domain:cds view:items on deliberate:delib_123 state=locked`
- `domain:ccare view:signals on area:payments active`
- `domain:runtime view:sessions on area:core active`

---

# 5. Domain Profiles

Each domain defines its own query surface.

---

## 5.1 CAS (Alignment)

Subjects:

- `view:posture`
- `view:trend`
- `view:tension`
- `view:structural`
- `view:overlap`
- `view:collaboration`
- `view:boundary_pressure`
- `view:global_landscape`
- `view:explain`
- `raw:fields`

---

## 5.2 CDS (Deliberate)

Subjects:

- `view:items`
- `view:board`
- `view:history`
- `view:lineage`
- `view:closure`
- `raw:fields`

---

## 5.3 CCare

Subjects:

- `view:signals`
- `view:requests`
- `view:suggestions`
- `view:supportability`
- `raw:fields`

---

## 5.4 CIS (Identity)

Subjects:

- `view:membership`
- `view:overlap`
- `view:collaboration`
- `view:boundary`
- `raw:fields`

---

## 5.5 CSG (Structure)

Subjects:

- `view:graph`
- `view:neighbors`
- `view:lineage`
- `view:active_projection`
- `raw:fields`

---

## 5.6 CCS (Commits)

Subjects:

- `view:commits`
- `view:references`
- `view:lineage`
- `raw:fields`

---

## 5.7 Runtime / Legitimacy

Subjects:

- `view:sessions`
- `view:reviews`
- `view:receipts`
- `view:workspaces`
- `raw:fields`

---

# 6. Target Grammar

## 6.1 Single Targets

Examples:

- `resolution:res_123`
- `area:checkout`
- `identity:core_platform`
- `deliberate:delib_1`
- `session:sess_1`
- `global`

---

## 6.2 Pair Targets

Examples:

- `pair:identity(platform),identity(security)`
- `pair:area(payments),area(fulfillment)`

Used for:

- comparison  
- overlap  
- collaboration  

---

# 7. Scope Grammar

Unchanged in structure, generalized across domains.

Examples:

- `active`
- `historical`
- `since=...`
- `until=...`
- `posture=...`
- `mode=local`

---

# 8. Filter Grammar

Domain-specific but structurally consistent.

Examples:

- `state=misaligned` (CAS)
- `state=locked` (CDS)
- `type=checkin` (CCare)

---

# 9. Output Grammar

Unchanged.

---

# 10. Example Queries

### Alignment posture
`domain:cas view:posture on area:payments active`

### Deliberate locked items
`domain:cds view:items on deliberate:delib_123 state=locked`

### Care signals
`domain:ccare view:signals on identity:platform active`

### Identity membership
`domain:cis view:membership on identity:platform active`

### Runtime sessions
`domain:runtime view:sessions on area:core active`

---

# 11. Extension Model

## 11.1 Extension Naming

`view:x.<domain>.<host>.<name>`

Examples:

- `view:x.cas.myhost.risk_surface`
- `view:x.cds.internal.workflow_map`

---

## 11.2 Rules

Extensions:

- must be read-only  
- must not redefine canonical views  
- must operate on domain-owned data  
- must not introduce side effects  

---

# 12. Determinism Rules

Given identical:

- domain  
- subject  
- target  
- scope  
- filters  
- output  

Results must be identical.

CQL must not allow:

- mutation  
- hidden state  
- non-deterministic evaluation  

---

# 13. Relationship to Hosts

CQL is host-agnostic.

It may be expressed as:

- CLI  
- JSON  
- API  
- library bindings  

Logical meaning must remain identical across representations.

---

# 14. Mental Model

A CQL query asks:

- what domain am I querying  
- what do I want to see  
- over what target  
- under what constraints  
- in what form  

CQL selects existing truth.  
It does not compute new truth.

---

# 15. Final Principle

CQL exists to unify access across Charter without collapsing its architecture.

It ensures that:

- all substrates remain independent  
- all data remains non-authoritative unless defined elsewhere  
- all queries remain read-only and deterministic  

so that systems like CGL can reason across Charter  
without introducing coupling or interpretation.
