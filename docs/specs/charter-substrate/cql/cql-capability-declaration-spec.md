# CQL Capability Declaration Specification

Status: FOUNDATIONAL DRAFT  
Applies to: CQL Core, CQL JSON IL, CQL DSL, adapter contracts, capability discovery, validation, host-defined domains, Charter-defined domains, managed views, raw surfaces, and result envelope metadata  
Depends On: CQL Foundation, CQL Shared Invariants, CQL JSON IL Foundation, CQL DSL Foundation, Module Boundaries, Canonical Naming  
Does NOT define: Rust APIs, storage backends, handler implementation, transport protocols, execution performance model, or substrate internals  

---

# 1. Purpose

This document defines the conceptual model for CQL capability declarations.

A capability declaration describes what a CQL domain, view, raw surface, field, output mode, scope, filter, and target can support.

Capability declarations are the validation contract for CQL.

They answer:

- What domains are available?
- What views can be queried?
- What raw surfaces can be queried?
- What target kinds are supported?
- What scope fields are supported?
- What filters are supported?
- What output modes are supported?
- What fields are selectable?
- What defaults exist?
- What versions are active?
- What dependencies exist?
- What result conditions may be returned?
- What boundedness constraints apply?

Capability declarations make CQL discoverable, validatable, explainable, and safe.

Capability Declaration not only defines what may be queried.

It also defines:

- field dependency relationships
- output conformance rules
- attribution behavior
- partial result behavior
- dependency execution expectations
- boundedness expectations
- visibility expectations

Capability Declaration is the declarative contract that allows CQL to validate queries, constrain execution, derive partial result behavior, preserve attribution, and finalize trustworthy Result Envelopes.


---

# 2. Foundational Principle

A CQL query is valid only when it matches declared capabilities.

Capability declarations are not documentation only.

They are enforceable query contracts.

If a domain, view, raw surface, target, scope, filter, output mode, field, or argument is not declared as supported, CQL must reject the query before dispatch.

CQL must not guess.

CQL must not silently ignore unsupported query features.

CQL must not treat undeclared behavior as valid.

Capability Declaration is the declared execution contract of a domain or view.

Execution Context controls runtime execution posture.

Adapter Outcome preserves handler output.

Result Envelope preserves execution truth.

Capability Declaration defines what behaviors are valid and how fields, dependencies, outputs, and partial behavior relate.

---

# 3. Capability Declaration Identity

A capability declaration is a versioned description of queryable read surfaces.

It belongs to the owner of a domain or view.

A capability declaration may be supplied by:

- a Charter substrate
- a host-defined domain
- a host integration
- an adapter
- a runtime registry
- an extension package

The declaration tells CQL what can be queried.

The declaration does not transfer ownership of meaning to CQL.

The domain or view owner remains responsible for the meaning of what is exposed.

---

# 4. Capability Layering

Capabilities are layered.

The main layers are:

1. domain
2. raw surface
3. field
4. view
5. output mode
6. target
7. scope
8. filter
9. dependency
10. result condition
11. metadata

The domain declares the ownership boundary.

Raw surfaces and fields define the lower-level data contract.

Views define managed read surfaces.

Output modes define result shape.

Targets define what can be addressed.

Scope defines visibility.

Filters constrain visible data.

Dependencies describe managed composition.

Result conditions describe possible outcomes.

Metadata describes versioning, identity, stability, display information, and diagnostics.

---

# 5. Primary Unit of Capability

The primary unit of query capability is the declared view or raw surface.

A domain may declare broad defaults, but specific query support is determined by the selected view or raw surface.

A view or raw surface must declare:

- supported target kinds
- supported scope fields
- supported filters
- supported output modes
- supported fields
- boundedness rules
- version metadata
- result conditions
- dependencies when applicable

A domain exists as the ownership boundary.

A view or raw surface exists as the queryable contract.

---

# 6. Domain Capability

## 6.1 Purpose

A domain capability declares a named query ownership boundary.

Domains are adapter-declared.

CQL Core must not assume a fixed universe of domains.

Charter domains are registered domains in the Charter ecosystem, not hard-coded assumptions in CQL Core.

Host-defined domains are valid when registered and capability-declared.

---

## 6.2 Domain Requirements

A domain capability must declare:

- canonical domain name
- domain version
- owner
- available views
- available raw surfaces when exposed
- domain-level defaults when applicable
- metadata
- stability status
- supported capability version
- authorization/discovery posture where applicable

A domain capability may declare:

