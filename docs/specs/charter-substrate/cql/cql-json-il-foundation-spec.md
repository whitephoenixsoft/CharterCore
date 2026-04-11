# Charter Query Layer (CQL) — JSON IL Foundation Specification

Status: FOUNDATIONAL (DRAFT)  
Applies to: All Charter-readable substrates and managed read surfaces  
Depends On: CQL Foundation Specification, Determinism, Non-Interpretation Principle, Versioning & Identity Model, Provenance Model  
Does NOT define: DSL syntax, UI rendering, storage engine implementation, mutation semantics, or host-specific transport bindings  

---

# 1. Purpose

This document defines the **JSON Intermediate Language (JSON IL)** for CQL.

It exists to:

- provide a canonical machine-readable representation of CQL queries  
- unify query behavior across libraries, hosts, and interfaces  
- support deterministic read access over managed Charter surfaces  
- provide a stable substrate-neutral query contract  
- allow future DSLs and host adapters to compile into a common form  

CQL JSON IL is the canonical structural representation of a query.

The human DSL, if present, is an ergonomic layer that compiles into this form.

---

# 2. Core Principle

> The JSON IL is the canonical CQL query form.

All CQL queries must be representable as a deterministic JSON query object.

That object must be:

- read-only  
- deterministic  
- substrate-neutral  
- explicit  
- serializable  
- validatable independent of host language  

---

# 3. Query Object Model

A CQL JSON IL query is a structured object containing six conceptual parts:

1. domain  
2. subject  
3. target  
4. scope  
5. filters  
6. output  

Additional metadata may be attached where explicitly defined.

---

# 4. Top-Level Query Shape

A canonical JSON IL query object must support the following top-level fields:

- `domain`
- `subject`
- `target`
- `scope`
- `filters`
- `output`

Optional top-level fields may include:

- `context`
- `metadata`

No top-level field may imply mutation.

---

# 5. Domain

## 5.1 Purpose

`domain` identifies which Charter read domain is being queried.

A domain corresponds to a managed read surface owned by or exposed through a Charter substrate.

---

## 5.2 Properties

A domain:

- must be explicit  
- must resolve to a deterministic read surface  
- must not imply mutation  
- must not imply interpretation beyond domain-defined query behavior  

---

## 5.3 Canonical Domains

Canonical domains may include:

- `runtime`
- `review`
- `legitimacy`
- `ccs`
- `csg`
- `cis`
- `cas`
- `ccare`
- `cds`
- `csp`
- `crs`
- `audit`

Additional domains may be introduced only through explicit versioned extension.

---

# 6. Subject

## 6.1 Purpose

`subject` identifies what kind of thing is being requested from the chosen domain.

Two subject classes exist:

- `view`
- `raw`

---

## 6.2 View Subject

A `view` requests a named domain-defined surface.

Examples:

- posture
- graph
- items
- provenance
- reviews
- proposals

---

## 6.3 Raw Subject

A `raw` subject requests explicitly named fields or structurally direct domain data.

Raw access must remain:

- read-only  
- explicit  
- domain-bounded  

---

## 6.4 Subject Shape

`subject` must be an object with:

- `kind`
- `name`

Optional fields may include:

- `fields`

---

# 7. Target

## 7.1 Purpose

`target` identifies the object, collection, or scope to which the query applies.

Targets must be explicit and domain-valid.

---

## 7.2 Common Target Kinds

Target kinds may include:

- `resolution`
- `area`
- `identity`
- `global`
- `pair`
- `deliberate`
- `item`
- `session`
- `review`
- `proposal`
- `signal`
- `receipt`
- `commit`
- `feed`
- `pipeline`

---

## 7.3 Target Shape

`target` must be an object containing:

- `kind`

Optional fields may include:

- `id`
- `ids`
- `left`
- `right`

---

## 7.4 Pair Targets

Pair targets are used for comparison-like queries.

A pair target must explicitly identify both sides.

No implicit pairing is allowed.

---

# 8. Scope

## 8.1 Purpose

`scope` constrains the slice of data examined by the query.

Scope does not change the meaning of the domain.  
It narrows what part of that domain is visible to the query.

---

## 8.2 Common Scope Dimensions

Scope dimensions may include:

- `activity`
- `time`
- `posture`
- `mode`
- `projection`
- `round`
- `history`

---

## 8.3 Activity

Examples:

- `active`
- `historical`

---

## 8.4 Time

Time is observational only and must not alter meaning.

Examples:

- `since`
- `until`

---

## 8.5 Projection

Projection is explicit where supported.

Examples:

- `resolution`
- `item`

Mixed projections are not the default unless explicitly defined by the target domain.

---

## 8.6 Round and History

Domains that preserve rounds or historical snapshots may expose scope fields such as:

- `current_round_only`
- `include_round_history`

These must remain explicit.

---

# 9. Filters

## 9.1 Purpose

`filters` constrain result sets within the selected domain and scope.

Filters are domain-specific but structurally consistent.

---

## 9.2 Common Filter Dimensions

Examples may include:

