# CQL Result Envelope Specification

Status: FOUNDATIONAL DRAFT  
Applies to: CQL Core, JSON IL, DSL, adapter contracts, capability declarations, managed composition, validation, execution context, query identity, diagnostics, and replay metadata  
Depends On: CQL Foundation, CQL Capability Declaration Specification, Shared Invariants, JSON IL Foundation, DSL Foundation, Module Boundaries, Canonical Naming  
Does NOT define: Rust APIs, serialization format, storage backends, transport protocols, logging systems, tracing implementation, or execution engine internals  

---

# 1. Purpose

This document defines the conceptual model for the CQL Result Envelope.

The Result Envelope is the standard response wrapper returned by compliant CQL execution.

The Result Envelope answers:

What happened when this query executed?

The Result Envelope preserves:

- execution outcome
- result condition
- domain-owned payload data
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

The Result Envelope is the trust-preserving response contract for CQL.

Without the Result Envelope, CQL cannot reliably preserve the distinction between:

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

The Result Envelope exists to preserve those distinctions explicitly.

---

# 2. Foundational Principle

The Result Envelope owns response truth.

Domains own payload meaning.

The Result Envelope explains what happened during execution.

The Result Envelope does not reinterpret what the payload means.

Examples:

CQL may report:

- the query succeeded
- the query partially succeeded
- the query failed validation
- the selected view resolved to version v3
- the result was partial
- a dependency was unavailable
- the selected fields were id,status,severity

CQL does not report:

- the area is healthy
- the resolution is legitimate
- the observation is true
- the identity is authoritative

Those meanings belong to domains and higher explanation layers.

---

# 3. Result Envelope Ownership

The Result Envelope is CQL-owned.

Payload data is domain-owned.

CQL owns:

- envelope structure
- required metadata
- result condition normalization
- query identity
- version reporting
- selected field reporting
- validation outcome reporting
- dependency reporting
- partial reporting
- envelope compliance

Domains own:

- payload meaning
- domain-specific calculations
- domain-specific interpretation
- domain-specific warnings and diagnostics
- domain-specific field semantics

CQL preserves the boundary between execution truth and domain meaning.

---

# 4. Result Envelope Structure

Conceptually, a compliant Result Envelope contains:

- status
- condition
- data
- metadata
- diagnostics
- errors
- warnings
- partial
- sources
- versions
- query
- trace

Implementations may organize these differently internally, but the conceptual sections must remain preserved.

---

# 5. Status

## 5.1 Purpose

Status provides the broad execution outcome.

Status answers:

Did the query execute successfully, partially, fail, or get rejected?

---

## 5.2 Standard Status Values

Recommended status values include:

- success
- partial
- failed
- rejected

---

## 5.3 Status Meaning

### success

The query validated and executed successfully.

### partial

The query validated and executed, but some portion of the requested result could not be fully produced.

### failed

The query validated but execution failed.

### rejected

The query failed validation or policy checks before execution.

---

# 6. Result Condition

## 6.1 Purpose

Condition provides the precise trust-preserving execution outcome.

Condition answers:

What specifically happened?

Condition is more precise than status.

---

## 6.2 Standard Conditions

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

---

## 6.3 Examples

### Successful result

status success  
condition ok

### Empty result

status success  
condition empty

### Unsupported field

status rejected  
condition unsupported

### Missing dependency

status failed  
condition unavailable

### Partial composed result

status partial  
condition partial

---

## 6.4 Hidden Means Encapsulated

Hidden means encapsulated.

Hidden must not silently imply nonexistent.

A hidden condition indicates that the queried surface exists behind a boundary that intentionally does not expose details.

---

## 6.5 Unauthorized Does Not Mean Nonexistent

Unauthorized means the caller is not permitted to access the surface.

Unauthorized must not silently imply nonexistent unless the host intentionally chooses that security posture.

If the host chooses that posture, that is a host security policy, not a CQL truth claim.

---

## 6.6 Unsupported Does Not Mean No Data

Unsupported means the capability contract does not expose the requested behavior.

Unsupported does not imply the underlying domain lacks the data internally.

---

# 7. Data

## 7.1 Purpose

Data is the domain-owned payload returned by execution.

The Result Envelope wraps data.

CQL does not redefine data meaning.

---

## 7.2 Data Rules

Data must:

- conform to the selected output mode
- conform to selected field declarations
- respect authorization
- respect scope
- respect visibility
- avoid undeclared fields
- remain read-only from the CQL perspective

