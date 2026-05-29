# CQL Execution Context Specification

Status: FOUNDATIONAL DRAFT  
Applies to: CQL execution, adapter dispatch, managed composed views, dependency calls, authorization posture, tracing, boundedness, partial failure handling, and envelope finalization  
Depends On: CQL Foundation, CQL Capability Declaration Specification, CQL Result Envelope Specification, Shared Invariants, JSON IL Foundation, DSL Foundation  
Does NOT define: Rust APIs, adapter outcome schema, storage engines, authorization implementation, tracing implementation, or transport protocols  

---

# 1. Purpose

This document defines the conceptual model for CQL Execution Context.

Execution Context answers:

Under what controlled conditions did this query execute?

Execution Context carries the runtime conditions needed to safely execute a validated CQL query.

It is especially important for managed composed views because composed views may internally query other CQL domains, views, or raw surfaces.

Execution Context preserves:

- caller context
- authorization context
- visibility context
- trace context
- recursion depth
- execution budget
- scope propagation
- dependency call policy
- partial failure policy
- boundedness controls
- version and capability resolution context
- envelope construction context

Execution Context exists to prevent execution behavior from becoming implicit host behavior.

---

# 2. Foundational Principle

Execution Context is the controlled runtime boundary of CQL execution.

It does not define the query.

It does not define the result.

It does not define domain meaning.

It defines the controlled conditions under which the query executes.

CQL uses Execution Context to preserve:

- read-only behavior
- validation boundaries
- authorization posture
- scope boundaries
- dependency-call safety
- boundedness
- traceability
- envelope finalization support

---

# 3. Execution Pipeline Position

Execution Context is established after query validation and before adapter execution.

The conceptual execution flow is:

1. Query is authored through JSON IL, DSL, or query builder.
2. Query is normalized into canonical JSON IL or typed equivalent.
3. Query is validated against capability declarations.
4. Execution Context is established.
5. Adapter is invoked with controlled context access.
6. Adapter may return an Adapter Outcome.
7. Adapter may perform managed dependency calls through Execution Context when allowed.
8. CQL constructs and finalizes the Result Envelope.

The conceptual pipeline is:

Capability Declaration  
Query Validation  
Execution Context  
Adapter Outcome  
Envelope Builder  
Envelope Finalizer  
Result Envelope

---

# 4. Context Ownership

Execution Context is CQL-owned.

Adapters may receive controlled access to Execution Context.

Adapters must not redefine Execution Context.

Adapters must not use Execution Context to bypass:

- validation
- authorization
- scope
- filters
- output mode
- selected fields
- capability declarations
- boundedness rules
- envelope finalization

Execution Context guides and constrains execution.

It is not an escape hatch.

---

# 5. Execution Context vs Query

Execution Context must not silently change query meaning.

The query defines:

- domain
- subject
- target
- scope
- filters
- output
- selected fields
- context fields explicitly present in JSON IL

Execution Context defines:

- caller posture
- authorization posture
- dependency-call conditions
- trace posture
- execution budget
- recursion limits
- partial failure policy
- boundedness enforcement
- envelope construction support

Execution Context may enforce or constrain execution.

It must not secretly rewrite the query.

---

# 6. Caller Context

## 6.1 Purpose

Caller Context identifies the caller posture under which the query executes.

Caller Context may affect authorization, visibility, tracing, and diagnostics.

---

## 6.2 Caller Context May Include

Caller Context may include:

- caller identity reference
- caller role
- caller tenant
- caller organization
- caller boundary
- request origin
- service identity
- delegation posture
- correlation identifier

Caller Context must not become substrate truth.

It is execution posture.

---

## 6.3 Caller Context Rule

Caller Context may affect what the caller is allowed to see.

Caller Context must not alter domain meaning.

---

# 7. Authorization Context

## 7.1 Purpose

Authorization Context carries the permission posture for execution.

