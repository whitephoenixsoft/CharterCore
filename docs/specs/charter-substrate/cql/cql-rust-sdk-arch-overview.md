# CQL Rust SDK Architecture Overview

Status: HIGH-LEVEL ARCHITECTURE DRAFT
Applies to: CQL SDK, CQL Core, JSON IL, DSL, adapters, capability declarations, execution context, adapter outcomes, result envelopes, Charter integration
Does NOT define: final Rust APIs, trait signatures, implementation details, storage backends, transport protocols, or substrate internals

---

# 1. Purpose

This document defines the high-level architecture for CQL as a Rust SDK.

CQL is a standalone Rust SDK for declaring, validating, dispatching, and composing read-only query surfaces across independently owned domains.

Charter uses CQL as its preferred query layer, but CQL is not limited to Charter.

CQL should be usable by any Rust host that wants to make its APIs queryable through a deterministic, discoverable, capability-declared read contract.

---

# 2. Architectural Identity

CQL is not a database.

CQL is not a graph engine.

CQL is not a semantic engine.

CQL is not a legitimacy engine.

CQL is not an explanation layer.

CQL is a read-only query SDK.

CQL owns:

- query structure
- canonical JSON IL
- DSL compilation
- query validation
- capability discovery
- adapter contracts
- execution orchestration
- result envelope construction
- response truth reporting

CQL does not own:

- domain meaning
- substrate semantics
- storage implementation
- host business logic
- legitimacy truth
- graph truth
- identity truth
- semantic interpretation
- final explanation behavior

The core rule is:

Domains own meaning.
CQL owns access, validation, dispatch, and execution truth reporting.

---

# 3. Core Architectural Principle

The CQL trust chain is:

JSON IL
→ Capability Declaration
→ Query Validation
→ Execution Context
→ Adapter Outcome
→ Result Envelope

Each layer has a distinct role.

JSON IL defines what is being asked.

Capability Declaration defines what is legal and supported.

Query Validation confirms the query matches declared capabilities.

Execution Context controls the runtime boundary.

Adapter Outcome captures what the domain handler produced.

Result Envelope finalizes the response truth.

---

# 4. SDK-First Position

CQL should be designed as an SDK first.

The DSL is important, but it is not the center of the architecture.

The canonical query form is JSON IL or a typed Rust equivalent.

The main authoring surfaces are:

1. JSON IL
2. Rust query builders
3. Human DSL

All authoring surfaces must normalize into the same query model.

No authoring surface may bypass validation.

No authoring surface may introduce hidden semantics.

---

# 5. Rust Crate Direction

CQL should be a Rust crate family.

Recommended conceptual crates:

## cql-core

Owns the domain-neutral query model.

Responsibilities:

- query data structures
- domain names
- subjects
- targets
- scopes
- filters
- output definitions
- metadata concepts
- shared errors
- shared result condition types

## cql-json-il

Owns canonical JSON IL support.

Responsibilities:

- serialization
- deserialization
- normalization
- JSON IL validation
- compatibility with persisted query forms

## cql-adapter

Owns adapter-facing contracts.

Responsibilities:

- adapter concepts
- handler concepts
- adapter outcomes
- capability declaration types
- dependency declarations
- domain registration contracts

## cql-engine

Owns execution orchestration.

Responsibilities:

- registry
- capability lookup
- validation pipeline
- execution context creation
- adapter dispatch
- dependency call control
- envelope finalization

## cql-dsl

Owns the human DSL.

Responsibilities:

- parsing
- syntax validation
- DSL-to-JSON-IL compilation
- human-friendly errors

## cql-macros

Optional crate for host ergonomics.

Responsibilities:

- derive helpers
- registration helpers
- declaration helpers

## cql-charter

Optional Charter integration crate.

Responsibilities:

- Charter domain conventions
- Charter adapter registration helpers
- Charter-specific query helper builders
- integration glue for Charter substrates

CQL core must not depend on Charter.

Charter may depend on CQL.

---

# 6. Dependency Direction

The dependency direction must remain strict.

Correct direction:

- CQL core is domain-neutral.
- CQL engine is domain-neutral.
- CQL DSL is domain-neutral.
- Charter substrates implement CQL adapter contracts.
- Host applications register domains into a CQL registry.
- Charter may provide integration crates over CQL.

Incorrect direction:

- CQL core imports CAS semantics.
- CQL core imports CSG semantics.
- CQL core imports Charter legitimacy concepts.
- CQL assumes only Charter domains exist.
- CQL owns the meaning of host-defined domains.

---

# 7. Domain Model

A CQL domain is a declared ownership boundary for queryable read surfaces.

Domains are not hard-coded.

Domains are registered through adapters or capability providers.

Examples of Charter domains:

- charter
- cas
- csg
- cis
- ccare
- csp
- runtime

