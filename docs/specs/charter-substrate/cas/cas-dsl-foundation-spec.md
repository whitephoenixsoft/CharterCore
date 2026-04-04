# Charter Alignment System (CAS) — Foundational Query DSL Specification

Status: FOUNDATIONAL  
Applies to: CAS Query Layer, Built-In Views, Raw Field Access, Host Extensions  
Depends On: Charter Alignment System (CAS) — Foundation Specification  
Does NOT define: transport syntax, UI rendering, storage backends, or host language bindings  

---

# 1. Purpose

This document defines a minimal, stable DSL for querying the Charter Alignment System (CAS).

The DSL exists to:

- query raw alignment fields
- query built-in alignment views
- scope queries to structural targets
- apply filters and context
- support host-defined extension views

The DSL is:

- read-only
- deterministic
- host-agnostic
- composable
- selector-oriented

It is not a programming language.

---

# 2. Core Principle

> A CAS query selects a view or field set over a target, under a scope, with optional filters and context.

The DSL must remain:

- simple enough for CLI and JSON use
- structured enough for library embedding
- stable enough for host-defined extensions

---

# 3. Query Model

A CAS query consists of five conceptual parts:

1. subject
2. target
3. scope
4. filters
5. output

Not all parts are required in every query.

---

## 3.1 Subject

The subject defines what is being requested.

Two subject classes exist:

- `view`
- `raw`

Examples:

- `view:posture`
- `view:trend`
- `view:overlap`
- `raw:fields`

---

## 3.2 Target

The target defines where the query applies.

Supported target kinds:

- `resolution`
- `area`
- `identity`
- `global`
- `pair`

Examples:

- `resolution:res_123`
- `area:payments`
- `identity:platform`
- `global`
- `pair:identity(platform),identity(security)`

---

## 3.3 Scope

Scope constrains the slice of derived state.

Supported scope dimensions:

- activity
- time
- history
- posture
- structure mode

Examples:

- `active`
- `historical`
- `since=2026-01-01`
- `until=2026-03-31`
- `posture=transition_window`
- `mode=local`
- `mode=global`

---

## 3.4 Filters

Filters narrow the returned results.

Supported filter dimensions include:

- semantic state
- volatility
- category
- confidence
- overlap-only
- collaboration-only

Examples:

- `state=reduced_capacity`
- `volatility=increasing`
- `category=tension`
- `confidence>=medium`

---

## 3.5 Output

Output controls result form.

Supported output modes:

- `summary`
- `structured`
- `detailed`

Examples:

- `output=summary`
- `output=structured`
- `output=detailed`

---

# 4. Canonical Query Shape

The canonical logical shape is:

subject on target [scope] [filters] [output]

Examples:

- `view:posture on area:payments active`
- `view:trend on identity:platform since=2026-01-01 until=2026-03-31`
- `raw:fields on resolution:res_123 active`
- `view:overlap on pair:identity(product),identity(devex) active`
- `view:collaboration on pair:identity(platform),identity(security) active output=detailed`

---

# 5. Built-In Subjects

## 5.1 Raw Subject

`raw:fields`

Purpose:

- expose computed raw field values directly

Typical outputs may include:

- drift
- variance
- signal_density
- confidence
- propagation_strength
- tension_density
- stability_index
- boundary_pressure

Example:

`raw:fields on identity:platform active`

---

## 5.2 Built-In View Subjects

Built-in views are exposed as `view:<name>`.

Canonical built-in views include:

- `view:posture`
- `view:trend`
- `view:structural`
- `view:tension`
- `view:influence`
- `view:overlap`
- `view:collaboration`
- `view:boundary_pressure`
- `view:global_landscape`
- `view:explain`

These names are stable query subjects.

---

# 6. Target Grammar

## 6.1 Single Targets

Single targets identify one structural subject.

Examples:

- `resolution:res_123`
- `area:checkout`
- `identity:core_platform`
- `global`