CQL is not the authorization system, but Execution Context must preserve authorization boundaries.

---

## 7.2 Authorization Context May Include

Authorization Context may include:

- authorization decision references
- permission scope
- visibility scope
- delegation rules
- service authority posture
- caller authority boundary
- policy evaluation reference

---

## 7.3 Authorization Rule

Dependency calls must preserve caller authorization posture unless an explicitly declared delegation or service-authority model applies.

Authorization failures must not be collapsed into nonexistent or empty results.

---

# 8. Visibility Context

## 8.1 Purpose

Visibility Context controls what execution detail may be exposed.

Visibility affects:

- result data
- diagnostics
- warnings
- trace detail
- dependency visibility
- hidden/encapsulated details

---

## 8.2 Visibility Conditions

Execution Context must preserve distinctions between:

- visible
- unauthorized
- hidden
- encapsulated
- unavailable
- unsupported

Hidden means encapsulated.

Hidden must not silently imply nonexistent.

---

# 9. Trace Context

## 9.1 Purpose

Trace Context controls how execution is traced.

Trace supports:

- debugging
- diagnostics
- auditing
- replay analysis
- dependency visibility
- composition visibility

---

## 9.2 Trace Context May Include

Trace Context may include:

- trace id
- correlation id
- trace level
- trace visibility
- trace sampling posture
- trace retention posture
- trace redaction rules

---

## 9.3 Trace Rule

Trace must not leak hidden or unauthorized details.

If trace exists but is hidden, the Result Envelope should report that trace is unavailable or hidden according to policy.

---

# 10. Execution Budget

## 10.1 Purpose

Execution Budget prevents unbounded execution.

Execution Context must carry boundedness controls for query execution.

---

## 10.2 Budget Controls

Execution Budget may include:

- timeout
- max dependency calls
- max recursion depth
- max fanout
- max result size
- max page size
- max collection size
- max execution cost
- max trace detail
- max composed-view depth

---

## 10.3 Budget Rule

Execution must respect declared budgets.

If execution exceeds budget, the Result Envelope must report that explicitly.

Budget exhaustion must not silently become empty data.

---

# 11. Recursion and Cycle Control

## 11.1 Purpose

Managed composition can create recursive dependency paths.

Execution Context must prevent infinite recursion and unsafe cycles.

---

## 11.2 Recursion Controls

Execution Context should support:

- current depth
- maximum depth
- dependency path
- visited dependency set
- cycle detection
- recursion rejection

---

## 11.3 Cycle Rule

If a cycle is detected, execution must stop or reject according to policy.

Cycle failure must be explicit.

It must not be hidden as partial success unless the dependency was optional and policy permits degraded output.

---

# 12. Dependency Call Policy

## 12.1 Purpose

Managed composed views may internally query other CQL domains, views, or raw surfaces.

Dependency calls must go through controlled CQL execution.

---

## 12.2 Dependency Call Rule

Adapters must not bypass CQL validation when querying dependency domains.

Dependency calls must go through Execution Context or an equivalent controlled CQL executor.

This ensures dependency calls preserve:

- validation
- authorization
- scope propagation
- boundedness
- traceability
- result envelope compatibility

---

## 12.3 Dependency Call Policy May Include

Dependency policy may define:

- allowed dependency domains
- allowed dependency views
- required versions
- version resolution rules
- required output modes
- required selected fields
- required vs optional dependencies
- failure behavior
- scope propagation rules
- trace visibility rules
- maximum dependency depth

---

# 13. Scope Propagation

## 13.1 Purpose

Scope propagation controls how caller scope flows into dependency calls.

This is critical for managed composed views.

---

## 13.2 Scope Propagation Rule

A composed view must preserve or narrow caller scope unless explicitly declared otherwise by capability and allowed by policy.

A dependency call must not silently widen scope.

---

## 13.3 Scope Propagation May Include

Scope propagation may define:

- inherited scope fields
- narrowed scope fields
- prohibited widening
- explicit widening policy
- trust boundary propagation
- time window propagation
- authorization boundary propagation
- target propagation

---

## 13.4 Scope Widening

Scope widening is allowed only when:

- the capability declaration explicitly permits it
- authorization permits it
- the execution context permits it
- the Result Envelope can report it when relevant

Silent widening is forbidden.

---

# 14. Partial Failure Policy

## 14.1 Purpose

Partial Failure Policy controls what happens when a dependency or part of execution fails.

This is especially important for composed views.

---

## 14.2 Partial Failure Policy May Define

Partial Failure Policy may define:

- required dependencies
- optional dependencies
- degraded output behavior
- whether partial data may be returned
- whether partial data is safe to use
- whether execution should fail fast
- whether diagnostics should be included
- whether fallback behavior is allowed

---

## 14.3 Partial Failure Rule

Partial failure must be explicit.

A failed dependency must not silently become empty data.

A partial result must report:

- what succeeded
- what failed
- what was omitted
- whether the result is complete
- whether the result is safe to use

---

# 15. Boundedness Controls

## 15.1 Purpose

Boundedness controls prevent accidental unbounded reads and unsafe composition.

Execution Context applies boundedness policies declared by capabilities.

---

## 15.2 Boundedness May Include

Boundedness controls may include:

- required target
- required scope
- default limit
- maximum limit
- pagination requirement
- maximum window
- maximum fanout
- maximum dependency depth
- global query permission
- timeout policy
- execution budget policy

---

## 15.3 Global Query Rule

Global queries are allowed only when the selected domain, view, or raw surface explicitly declares support.

Execution Context must enforce declared global query policy.

---

# 16. Version and Capability Resolution Context

## 16.1 Purpose

Execution Context preserves the resolved capability contract used for execution.

This is necessary for validation, dependency calls, result metadata, and replay.

---

## 16.2 Resolution Context May Include

Resolution context may include:

- requested domain
- resolved domain version
- requested view
- resolved view version
- capability declaration version
- adapter version
- output mode version
- selected field versions
- dependency capability versions

---

## 16.3 Version Rule

If an unversioned reference resolves to latest, the resolved version must remain available to envelope finalization.

Latest resolution must never be invisible.

---

# 17. Envelope Construction Context

## 17.1 Purpose

Execution Context must provide enough information for Result Envelope construction and finalization.

Execution Context does not build the envelope itself.

It supplies the conditions and metadata needed for envelope construction.

---

## 17.2 Envelope Construction Context May Include

Envelope construction context may include:

- normalized query reference
- query id
- query hash
- selected fields
- selected output mode
- resolved versions
- trace reference
- caller context summary
- dependency outcomes
- partial failure state
- boundedness state
- diagnostics generated during execution

---

## 17.3 Envelope Boundary Rule

Execution Context supports envelope finalization.

It does not replace the Envelope Builder or Envelope Finalizer.

Adapter Outcome remains a separate contract.

---

# 18. Managed Composition

## 18.1 Purpose

Managed composition allows a declared view to internally query other CQL domains or views.

Execution Context is what makes managed composition safe.

---

## 18.2 Managed Composition Rule

A composed view may call dependency views only through controlled CQL execution.

The composed view must not directly bypass:

- query validation
- capability validation
- authorization
- scope propagation
- boundedness
- trace policy
- envelope construction

---

## 18.3 Managed Composition Must Preserve

Managed composition must preserve:

- source attribution
- dependency visibility
- caller scope
- caller authorization posture
- traceability
- partial failure reporting
- version reporting
- bounded execution

---

# 19. Adapter Interaction

## 19.1 Purpose

Adapters use Execution Context to execute safely.

Execution Context gives adapters controlled access to execution services and execution posture.

---

## 19.2 Adapter May Use Context To

Adapters may use Execution Context to:

- inspect caller posture
- inspect authorization posture
- report diagnostics
- invoke dependency queries through controlled CQL execution
- access trace identifiers
- observe execution budgets
- report partial dependency outcomes
- preserve source attribution

---

## 19.3 Adapter Must Not Use Context To

Adapters must not use Execution Context to:

- mutate state
- bypass validation
- widen scope silently
- bypass authorization
- hide failed dependencies
- return undeclared fields
- bypass envelope finalization
- reinterpret another domain’s meaning
- perform arbitrary joins outside declared managed views

---

# 20. Diagnostics and Reporting

## 20.1 Purpose

Execution Context records or carries diagnostics generated during execution.

Diagnostics may later be included in the Result Envelope according to visibility policy.

---

## 20.2 Diagnostics May Include

Diagnostics may include:

- budget applied
- default applied
- dependency skipped
- dependency failed
- scope narrowed
- scope widening rejected
- trace reduced
- visibility filtered
- latest version resolved
- capability mismatch

Diagnostics must not leak hidden details without authorization.

---

# 21. Execution Context Invariants

## EC-INV-01 — Execution Context Is CQL-Owned

Execution Context is owned by CQL execution.

Adapters may receive controlled access but must not redefine it.

---

## EC-INV-02 — Context Does Not Change Query Meaning

Execution Context controls execution conditions.

It must not silently change domain, subject, target, scope, filters, output, or selected fields.

---

## EC-INV-03 — Dependency Calls Must Go Through CQL

Managed composition must use Execution Context or equivalent controlled CQL execution.

Adapters must not bypass CQL validation when querying dependency domains.

---

## EC-INV-04 — Scope Must Propagate Explicitly

A composed view must preserve or narrow caller scope unless widening is explicitly declared, authorized, and allowed by policy.

---

## EC-INV-05 — Authorization Must Propagate

Dependency calls must preserve caller authorization posture unless an explicitly declared delegation or service-authority model applies.

---

## EC-INV-06 — Recursion Must Be Bounded

Managed composition must have recursion depth limits and cycle detection.

---

## EC-INV-07 — Execution Budget Must Be Bounded

Execution Context must carry budget controls such as timeout, max dependency calls, max fanout, max result size, max recursion depth, or equivalent limits.

---

## EC-INV-08 — Trace Must Be Controlled

Trace behavior must be controlled by Execution Context and visibility policy.

Trace must not leak hidden or unauthorized details.

---

## EC-INV-09 — Partial Failure Policy Must Be Explicit

Execution Context must carry or resolve the policy for required versus optional dependency failure.

---

## EC-INV-10 — Context Feeds the Result Envelope

Execution Context must provide enough metadata for envelope finalization, including query identity, trace reference, selected fields, resolved versions, boundedness state, and dependency outcomes.

---

## EC-INV-11 — Context Is Not a Template System

Execution Context guides execution.

It does not template host behavior.

It does not replace adapter outcomes.

It does not replace envelope finalization.

---

## EC-INV-12 — Managed Composition Must Be Controlled

Managed composed views are allowed only when dependency execution remains validated, authorized, bounded, traceable, and envelope-compliant.

---

# 22. Non-Goals

This specification does not define:

- Rust APIs
- Rust structs
- adapter outcome schema
- storage implementation
- authorization implementation
- tracing backend
- logging backend
- transport protocol
- query parser behavior
- result envelope schema
- persistence model
- scheduling model
- mutation behavior

This specification defines the conceptual runtime control boundary for CQL execution.

---

# 23. Final Principle

Execution Context is the controlled runtime boundary that makes CQL execution trustworthy.

It ensures validated queries execute under explicit conditions.

It ensures managed composition does not become arbitrary joining.

It ensures dependency calls remain validated, authorized, bounded, traceable, and scope-preserving.

It supplies the information needed for envelope finalization without replacing the Result Envelope contract.

It lets hosts plug into a rigid CQL execution framework without requiring templating or allowing trust-critical behavior to become informal.