- display name
- description
- examples
- documentation reference
- deprecation metadata
- global boundedness policy
- default output mode
- default scope
- extension namespace support

---

## 6.3 Domain Ownership

A domain owns the meaning of its views and raw surfaces.

CQL validates and dispatches queries against the domain declaration.

CQL does not reinterpret the domain’s meaning.

Examples:

- CAS owns CAS posture meaning.
- CSG owns graph boundary meaning.
- CCare owns observation meaning.
- A runtime host owns runtime status meaning.
- An orders host owns order-status meaning.

CQL owns access rules.

Domains own meaning.

---

# 7. Host-Defined Domains

Host-defined domains are first-class.

A host may define a domain when the host owns the meaning of the exposed read surface.

A host-defined domain is appropriate when:

- the host owns the view meaning
- the host composes multiple data sources into a new meaning
- the view is not naturally owned by an existing substrate
- the result represents a host-level concern
- the host wants a stable query surface for its own API

Host-defined domains must follow all CQL rules:

- read-only
- deterministic
- capability-declared
- validated before execution
- non-mutating
- explicit about target, scope, filters, fields, and output
- compatible with JSON IL
- bounded
- versioned
- non-inferential

---

# 8. Extension Views vs Host-Defined Domains

CQL distinguishes host-defined domains from extension views.

## 8.1 Extension View

An extension view is attached to an existing domain’s conceptual area.

Use an extension view when the meaning remains conceptually attached to the existing domain.

Example:

domain cas  
view x.runtime.payment_posture_summary  
target area payments

This means a runtime host provides a runtime-specific view related to CAS meaning.

The conceptual domain remains CAS.

---

## 8.2 Host-Defined Domain

A host-defined domain owns its own meaning.

Use a host-defined domain when the host composes or defines a new read surface that is not naturally owned by an existing domain.

Example:

domain runtime  
view payment_area_status  
target area payments

This means runtime owns the meaning of payment_area_status.

The result is not merely a CAS extension.

It is a runtime-owned read surface.

---

# 9. Version Resolution

Versioning is mandatory.

Domains, views, raw surfaces, fields, output modes, capability declarations, result envelopes, adapters, and dependency contracts may change.

Therefore, capability declarations must carry version metadata.

---

## 9.1 Unversioned References

If a query does not specify a version, the reference resolves to the latest declared compatible version.

Example:

view area_status

resolves to the latest registered version of area_status.

The caller does not need to write latest explicitly.

---

## 9.2 Explicit Version References

A query may specify an explicit version.

Example:

view area_status:v1

An explicit version must resolve to that declared version or fail validation.

Explicit versions are preferred for stable clients and compatibility-sensitive integrations.

---

## 9.3 Latest Resolution Metadata

When an unversioned reference resolves to latest, the result envelope must report the resolved version.

A query may ask for:

view area_status

But the result metadata should identify:

resolved view area_status:v3

This preserves usability without hiding version reality.

---

## 9.4 Latest as Alias

A host may expose latest as an alias.

However, latest does not need to be written by callers.

The absence of a version means latest.

Latest is a moving reference and should be treated as less stable than an explicit version.

---

# 10. View Capability

## 10.1 Purpose

A view capability declares a managed read surface.

A view is the primary public query surface in CQL.

A view may expose:

- raw records
- summaries
- derived state
- materialized projections
- operational snapshots
- audit surfaces
- graph views
- semantic views
- host-composed views

A view must be declared before it can be queried.

---

## 10.2 View Requirements

A view capability must declare:

- canonical view name
- view version
- owning domain
- supported target kinds
- supported scope fields
- supported filters
- supported output modes
- supported fields per output mode
- boundedness rules
- result conditions
- stability status
- metadata

A view capability may declare:

- display name
- description
- examples
- documentation reference
- deprecation metadata
- dependencies
- default output mode
- default scope
- default ordering
- pagination support
- authorization posture
- discovery posture

---

## 10.3 View Meaning

The view owner defines what the view means.

CQL does not reinterpret view meaning.

A view may combine fields, shape output, define result conditions, and declare supported query behavior.

A view must remain read-only from the CQL perspective.

---

## 10.4 View Stability

A view should declare stability status.

Recommended stability statuses include:

- experimental
- stable
- deprecated
- internal
- hidden

Stability does not change query meaning.

Stability helps tooling, generated clients, documentation, CLI help, warnings, and migration planning.

---

# 11. Raw Surface Capability

## 11.1 Purpose

