Charter CQL — Substrate Integration Contract

Status: FOUNDATIONAL DRAFT
Purpose: Define what Charter substrates should expect when integrating with CQL

---

1. Overview

This document defines how a Charter substrate integrates with CQL.

CQL is the read-only query layer for Charter-compatible substrates.

CQL does not own substrate truth.
CQL does not define substrate meaning.
CQL does not mutate substrate state.

CQL provides:

- a canonical query model
- a human DSL
- capability discovery
- pre-dispatch validation
- adapter dispatch
- execution context propagation
- result envelope construction

Substrates provide:

- declared query capabilities
- read handlers or adapters
- domain-owned payloads
- domain-owned diagnostics
- clear error and absence behavior

---

2. What CQL Expects From a Substrate

A substrate integrating with CQL must declare:

- its CQL domain name
- supported views
- supported raw surfaces, if any
- supported targets
- supported filters
- supported scopes
- supported output modes
- selectable fields
- version information
- boundedness rules
- possible result conditions

A substrate must not rely on undocumented query behavior.

If something is not declared, CQL must treat it as unsupported.

---

3. What a Substrate Can Expect From CQL

A substrate can expect CQL to:

- receive queries through JSON IL, DSL, SDK builders, or CLI surfaces
- compile DSL queries into canonical JSON IL before execution
- validate queries against declared substrate capabilities
- reject unsupported queries before dispatch
- pass a controlled execution context
- call the registered adapter or handler
- receive an adapter outcome
- wrap the outcome in a standard result envelope

CQL should not ask a substrate to execute a query that violates its declared capabilities.

---

4. Query Model

CQL queries are read-only access requests.

A CQL query may include:

- domain
- subject
- target
- scope
- filters
- output options
- selected fields
- context
- metadata

The canonical form is JSON Intermediate Language.

The DSL is only a human-friendly authoring syntax.

The DSL must compile into JSON IL before validation or execution.

---

5. Capability Declaration

Capability declarations are the main contract between CQL and a substrate.

They answer:

- What can be queried?
- How can it be queried?
- What fields can be selected?
- What output modes are valid?
- What versions are available?
- What constraints apply?
- What result conditions may occur?

Capability declarations are enforceable contracts, not documentation only.

CQL must not guess substrate behavior.

---

6. Adapter Integration

A substrate integrates with CQL through an adapter or handler.

The adapter receives:

- validated query
- resolved capability information
- execution context
- target and scope information
- output requirements

The adapter returns an adapter outcome.

The adapter outcome should contain:

- payload data
- domain-specific diagnostics
- domain-specific warnings
- dependency notes
- local execution status

The substrate should not manually construct the final CQL result envelope.

CQL owns final envelope construction.

---

7. Result Envelope

CQL wraps adapter outcomes in a standard result envelope.

The result envelope preserves:

- execution outcome
- result condition
- payload
- metadata
- diagnostics
- warnings
- errors
- partial status
- source attribution
- version resolution
- selected fields
- query identity
- replay metadata
- trace references

The envelope must preserve distinctions between:

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

These states must not be collapsed into a generic empty result.

---

8. Execution Context

CQL passes execution context to the substrate adapter.

Execution context may include:

- caller context
- authorization context
- visibility context
- trace context
- execution budget
- recursion depth
- dependency policy
- partial failure policy
- version resolution context
- boundedness controls

The substrate must honor the execution context.

If the substrate cannot honor it, it must return an explicit adapter outcome condition.

---

9. Runtime Integration

Runtime may use CQL to route queries across substrates.

Runtime may:

- register CQL adapters
- discover capabilities
- route validated queries
- coordinate managed composed views
- propagate execution context
- collect result envelopes

Runtime does not redefine substrate meaning.

CQL does not make Runtime authoritative over substrate truth.

---

10. Persistence Expectations

CQL does not require a substrate to use a specific storage model.

A substrate may use:

- local storage
- derived storage
- cache storage
- external service reads
- in-memory read models

However, the substrate must declare what its CQL surfaces expose and whether those surfaces are authoritative, derived, cached, or externally dependent.

---

11. Federation Behavior

CQL is not the federation transport.

Federation remains a CCS / CRS concern.

However, CQL results should preserve metadata needed for federated interpretation, including:

- source substrate
- authority level
- version information
- provenance
- trace references
- result condition

---

12. Error Surface

A substrate must make errors explicit.

Expected error categories include:

- invalid query
- unsupported capability
- unauthorized access
- hidden resource
- nonexistent resource
- unavailable dependency
- timeout or budget exceeded
- partial result
- adapter failure
- internal substrate error

CQL should preserve these conditions in the result envelope.

---

13. Determinism Guarantees

CQL expects query execution to be deterministic when:

- the same query is used
- the same capability version is used
- the same execution context is used
- the substrate state has not changed

If a substrate depends on external systems, caches, clocks, live feeds, or unstable state, that dependency must be declared.

---

14. Constraints

A CQL-integrated substrate must not:

- mutate state during query execution
- redefine another substrate's truth
- expose undeclared query behavior
- ignore execution context constraints
- collapse hidden, unauthorized, nonexistent, and empty into the same state
- return payloads that violate declared output mode or field rules
- depend on implicit joins that were not declared as managed views

---

15. Integration Notes

Runtime

Runtime should treat CQL as the preferred read surface for substrate access.

CLI

CLI should author queries through the CQL DSL or JSON IL.

VDS

VDS should use CQL to inspect declared read surfaces, not private substrate internals.

VLS

VLS should use CQL result envelopes as trust-preserving read results and should not infer hidden substrate meaning.

---

16. Compliance Statement

A substrate is CQL-compatible when it:

- publishes capability declarations
- exposes read-only adapters or handlers
- supports validation before dispatch
- honors execution context
- returns adapter outcomes
- allows CQL to construct result envelopes
- preserves explicit absence and error states
- declares versions and boundedness
- keeps substrate meaning owned by the substrate