- `state`
- `status`
- `volatility`
- `confidence`
- `relationship_type`
- `provenance`
- `blocking`
- `rule_identity`

---

## 9.3 Filter Principle

Filters must:

- be explicit  
- be deterministic  
- be domain-valid  
- not imply joins or query-time inference  

Unsupported filters must fail explicitly.

---

# 10. Output

## 10.1 Purpose

`output` controls the structural presentation level of the query result.

It does not alter truth.  
It only alters how much structure is requested.

---

## 10.2 Canonical Output Modes

Supported output modes include:

- `summary`
- `structured`
- `detailed`

---

## 10.3 Output Shape

`output` must be an object with at least:

- `mode`

Optional fields may include host-neutral presentation hints that do not alter semantics.

---

# 11. Context

## 11.1 Purpose

`context` provides optional query-time modifiers that affect interpretation posture without mutating truth.

This is distinct from filters and must remain non-authoritative.

---

## 11.2 Constraints

Context must not:

- mutate state  
- redefine domain semantics  
- create inferred relationships  
- override explicit stored data  

---

# 12. Metadata

## 12.1 Purpose

`metadata` may carry host-neutral query metadata such as:

- query identifier
- client correlation identifier
- requested rule identity surface

Metadata must not affect deterministic result meaning unless explicitly standardized.

---

# 13. Managed Read Surface Model

## 13.1 Principle

CQL queries managed read surfaces.

A managed read surface may be backed by:

- operational stores  
- review stores  
- durable stores  
- derived stores  
- isolated stores  
- untrusted stores  

---

## 13.2 Store-First Rule

CQL should query store-backed or materialized surfaces by default.

Implicit in-memory live process inspection is not the default model.

---

## 13.3 Domain Surface Mapping

Each domain must define its primary managed read surface.

Examples:

- `review` → Review Store
- `runtime` → runtime-managed operational surface
- `legitimacy` → legitimacy/session object surfaces
- `ccs` → Commit Store
- `csg` → Graph Store
- `cas` → Alignment Store
- `audit` → Audit Store
- `cds` → deliberate workspace/artifact surfaces
- `csp` → feed/pipeline surfaces
- `crs` → untrusted relay/foreign artifact surfaces

This mapping is domain-owned and versioned outside the JSON IL core.

---

# 14. Determinism Rules

Given identical:

- domain
- subject
- target
- scope
- filters
- output
- context

the JSON IL query must resolve to identical meaning.

CQL must not depend on:

- mutation timing
- storage iteration order
- hidden defaults
- live process coincidence
- inferred joins
- ambient host state

---

# 15. Non-Interpretation Rules

CQL JSON IL must not:

- infer intent
- infer missing relationships
- infer missing data as false
- reinterpret unknown provenance
- synthesize joins not explicitly defined by a domain view

It selects and constrains visible data only.

---

# 16. Extension Model

## 16.1 Principle

Hosts may expose extension views while preserving the canonical JSON IL object shape.

Extensions must not alter the JSON IL grammar.

---

## 16.2 Extension Subject Naming

Extension views must use namespaced naming consistent with the CQL foundation.

Examples:

- `x.cas.myhost.ops_surface`
- `x.review.myhost.merge_panel`

---

## 16.3 Extension Constraints

Extensions must:

- be read-only  
- be deterministic  
- remain externally atomic  
- not redefine canonical fields or grammar  

---

# 17. Transport Neutrality

The JSON IL is transport-neutral.

It may be used in:

- library calls
- CLI-compiled execution
- APIs
- FFI boundaries
- test fixtures

Its meaning must remain stable across all representations.

---

# 18. Relationship to DSL

A human-facing CQL DSL may exist later.

If present:

- the DSL must compile into valid JSON IL  
- the JSON IL remains canonical  
- DSL convenience must not introduce new semantics unavailable in JSON IL  

---

# 19. Relationship to Results

This specification defines only query representation.

It does not define:

- result schema
- domain response payload structure
- host rendering
- pagination rules
- mutation hooks

Those belong to domain-specific or host-specific query result contracts.

---

# 20. Invariants

- JSON IL is the canonical CQL representation  
- all query components must be explicit where required  
- domains map to managed read surfaces  
- store-backed querying is the default model  
- filters are deterministic and domain-bounded  
- output affects presentation depth, not truth  
- context must not mutate semantics beyond declared posture behavior  
- JSON IL must remain transport-neutral  
- extensions may extend surfaces, not grammar  
- CQL remains read-only and non-interpreting  

Violation of these invariants compromises query integrity.

---

# 21. Mental Model

The JSON IL asks:

- what domain am I querying?
- what do I want from it?
- what object or collection is the target?
- what slice of that domain should be visible?
- what filters narrow the result?
- how much structure should be returned?

It is the stable machine form of a Charter query.

---

# 22. Final Principle

CQL JSON IL exists so that all Charter systems can speak one read language internally.

It makes queries:

- explicit
- deterministic
- substrate-neutral
- transportable
- safe to embed

The DSL may come later.

The JSON form is the foundation.