A raw surface is a lower-level managed read surface.

Raw does not mean unrestricted storage access.

Raw does not mean bypassing CQL.

Raw surfaces must still be registered, typed, scoped, filtered where applicable, output-shaped, authorization-aware, versioned, and read-only.

---

## 11.2 Raw Surface Requirements

A raw surface capability must declare:

- canonical raw surface name
- raw surface version
- owning domain
- typed fields
- supported target kinds
- supported scope fields
- supported filters
- supported output modes
- boundedness rules
- authorization posture
- stability status
- metadata

Raw surfaces may declare:

- display name
- description
- examples
- documentation reference
- deprecation metadata
- field dependencies
- ordering
- pagination
- result conditions

---

## 11.3 Raw Surface Guardrail

Raw surfaces must not become escape hatches.

A raw surface must not:

- expose arbitrary storage internals by default
- bypass authorization
- bypass scope
- bypass filters
- bypass output shaping
- bypass field declaration
- bypass versioning
- bypass boundedness
- perform mutation

Raw access is still managed access.

---
## 11.4 Raw Surface Dependencies 

Raw surfaces may participate in field dependency declarations.

Raw surfaces must remain:

- registered
- typed
- capability-declared
- bounded

Capability declarations may define raw-surface dependency behavior including:

- attribution requirements
- boundedness requirements
- visibility policy
- dependency failure behavior

---

# 12. Field Capability

## 12.1 Purpose

Fields are declared capability units.

A field must be declared before it can be selected, returned, filtered, documented, or used as part of a view contract.

No field should appear accidentally.

No field should be selectable unless the selected output mode supports it.

---

## 12.2 Field Layers

Fields may be declared at different layers:

- domain field registry
- raw surface field registry
- derived or calculated field registry
- view-local field contract

Views should prefer publishing already-declared domain or raw fields when the same information is shared across multiple views.

Views may define view-local fields when the field is genuinely specific to that view.

Calculated fields must be declared as derived fields or view-local fields with clear ownership.

---

## 12.3 Field Requirements

A field capability must declare:

- canonical field name
- field type
- owner
- version
- availability
- supported output modes
- stability status
- metadata

A field capability may declare:

- display name
- description
- examples
- documentation reference
- deprecation metadata
- origin
- dependencies
- nullability
- cardinality
- units
- enum values
- formatting hints
- authorization posture
- filterability
- sortability

---

## 12.4 Field Origin

A field should declare its origin.

Recommended field origins include:

- raw
- derived
- calculated
- composed
- view_local
- external
- metadata

Origin helps callers understand whether a field is directly read, derived, computed, composed, or local to a view.

---

## 12.5 Calculated Fields

Calculated fields must be declared.

A calculated field should identify:

- canonical name
- type
- owner
- calculation ownership
- dependencies when known
- version
- supported output modes
- stability
- description

A calculated field must not appear as hidden behavior.

If a view exposes a calculated field, the field must be part of the view’s capability contract or the domain’s derived field registry.

---

## 12.6 View-Local Fields

A view-local field is a field defined only for a specific view.

View-local fields are allowed when the field is genuinely specific to the view.

Examples may include:

- summary_text
- display_label
- status_explanation_hint
- compact_status

A view-local field must still be:

- declared
- typed
- versioned
- owned
- output-mode supported
- capability-discoverable when visible
- rejected when unsupported

View-local fields must not become undeclared arbitrary payload keys.

---

# 13. Output Mode Capability

## 13.1 Purpose

An output mode declares the shape and detail level a view can return.

The domain or view owner defines what output modes are supported.

CQL may recognize common output mode names, but the view declares whether they are available.

---

## 13.2 Output Mode Rules

Detailed output is the default.

Summary output is optional.

A view must not publish summary unless it can support summary honestly.

Summary may later support AI-generated summaries when the domain or view owner explicitly supports that behavior.

CQL itself does not perform explanation or summarization by default.

---

## 13.3 Output Mode Requirements

An output mode capability must declare:

- mode name
- owning view or raw surface
- supported fields
- default field set
- whether field selection is allowed
- metadata inclusion behavior
- provenance inclusion behavior
- warnings inclusion behavior
- result condition support
- version metadata

An output mode may declare:

- display name
- description
- examples
- maximum field count
- default limit
- pagination behavior
- ordering behavior
- stability status
- deprecation metadata

Output modes may affect:

- field dependency activation
- attribution behavior
- partial behavior
- dependency requirements
- visibility behavior