---

## 7.3 Data Visibility by Condition

### ok

Data may be returned normally.

### empty

Data should return an empty shape appropriate to the output mode.

### nonexistent

Data should generally be absent or null.

### unauthorized

Data should generally be absent unless explicitly allowed by policy.

### hidden

Data may contain safe placeholder information only when explicitly declared.

### partial

Partial data may be returned with explicit partial metadata.

### unsupported

Data should generally be absent.

### invalid

Data should generally be absent.

### unavailable

Data should generally be absent.

### error

Data should generally be absent.

---

# 8. Metadata

## 8.1 Purpose

Metadata preserves execution context needed to trust and understand the result.

Metadata explains how the result was produced.

---

## 8.2 Metadata Content

Metadata should support:

- query identity
- result identity
- execution timestamp
- normalized query reference
- resolved versions
- selected fields
- selected output mode
- capability references
- execution duration when available
- source attribution
- replay support
- diagnostics references

---

## 8.3 Required Metadata

A compliant Result Envelope should preserve:

- envelope version
- status
- condition
- resolved domain
- resolved view
- resolved versions
- selected output mode
- selected fields
- query identity reference

---

# 9. Versions

## 9.1 Purpose

Version metadata preserves the contract context used during execution.

Anything affecting query meaning or validation should be version-visible.

---

## 9.2 Version Metadata

The Result Envelope should preserve:

- cql version
- json il version
- dsl version when applicable
- domain version
- view version
- capability version
- adapter version
- envelope version
- dependency versions when relevant

---

## 9.3 Latest Resolution Reporting

If a query omits a version reference and resolves to latest, the resolved version must be reported.

Example:

requested view area_status

resolved view area_status:v3

This preserves usability without hiding version reality.

---

# 10. Query Identity

## 10.1 Purpose

The Result Envelope preserves query identity for:

- replay
- auditing
- diagnostics
- comparison
- debugging
- traceability

---

## 10.2 Query Identity Metadata

Query identity may include:

- query id
- query hash
- normalized JSON IL hash
- DSL source hash when applicable
- requested domain
- requested view
- requested version
- selected output mode
- selected fields

---

## 10.3 Replay Principle

The Result Envelope should preserve enough information to understand:

- what was asked
- what versions answered
- what output mode was used
- what fields were selected
- what dependencies participated
- what conditions occurred

CQL does not need to become a persistence system to preserve replay metadata.

---

# 11. Diagnostics

## 11.1 Purpose

Diagnostics describe important execution details that are not necessarily errors.

Diagnostics explain what occurred during execution.

---

## 11.2 Diagnostic Examples

Examples include:

- default output mode applied
- unversioned view resolved to latest
- dependency unavailable
- optional dependency skipped
- fields omitted because not selected
- output mode fallback rejected
- result truncated by limit
- trace visibility reduced
- deprecated capability used

Diagnostics are informational unless policy elevates them.

---

# 12. Errors

## 12.1 Purpose

Errors describe structured execution or validation failures.

Errors are diagnostic.

Errors do not redefine domain truth.

---

## 12.2 Error Requirements

Errors should preserve:

- error code
- message
- condition
- severity
- affected path
- related domain/view when applicable
- recoverability
- dependency attribution when relevant

---

## 12.3 Example Error Types

Examples include:

- UNKNOWN_DOMAIN
- UNKNOWN_VIEW
- UNSUPPORTED_FIELD
- UNSUPPORTED_OUTPUT_MODE
- VERSION_NOT_FOUND
- CAPABILITY_MISMATCH
- UNAUTHORIZED
- HIDDEN
- DEPENDENCY_UNAVAILABLE
- UNBOUNDED_QUERY_REJECTED
- ENVELOPE_CONSTRUCTION_FAILED

---

# 13. Warnings

## 13.1 Purpose

Warnings describe non-fatal concerns.

Warnings indicate behavior that callers should understand but that did not invalidate execution.

---

## 13.2 Warning Examples

Examples include:

- deprecated view version used
- latest resolved to deprecated version
- experimental output mode used
- partial dependency trace hidden
- result truncated
- selected fields omitted by authorization
- optional dependency unavailable

Warnings should not silently become errors.

---

# 14. Partial Results

## 14.1 Purpose

Partial results preserve explicit trust boundaries when some portion of execution succeeded and another portion failed or became unavailable.

Partial must never be hidden as success.

---

## 14.2 Partial Metadata

Partial metadata should explain:

- what succeeded
- what failed
- what was omitted
- what dependencies failed
- whether the result is complete
- whether the data is safe to use

---

## 14.3 Partial Usability

A partial result should expose whether the result remains usable.

Conceptual examples include:

- usable
- degraded
- diagnostic_only
- not_reliable

Or simpler concepts such as:

- complete true or false
- safe_to_use true or false

---

## 14.4 Partial Dependency Failure

If a composed view loses a dependency, the Result Envelope must preserve that explicitly.

Missing dependency data must not silently become empty data.

---

# 15. Sources

## 15.1 Purpose

Sources preserve attribution for the domains, views, and dependencies involved in producing a result.

---

## 15.2 Source Attribution

Source metadata may include:

- source domain
- source view
- source version
- source output mode
- dependency role
- optional vs required dependency
- source visibility posture

---

## 15.3 Composed Views

Composed views should preserve source attribution when relevant and allowed.

This does not require exposing all underlying data.

It preserves execution traceability and ownership visibility.

---

# 16. Trace

## 16.1 Purpose

Trace metadata allows execution flow, dependency activity, and diagnostic behavior to be referenced.

Trace supports:

- debugging
- auditing
- replay analysis
- diagnostics
- composition visibility

---

## 16.2 Trace Reference

The Result Envelope should preserve a trace reference rather than always embedding full trace detail.

Examples include:

- trace id
- trace availability
- trace level
- trace visibility
- trace summary

---

## 16.3 Trace Visibility

Trace visibility may be reduced by authorization or encapsulation policy.

If trace detail is hidden, the Result Envelope should preserve that honestly.

Example:

trace available true  
trace visible false  
trace hidden reason unauthorized

---

# 17. Adapter Outcome

## 17.1 Purpose

Adapters should not handcraft final Result Envelopes directly.

Adapters return Adapter Outcomes.

CQL then constructs and finalizes the Result Envelope.

This prevents accidental omission of trust-critical metadata.

---

## 17.2 Adapter Outcome Responsibility

Adapter Outcomes may contain:

- payload data
- domain-specific diagnostics
- domain-specific errors
- domain-specific warnings
- source attribution
- dependency outcomes
- domain-specific condition detail

Adapter Outcomes do not own final envelope consistency.

---

# 18. Envelope Builder

## 18.1 Purpose

The Envelope Builder constructs the compliant Result Envelope using:

- normalized query
- capability declarations
- execution context
- validation outcome
- adapter outcome
- selected fields
- resolved versions
- dependency metadata
- trace metadata

---

## 18.2 Builder Responsibility

The Envelope Builder ensures required Result Envelope structure exists before finalization.

The Builder should prevent missing required sections.

---

# 19. Envelope Finalization

## 19.1 Purpose

Envelope Finalization is the final trust-preserving verification step before returning a Result Envelope.

---

## 19.2 Finalization Responsibilities

Envelope Finalization verifies:

- status exists
- condition exists
- required metadata exists
- version metadata exists
- selected fields are preserved
- query identity exists
- result conditions are normalized
- partial status is explicit
- data conforms to selected fields
- data conforms to selected output mode
- hidden and unauthorized conditions are preserved honestly
- unsupported behavior was not silently ignored

---

## 19.3 Envelope Compliance Failure

If a compliant Result Envelope cannot be produced, CQL must return an explicit envelope construction or compliance failure.

Malformed envelopes must not silently escape CQL execution.

---

# 20. Envelope Compliance

## 20.1 Purpose

A Result Envelope must be either compliant or non-compliant.

Compliance means the envelope preserves required trust guarantees.

---

## 20.2 Compliance Rules

A compliant Result Envelope must:

- preserve explicit result conditions
- preserve version metadata
- preserve query identity
- preserve selected field reporting
- preserve output mode reporting
- preserve partial status explicitly
- preserve errors and warnings structurally
- preserve source attribution when relevant
- preserve trace references when enabled
- avoid undeclared fields
- avoid collapsing conditions into falsehood or emptiness

---

# 21. Condition Normalization

## 21.1 Purpose

Domains may use domain-specific condition detail internally.

CQL normalizes conditions into common Result Envelope conditions.

---

## 21.2 Example

A domain-specific internal condition:

resolution_not_visible_due_to_authority_boundary

may normalize into:

condition hidden

with additional detail preserved separately.

This allows consistent client behavior while preserving domain precision.

---

# 22. Output Conformance

## 22.1 Purpose

