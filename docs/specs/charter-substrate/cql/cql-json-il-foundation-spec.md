# Charter Query Layer (CQL) — JSON IL Foundation Specification

Status: FOUNDATIONAL (DRAFT)  
Applies to: All Charter-readable substrates and managed read surfaces  
Depends On: CQL Foundation Specification, Determinism, Non-Interpretation Principle, Versioning & Identity Model, Provenance Model  
Does NOT define: DSL syntax, UI rendering, storage engine implementation, mutation semantics, or host-specific transport bindings  

---

# 1. Purpose

This document defines the JSON Intermediate Language for CQL.

It exists to:

- provide a canonical machine-readable representation of CQL queries  
- unify query behavior across libraries, hosts, and interfaces  
- support deterministic read access over managed Charter surfaces  
- provide a stable substrate-neutral query contract  
- allow DSLs and host adapters to compile into a common form  

CQL JSON Intermediate Language is the canonical structural representation of a query.

The human DSL, if present, is an ergonomic layer that compiles into this form.

---

# 2. Core Principle

The JSON Intermediate Language is the canonical CQL query form.

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

A CQL JSON Intermediate Language query is a structured object containing six conceptual parts:

1. domain  
2. subject  
3. target  
4. scope  
5. filters  
6. output  

Additional metadata may be attached where explicitly defined.

---

# 4. Top-Level Query Shape

A canonical JSON Intermediate Language query object must support the following top-level fields:

- domain  
- subject  
- target  
- scope  
- filters  
- output  

Optional top-level fields may include:

- context  
- metadata  

No top-level field may imply mutation.

---

# 5. Domain

## 5.1 Purpose

The domain field identifies which Charter read domain or domains are being queried.

A domain corresponds to a managed read surface owned by or exposed through a Charter substrate.

---

## 5.2 Properties

A domain:

- must be explicit  
- must resolve to deterministic read surfaces  
- must not imply mutation  
- must not imply interpretation beyond domain-defined query behavior  

---

## 5.3 Cardinality

The domain field may be:

- a single domain  
- an explicit array of domains  

When multiple domains are specified:

- execution must remain explicit  
- no implicit merging is allowed  
- results must preserve domain attribution  

---

## 5.4 Canonical Domains

Canonical domains may include:

- runtime  
- review  
- legitimacy  
- ccs  
- csg  
- cis  
- cas  
- ccare  
- cds  
- csp  
- crs  
- audit  

Additional domains may be introduced only through explicit versioned extension.

---

# 6. Subject

## 6.1 Purpose

The subject identifies what kind of data or view is being requested from the selected domain.

Two subject classes exist:

- view  
- raw  

---

## 6.2 View Subject

A view requests a named domain-defined surface.

Examples include:

- posture  
- graph  
- items  
- provenance  
- reviews  
- proposals  

---

## 6.3 Raw Subject

A raw subject requests explicitly defined structural data or fields from a domain.

Raw access must remain:

- read-only  
- explicit  
- domain-bounded  

---

## 6.4 Subject Shape

The subject must be an object containing:

- kind  
- name  

Optional fields may include:

- fields  
- args  

---

## 6.5 Arguments

Arguments allow parameterization of subject behavior.

Arguments must:

- be explicitly defined by the domain  
- be deterministic  
- not introduce implicit semantics  

---

# 7. Target

## 7.1 Purpose

The target identifies the object, collection, or scope to which the query applies.

Targets must be explicit and domain-valid.

---

## 7.2 Common Target Kinds

Target kinds may include:

- resolution  
- area  
- identity  
- global  
- pair  
- deliberate  
- item  
- session  
- review  
- proposal  
- signal  
- receipt  
- commit  
- feed  
- pipeline  

---

## 7.3 Target Shape

The target must be an object containing:

- kind  

Optional fields may include:

- id  
- ids  
- left  
- right  

---

## 7.4 Collection Targets

Targets may explicitly identify collections through ids.

Collection targets must be:

- explicit  
- finite  
- deterministic  

---

## 7.5 Pair Targets

Pair targets must explicitly identify both sides.

No implicit pairing is allowed.

---

# 8. Scope

## 8.1 Purpose

Scope constrains the slice of data examined by the query.

Scope defines where data is visible within a domain.

---

## 8.2 Scope Rule

Scope must not reduce a result set based on value conditions.

Scope must not introduce filtering logic.

---

## 8.3 Common Scope Dimensions

Scope dimensions may include:

- activity  
- time  
- posture  
- mode  
- projection  
- round  
- history  

---

## 8.4 Activity

Examples include:

- active  
- historical  

---

## 8.5 Time

Time is observational only and must not alter meaning.

Examples include:

- since  
- until  

---

## 8.6 Projection

Projection is explicit where supported.

Examples include:

- resolution  
- item  

Mixed projections must be explicitly supported by the domain.

---

## 8.7 Round and History