Capability declarations may define different dependency behavior for:

- detailed
- structured
- summary
- raw

Summary outputs may support reduced attribution or reduced dependency detail when explicitly declared.


---

## 13.4 Field Support Per Output Mode

Fields are declared per output mode.

A field available in detailed output is not automatically available in summary output.

A field available in summary output is not automatically available in detailed output unless declared.

A requested field must be supported by:

- the selected view or raw surface
- the selected output mode
- the selected target/scope/filter combination when relevant

Unsupported fields must be rejected.

---

# 14. Field Selection

## 14.1 Purpose

Field selection lets callers request only the fields they need.

Field selection is output shaping only.

It does not change query meaning.

---

## 14.2 V1 Field Selection Rule

CQL V1 supports modest flat field selection.

Allowed:

- selecting declared fields from the selected output mode

Avoid in V1:

- nested selection trees
- aliases
- fragments
- directives
- recursive selection
- cross-domain traversal
- arbitrary computed fields
- client-defined resolver chains

---

## 14.3 Field Selection Constraints

Field selection must not:

- alter query meaning
- widen scope
- imply joins
- trigger hidden computation
- bypass the declared output mode
- bypass authorization
- return undeclared fields
- cause the domain to invent unavailable data

Field selection changes what is returned, not what is true.

---
## 14.4 Field Selection Dependencies

Field selection interacts with field dependency declarations.

Selected fields may activate dependency requirements declared by the capability contract.

Capability declarations may define:

- field dependency surfaces
- field-specific boundedness
- field-specific attribution
- field-specific partial behavior
- field-specific visibility behavior

Field selection must not silently bypass dependency declarations.

---

# 15. Target Capability

## 15.1 Purpose

Target capabilities declare what kinds of objects, collections, pairs, or global surfaces a view can address.

A target kind must be declared before it can be queried.

---

## 15.2 Target Requirements

A target capability should declare:

- target kind
- required target fields
- optional target fields
- allowed cardinality
- identifier requirements
- supported scope combinations
- supported filter combinations
- boundedness constraints

Examples of target kinds may include:

- area
- resolution
- identity
- item
- session
- commit
- graph
- collection
- pair
- global

Target kinds are not globally assumed.

They are valid when declared by the selected domain or view.

---

# 16. Scope Capability

## 16.1 Purpose

Scope capabilities declare what visibility boundaries a view supports.

Scope defines what data is visible before filters are applied.

---

## 16.2 Scope Requirements

A scope capability should declare:

- scope field name
- supported values
- default value when applicable
- whether the scope is required
- compatibility with target kinds
- compatibility with filters
- boundedness implications
- version metadata

Examples of scope fields may include:

- active
- historical
- since
- until
- window
- projection
- current_round
- include_superseded
- trust_boundary

Scope must remain separate from filters.

Filters must not expand scope.

---

# 17. Filter Capability

## 17.1 Purpose

Filter capabilities declare what constraints may be applied to visible data.

Filters operate inside the selected scope.

---

## 17.2 Filter Requirements

A filter capability should declare:

- filter name
- supported value types
- supported operators when applicable
- allowed values when finite
- compatibility with target kinds
- compatibility with scopes
- compatibility with output modes when relevant
- whether the filter is indexed or efficient when useful
- version metadata

Examples of filter fields may include:

- state
- status
- confidence
- volatility
- provenance
- relationship_type
- semantic_state
- participant
- identity
- severity

Unsupported filters must be rejected.

Unsupported filter values must be rejected.

---

# 18. Defaults

## 18.1 Principle

Defaults must be declared.

The parser, builder, or engine must not invent defaults.

Defaults must be normalized into explicit JSON IL before execution.

---

## 18.2 Default Precedence

Default precedence should be conceptualized as:

1. explicit query value
2. output-mode default when applicable
3. view default
4. domain default
5. no default

The closest applicable declaration wins.

---

## 18.3 Default Requirements

A default declaration should identify:

- field being defaulted
- default value
- owning declaration
- version
- whether the default is stable
- whether the default is visible in normalized query metadata

Hidden defaults are forbidden.

---

# 19. Result Conditions

## 19.1 Purpose

Capability declarations should identify possible result conditions.

Result conditions help callers understand what may happen when a query executes.

They also support result envelope design.

---

## 19.2 Standard Result Conditions

CQL should distinguish:

- ok
- empty
- nonexistent
- unauthorized
- hidden
- unsupported
- invalid
- unavailable
- partial
- error