Returned payload data must conform to the declared capability contract.

---

## 22.2 Output Conformance Rules

Returned data must conform to:

- selected output mode
- selected fields
- declared field availability
- declared visibility rules
- declared authorization rules

A query requesting:

fields id,status

must not silently receive undeclared or unselected fields unless explicitly allowed by the capability contract.

---

# 23. Visibility Filtering

## 23.1 Purpose

Result detail may be filtered by authorization or encapsulation.

However, hidden detail must still be represented honestly.

---

## 23.2 Visibility Rule

The Result Envelope may reduce visible detail.

The Result Envelope must not lie about the reduction.

Examples:

- trace available but hidden
- dependency exists but encapsulated
- field exists but unauthorized
- result exists but hidden

---

# 24. Explanation Boundary

## 24.1 Principle

The Result Envelope may preserve explanation-ready metadata.

The Result Envelope is not the explanation layer.

---

## 24.2 Allowed Explanation-Ready Metadata

The Result Envelope may expose:

- provenance
- source attribution
- diagnostics
- warnings
- dependency visibility
- trace references
- summary output when domain-supported

The Result Envelope does not:

- narrate
- deliberate
- advise
- reconcile
- reinterpret payload meaning

Higher layers such as CGL, CDS, UI systems, CLI presentation, or assistants perform explanation.

---

# 25. Boundedness

## 25.1 Principle

Result metadata should preserve boundedness behavior when relevant.

Examples include:

- truncation
- pagination
- limit enforcement
- timeout behavior
- partial execution due to execution budget

---

## 25.2 Truncation Reporting

If a result is truncated, the Result Envelope should preserve that explicitly.

Truncated results must not silently appear complete.

---

# 26. Result Envelope Versioning

## 26.1 Purpose

The Result Envelope itself is versioned.

Envelope behavior may evolve over time.

---

## 26.2 Envelope Version Visibility

A compliant Result Envelope should expose its own envelope version.

Clients should be able to understand what envelope contract they are interpreting.

---

# 27. Result Envelope Invariants

## RES-INV-01 — Result Envelope Is CQL-Owned

The Result Envelope is owned by CQL.

Payload meaning remains domain-owned.

---

## RES-INV-02 — Result Conditions Must Remain Distinct

CQL must distinguish:

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

---

## RES-INV-03 — Hidden Means Encapsulated

Hidden means encapsulated and must not silently imply nonexistent.

---

## RES-INV-04 — Unauthorized Does Not Mean Nonexistent

Unauthorized must remain distinct from nonexistent unless explicitly hidden by host policy.

---

## RES-INV-05 — Missing Dependency Is Explicit

Missing, unavailable, unauthorized, or incompatible dependencies must not silently become empty data.

---

## RES-INV-06 — Partial Must Be Explicit

Partial results must be represented explicitly.

Partial must not silently become success.

---

## RES-INV-07 — Versions Must Be Visible

Resolved versions must be preserved in metadata.

Latest resolution must be visible.

---

## RES-INV-08 — Query Identity Must Be Preserved

A Result Envelope must preserve sufficient query identity for diagnostics and replay understanding.

---

## RES-INV-09 — Envelope Finalization Is Required

A compliant CQL result must pass envelope finalization.

---

## RES-INV-10 — Output Must Conform to Capability Contracts

Returned payload data must conform to declared output modes and selected fields.

---

## RES-INV-11 — Envelope Compliance Is Mandatory

Malformed or incomplete envelopes must not silently escape CQL execution.

---

## RES-INV-12 — Explanation Belongs Outside the Envelope

The Result Envelope may preserve explanation-ready metadata but is not itself the explanation layer.

---

# 28. Non-Goals

This specification does not define:

- Rust structs
- serialization formats
- transport protocols
- logging systems
- tracing implementations
- OpenTelemetry integration
- storage engines
- GraphQL compatibility
- SQL compatibility
- UI rendering
- assistant behavior
- explanation generation
- persistence systems

This specification defines the conceptual trust-preserving response contract for CQL execution.

---

# 29. Final Principle

The Result Envelope is the trust boundary of CQL execution.

It preserves what happened without pretending to own what the payload means.

It exists so callers can distinguish:

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

It preserves query identity, version resolution, selected fields, diagnostics, dependency visibility, and trace references.

It prevents hosts and adapters from accidentally weakening the trust guarantees of CQL.

The Result Envelope makes explicit execution behavior a foundational part of the CQL architecture.