# CQL Adapter Outcome Specification

Status: FOUNDATIONAL DRAFT  
Applies to: CQL adapter execution, managed composition, result construction, envelope finalization, capability validation, and host integration  
Depends On: CQL Foundation, CQL Capability Declaration Specification, CQL Execution Context Specification, CQL Result Envelope Specification, Shared Invariants  
Does NOT define: Rust APIs, serialization format, transport protocols, storage engines, tracing implementation, authorization implementation, or envelope serialization structure  

---

# 1. Purpose

This document defines the conceptual model for CQL Adapter Outcomes.

Adapter Outcome is the minimal execution response returned by a CQL adapter or handler before CQL constructs and finalizes the Result Envelope.

Adapter Outcome answers:

What did the handler produce?

Adapter Outcome exists to keep host integration:

- simple
- minimal
- explicit
- envelope-safe
- capability-aligned

Hosts should not manually construct final CQL Result Envelopes.

Hosts return Adapter Outcomes.

CQL converts Adapter Outcomes into compliant Result Envelopes.

---

# 2. Foundational Principle

Adapter Outcome is the minimal domain-owned execution response.

Result Envelope is the CQL-owned execution response contract.

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
- envelope compliance
- partial reporting
- attribution integration
- final execution truth reporting

Adapter Outcomes exist to keep onboarding easy without weakening CQL trust guarantees.

---

# 3. Adapter Outcome vs Result Envelope

## 3.1 Adapter Outcome

Adapter Outcome is the adapter’s local execution response.

It is intentionally minimal.

It should be easy to return from simple handlers.

---

## 3.2 Result Envelope

The Result Envelope is the finalized CQL response.

CQL constructs it using:

- validated query
- capability declarations
- execution context
- adapter outcome
- dependency outcomes
- selected fields
- resolved versions
- diagnostics
- trace metadata

---

## 3.3 Boundary Rule

Hosts return Adapter Outcomes.

Hosts do not handcraft compliant Result Envelopes directly during normal CQL execution.

This prevents accidental omission of trust-critical metadata.

---

# 4. Minimal Outcome Model

## 4.1 Purpose

Adapter Outcome should remain minimal for onboarding and usability.

Simple read handlers should not require complex response construction.

---

## 4.2 Minimal Conceptual Structure

Conceptually, Adapter Outcome contains:

- condition
- data when applicable
- optional domain details
- optional warnings
- optional errors
- optional dependency notes
- optional attribution notes

Most handlers should only need condition and data.

---

## 4.3 Helper Constructor Principle

Implementations should support simple helper-style construction patterns conceptually equivalent to:

- ok(data)
- empty()
- nonexistent()
- unauthorized()
- hidden()
- unsupported()
- unavailable()
- partial(data)
- error(code,message)

Hosts should not need to manually construct verbose objects for ordinary cases.

---

# 5. Standard Outcome Conditions

## 5.1 Purpose

Adapter Outcomes use normalized CQL result conditions.

This allows Result Envelope construction to remain consistent across domains.

---

## 5.2 Standard Conditions

Adapter Outcomes should support:

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

---

## 5.3 Condition Distinction Rule

Conditions must remain distinct.

The following must not silently collapse into each other:

- nonexistent
- unauthorized
- hidden
- unavailable
- unsupported
- empty

---

# 6. OK Outcomes

## 6.1 Purpose

OK outcomes represent successful handler execution with usable payload data.

---

## 6.2 OK Rule

An OK outcome implies:

- condition ok
- payload data present
- no execution failure
- capability-aligned output

The Result Envelope may still include warnings, diagnostics, attribution, trace references, or version metadata.

---

## 6.3 OK Null Rule

OK outcomes should not use null payloads to represent absence.

Explicit conditions such as:

- empty
- nonexistent
- hidden
- unavailable

should be used instead.

---

# 7. Empty Outcomes

## 7.1 Purpose

Empty means the query executed successfully but returned no matching data.

---

## 7.2 Empty Rule

Empty is distinct from:

- nonexistent
- unauthorized
- hidden
- unavailable

Empty means the requested surface exists and was successfully queried.

---

# 8. Nonexistent Outcomes

## 8.1 Purpose

Nonexistent means the requested entity or surface does not exist within the effective query scope.

---

## 8.2 Nonexistent Rule

Nonexistent must not silently imply unauthorized or hidden.

---

# 9. Unauthorized Outcomes

## 9.1 Purpose

Unauthorized means the caller lacks permission to access the requested surface or data.

---

## 9.2 Unauthorized Rule

Unauthorized must remain distinct from nonexistent unless host policy intentionally chooses concealment behavior.

