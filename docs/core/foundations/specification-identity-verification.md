# Charter Core — Specification Identity & Verification Foundation
Status: FOUNDATIONAL (V1+ Structural Integrity Layer)  
Layer: Engine-Level Rule Identity  
Audience: Engine implementers, CLI maintainers, auditors, long-term stewards  

---

## 1. Purpose

This document defines how Charter Core binds its executable to its governing specifications.

The purpose of specification identity is to:

- Anchor legitimacy rules to a specific binary
- Prevent silent reinterpretation of history
- Enable deterministic cross-version verification
- Provide long-term institutional traceability
- Distinguish forks without prohibiting them

Specification identity does not create legitimacy.  
It provides evidence of the rules under which legitimacy was enforced.

---

## 2. Core Principle

A Charter engine instance must be able to answer:

**“What exact rule set does this binary claim to enforce?”**

This answer must be:

- Deterministic
- Machine-verifiable
- Immutable for that binary
- Independent of external repositories

The executable must be self-describing with respect to its rule set.

---

## 3. Scope

This specification applies to:

- Charter Core Engine binaries
- Rule enforcement semantics
- Embedded specification identity

This specification does not apply to:

- CLI behavior
- Relay transport
- Guidance interpretation
- Federation policy
- Storage implementation

It concerns only the identity of the rule set enforced by the Engine.

---

## 4. Embedded Specification Identity

Each Engine binary must embed a read-only specification manifest.

The manifest must contain:

- Engine version identifier
- A complete list of enforced foundational specifications
- A deterministic cryptographic hash of each specification
- A deterministic aggregate hash of the full specification set

The manifest must be:

- Immutable at runtime
- Deterministic across identical builds
- Independent of external network state
- Sufficient to uniquely identify the enforced rule set

Specifications referenced by the manifest are normative for that binary.

---

## 5. Specification Integrity Requirements

For a specification to be considered valid within a given Engine version:

- The specification must exist in the embedded manifest.
- The hash of the embedded specification must match the declared value.
- Locked specifications must not change without a version increment.
- The implementation must not claim enforcement of specifications that are not embedded.

A binary that violates these conditions must be considered inconsistent.

---

## 6. Relationship to Legitimacy

Specification identity does not create legitimacy.

Legitimacy is created by:

- Sessions
- Authority evaluation
- Explicit acceptance
- Immutable audit history

Specification identity provides evidence of:

- The rule set used to enforce those actions.

In disputes, historical interpretation must consider:

- The audit log
- The binary’s specification manifest
- The Engine version that executed transitions

Together, these allow reconstruction of:

- What happened
- Under which rules it happened

---

## 7. Forking and Divergence

Forking is legitimate.

Charter does not prohibit:

- Alternative implementations
- Modified specifications
- Divergent rule sets

However:

A modified binary must not silently represent itself as enforcing a different specification set.

Specification identity allows:

- Transparent divergence
- Explicit fork recognition
- Machine-verifiable differentiation

It does not:

- Centralize authority
- Prevent modification
- Enforce trust
- Impose canonical ownership

It provides clarity, not control.

---

## 8. Binary Authenticity (Optional Layer)

Independent of specification identity, a binary may optionally support:

- Cryptographic signatures
- Provenance metadata
- Build attestations

These mechanisms may allow users to verify:

- Authentic origin
- Build integrity
- Signature issuer

Authenticity is distinct from legitimacy.

A signed binary is identifiable.
A legitimate decision is mechanically valid.

These concerns must remain separate.

---

## 9. Stability Guarantees

The specification identity mechanism must remain:

- Deterministic across builds
- Backwards-auditable
- Resistant to silent mutation
- Independent of runtime configuration

Future versions may:

- Add new specifications
- Deprecate specifications with version change
- Expand manifest detail

Future versions must not:

- Allow runtime modification of embedded specs
- Allow mutable rule identity
- Permit undocumented rule enforcement

---

## 10. Design Philosophy

Without embedded specification identity, Charter is a database.

With embedded specification identity, Charter becomes:

- A rule-bound institutional system
- A time-stable legitimacy machine
- A self-describing enforcement artifact

Specification identity ensures that:

- History cannot be silently reinterpreted
- Rule changes are visible and versioned
- Enforcement claims are provable
- The system can survive author turnover
- Legitimacy remains reconstructible decades later

This layer exists for durability, not convenience.

---

## 11. Mental Model

Audit answers:
> “What happened?”

Specification identity answers:
> “Under which rules did it happen?”

Both are required for long-term institutional integrity.

Nothing in this mechanism grants authority.
It only reveals it.