These conditions must not be collapsed into each other.

Empty data is not the same as nonexistent.

Unauthorized is not the same as nonexistent.

Hidden is not the same as empty.

Unsupported is not the same as no matching data.

Unavailable is not the same as false.

---

## 19.3 Hidden Means Encapsulated

Hidden means encapsulated.

It should not break trust.

A hidden result indicates that the queried surface exists behind a boundary that intentionally does not expose details.

Hidden must not silently imply nonexistence.

---

# 20. Capability Discovery

## 20.1 Purpose

Capability discovery allows callers to inspect what can be queried.

Discovery is required for CQL to function well as an SDK.

Discovery supports:

- validation
- CLI help
- generated documentation
- IDE tooling
- user guidance
- testing
- client generation
- debugging
- host usability

---

## 20.2 Discovery Content

Discovery should be able to expose:

- domains
- views
- raw surfaces
- target kinds
- scope fields
- filters
- output modes
- selectable fields
- defaults
- versions
- dependencies
- extensions
- display names
- descriptions
- examples
- deprecation metadata
- stability status
- result conditions
- boundedness rules

---

## 20.3 Filtered Discovery

Discovery itself may be filtered by authorization or encapsulation.

When safe and appropriate, discovery should distinguish:

- not registered
- not authorized
- hidden
- unavailable

Discovery must not lie by presenting hidden or unauthorized capabilities as nonexistent unless the host intentionally chooses that security posture.

When that posture is chosen, it should be understood as a host security policy, not a CQL truth claim.

---
## 20.4 Discovery Dependency and Attribution

Capability discovery should expose dependency and attribution metadata when supported.

Discovery may expose:

- field dependencies
- required dependencies
- optional dependencies
- attribution levels
- partial-result policies
- safe-if-missing behavior
- dependency versions
- dependency boundedness policies

This supports:

- IDE tooling
- diagnostics
- replay analysis
- explainability layers
- managed composition debugging
- generated documentation

---

# 21. Managed Composition Capability

## 21.1 Purpose

A managed composed view is a declared view whose implementation internally queries one or more other CQL domains, views, or raw surfaces.

Managed composition is allowed.

Public arbitrary joins remain out of V1.

---

## 21.2 Composition Requirements

A composed view must:

- be declared as a view
- own its returned meaning
- remain read-only
- preserve or narrow caller scope unless explicitly declared otherwise
- respect authorization
- avoid unbounded fan-out
- avoid infinite recursion
- report partial failure
- preserve source-domain attribution where relevant
- be traceable

Managed composed views should preserve declared dependency relationships.

Capability declarations may define:

- dependency surfaces
- dependency versions
- dependency requirements
- dependency boundedness
- dependency attribution
- dependency partial behavior

Execution Context records dependency execution outcomes.

Result Envelope finalization combines:

- capability declarations
- dependency outcomes
- selected fields
- adapter outcomes

to produce explicit partial-result reporting and attribution reporting.

---

## 21.3 Dependency Declaration

A composed view should declare dependencies.

Known dependencies should be statically declared.

Dynamic dependencies may be reported through trace or diagnostics.

Dependency declarations may include:

- source domain
- source view or raw surface
- required version or version policy
- required output mode
- required fields
- required scope propagation
- failure policy
- whether dependency is required or optional

Dependency declarations support:

- introspection
- debugging
- validation
- trace output
- cycle detection
- execution planning
- partial failure reporting

---

## 21.4 Missing Dependency Behavior

If a composed view depends on another CQL interface that is missing, unavailable, unauthorized, or incompatible, the composed view must return an appropriate explicit condition or error.

A missing dependency must not be collapsed into empty data.

---

# 22. Boundedness Capability

## 22.1 Purpose

Boundedness prevents accidental unbounded reads.

Every view and raw surface should declare its boundedness behavior.

---

## 22.2 Boundedness Declarations

A boundedness declaration may include:

- requires_target
- supports_global
- requires_scope
- default_limit
- max_limit
- supports_pagination
- max_window
- max_page_size
- max_collection_size
- max_dependency_depth
- max_fanout
- timeout_policy
- execution_budget_policy

A query should be bounded by at least one of:

- target
- scope
- limit
- capability policy
- declared global support
- pagination
- execution budget
- authorization boundary

Global queries are allowed only when explicitly declared.

Capability declarations may define boundedness rules for:

- dependency calls
- dependency depth
- dependency fanout
- field activation
- output modes
- composed views
- raw surface access