That concealment policy belongs to the host, not to CQL truth semantics.

---

# 10. Hidden Outcomes

## 10.1 Purpose

Hidden means encapsulated.

The requested surface exists but is intentionally hidden behind a boundary.

---

## 10.2 Hidden Rule

Hidden must not silently imply nonexistent.

The Result Envelope should preserve hidden honestly.

---

# 11. Unsupported Outcomes

## 11.1 Purpose

Unsupported means the requested capability behavior is not supported by the selected capability contract.

---

## 11.2 Unsupported Rule

Unsupported does not imply the underlying substrate lacks the information internally.

It means the declared contract does not expose it.

---

# 12. Unavailable Outcomes

## 12.1 Purpose

Unavailable means the requested execution could not complete because a required dependency or execution resource was unavailable.

---

## 12.2 Unavailable Rule

Unavailable is distinct from:

- nonexistent
- empty
- unsupported
- hidden

Unavailable indicates execution inability, not data absence.

---

# 13. Invalid Outcomes

## 13.1 Purpose

Invalid means the adapter determined the request or execution state violated domain-level execution requirements.

---

## 13.2 Invalid Rule

Validation should normally occur before adapter execution.

However, adapters may still encounter domain-level invalid conditions during execution.

These must remain explicit.

---

# 14. Error Outcomes

## 14.1 Purpose

Error outcomes represent execution failures.

---

## 14.2 Error Rule

Error outcomes should preserve:

- error code
- message
- optional severity
- optional dependency attribution
- optional recoverability hints

Errors are diagnostic.

They do not redefine domain meaning.

---

# 15. Partial Outcomes

## 15.1 Purpose

Partial outcomes represent execution that produced incomplete but potentially usable data.

Partial outcomes are especially important for managed composed views.

---

## 15.2 Partial Rule

Partial outcomes must remain explicit.

Partial must not silently become OK.

---

## 15.3 Minimal Partial Principle

Partial outcomes should still be easy to generate.

Hosts should not need to manually construct complex dependency explanations for ordinary composed-view scenarios.

---

## 15.4 Partial Metadata

Partial outcomes may preserve:

- dependency failures
- omitted fields
- degraded state
- optional dependency failures
- required dependency failures
- attribution notes
- safety hints

---

## 15.5 Partial Safety

Partial outcomes may preserve conceptual safety hints such as:

- safe_to_use
- complete
- degraded
- diagnostic_only
- not_reliable

These may later be normalized into Result Envelope partial metadata.

---

# 16. Domain-Specific Detail

## 16.1 Purpose

Domains may preserve additional domain-specific condition detail.

---

## 16.2 Domain Detail Rule

Domain-specific detail must not replace normalized CQL conditions.

Example:

condition hidden

with domain detail:

resolution_hidden_by_authority_boundary

This preserves both:

- common CQL behavior
- domain precision

---

# 17. Warnings

## 17.1 Purpose

Warnings preserve non-fatal execution concerns.

---

## 17.2 Warning Examples

Warnings may include:

- degraded calculation
- fallback behavior
- deprecated field usage
- optional dependency omission
- reduced confidence
- experimental behavior

Warnings are optional.

---

# 18. Errors and Diagnostics

## 18.1 Purpose

Adapter Outcomes may preserve execution diagnostics and domain-specific errors.

---

## 18.2 Diagnostic Rule

Diagnostics must not:

- bypass Result Envelope normalization
- bypass visibility policy
- bypass authorization
- leak hidden details without authorization

---

# 19. Attribution Notes

## 19.1 Purpose

Adapter Outcomes may preserve attribution and dependency notes for composed views.

---

## 19.2 Attribution Levels

Attribution may exist at:

- result level
- dependency level
- field level

---

## 19.3 Attribution Rule

Simple views may rely on CQL-inferred attribution.

Composed views should preserve enough attribution for partial reporting and dependency traceability.

---

# 20. Field Dependency Awareness

## 20.1 Purpose

Partial behavior and attribution should integrate with capability-declared field dependencies.

---

## 20.2 Dependency Awareness Rule

Capability declarations may define:

- field dependencies
- required dependencies
- optional dependencies
- failure behavior
- safe-if-missing behavior
- attribution policy

Execution Context records dependency outcomes.

Envelope Finalization combines:

- selected fields
- field dependencies
- dependency outcomes
- adapter outcome

to produce explicit partial reporting.

---

# 21. Output Conformance

## 21.1 Purpose

Adapter Outcomes must conform to declared capability contracts.

---

## 21.2 Output Rule

Returned payload data must conform to:

- selected fields
- selected output mode
- declared field visibility
- declared field authorization
- declared capability rules

