# Charter Query Layer (CQL) — JSON Intermediate Language Foundation Specification

Status: FOUNDATIONAL (DRAFT)  
Applies to: CQL Core, JSON IL, SDK query builders, DSL compilation targets, adapter dispatch paths, Charter domains, host-defined domains, and managed read surfaces
Depends On: CQL Shared Invariants,  determinism principle, non-interpretation principle, versioning and identity model, provenance model  
Does NOT define: DSL syntax, UI rendering, substrate storage implementation, mutation semantics, host-specific transport bindings, or final result payload schemas  

---

# 1. Purpose

This document defines the JSON Intermediate Language for the Charter Query Layer.

CQL JSON Intermediate Language exists to provide a canonical machine-readable representation of CQL queries.

It supports:

- deterministic read access across host systems
- substrate-neutral query expression
- adapter-safe dispatch
- SDK construction of read-only queries
- CLI and host integration
- canonical logging and replay
- DSL compilation into a stable query structure
- capability validation before execution

CQL JSON Intermediate Language is the canonical structural representation of a CQL query. JSON IL defines what is being asked. It does not define what is legal, how execution is controlled, what the adapter produced, or what happened overall.

The human DSL, if present, is an ergonomic layer that compiles into this form.

SDK builders, CLI input, host-specific query helpers, and future authoring surfaces must normalize into JSON Intermediate Language or a typed equivalent before execution.

---

# 2. CQL Position in the Charter Ecosystem

CQL is a standalone, domain-neutral query foundation. Charter uses CQL as its native read layer, but CQL Core is not Charter-specific.

Charter domains are registered domains in CQL, not hard-coded CQL assumptions.

CQL is not a substrate.

It is not a database.

It is not an analysis engine.

It is not a legitimacy engine.

It is not a graph engine.

It is not a semantic interpretation layer.

CQL provides a common query contract that allows independent hosts to expose managed read surfaces through adapters.

Each host remains responsible for its own truth, semantics, storage, derivation rules, and read behavior.

CQL standardizes how queries are expressed, validated, routed, and shaped.

The hosts own what the queried data means.

---
# 3. CQL Trust Chain

JSON IL defines what is being asked.

Capability Declaration defines what is legal.

Execution Context controls and records runtime execution behavior.

Adapter Outcome reports what the handler produced.

Result Envelope reports what happened overall.

JSON IL remains canonical query representation, but it does not own capability legality, execution context, handler output, or response truth.

---

# 4. Comparative Design References

CQL intentionally borrows proven design ideas from existing query systems while remaining Charter-specific.

The following references are locked as design guidance:

- OpenSearch and Elasticsearch Query DSL inform the use of a canonical JSON query representation.
- GraphQL informs capability discovery, schema-like adapter declarations, and static validation before execution.
- OData informs separation of read options such as target, scope, filters, selection, and output.
- Kusto Query Language informs human DSL ergonomics and CLI-friendly query authoring.
- PromQL informs explicit temporal scope and time-windowed read behavior.
- Cypher and SPARQL inform bounded graph read surfaces while avoiding unrestricted graph traversal as the default model.

CQL does not copy any one of these systems directly.

CQL remains a custom Charter-specific read-only query contract and SDK model.

The intended pattern is:

Human DSL compiles to JSON Intermediate Language.

JSON Intermediate Language is validated against adapter capabilities.

Validated queries dispatch to substrate-owned read adapters.

Adapters return substrate-owned payloads inside CQL-compatible result handling.

---

# 5. Core Principle

The JSON Intermediate Language is the canonical CQL query form.

All CQL queries must be representable as deterministic JSON query objects.

A CQL JSON Intermediate Language object must be:

- read-only
- deterministic
- substrate-neutral
- explicit
- serializable
- validatable independent of host language
- adapter-dispatchable
- transport-neutral
- non-mutating
- non-interpreting

The JSON Intermediate Language defines what is being queried.

It does not define the substrate’s internal truth.

---

# 6. Foundational Shared Invariant

CQL queries are deterministic read-only access requests.

JSON Intermediate Language is the canonical representation.

The DSL is a human-authoring syntax that compiles into JSON Intermediate Language.