Examples of non-Charter domains:

- orders
- inventory
- billing
- workflow
- analytics
- compliance
- crm
- logistics

A domain is valid only when declared.

CQL must not assume a fixed universe of domains.

---

# 8. Host-Defined Domains

Host-defined domains are first-class.

A host may define its own domain when it owns the meaning of the read surface.

Host-defined domains are appropriate when:

- the host owns the business meaning
- the host exposes its own API surface
- the host composes data from multiple domains
- the host needs a stable queryable read contract
- the view does not naturally belong to an existing substrate

A host-defined domain must still follow CQL rules:

- read-only
- deterministic
- capability-declared
- validated before dispatch
- bounded
- explicit about targets
- explicit about scope
- explicit about filters
- explicit about output
- explicit about result conditions

---

# 9. Views and Raw Surfaces

The primary unit of query capability is a declared view or raw surface.

A view is a managed read surface.

A raw surface is a registered typed lower-level read surface.

Views are preferred for public query access.

Raw surfaces are allowed only when registered, typed, and capability-declared.

A view or raw surface should declare:

- supported target kinds
- supported scope fields
- supported filters
- supported output modes
- supported selectable fields
- boundedness rules
- version metadata
- result conditions
- dependencies when applicable

---

# 10. Managed Composition

Managed composition is allowed.

Public arbitrary joins are not part of CQL V1.

A host may expose a composed view that internally queries other CQL domains.

Example concept:

A runtime domain may expose:

- runtime.area_status

Internally, that view may query:

- cas.posture
- csg.boundary
- ccare.recent_observations
- charter.active_resolutions

The caller sees one declared runtime-owned view.

The host owns the composition.

CQL validates and controls execution.

This preserves boundedness while allowing useful higher-level views.

---

# 11. Composition Guardrails

Managed composition must remain safe.

A composed view must not allow:

- hidden mutation
- infinite recursion
- silent scope widening
- authorization bypass
- undeclared dependencies
- unbounded fan-out
- hidden partial failure
- semantic ownership collapse

Composed views should declare dependencies.

Dependency declarations support:

- introspection
- debugging
- validation
- tracing
- cycle detection
- partial result reporting
- explanation of source contribution

---

# 12. Capability Declarations

Capability declarations are enforceable contracts.

They are not documentation only.

A query is valid only when it matches declared capabilities.

If a domain, view, raw surface, target, scope, filter, output mode, field, or argument is not declared, CQL must reject the query before dispatch.

CQL must not guess.

CQL must not silently ignore unsupported query features.

Capability declarations should support:

- domain discovery
- view discovery
- raw surface discovery
- target discovery
- scope discovery
- filter discovery
- output discovery
- field discovery
- dependency discovery
- version discovery
- result condition discovery
- boundedness rules

Capability discovery is essential for SDK usability.

---

# 13. Execution Context

Execution Context is the controlled runtime boundary for query execution.

Execution Context answers:

Under what controlled conditions did this query execute?

Execution Context carries:

- caller context
- authorization context
- visibility context
- trace context
- recursion depth
- execution budget
- scope propagation rules
- dependency call policy
- partial failure policy
- boundedness controls
- version resolution context
- envelope construction context

Execution Context does not define domain meaning.

Execution Context does not define query legality.

Capability declarations define legality.

Execution Context enforces and records safe execution behavior.

---

# 14. Adapter Outcomes

Adapter Outcome is the minimal response returned by an adapter or handler.

Hosts should not manually construct full CQL result envelopes during normal execution.

Hosts return Adapter Outcomes.

CQL converts Adapter Outcomes into Result Envelopes.

Common conceptual outcomes include:

- ok
- empty
- nonexistent
- unauthorized
- hidden
- unsupported
- unavailable
- partial
- error

This keeps host integration simple while preserving CQL trust guarantees.

The adapter owns:

- payload data
- domain-specific details
- domain-specific warnings
- domain-specific diagnostics
- domain-specific dependency notes

CQL owns:

- envelope structure
- query identity
- version metadata
- selected field reporting
- trace references
- condition normalization
- partial reporting
- final execution truth

---

# 15. Result Envelope

The Result Envelope is the standard CQL response wrapper.

It answers:

What happened when this query executed?

The Result Envelope preserves:

- execution status
- result condition
- data
- metadata
- diagnostics
- errors
- warnings
- partial status
- source attribution
- version resolution
- selected fields
- query identity
- replay metadata
- trace references

The Result Envelope must preserve the distinction between:

- success
- empty
- nonexistent
- unauthorized
- hidden
- unsupported
- unavailable
- partial
- invalid
- error

CQL owns response truth.

Domains own payload meaning.

---

# 16. Output and Field Selection

CQL should support output shaping.

Output modes may include:

- summary
- structured
- detailed

Detailed may be the default when supported.

Summary should only be declared when the domain or view can support it.

Flat field selection may be supported as output shaping.

Field selection must not:

- change query meaning
- widen scope
- imply joins
- trigger hidden semantic interpretation
- access undeclared fields
- bypass capabilities

Field selection only shapes the returned payload.

Nested GraphQL-style selection should remain out of V1 unless later justified.

---

# 17. Validation Pipeline

The conceptual validation flow is:

1. Receive query from JSON IL, DSL, or Rust builder.
2. Normalize query into canonical form.
3. Resolve requested domain.
4. Resolve requested subject, view, or raw surface.
5. Resolve target kind.
6. Validate scope.
7. Validate filters.
8. Validate output mode.
9. Validate selected fields.
10. Validate boundedness requirements.
11. Validate capability version.
12. Reject unsupported or invalid queries before dispatch.

Unsupported features must be rejected, not ignored.

---

# 18. Execution Pipeline

The conceptual execution flow is:

1. Query is authored.
2. Query is normalized.
3. Query is validated against capability declarations.
4. Execution Context is established.
5. Adapter is invoked.
6. Adapter may make managed dependency calls through Execution Context.
7. Adapter returns Adapter Outcome.
8. CQL builds the Result Envelope.
9. CQL finalizes metadata, diagnostics, versions, trace, and result condition.
10. Caller receives Result Envelope.

The execution pipeline must preserve:

- read-only behavior
- capability boundaries
- scope boundaries
- authorization posture
- boundedness
- traceability
- result truth

---

# 19. Charter Integration

Charter should use CQL as its preferred read layer.

Each Charter substrate should remain independent.

Each substrate may expose CQL adapters.

Examples:

- Charter legitimacy exposes resolution/session/area views.
- CAS exposes semantic and dynamic views.
- CSG exposes graph and structure views.
- CIS exposes identity boundary views.
- CCare exposes observation and signal views.
- CSP exposes feed views.
- CRS exposes federation-related read views.
- Runtime exposes host-owned composed views.

CQL should not absorb these substrates.

CQL should make them queryable.

---

# 20. Host Usability Requirement

Host usability is a first-class architectural requirement.

A host should be able to expose a queryable API without building a full query engine.

The SDK should make this path easy:

1. Define a domain.
2. Declare views or raw surfaces.
3. Declare targets.
4. Declare scopes.
5. Declare filters.
6. Declare output modes.
7. Declare selectable fields.
8. Attach handlers.
9. Register capabilities.
10. Execute validated queries.
11. Return Adapter Outcomes.

The host should not need to understand Charter to use CQL.

The host should not need to use the DSL.

The host should be able to use typed Rust builders or JSON IL directly.

---

# 21. Design Boundaries

CQL V1 should include:

- read-only query execution
- JSON IL
- DSL compilation
- Rust query builders
- adapter registration
- capability declarations
- capability discovery
- validation before dispatch
- execution context
- adapter outcomes
- result envelopes
- managed composed views
- flat field selection
- version metadata
- explicit absence and error conditions

CQL V1 should avoid:

- mutation
- arbitrary joins
- general graph traversal language
- hidden resolver chains
- nested GraphQL-style selections
- subscriptions
- write workflows
- semantic explanation ownership
- storage ownership
- substrate ownership
- host business logic ownership

---

# 22. Core Invariants

CQL core is domain-neutral.

Domains are adapter-declared.

Hosts may define domains.

Hosts may define managed views.

Managed composition is internal to declared views.

Public arbitrary joins are out of V1.

Capability validation happens before dispatch.

Unsupported behavior is rejected, not ignored.

Execution Context must not change query meaning.

Adapter Outcomes are not final envelopes.

Result Envelopes preserve execution truth.

Domains own payload meaning.

CQL owns access and response truth.

Field selection shapes output only.

Hidden is distinct from nonexistent.

Unauthorized is distinct from hidden.

Empty is distinct from nonexistent.

Partial must be explicit.

Version metadata is required.

Traceability must be possible.

---

# 23. High-Level Architecture Summary

CQL is the reusable query spine for Charter and for any Rust host that wants deterministic, discoverable, read-only query surfaces.

The architecture should remain simple:

- JSON IL defines the query.
- Capabilities define what is legal.
- Validation protects the boundary.
- Execution Context controls runtime behavior.
- Adapters execute domain-owned logic.
- Adapter Outcomes keep host integration simple.
- Result Envelopes preserve execution truth.
- Domains retain semantic ownership.

This lets CQL serve Charter without becoming Charter-only.

It also lets non-Charter hosts use CQL without adopting Charter semantics.

The result is a standalone Rust SDK that makes APIs queryable while preserving explicit boundaries, deterministic validation, and trustworthy results.