Adapters must not silently return undeclared fields.

---

# 22. Interaction with Execution Context

## 22.1 Purpose

Execution Context controls execution posture and dependency execution.

Adapter Outcomes preserve what the handler produced.

---

## 22.2 Interaction Rule

Execution Context may provide:

- dependency execution services
- caller posture
- authorization posture
- trace context
- boundedness controls
- dependency outcomes

Adapter Outcomes do not replace Execution Context.

---

# 23. Interaction with Result Envelope

## 23.1 Purpose

Result Envelope construction finalizes execution truth.

---

## 23.2 Finalization Rule

CQL uses Adapter Outcome together with:

- validated query
- execution context
- capability declarations
- selected fields
- resolved versions
- dependency outcomes
- diagnostics
- trace metadata

to construct a compliant Result Envelope.

---

## 23.3 Finalization Boundary

Adapter Outcomes are not final envelopes.

CQL owns:

- envelope compliance
- condition normalization
- query identity
- version reporting
- trace reporting
- partial reporting
- boundedness reporting

---

# 24. Interaction with Capability Declarations

## 24.1 Purpose

Capability declarations define what outputs are valid.

---

## 24.2 Capability Rule

Adapter Outcomes must remain capability-aligned.

Capability declarations may define:

- supported fields
- supported outputs
- field dependencies
- partial behavior
- attribution rules
- dependency policies

Adapter Outcomes must not bypass declared capability contracts.

---

# 25. Interaction with Managed Composition

## 25.1 Purpose

Managed composed views may internally query dependency domains.

---

## 25.2 Composition Rule

Composed views should preserve enough dependency information for:

- attribution
- partial reporting
- diagnostics
- boundedness reporting
- traceability

Hosts should not need to manually assemble full dependency explanations for ordinary cases.

---

# 26. Interaction with Envelope Finalization

## 26.1 Purpose

Envelope Finalization transforms Adapter Outcomes into compliant Result Envelopes.

---

## 26.2 Finalization Responsibilities

Envelope Finalization may:

- normalize conditions
- attach query identity
- attach selected fields
- attach resolved versions
- attach trace references
- attach attribution metadata
- derive partial metadata
- validate output conformance
- validate capability alignment
- validate envelope compliance

---

# 27. Adapter Outcome Invariants

## AO-INV-01 — Adapter Outcome Is Minimal

Adapter Outcome should remain minimal and onboarding-friendly.

---

## AO-INV-02 — Hosts Return Outcomes, Not Final Envelopes

Hosts return Adapter Outcomes.

CQL constructs compliant Result Envelopes.

---

## AO-INV-03 — Conditions Must Remain Distinct

Adapter Outcomes must preserve explicit condition distinctions.

---

## AO-INV-04 — Partial Must Be Explicit

Partial outcomes must not silently become OK outcomes.

---

## AO-INV-05 — Output Must Conform to Capability Contracts

Adapter Outcomes must conform to declared output modes and selected fields.

---

## AO-INV-06 — Adapter Outcomes Must Not Bypass Envelope Finalization

Adapter Outcomes must not bypass envelope construction or compliance validation.

---

## AO-INV-07 — Attribution Must Be Preservable

Composed views must preserve enough attribution for dependency traceability and partial reporting.

---

## AO-INV-08 — Field Dependency Awareness Must Be Supported

Partial reporting and attribution may derive from declared field dependency metadata.

---

## AO-INV-09 — Hidden Means Encapsulated

Hidden must remain distinct from nonexistent.

---

## AO-INV-10 — Execution Context Owns Runtime Control

Adapter Outcomes preserve handler results.

Execution Context preserves execution conditions.

These responsibilities must remain separate.

---

# 28. Non-Goals

This specification does not define:

- Rust APIs
- helper function syntax
- serialization formats
- transport protocols
- tracing implementation
- authorization implementation
- storage implementation
- query parsing
- mutation semantics
- scheduling systems
- persistence systems
- envelope serialization
- UI rendering

This specification defines the conceptual host-facing execution response contract for CQL adapters.

---

# 29. Final Principle

Adapter Outcome is the minimal host-facing execution response contract of CQL.

It allows hosts to expose queryable read surfaces without manually constructing complex Result Envelopes.

It keeps onboarding simple while preserving explicit execution truth.

It separates:

- handler output
- execution conditions
- capability declarations
- envelope finalization

so that:

- hosts remain productive
- managed composition remains bounded
- partial results remain explicit
- attribution remains traceable
- Result Envelopes remain trustworthy

Adapter Outcome exists so hosts can plug into a rigid CQL execution framework without needing templating or unsafe manual envelope construction.