Neither layer may mutate state, infer authority, redefine substrate meaning, or bypass adapter-declared capabilities.

---

# 7. Query Object Model

A CQL JSON IL query is a structured object containing six conceptual parts:

1. domain
2. subject
3. target
4. scope
5. filters
6. output

Additional fields may be present when explicitly defined:

7. context
8. metadata

The first six fields form the core query model.

Context and metadata are optional but must have deterministic behavior when present.

The query object model must remain stable across transports, SDK languages, host implementations, and adapter boundaries.

---

# 8. Top-Level Query Shape

A canonical JSON IL query object must support the following top-level fields:

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

Unknown top-level fields must be rejected unless they are inside a sanctioned extension namespace and explicitly supported by the selected adapter capability declaration.

Top-level fields must remain structurally separate.

In particular:

- scope must not be flattened into filters
- filters must not redefine scope
- output must not redefine meaning
- metadata must not affect query meaning unless explicitly standardized
- context must not mutate truth or override substrate semantics

---

# 9. Domain

## 9.1 Purpose

The domain field identifies which read domain or domains are being queried.

A domain corresponds to a managed read surface owned by or exposed through a host system.

The domain determines which adapter or adapter group is responsible for validation and execution.

---

## 9.2 Properties

A domain must be:

- explicit
- deterministic
- read-only
- capability-declared
- resolvable to managed read surfaces
- valid within the runtime or host context

A domain must not:

- imply mutation
- imply interpretation beyond domain-defined query behavior
- silently route to another domain
- merge with another domain without explicit multi-domain behavior
- override substrate ownership

---

## 9.3 Cardinality

The domain field may identify:

- a single domain
- an explicit array of domains

When multiple domains are specified:

- execution must remain explicit
- no implicit merging is allowed
- results must preserve domain attribution
- ambiguity must remain visible
- adapters must not reinterpret each other’s outputs
- output envelopes must identify which domain produced each result section

Multi-domain queries are allowed only when the CQL implementation and selected adapters explicitly support them.

---

## 9.4 Canonical Domains

Domains are registered through capability declarations.

Charter may register domains such as...:

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

Non-Charter hosts may register domains such as orders, inventory, billing, workflow, analytics, logistics, or compliance.

Domain names must be stable enough to support validation, logging, replay, and capability discovery.

---

## 9.5 Domain Ownership

The domain determines ownership of subject interpretation.

A subject name is not globally interpreted unless explicitly defined as canonical across domains.

For example:

- cas owns the meaning of a CAS posture view
- csg owns the meaning of a graph view
- ccare owns the meaning of observation read surfaces
- cis owns the meaning of identity read surfaces

CQL may route to these domains.

CQL must not redefine their meaning.

---

## 9.6 Version References

If a query omits a version, the reference resolves to latest.

An explicit suffix such as area_status:v1 requests that exact declared version.

The resolved version is not stored as hidden meaning in JSON IL. It is reported by Result Envelope metadata after validation/execution.

---

# 10. Subject

## 10.1 Purpose

The subject identifies what kind of data, view, or raw surface is being requested from the selected domain.

Subject interpretation is domain-owned.

The same subject name may have different meaning in different domains unless explicitly standardized.

---

## 10.2 Subject Classes

Two primary subject classes exist:

- view
- raw

A view requests a named domain-defined read surface.

A raw subject requests explicitly defined structural data or fields from a domain.

Both subject classes must be read-only and capability-declared.

Views and raw surfaces are valid only when declared by capabilities.

Managed composed views appear in JSON IL as normal views.
Their composition is declared in capabilities and controlled by Execution Context.

JSON IL does not expose arbitrary joins.

---

## 10.3 View Subject

A view requests a named domain-defined surface. It can also derive it's own fields.

Examples of view subjects may include:

- posture
- graph
- items
- provenance
- reviews
- proposals
- observations
- identities
- sessions
- signals
- feeds
- alignment
- dynamics

A view may be backed by:

- durable storage
- materialized state
- derived read models
- adapter-composed read surfaces
- host-owned operational projections

A view must remain deterministic from the perspective of CQL query meaning.

---

## 10.4 Raw Subject

A raw subject requests explicitly defined structural data or fields from a domain.