Domains that preserve rounds or history may expose fields such as:

- current_round_only  
- include_round_history  

---

# 9. Filters

## 9.1 Purpose

Filters constrain result sets within the selected domain and scope.

---

## 9.2 Filter Rule

Filters must not change visibility boundaries of a domain.

Filters operate only on data already visible within scope.

---

## 9.3 Common Filter Dimensions

Examples include:

- state  
- status  
- volatility  
- confidence  
- relationship_type  
- provenance  
- blocking  
- rule_identity  

---

## 9.4 Constraints

Filters must:

- be explicit  
- be deterministic  
- be domain-valid  
- fail when unsupported  

Filters must not:

- imply joins  
- introduce inference  
- cross domain boundaries implicitly  

---

# 10. Output

## 10.1 Purpose

Output controls the structural presentation level of the query result.

---

## 10.2 Principle

Output does not alter truth.

Output only controls representation depth.

---

## 10.3 Canonical Output Modes

Supported output modes include:

- summary  
- structured  
- detailed  

---

## 10.4 Output Shape

The output must be an object containing:

- mode  

Optional fields may include host-neutral presentation hints.

---

## 11. Context

## 11.1 Purpose

Context provides optional modifiers that affect query posture without mutating truth.

---

## 11.2 Constraints

Context must:

- be explicitly defined by the domain  
- be non-default  
- be visible in result metadata  

Context must not:

- mutate state  
- redefine domain semantics  
- create inferred relationships  
- override explicit data  

---

# 12. Metadata

## 12.1 Purpose

Metadata carries host-neutral query metadata.

Examples include:

- query identifier  
- correlation identifier  
- issued timestamp  

---

## 12.2 Constraint

Metadata must not affect query meaning unless explicitly standardized.

---

# 13. Managed Read Surface Model

## 13.1 Principle

CQL queries managed read surfaces.

---

## 13.2 Surface Types

Managed read surfaces may include:

- operational surfaces  
- review surfaces  
- durable artifact stores  
- derived stores  
- isolated stores  
- untrusted stores  

---

## 13.3 Store-First Rule

CQL queries store-backed or materialized surfaces by default.

---

## 13.4 Runtime Surfaces

Runtime may expose materialized operational views that behave as managed read surfaces.

---

## 13.5 Domain Surface Mapping

Each domain must define its primary managed read surface.

This mapping is domain-owned and versioned outside the JSON Intermediate Language.

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

the query must resolve to identical meaning.

CQL must not depend on:

- mutation timing  
- storage iteration order  
- hidden defaults  
- ambient host state  
- implicit joins  

---

## 14.1 Ordering

Result ordering must be explicitly defined by the domain or explicitly requested.

No implicit ordering is allowed.

---

# 15. Non-Interpretation Rules

CQL must not:

- infer intent  
- infer missing relationships  
- infer missing data as false  
- reinterpret unknown provenance  
- synthesize joins not explicitly defined  

---

# 16. Extension Model

## 16.1 Principle

Extensions may add new views while preserving the JSON Intermediate Language structure.

---

## 16.2 Naming

Extension views must be namespaced.

---

## 16.3 Constraints

Extensions must:

- be read-only  
- be deterministic  
- remain structurally compatible  

Extensions must not:

- modify grammar  
- override canonical fields  

---

# 17. Transport Neutrality

The JSON Intermediate Language is transport-neutral.

It may be used in:

- library calls  
- CLI execution  
- APIs  
- FFI boundaries  
- testing  

Its meaning must remain stable across all transports.

---

# 18. Relationship to DSL

A DSL may exist as a human-facing layer.

If present:

- it must compile into valid JSON Intermediate Language  
- it must not introduce new semantics  

---

# 19. Relationship to Results

This document defines query structure only.

It does not define:

- result schema  
- payload structure  
- pagination  
- rendering  

---

## 19.1 Multi-Domain Results

When multiple domains are queried:

- results must preserve domain attribution  
- results must not be implicitly merged  
- ambiguity must remain visible  

---

# 20. Invariants

- JSON Intermediate Language is canonical  
- all query components are explicit  
- domains map to managed read surfaces  
- scope defines visibility, not filtering  
- filters constrain, not expose  
- output affects structure, not truth  
- context is explicit and non-authoritative  
- ordering is explicit or domain-defined  
- extensions do not alter grammar  
- CQL is read-only and non-interpreting  

---

# 21. Mental Model

The JSON Intermediate Language defines:

- what domain is queried  
- what data is requested  
- what object or set is targeted  
- what slice is visible  
- what constraints apply  
- how the result is shaped  

It is the canonical machine form of a Charter query.

---

# 22. Final Principle

The JSON Intermediate Language allows all Charter systems to share one internal query language.

It ensures queries are:

- explicit  
- deterministic  
- substrate-neutral  
- transportable  
- safe  

The DSL may evolve.

The JSON form remains the foundation.