Execution Context enforces boundedness during execution.

---

# 23. Authorization and Visibility Posture

## 23.1 Purpose

CQL is not an authorization system, but capability declarations must allow authorization and visibility posture to be represented.

Authorization affects whether a caller may query a surface.

Visibility affects whether a caller may discover or inspect a surface.

Encapsulation affects whether details are intentionally hidden.

---

## 23.2 Required Distinctions

CQL must distinguish, when safe and appropriate:

- nonexistent
- unauthorized
- hidden
- unsupported
- unavailable

These conditions must not be collapsed by default.

Host security policy may intentionally hide details, but that is a host posture, not a CQL semantic claim.

Capability declarations may define visibility posture for:

- fields
- dependencies
- attribution
- diagnostics
- trace visibility
- dependency reporting

Visibility declarations must preserve explicit distinctions between:

- visible
- unauthorized
- hidden
- unavailable
- unsupported

---

# 24. Metadata

## 24.1 Purpose

Capability metadata supports stable query identity, human discovery, documentation, versioning, and diagnostics.

---

## 24.2 Metadata Fields

Capability metadata should distinguish:

- canonical name
- display name
- description
- examples
- version
- stability status
- deprecation message
- documentation reference
- owner
- declared dependencies
- capability version
- schema version
- adapter version
- created timestamp when relevant
- modified timestamp when relevant

Canonical names are for queries.

Display names are for humans.

Descriptions are for discovery and documentation.

Capability declarations may define dependency-version requirements.

Examples include:

- required dependency versions
- compatible dependency ranges
- dependency capability versions
- output compatibility versions

Execution Context preserves resolved dependency versions for Result Envelope reporting.


---

# 25. Error Philosophy

CQL errors are diagnostic.

They describe what happened in the query contract or execution path.

They do not redefine domain truth.

Examples:

Unsupported field does not mean the field does not exist internally.

It means the field is not exposed by the declared capability contract.

Unauthorized does not mean nonexistent.

Dependency unavailable does not mean no data.

Capability mismatch does not mean domain falsehood.

CQL must preserve these distinctions.

---

# 26. Explanation Boundary

Capability declarations may expose explanation-ready fields or metadata.

CQL does not perform explanation.

CQL may expose:

- fields
- summaries when domain-supported
- metadata
- provenance
- diagnostics
- trace information
- warnings
- source-domain attribution

CQL does not:

- narrate
- deliberate
- advise
- reconcile
- interpret
- summarize unless the domain or view explicitly owns that output mode
- convert data into meaning beyond the domain contract

Explanation belongs to higher layers such as CGL, CDS, UI, CLI presentation, assistant tooling, or host-specific explanation systems.

---

# 27. Query Portability

CQL syntax is portable.

CQL meaning is contract-based.

A query is portable only when the destination host registers compatible domain, view, target, scope, filter, output, field, and dependency contracts.

If a required domain, view, dependency, or CQL interface is missing, the correct behavior is an explicit error or condition.

CQL must not pretend that a missing dependency is an empty result.

---

# 28. Query Identity and Replay

Capability declarations support query identity and replay by providing the contract context needed to understand a query.

A replayable query should be able to identify:

- normalized JSON IL
- query hash
- DSL source hash when applicable
- domain version
- view version
- capability version
- adapter version
- selected output mode
- selected fields
- execution context metadata
- execution timestamp
- resolved latest references

This supports reproducibility, logging, auditing, debugging, and diagnostics.

CQL does not need to be a persistence system to preserve replay metadata.

---
# 29. Field Dependency Declaration

## 29.1 Purpose

Capability declarations may define field dependency relationships.

Field dependency declarations allow CQL to understand:

- which fields depend on which dependency surfaces
- which dependencies are required
- which dependencies are optional
- what happens when dependencies fail
- whether partial results remain safe
- how attribution should be preserved

This enables trustworthy partial-result handling and managed composition.

---

## 29.2 Field Dependency Concepts

Field dependency declarations may conceptually define:

- dependency
- requiredness
- failure behavior
- safe-if-missing behavior
- attribution level

---

### Dependency

Dependency identifies the dependency surface required to produce a field.

Examples:

- cas.posture
- ccare.recent_observations
- csg.boundary
- runtime.payment_state

Dependencies may reference:

- domains
- views
- raw surfaces
- typed dependency contracts

---

## Requiredness

Requiredness defines whether a dependency is:

- required
- optional
- conditional

Required dependencies normally affect result completeness.