Raw access must remain:

- read-only
- explicit
- domain-bounded
- capability-declared
- adapter-validated

Raw access must not become an escape hatch around managed read surfaces.

Raw fields must be explicitly supported by the domain or adapter.

---

## 10.5 Subject Shape

The subject must be an object containing:

- kind
- name

Optional fields may include:

- fields
- args

The kind identifies whether the subject is a view, raw surface, or other explicitly supported subject class.

The name identifies the domain-owned subject.

Fields request specific returned fields when supported.

Arguments parameterize subject behavior when supported.

---

## 10.6 Subject Arguments

Arguments allow parameterization of subject behavior.

Arguments must:

- be explicitly defined by the domain
- be declared in adapter capabilities
- be deterministic
- be validated before execution
- not introduce implicit semantics
- not mutate state
- not expand scope unless modeled as explicit scope

Unsupported arguments must be rejected.

Arguments must not be silently ignored.

---

# 11. Target

## 11.1 Purpose

The target identifies the object, collection, pair, or global boundary to which the query applies.

Targets must be explicit and domain-valid.

A target identifier alone is not sufficient because the same string may refer to different kinds of objects in different domains.

---

## 11.2 Target Typing

Every target must declare what kind of thing it addresses.

Target typing is required for deterministic dispatch and validation.

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
- graph
- collection

Adapters may support additional target kinds through capability declarations.

Unsupported target kinds must be rejected.

---

## 11.3 Target Shape

The target must be an object containing:

- kind

Optional fields may include:

- id
- ids
- left
- right
- relation
- collection
- namespace

The allowed shape depends on the target kind and selected subject.

---

## 11.4 Single Targets

A single target identifies one object or boundary.

Examples include:

- one area
- one resolution
- one identity
- one session
- one item
- one commit
- one feed

The adapter must validate that the target kind is supported for the selected domain and subject.

---

## 11.5 Collection Targets

Targets may explicitly identify collections through ids or declared collection references.

Collection targets must be:

- explicit
- finite unless the domain declares bounded streaming or paginated behavior
- deterministic
- validated before execution

Collection targets must not become implicit whole-domain scans unless the subject and adapter explicitly support that behavior.

---

## 11.6 Pair Targets

Pair targets must explicitly identify both sides.

No implicit pairing is allowed.

Pair targets may support comparison, relationship lookup, dependency review, conflict checks, or other domain-declared read behavior.

Both sides of a pair must be typed or resolved under declared target rules.

---

## 11.7 Global Targets

A global target may be used only when the selected domain and subject explicitly support global read behavior.

Global targets must not bypass scope requirements.

A global target does not mean unrestricted access.

It means the adapter supports a domain-defined global read surface.

---

# 12. Scope

## 12.1 Purpose

Scope defines the visible slice of data examined by the query.

Scope determines what data is in bounds before filters are applied.

Scope is not the same as filtering.

---

## 12.2 Scope Rule

Scope must not reduce a result set based on value conditions.

Scope must not introduce ordinary filtering logic.

Scope defines visibility boundaries.

Filters constrain visible data.

A filter must never expand scope.

---

## 12.3 Common Scope Dimensions

Scope dimensions may include:

- activity
- time
- posture
- mode
- projection
- round
- history
- window
- federation
- visibility
- trust_boundary

Each scope dimension must be domain-valid and adapter-declared.

Unsupported scope dimensions must be rejected.

---

## 12.4 Activity Scope

Activity scope defines whether a query sees current, active, historical, superseded, paused, archived, or otherwise state-bounded surfaces.

Examples include:

- active
- historical
- current
- archived
- superseded
- all_supported

The exact values are domain-owned.

CQL requires explicit declaration and validation.

---

## 12.5 Time Scope

Time scope constrains the temporal visibility of a query.

Examples include:

- since
- until
- at
- window
- range
- latest
- current

Time scope is observational unless the domain explicitly defines stronger temporal semantics.

Time scope must not alter substrate meaning.

A time-bounded query over observations, signals, snapshots, or derived surfaces must preserve the domain’s own temporal rules.

---

## 12.6 Projection Scope

Projection is explicit where supported.

Examples include:

- resolution
- item
- commit
- graph
- signal
- observation
- snapshot

Mixed projections must be explicitly supported by the domain.

A query over one projection must not silently return another projection.

Projection is required when omission would cause ambiguity across representational layers.

---

## 12.7 Round and History Scope

Domains that preserve rounds, phases, history, or supersession may expose fields such as:

- current_round_only
- round_index
- include_round_history
- include_history
- include_superseded
- include_replaced
- as_of

These fields define visibility boundaries.

They must not be treated as ordinary value filters.

---

## 12.8 Federation and Trust Scope

Federated systems may expose scope dimensions for trust boundaries, relay origin, externality, or identity authority.

Examples include:

- local_only
- include_external
- trusted_only
- relay_origin
- authority_boundary

These dimensions must be explicitly declared by the relevant substrate or adapter.

CQL must not infer trust.

---

# 13. Filters

## 13.1 Purpose

Filters constrain result sets within the selected domain and scope.

Filters apply after scope determines visibility.

Filters must not expand visibility boundaries.

---

## 13.2 Filter Rule

Filters operate only on data already visible within scope.

Filters must not:

- expand scope
- imply joins
- introduce inference
- cross domain boundaries implicitly
- reinterpret unknown data
- turn absence into falsehood unless the domain explicitly defines that behavior

---

## 13.3 Common Filter Dimensions

Filter dimensions may include:

- state
- status
- volatility
- confidence
- relationship_type
- provenance
- blocking
- rule_identity
- semantic_state
- participant
- identity
- authority
- source
- severity
- signal_type
- result_kind

Each filter must be declared by the selected domain or adapter.

Unsupported filters must be rejected.

---

## 13.4 Filter Constraints

Filters must be:

- explicit
- deterministic
- domain-valid
- adapter-declared
- type-valid
- compatible with the selected subject
- compatible with the selected target
- compatible with the selected scope

Filters must fail validation when unsupported.

Filters must not be silently ignored.

---

## 13.5 Filter Values

Filter values must be validated according to the domain’s capability declaration.

A filter value may be:

- scalar
- list
- range
- enum
- structured object
- domain-defined expression

Domain-defined expressions must remain deterministic and read-only.

CQL V1 should avoid general-purpose expression languages unless the adapter explicitly declares a bounded supported form.

---

# 14. Output

## 14.1 Purpose

Output controls the structural presentation level of the query result.

Output defines how much or what shape of result is requested.

Output does not alter truth.

---

## 14.2 Output Principle

Output controls shape, not meaning.

Output may affect:

- field selection
- level of detail
- result envelope shape
- inclusion of metadata
- inclusion of provenance
- inclusion of warnings
- inclusion of counts
- inclusion of identifiers
- inclusion of domain attribution

Output must not cause:

- semantic reinterpretation
- authority inference
- hidden analysis
- legitimacy decisions
- CAS derivation changes
- graph recomputation
- mutation
- workflow execution

An output mode such as summary means “return a shorter supported form.”

It does not mean “interpret this for the user.”

Interpretation belongs in higher layers such as explanation, UI, CGL, assistant tooling, or host-specific presentation layers.

Output must avoid:
- nested selection trees
- fragments
- aliases
- cross-domain traversal
- recursive selection

---

## 14.3 Canonical Output Modes

Supported output modes may include:

- summary
- structured
- detailed
- ids
- count
- envelope
- raw

The core implementation may begin with:

- summary
- structured
- detailed

Additional modes may be introduced through adapter capabilities.

Unsupported output modes must be rejected.

---

## 14.4 Output Shape

The output must be an object containing:

- mode

Optional fields may include:

- fields
- include_metadata
- include_provenance
- include_warnings
- include_counts
- limit
- cursor
- ordering
- format

Output fields are request-shaping controls.

Output may include flat field selection.

Field selection is output shaping only.

Fields must be capability-declared.

Fields must be supported by the selected output mode.

Field selection must not imply joins, traversal, hidden computation, or scope widening.

Output fields must not redefine host semantics.

---

## 14.5 Ordering

Result ordering must be explicitly defined by the domain or explicitly requested.

No implicit ordering is guaranteed.

If a domain exposes default ordering, that ordering must be declared in adapter capabilities or result metadata.

