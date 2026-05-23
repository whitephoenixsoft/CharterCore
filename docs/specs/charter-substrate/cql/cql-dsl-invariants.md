# CQL DSL Invariants

Status: STRUCTURAL  
Applies to: CQL DSL, CLI query input, human-authored query strings  
Depends On: CQL Foundation Specification, Shared CQL Invariants, CQL JSON Intermediate Language Specification  
Does NOT define: canonical query structure, adapter execution, result payload schema, substrate semantics, or mutation behavior  

---

# 1. Purpose

This document defines invariants for the CQL human-facing DSL.

The DSL exists to make CQL usable by humans.

It is intended for:

- CLI usage
- documentation examples
- manual query construction
- host interfaces
- operator workflows
- debugging
- lightweight exploration

The DSL is not the canonical representation of a query.

Every valid DSL query must compile into JSON Intermediate Language.

---

# 2. Foundational Rule

The DSL is syntax.

JSON Intermediate Language is semantics.

A DSL query has no execution meaning until it has been compiled into JSON Intermediate Language and validated against adapter capabilities.

---

# 3. DSL Invariants

## DSL-INV-01 — DSL Compiles to JSON IL

Every valid DSL query must compile into valid JSON Intermediate Language.

No DSL query may execute directly.

The compiler must produce the canonical query components:

- domain
- subject
- target
- scope
- filters
- output
- context when applicable
- metadata when applicable

The DSL may provide a more ergonomic way to author the query, but it must not bypass the canonical query model.

---

## DSL-INV-02 — DSL Must Be Lossless

A DSL query must not lose meaning when compiled into JSON Intermediate Language.

All meaningful DSL components must have a corresponding JSON Intermediate Language representation.

If a DSL feature cannot be represented in JSON Intermediate Language, the DSL must not support that feature yet.

The DSL must not grow ahead of the canonical model.

This protects CQL from having two competing query languages.

---

## DSL-INV-03 — DSL Must Not Infer Hidden Meaning

The DSL compiler must not infer hidden substrate meaning.

It may parse explicit syntax.

It may apply declared defaults.

It may normalize aliases if aliases are declared.

It must not guess user intent.

The DSL must not transform vague language into substrate-specific meaning.

For example, a phrase like “show payments health” should not be treated as core CQL DSL unless the domain, subject, target, scope, filters, and output mapping are explicitly declared by a higher-level command layer.

The CQL DSL should remain precise enough that users can learn it and predict its compiled JSON Intermediate Language form.

---

## DSL-INV-04 — Defaults Must Be Capability-Declared

The DSL may support defaults only when those defaults are declared by the relevant domain or adapter capability profile.

The parser must not invent defaults.

Examples of possible declared defaults include:

- default output mode
- default projection
- default activity scope
- default result envelope
- default time window
- default target kind in a constrained command context

Defaults must be visible in capability metadata and must compile into explicit JSON Intermediate Language.

After compilation, defaults should no longer be implicit.

They should appear in the normalized query object.

---

## DSL-INV-05 — Readability Must Preserve Precision

The DSL should be readable by humans.

However, readability must not reduce precision.

A CQL DSL query should make the following visible:

- domain
- subject
- target
- scope when relevant
- filters when relevant
- output intent when relevant

Human-friendly syntax is allowed.

Ambiguous natural-language-like commands are not part of the core DSL unless they compile through an explicit declared command profile.

The DSL should feel ergonomic, not magical.

---

## DSL-INV-06 — Grammar Stability

The DSL grammar should remain small and stable.

The canonical shape is:

domain subject on target scope filters output

New capability should usually be added through:

- new views
- new raw surfaces
- new target kinds
- new scope fields
- new filters
- new output modes
- new extension views

New capability should not usually require grammar expansion.

This keeps the DSL learnable and prevents host-specific language drift.

---

## DSL-INV-07 — Extension Views Are Namespaced

Extension views must use explicit namespacing.

This prevents host-defined meaning from colliding with canonical CQL meaning.

Extension names should make ownership visible.

Examples of extension ownership patterns:

- x.cas.host_name.view_name
- x.cds.host_name.view_name
- x.runtime.host_name.view_name
- x.csg.host_name.view_name

Extension views may expose composed or host-specific read surfaces.

They must remain:

- read-only
- deterministic
- capability-declared
- structurally compatible with JSON Intermediate Language
- non-mutating
- non-interpreting outside their declared surface

Extension views must not change the DSL grammar.

---

## DSL-INV-08 — Rejection Beats Guessing

When a DSL query is ambiguous, malformed, unsupported, or incomplete, the compiler must reject it with a useful error.

The compiler must not guess.

Examples of rejection cases include:

- unknown domain
- unknown subject
- unknown view
- unknown raw surface
- missing target
- ambiguous target
- unsupported target kind
- unsupported scope field
- unsupported filter
- unsupported output mode
- unsupported extension view
- malformed argument list
- multiple possible parses
- filter not supported by selected view
- scope not supported by selected adapter

A failed query is safer than a guessed query.

The user should be told what was unsupported and, when possible, what valid options exist.

---

# 4. Compilation Requirements

DSL compilation must be:

- deterministic
- lossless
- explicit
- non-inferential
- adapter-validatable
- compatible with JSON Intermediate Language

Compilation must not:

- introduce mutation
- infer authority
- reinterpret domain semantics
- create implicit joins
- expand scope through filters
- ignore unknown fields
- bypass extension namespace rules
- execute against a substrate directly

The output of DSL compilation is JSON Intermediate Language.

Execution happens only after JSON Intermediate Language validation.

---

# 5. Suggested Core Grammar Shape

The DSL should preserve the canonical linear shape:

domain subject on target scope filters output

Where:

- domain identifies the read domain
- subject identifies the view or raw surface
- target identifies the object, collection, pair, or global scope
- scope defines visibility
- filters constrain visible data
- output defines result shape

The grammar may allow ergonomic forms, but every accepted form must map deterministically to the canonical query model.

---

# 6. Non-Goals

The CQL DSL does not define:

- canonical query semantics
- substrate truth
- CAS interpretation
- graph algorithms
- identity resolution
- legitimacy rules
- observation semantics
- adapter implementation
- result payload schemas
- mutation behavior
- natural language understanding

The DSL is not a chatbot language.

The DSL is not a general programming language.

The DSL is not a graph traversal language.

The DSL is not a SQL replacement.

The DSL is a precise human syntax for constructing CQL JSON Intermediate Language queries.

---

# 7. Summary

The CQL DSL exists to make CQL approachable for humans without weakening the canonical query model.

It must compile into JSON Intermediate Language.

It must preserve meaning without loss.

It must not infer hidden semantics.

It must reject ambiguity.

It must remain small, stable, readable, and precise.