Optional dependencies may allow degraded output.

---

## Failure Behavior

Failure behavior defines what occurs when a dependency fails, becomes unavailable, hidden, unauthorized, or unsupported.

Conceptual examples include:

- fail_result
- mark_partial
- omit_field
- return_hidden
- return_unavailable
- use_declared_default

Failure behavior must remain explicit.

Unavailable data must not silently become empty data.

---

## Safe-If-Missing Behavior

Capability declarations may define whether a result remains safe to use when a dependency fails.

Conceptual examples include:

- safe
- degraded
- diagnostic_only
- unsafe

This metadata supports Result Envelope partial reporting.

---

## Attribution Level

Capability declarations may define attribution requirements.

Attribution levels may include:

- result
- dependency
- field
- hidden

Attribution defines how much dependency ownership and provenance should remain visible during execution reporting.

---
# 30. Attribution Declaration

## Purpose

Capability declarations may define attribution behavior for views and fields.

Attribution preserves dependency ownership and execution provenance.

This is especially important for:

- managed composition
- partial results
- dependency failures
- traceability
- diagnostics
- replay analysis

---

## Attribution Levels

Capability declarations may support:

### Result-Level Attribution

The result preserves the owning domain/view attribution.

### Dependency-Level Attribution

The result preserves dependency source attribution.

### Field-Level Attribution

Specific fields preserve dependency attribution relationships.

---

## Attribution Rule

Capability declarations should preserve enough attribution information for Result Envelope finalization to honestly report:

- where data originated
- which dependencies participated
- which dependencies failed
- which fields became partial or degraded

---
# 31. Partial Result Declaration

## Purpose

Capability declarations may define partial-result behavior.

This allows CQL to derive trustworthy partial-result reporting automatically.

---

## Partial Declaration May Define

Capability declarations may define:

- required dependencies
- optional dependencies
- degraded behavior
- safe-if-missing behavior
- omission behavior
- fallback behavior
- partial visibility policy
- dependency attribution policy

---

## Partial Rule

Capability declarations must not allow dependency failure to silently appear as complete success.

Partial behavior must remain explicit.

---

# 32. Validation Requirements

A CQL validator must use capability declarations to reject unsupported behavior before dispatch.

Validation must reject:

- unknown domain
- unknown view
- unknown raw surface
- unsupported target kind
- invalid target shape
- unsupported scope field
- invalid scope value
- unsupported filter
- invalid filter value
- unsupported output mode
- unsupported field
- unsupported field for selected output mode
- unsupported dependency
- unsupported version
- unsupported ordering
- unsupported pagination
- unbounded query when boundedness is required
- unknown extension
- malformed capability reference

Validation must not silently ignore unsupported features.

Validation should verify:

- selected fields are capability-declared
- dependency declarations are valid
- dependency references resolve
- dependency boundedness is legal
- attribution rules are legal
- field dependency rules are compatible with output modes
- field selection does not bypass capability contracts

Validation does not execute dependency calls.

Execution Context controls runtime dependency execution.

---
# 33. Execution Context Integration 

Execution Context uses capability declarations to determine:

- dependency-call legality
- dependency boundedness
- scope propagation rules
- attribution behavior
- partial-result behavior
- visibility posture
- dependency version rules
- output conformance rules

Capability declarations define what execution behavior is allowed.

Execution Context enforces it during runtime execution.

---
# 34. Result Envelope Integration 

Result Envelope finalization uses capability declarations to derive:

- attribution reporting
- partial-result reporting
- dependency visibility
- selected field conformance
- output conformance
- dependency failure reporting
- safe-if-missing reporting

Capability declarations provide the declarative trust rules used during envelope finalization.

---

# 35. Capability Declaration Invariants

## CAP-INV-01 — Capabilities Are Contracts

Capability declarations are enforceable validation contracts, not documentation only.

---

## CAP-INV-02 — Domains Are Declared

A domain is queryable only when registered and capability-declared.

CQL Core must not assume a fixed domain universe.

---

## CAP-INV-03 — Views and Raw Surfaces Are Primary Query Units

The selected view or raw surface determines most query capabilities.

Domain-level declarations provide ownership and defaults but do not replace view/raw-surface capability contracts.

---

## CAP-INV-04 — Fields Are Declared Capability Units

A field must be declared before it can be selected, returned, filtered, or documented as part of a capability contract.

---

## CAP-INV-05 — Output Modes Own Field Availability

