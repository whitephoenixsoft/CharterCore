# CQL JSON Intermediate Language Invariants

Status: FOUNDATIONAL  
Applies to: CQL JSON Intermediate Language  
Depends On: CQL Foundation Specification, Shared CQL Invariants, Adapter Contract Specification  
Does NOT define: DSL syntax, substrate storage, result payload schema, mutation semantics, or host-specific transport bindings  

---

# 1. Purpose

This document defines invariants for the CQL JSON Intermediate Language.

JSON Intermediate Language is the canonical machine-readable representation of a CQL query.

All CQL query paths must eventually normalize into JSON Intermediate Language or a typed equivalent with the same structure and semantics.

The JSON Intermediate Language exists to provide:

- deterministic query representation
- substrate-neutral validation
- adapter-safe execution
- stable SDK behavior
- explicit query routing
- canonical logging and replay
- a compilation target for the human DSL

---

# 2. Foundational Rule

JSON Intermediate Language is the canonical CQL query form.

The DSL is not canonical.

SDK helper APIs are not canonical.

CLI text is not canonical.

Any supported authoring surface must compile, construct, or normalize into JSON Intermediate Language before execution.

---

# 3. JSON IL Invariants

## JSON-INV-01 — JSON IL Is Canonical

The JSON Intermediate Language form is the authoritative representation of a CQL query.

If a query begins as DSL, SDK builder calls, CLI input, or host-specific syntax, it must be compiled or normalized into JSON Intermediate Language before execution.

If DSL surface meaning and JSON Intermediate Language meaning ever appear to conflict, JSON Intermediate Language semantics win.

Execution must not depend on the original textual form once JSON Intermediate Language has been produced.

---

## JSON-INV-02 — Complete Structural Normalization

Before execution, every JSON Intermediate Language query must normalize into the same conceptual shape.

The canonical conceptual parts are:

- domain
- subject
- target
- scope
- filters
- output
- context
- metadata

The first six are the core query model:

- domain
- subject
- target
- scope
- filters
- output

Context and metadata may be optional, but they must have defined behavior when present.

An execution adapter should receive a normalized query object, not a partially parsed or surface-specific representation.

---

## JSON-INV-03 — Domain Owns Subject Interpretation

Subjects are resolved inside domains.

The same subject name must not be assumed to mean the same thing globally unless it is explicitly defined as canonical across domains.

For example:

- domain cas with view posture means CAS owns the posture view meaning.
- domain csg with view graph means CSG owns the graph view meaning.
- domain ccare with view observations means CCare owns the observation view meaning.

CQL may route to these subjects.

CQL must not redefine their substrate-owned meaning.

---

## JSON-INV-04 — Target Must Be Typed

Every target must declare what kind of thing it addresses.

A target identifier alone is not sufficient.

Target values must be typed because the same string may refer to different kinds of objects in different domains.

Target kinds may include:

- area
- resolution
- identity
- item
- session
- commit
- graph
- global
- pair
- collection

Invalid example:

target equals payments

Valid conceptual example:

target kind equals area, id equals payments

Typed targets are required for deterministic dispatch and validation.

---

## JSON-INV-05 — Scope and Filters Are Separate

JSON Intermediate Language must preserve a structural distinction between scope and filters.

Scope defines visibility.

Filters constrain visible data.

Scope fields must not be flattened into the same object as filters.

This distinction is required so that validation can determine whether a query is changing what data is visible or merely constraining already-visible data.

Examples of scope fields:

- activity
- since
- until
- window
- projection
- include_history
- include_superseded

Examples of filter fields:

- state
- confidence
- volatility
- semantic_state
- provenance
- relationship_type

A filter must not expand scope.

A scope declaration must not act as a hidden value filter.

---

## JSON-INV-06 — Unknown Fields Are Rejected

Unknown fields must not be silently ignored.

A JSON Intermediate Language validator must reject:

- unknown top-level fields
- unknown subject fields
- unknown target kinds
- unknown scope fields
- unknown filters
- unknown output modes
- unknown argument names
- malformed extension fields

The only exception is a field inside a sanctioned extension namespace that is explicitly accepted by the relevant adapter capability declaration.

Silent ignoring is forbidden because it causes false confidence.

A query author must be able to trust that the executed query is the query they wrote.

---

## JSON-INV-07 — Output Controls Shape, Not Meaning

Output controls result shape, detail level, or presentation intent.

Output must not change the underlying semantic meaning of the query.

Examples of output modes may include:

- summary
- structured
- detailed
- ids
- count
- envelope
- raw

Output may affect:

- field selection
- level of detail
- envelope shape
- inclusion of metadata
- inclusion of provenance
- inclusion of warnings

Output must not cause:

- semantic reinterpretation
- authority inference
- hidden analysis
- legitimacy decisions
- CAS derivation changes
- graph recomputation
- mutation
- workflow execution

An output mode such as summary must mean “return a shorter supported form,” not “interpret this for the user.”

Interpretation belongs in higher layers such as explanation, UI, CGL, or assistant tooling.

---

## JSON-INV-08 — Adapters Receive Normalized Query Context

Adapters must not receive raw DSL text as their execution input.

Adapters should receive normalized JSON Intermediate Language or a typed equivalent.

The adapter input must include the parsed and validated form of:

- domain
- subject
- target
- scope
- filters
- output
- context
- metadata

This prevents each adapter from inventing its own parser.

It also ensures that validation, capability checking, and deterministic query meaning are enforced before substrate execution.

---

# 4. Validation Requirements

A JSON Intermediate Language validator must verify:

- the query is read-only
- the domain is known
- the subject is valid for the domain
- the target kind is supported by the subject
- the scope fields are supported
- the filters are supported
- the output mode is supported
- arguments are valid
- extensions are namespaced
- unknown fields are rejected
- multi-domain behavior is explicit
- projection behavior is explicit when required

Validation must happen before adapter execution.

---

# 5. Adapter Dispatch Requirements

After validation, JSON Intermediate Language dispatch must preserve:

- domain attribution
- subject identity
- target typing
- scope boundaries
- filter constraints
- output intent
- extension namespaces
- provenance context

When multiple domains are queried, CQL must not implicitly merge their meanings.

Results must preserve domain attribution.

---

# 6. Non-Goals

JSON Intermediate Language does not define:

- the CQL DSL syntax
- storage engine implementation
- graph traversal algorithms
- CAS derivation algorithms
- legitimacy calculation
- identity resolution rules
- observation semantics
- mutation semantics
- UI display rules
- human explanation behavior

JSON Intermediate Language is the query contract.

It is not the substrate model.

---

# 7. Summary

JSON Intermediate Language is the canonical representation of a CQL query.

It must be explicit, deterministic, read-only, substrate-neutral, and adapter-validatable.

It must preserve the distinction between domain, subject, target, scope, filters, and output.

It must reject ambiguity rather than guess.

It must route queries without redefining substrate meaning.