Ordering must not alter query meaning.

---

## 14.6 Pagination and Limits

Pagination and limits are output controls unless a domain explicitly models them as scope.

Examples include:

- limit
- cursor
- page_size
- continuation_token

Pagination must preserve deterministic query meaning across repeated calls when the substrate can support stable continuation.

If stable pagination cannot be guaranteed, the result should expose that limitation.

---
## 14.7 Output Dependencies 

JSON IL may select fields.

Capability Declaration defines field dependencies.

Execution Context records dependency outcomes.

Result Envelope reports attribution and partial status.

JSON IL itself does not define dependency execution behavior.

---
# 15. JSON IL Context

## 15.1 Purpose

JSON IL context is authored query context. In this section, context implies JSON IL context.

Execution Context is runtime-controlled execution context.

A caller must not use JSON IL context to inject authorization decisions, execution budget authority, trace authority, dependency-call permission, or scope-widening behavior unless explicitly allowed by capability and host policy.

JSON IL context provides optional modifiers that affect query posture without mutating truth.

JSON IL context may carry execution posture, caller posture, validation context, or host-neutral query context.

JSON IL context is not host truth.

---
## 15.2 Context Constraints

Context must:

- be explicitly defined by the domain or CQL specification
- be non-mutating
- be visible in result metadata when it affects execution posture
- be validated before execution
- preserve deterministic query meaning

Context must not:

- mutate state
- redefine domain semantics
- create inferred relationships
- override explicit data
- bypass capability validation
- silently change scope
- silently change filters

---

## 15.3 Context Examples

Context may include:

- caller role when used only for read authorization
- execution profile
- validation mode
- explanation preference
- federation posture
- trust posture
- host environment identifier
- correlation context

If context affects visibility, it must be treated as a scope or authorization concern, not as a hidden query modifier.

---

# 16. Metadata

## 16.1 Purpose

Metadata carries host-neutral query metadata.

Metadata supports tracing, correlation, logging, replay, and diagnostics.

Metadata must not be used as hidden query input unless explicitly standardized.

JSON IL metadata may carry query identifiers, correlation identifiers, source hashes, and authoring metadata.

Execution metadata, resolved versions, dependency outcomes, trace references, and final status belong to Result Envelope metadata.

---

## 16.2 Metadata Examples

Metadata may include:

- query identifier
- correlation identifier
- issued timestamp
- client identifier
- SDK version
- DSL source hash
- compiled query hash
- adapter version
- trace identifier

---

## 16.3 Metadata Constraint

Metadata must not affect query meaning unless explicitly standardized.

If metadata affects execution posture, validation, visibility, or result shape, it must be moved into a declared context, scope, filter, or output field.

---

# 17. Managed Read Surface Model

## 17.1 Principle

CQL queries managed read surfaces.

Managed read surfaces may be owned by Charter substrates or non-Charter hosts.

Managed composed views are valid managed read surfaces when declared by capability.

Public arbitrary joins remain outside JSON IL V1.

---

## 17.2 Surface Types

Managed read surfaces may include:

- operational surfaces
- review surfaces
- durable artifact stores
- derived stores
- isolated stores
- untrusted stores
- materialized read models
- audit views
- runtime snapshots
- host-composed views
- adapter-provided projections

Each surface must have declared capabilities before CQL can validate queries against it.

---

## 17.3 Store-First Rule

CQL queries store-backed or materialized surfaces by default.

This protects CQL from becoming an implicit computation engine.

Runtime or dynamic surfaces may be exposed only when they behave as managed read surfaces with deterministic query contracts.

---

## 17.4 Runtime Surfaces

Runtime may expose materialized operational views that behave as managed read surfaces.

Runtime surfaces must remain:

- read-only from the CQL perspective
- deterministic in query meaning
- capability-declared
- bounded by explicit scope
- adapter-validated

Runtime surfaces must not use CQL as a write or command path.

---

## 17.5 Domain Surface Mapping

Each domain must define its primary managed read surfaces.

This mapping is domain-owned and versioned outside the JSON Intermediate Language.

CQL may depend on the capability declaration.

CQL must not own the substrate’s surface implementation.

---