---

## 6.2 Pair Targets

Pair targets define two comparable or interacting targets.

Examples:

- `pair:identity(platform),identity(security)`
- `pair:area(payments),area(fulfillment)`

Pair targets are used for:

- overlap
- collaboration
- comparison
- boundary queries

---

# 7. Scope Grammar

Scope is additive.

Examples:

- `active`
- `historical`
- `since=2026-01-01`
- `until=2026-03-31`
- `posture=transition_window`
- `posture=experiment`
- `mode=local`
- `mode=global`

Multiple scope clauses may be combined:

`view:trend on identity:platform historical since=2026-01-01 until=2026-03-31`

---

# 8. Filter Grammar

Filters constrain the result set or interpretation focus.

Examples:

- `state=misaligned`
- `state=reduced_capacity`
- `volatility=stable`
- `volatility=increasing`
- `category=tension`
- `category=propagation`
- `confidence>=medium`

Filters are optional.

---

# 9. Output Grammar

Output modifies result verbosity and structure.

Examples:

- `output=summary`
- `output=structured`
- `output=detailed`

Default output is host-defined, but must be deterministic.

---

# 10. Example Queries

## 10.1 Current posture of an area

`view:posture on area:payments active`

---

## 10.2 Trend over time for an identity

`view:trend on identity:core_platform since=2026-01-01 until=2026-03-31`

---

## 10.3 Raw fields for a resolution

`raw:fields on resolution:res_123 active output=structured`

---

## 10.4 Collaboration between identities

`view:collaboration on pair:identity(platform),identity(security) active`

---

## 10.5 Overlap between identities

`view:overlap on pair:identity(product),identity(devex) active`

---

## 10.6 Tension regions with filters

`view:tension on global active state=misaligned volatility=increasing`

---

## 10.7 Boundary pressure during posture context

`view:boundary_pressure on identity:migration_team active posture=transition_window`

---

# 11. Extension Model

CAS must support host-defined views.

## 11.1 Extension View Naming

Extension views use:

`view:x.<host>.<name>`

Examples:

- `view:x.myhost.risk_surface`
- `view:x.internal.capacity_map`

This avoids collision with canonical CAS view names.

---

## 11.2 Extension Rules

Extension views:

- must consume CAS-derived state only
- must not mutate CAS state
- must not redefine canonical view names
- must remain read-only

Hosts may define new views over:

- raw fields
- built-in derived state
- identity-aware slices
- posture-aware slices

---

## 11.3 Canonical vs Extension Boundary

CAS owns:

- the canonical DSL grammar
- canonical built-in subjects
- canonical target model
- canonical scope/filter model

Hosts own:

- extension view implementations
- host-specific rendering
- additional derived projections

---

# 12. Determinism Rules

Given identical:

- derived state
- target
- scope
- filters
- output mode

query results must be identical.

The DSL must not permit:

- hidden mutation
- procedural side effects
- non-deterministic expansion

---

# 13. Relationship to Hosts

This DSL is conceptual and host-agnostic.

It may be expressed as:

- CLI arguments
- JSON payloads
- library builders
- API parameters

Example conceptual mapping:

CLI:
`alignment view posture --area payments --active`

JSON:
- subject = `view:posture`
- target = `area:payments`
- scope = `active`

Library:
- subject(`view:posture`)
- target(`area:payments`)
- scope(`active`)

The logical meaning must remain unchanged across hosts.

---

# 14. Mental Model

A CAS query asks:

- what do I want to see
- over what part of the system
- under what context
- with what constraints
- in what result shape

The DSL does not describe computation steps.

It describes selection of already-derived truth.

---

# 15. Final Principle

The CAS DSL exists to make alignment queryable without making CAS procedural.

It must remain:

- simple
- stable
- deterministic
- extensible
- read-only

so that hosts can ask better questions
without changing what CAS knows.