A field must be supported by the selected output mode before it can be returned or selected.

---

## CAP-INV-06 — Detailed Is the Default Output Mode

Detailed output is the default unless a closer declared default overrides it.

Summary is optional and must not be published unless honestly supported.

---

## CAP-INV-07 — Unversioned Means Latest

If no version is specified, the reference resolves to latest.

The resolved version must be reported in metadata.

---

## CAP-INV-08 — Unknown Means Rejected

Unknown domains, views, raw surfaces, fields, filters, scopes, outputs, and versions must be rejected unless explicitly handled by a declared extension capability.

---

## CAP-INV-09 — Hidden Is Not Nonexistent

Hidden means encapsulated.

Hidden must not be silently collapsed into nonexistent.

---

## CAP-INV-10 — Missing Dependency Is Not Empty Data

A missing, unavailable, unauthorized, or incompatible dependency must produce an explicit condition or error.

It must not be treated as empty data.

---

## CAP-INV-11 — Composition Is Managed

Managed composed views are allowed.

Public arbitrary joins remain out of V1.

---

## CAP-INV-12 — Composition Must Be Traceable

Composed views must support dependency visibility through declarations, metadata, trace, diagnostics, or result envelope reporting.

---

## CAP-INV-13 — Raw Does Not Mean Unrestricted

Raw surfaces must be registered, typed, bounded, authorized, versioned, and read-only.

---

## CAP-INV-14 — Capability Discovery Is Required

A CQL host must support capability discovery for registered query surfaces, subject to authorization and encapsulation rules.

---

## CAP-INV-15 — Capability Metadata Separates Canonical and Display Names

Canonical names are used for query identity.

Display names and descriptions are used for humans.

---

## CAP-INV-16 — Boundedness Is Declared

Views and raw surfaces must declare boundedness behavior sufficient to prevent accidental unbounded reads.

---

## CAP-INV-17 — CQL Does Not Explain

Capabilities may expose explanation-ready data.

CQL does not become the explanation layer.

---

## CAP-INV-18 — Portability Is Contract-Based

A query is portable only across hosts that expose compatible capability contracts.

---
## CAP-INV-19 — Field Dependencies Must Be Declarable

Capability declarations may define field dependency relationships.

Dependency relationships must remain explicit.

---

## CAP-INV-20 — Dependency Failure Must Remain Explicit

Capability declarations must not allow dependency failure to silently appear as successful complete data.

---

## CAP-INV-21 — Attribution Must Be Preservable

Capability declarations must support attribution behavior sufficient for trustworthy partial-result and dependency reporting.

---

## CAP-INV-22 — Field Selection Must Respect Dependencies

Field selection must not bypass field dependency declarations or dependency boundedness rules.

---

## CAP-INV-23 — Capability Declarations Define Execution Legality

Capability declarations define what dependency behavior, attribution behavior, partial behavior, and boundedness behavior are legal.

Execution Context enforces those rules during execution.

---

## CAP-INV-24 — Capability Declarations Support Envelope Finalization

Capability declarations must preserve enough metadata for Result Envelope finalization to derive:

- attribution
- partial behavior
- dependency reporting
- selected field conformance
- output conformance

---

# 36. Non-Goals

This specification does not define:

- Rust trait names
- Rust structs
- storage schemas
- parser implementation
- network protocol
- authorization implementation
- result envelope schema
- execution context implementation
- query planning algorithms
- performance model
- GraphQL compatibility
- SQL compatibility
- arbitrary joins
- mutation behavior
- subscription behavior

This specification defines the conceptual contract for what can be queried and how that queryability is declared.

---

# 37. Final Principle

Capability declarations are the trust surface of CQL.

They make query behavior explicit before execution.

They allow hosts to expose read-only query surfaces without surrendering domain ownership.

They allow clients to discover what can be queried.

They allow CQL to validate before dispatch.

They preserve the difference between unsupported, unauthorized, hidden, nonexistent, unavailable, partial, empty, and error.

They keep CQL domain-neutral while making Charter and non-Charter hosts queryable through one consistent foundation.

Capability Declaration is not merely a query-validation contract.

It is the declarative execution contract that allows CQL to preserve:

- bounded execution
- dependency legality
- attribution
- partial-result trust
- output conformance
- visibility boundaries
- managed composition safety
- Result Envelope honesty

Capability Declaration defines what execution behaviors are legal.

Execution Context enforces those behaviors.

Adapter Outcomes preserve handler output.

Result Envelope finalization preserves execution truth.