# 18. Adapter Capability Declaration Model

## 18.1 Purpose

Capability Declaration is the source of legality.

JSON IL is validated against capability declarations before execution.

Adapters expose host-owned read surfaces to CQL.

The adapter capability declaration model defines what a domain supports before execution.

A query must be validated against the selected adapter’s capabilities.

JSON IL does not decide whether a field, filter, view, output, dependency, or target is valid.

Capabilities decide that.

---

## 18.2 Capability Declaration

An adapter capability declaration should identify:

- domain name
- supported subjects
- supported views
- supported raw surfaces
- supported target kinds
- supported scope dimensions
- supported filters
- supported output modes
- supported extensions
- supported defaults
- supported ordering
- supported pagination
- supported projections
- unsupported features when useful for diagnostics

Capability declarations allow clients, SDKs, CLIs, and validators to determine what can be queried.

---

## 18.3 Defaults

Defaults are allowed only when declared by the adapter capability profile.

The JSON Intermediate Language should normalize defaults into explicit fields before execution.

Hidden defaults are forbidden.

A query should not depend on ambient host behavior to determine meaning.

---

## 18.4 Validation

Adapter validation must reject unsupported:

- domains
- subjects
- target kinds
- scope dimensions
- filters
- filter values
- output modes
- extension views
- subject arguments
- projections
- ordering fields
- pagination modes

Validation must happen before execution.

---
# 19. Determinism Rules

Given identical:

- domain
- subject
- target
- scope
- filters
- output
- context
- metadata when standardized as meaningful
- adapter capability declaration
- substrate state

the query must resolve to identical meaning.

CQL must not depend on:

- mutation timing
- storage iteration order
- hidden defaults
- ambient host state
- implicit joins
- parser guesses
- non-declared adapter behavior
- non-deterministic extension behavior

Determinism applies to query meaning.

Result values may differ when substrate state changes.

---

# 20. Non-Interpretation Rules

CQL must not:

- infer intent
- infer authority
- infer missing relationships
- infer missing data as false
- reinterpret unknown provenance
- synthesize joins not explicitly defined
- classify semantic meaning outside substrate-defined views
- convert observations into legitimacy
- convert signals into truth
- convert graph relationships into authority
- convert output mode into explanation semantics

CQL may expose data and views.

It must not become the interpreter of those views.

---

# 21. Extension Model

## 21.1 Principle

Extensions may add managed read surfaces while preserving the JSON Intermediate Language structure.

Extensions add queryable surfaces.

They do not add new language semantics.

---

## 21.2 Extension Naming

Extension views must be namespaced.

Extension names should make ownership visible.

Examples of extension ownership patterns include:

- x.cas.host_name.view_name
- x.cds.host_name.view_name
- x.runtime.host_name.view_name
- x.csg.host_name.view_name
- x.ccare.host_name.view_name

Namespacing prevents host-defined meaning from colliding with canonical CQL meaning.

---

## 21.3 Extension Constraints

Extensions must be:

- read-only
- deterministic
- capability-declared
- structurally compatible with JSON Intermediate Language
- non-mutating
- domain-attributed
- adapter-validatable

Extensions must not:

- modify grammar
- override canonical fields
- redefine substrate truth
- bypass capability validation
- add implicit joins
- introduce mutation
- silently expand scope

---

## 21.4 Extension Dispatch

Extension dispatch must preserve:

- extension namespace
- owning domain
- target typing
- scope boundaries
- filter constraints
- output mode
- provenance context

Extension results must remain distinguishable from canonical domain results when necessary.

---

# 22. Transport Neutrality

The JSON Intermediate Language is transport-neutral.

It may be used in:

- library calls
- CLI execution
- APIs
- FFI boundaries
- tests
- replay systems
- host integrations
- SDK builders

Its meaning must remain stable across all transports.

Transport bindings must not change query meaning.

A query sent through an API must mean the same thing as the same normalized query sent through an SDK call, assuming the same adapter capabilities and substrate state.

---

# 23. Relationship to DSL

A DSL may exist as a human-facing layer.

If present, it must compile into valid JSON Intermediate Language.

The DSL must not introduce semantics that JSON Intermediate Language cannot represent.

The DSL is syntax.

JSON Intermediate Language is semantics.

A DSL query must not execute directly.

The DSL must not:

- infer hidden meaning
- mutate state
- bypass validation
- introduce implicit joins
- add grammar extensions through host-specific views
- silently ignore unsupported features
- redefine output meaning
- redefine substrate semantics

DSL defaults must come from adapter capability declarations.

After compilation, defaults should appear explicitly in the normalized JSON Intermediate Language query.

---

# 24. Relationship to SDK Builders

SDK builders may construct JSON Intermediate Language queries programmatically.

SDK builders are authoring helpers.

They are not canonical query semantics.

An SDK builder must produce valid JSON Intermediate Language or a typed equivalent.

SDK builders must not bypass:

- validation
- target typing
- scope and filter separation
- output rules
- adapter capability checking
- extension namespace rules
- read-only constraints

SDK builders should make valid queries easy to construct and invalid queries difficult to construct.

---

# 25. Relationship to Results

JSON IL defines query structure.

Adapters return Adapter Outcomes.

CQL constructs and finalizes Result Envelopes.

Result payloads remain domain-owned.

Result Envelope preserves what happened overall.

---

## 25.1 Multi-Domain Results

When multiple domains are queried:

- results must preserve domain attribution
- results must not be implicitly merged
- ambiguity must remain visible
- each result section must remain traceable to its domain and adapter
- output mode must not erase domain boundaries
- conflicts between domains must not be automatically reconciled by CQL

CQL can expose multiple domain results.

It must not decide what cross-domain disagreement means unless a domain-defined managed read surface explicitly provides that behavior.

---

## 25.2 Result Payload Ownership

Result payloads are owned by the responding domain or adapter.

CQL may standardize the envelope.

CQL must not redefine the payload’s substrate-owned semantics.

For example:

- CAS owns the meaning of CAS semantic status and dynamics.
- CSG owns graph structure and topology payload meaning.
- CIS owns identity payload meaning.
- CCare owns observation payload meaning.
- Charter owns legitimacy payload meaning.

---

# 26. Validation Requirements

A JSON Intermediate Language validator must verify:

- the query is read-only
- the domain is known
- the subject is valid for the domain
- the target kind is supported by the subject
- the target shape is valid
- the scope fields are supported
- the scope values are valid
- the filters are supported
- the filter values are valid
- scope and filters are structurally separate
- the output mode is supported
- output options are valid
- subject arguments are valid
- extensions are namespaced
- unknown fields are rejected
- multi-domain behavior is explicit
- projection behavior is explicit when required
- defaults are declared before being applied
- adapter capabilities support the requested query
- registered domain
- registered view/raw surface
- requested or default latest version resolution
- supported target
- supported scope
- supported filter
- supported output mode
- selected fields
- field support for output mode
- raw surface registration
- boundedness requirements
- managed composed view legality
- unknown fields rejected

Validation must happen before adapter execution.

---

# 27. Adapter Dispatch Requirements

After validation, JSON Intermediate Language dispatch must preserve:

- domain attribution
- subject identity
- target typing
- scope boundaries
- filter constraints
- output intent
- context posture
- metadata used for tracing
- extension namespaces
- provenance context

Adapters must not receive raw DSL text as their execution input.

Adapters should receive normalized JSON Intermediate Language or a typed equivalent.

This prevents each adapter from inventing its own parser and ensures validation is enforced before substrate execution.

The Adapter Dispatch flow looks like:

validated JSON IL
→ execution context established
→ adapter invoked
→ adapter outcome returned
→ envelope finalized

---

# 28. Unknown Field Handling

Unknown fields must not be silently ignored.

A JSON Intermediate Language validator must reject:

- unknown top-level fields
- unknown subject fields
- unknown target kinds
- unknown target fields
- unknown scope fields
- unknown filters
- unknown output modes
- unknown argument names
- malformed extension fields

The only exception is a field inside a sanctioned extension namespace that is explicitly accepted by the relevant adapter capability declaration.

Silent ignoring is forbidden because it creates false confidence.

A query author must be able to trust that the executed query is the query they wrote.

---

# 29. Error Behavior

CQL validation and execution should produce explicit errors for invalid query behavior.

Error categories should include:

- unknown domain
- unknown subject
- unsupported view
- unsupported raw surface
- invalid target kind
- invalid target shape
- missing required target
- unsupported scope
- unsupported filter
- invalid filter value
- unsupported output mode
- unsupported projection
- unsupported extension view
- unknown field
- ambiguous query
- attempted mutation
- capability mismatch
- adapter unavailable
- non-deterministic adapter behavior

Error details should help the user or SDK identify what was invalid and what capabilities are available when safe to expose.

Errors must not be converted into guessed behavior.

---
# 30. Security and Authorization Boundary

CQL is a query contract, not an authorization system.

However, CQL must not bypass authorization or visibility rules enforced by the host, adapter, or substrate.

Authorization may affect which managed read surfaces are visible.

If authorization affects visibility, it must be enforced before or during adapter execution.

CQL must not treat authorization failures as empty truth unless the host explicitly chooses that behavior and records it appropriately.

A result should distinguish, where safe and appropriate, between:

- no matching data
- unsupported query
- unauthorized query
- unavailable adapter
- hidden data due to visibility rules

JSON IL must not collapse unauthorized, hidden, nonexistent, unsupported, unavailable, or empty.

Those distinctions are preserved through validation and Result Envelope.

---

# 31. Non-Goals

JSON Intermediate Language does not define:

- the CQL DSL syntax
- UI display behavior
- storage engine implementation
- graph traversal algorithms
- CAS derivation algorithms
- legitimacy calculation
- identity resolution rules
- observation semantics
- federation protocols
- mutation semantics
- workflow execution
- natural language understanding
- final result payload schemas
- host-specific transport protocols

JSON Intermediate Language is the query contract.

It is not the substrate model.

---

# 32. Implementation Guidance

A compliant implementation should follow this lifecycle:

1. Accept query input from DSL, SDK builder, API, CLI, or host integration.
2. Compile or normalize the input into JSON IL.
3. Validate the query shape.
4. Resolve capability declarations.
5. Resolve latest references where applicable.
6. Validate fields/output/scope/filter/target.
7. Establish Execution Context.
8. Dispatch the normalized query to the adapter.
9. Receive Adapter Outcome.
10. Finalize Result Envelope.

The implementation should keep parsing, validation, dispatch, and adapter execution separate.

---

# 33. Invariants Summary

The JSON Intermediate Language must preserve the following invariants:

- JSON Intermediate Language is canonical.
- CQL is read-only.
- CQL owns access, not meaning.
- All query components are explicit.
- Domains map to managed read surfaces.
- Subjects are interpreted inside domains.
- Targets are typed.
- Scope defines visibility.
- Filters constrain visible data.
- Filters do not expand scope.
- Output affects shape, not truth.
- Context is explicit and non-authoritative.
- Metadata does not affect meaning unless standardized.
- Ordering is explicit or domain-defined.
- Defaults are capability-declared.
- Unknown fields are rejected.
- Extensions do not alter grammar.
- Joins are not part of the V1 query language.
- Adapters receive normalized query context.
- Validation happens before execution.
- Results preserve domain attribution.
- CQL is non-interpreting.
- JSON IL defines what is asked, not what is legal.
- Capability Declaration defines query legality.
- Execution Context controls runtime execution.
- Adapter Outcome reports handler output.
- Result Envelope reports what happened.
- Domains are registered, not hard-coded.
- Unversioned references resolve to latest.
- Selected fields must be capability-declared.
- Field selection shapes output only.
- Managed composed views are queried as views, not joins.

---

# 34. Mental Model

The JSON Intermediate Language defines:

- what domain is queried
- what data or view is requested
- what object, collection, pair, or global surface is targeted
- what slice of data is visible
- what constraints apply inside that visible slice
- how the result should be shaped
- what context affects execution posture
- what metadata supports tracing and replay

It is the canonical machine form of a Charter query.

---

# 35. Final Principle

JSON IL is the canonical form of the question.

It preserves the explicit structure of what is being asked while leaving legality, execution control, handler output, and result truth to the appropriate CQL contracts.

This keeps CQL explicit, domain-neutral, capability-validated, execution-controlled, and envelope-trustworthy.