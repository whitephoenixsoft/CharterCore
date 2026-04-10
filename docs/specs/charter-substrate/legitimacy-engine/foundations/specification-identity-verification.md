# Charter Core — Specification Identity & Verification Foundation

Status: FOUNDATIONAL  
Layer: Engine-Level Rule Identity  
Audience: Engine implementers, CLI maintainers, auditors, long-term stewards  

---

## 1. Purpose

This document defines how Charter Core binds its executable to its governing specifications.

The purpose of specification identity is to:

- Anchor legitimacy rules to a specific Engine implementation  
- Prevent silent reinterpretation of history  
- Enable deterministic cross-version verification  
- Provide long-term institutional traceability  
- Distinguish forks without prohibiting them  

Specification identity does not create legitimacy.  
It provides evidence of the rules under which legitimacy was enforced.

---

## 2. Core Principle

A Charter Engine instance must be able to answer:

**“What exact rule set is this Engine deterministically enforcing?”**

This answer must be:

- Deterministic  
- Machine-verifiable  
- Immutable for that Engine instance  
- Independent of external repositories  

The Engine must be self-describing with respect to its rule set.

---

## 3. Scope

This specification applies to:

- Charter Core Engine implementations  
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

Each Engine instance must embed a read-only specification manifest.

The manifest must contain:

- Engine version identifier  
- A deterministic representation of the enforced specification set  
- A canonical aggregate hash of the specification set (**spec_set_hash**)  

The manifest must be:

- Immutable at runtime  
- Deterministic across identical builds  
- Independent of external network state  
- Sufficient to uniquely identify the enforced rule set  

The **spec_set_hash** is the canonical identity of the rule set.

---

## 5. Specification Integrity Requirements

For a specification set to be considered valid for an Engine instance:

- The specification set must be represented in the embedded manifest  
- The computed spec_set_hash must match the declared value  
- Locked specifications must not change without a version increment  
- The Engine must not enforce rules that are not represented in the manifest  

An Engine that violates these conditions must be considered inconsistent.

---

## 6. Relationship to Legitimacy

Specification identity does not create legitimacy.

Legitimacy is created by:

- Sessions  
- Authority evaluation  
- Explicit acceptance  
- Immutable history  

Specification identity provides evidence of:

- The rule set used to enforce those actions  

In disputes or audits, interpretation must consider:

- The recorded artifacts (sessions, resolutions, receipts)  
- The Engine’s specification identity  
- The Engine version that executed transitions  

Together, these allow reconstruction of:

- What happened  
- Under which rules it happened  

---

## 7. Receipt Binding

Receipts must include the **spec_set_hash** under which they were produced.

This ensures that:

- Each legitimacy event is bound to a specific rule set  
- Historical outcomes can be verified against the correct rules  
- Cross-version verification remains deterministic  

Receipts are the bridge between:

- recorded outcomes  
- and the rule set that produced them  

---

## 8. Deterministic Execution Requirement

Specification identity is meaningful only if execution is deterministic.

Given identical:

- specification identity (spec_set_hash)  
- domain state  
- command inputs  

the Engine must produce identical outcomes.

The Engine must not depend on:

- timestamps  
- environment  
- storage order  
- external systems  

Rule identity and deterministic execution are inseparable.

---

## 9. Verification Behavior

When processing historical artifacts:

- The Engine must verify the spec_set_hash recorded in receipts  
- The Engine must detect mismatches between receipt identity and runtime identity  
- The Engine must not silently reinterpret artifacts under a different rule set  

If a mismatch occurs:

- it must be explicitly surfaced  
- no silent fallback or reinterpretation is allowed  

Correctness is prioritized over convenience.

---

## 10. Forking and Divergence

Forking is legitimate.

Charter does not prohibit:

- Alternative implementations  
- Modified specifications  
- Divergent rule sets  

However:

A modified Engine must not represent itself as enforcing a different specification set than it actually embeds.

Specification identity enables:

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

## 11. Binary Authenticity (Optional Layer)

Independent of specification identity, an Engine may optionally support:

- Cryptographic signatures  
- Provenance metadata  
- Build attestations  

These mechanisms may allow users to verify:

- Authentic origin  
- Build integrity  
- Signature issuer  

Authenticity is distinct from legitimacy.

A signed Engine is identifiable.  
A legitimate decision is mechanically valid.

These concerns must remain separate.

---

## 12. Stability Guarantees

Specification identity must remain:

- Deterministic across builds  
- Backwards-auditable  
- Resistant to silent mutation  
- Independent of runtime configuration  

Future versions may:

- Add new specifications  
- Deprecate specifications with version change  
- Expand manifest detail  

Future versions must not:

- Allow runtime modification of specification identity  
- Allow mutable rule identity  
- Permit undocumented rule enforcement  

---

## 13. Design Philosophy

Without specification identity, Charter is a data system.

With specification identity, Charter becomes:

- A rule-bound institutional system  
- A time-stable legitimacy system  
- A self-describing enforcement artifact  

Specification identity ensures that:

- History cannot be silently reinterpreted  
- Rule changes are visible and versioned  
- Enforcement claims are provable  
- The system can survive author turnover  
- Legitimacy remains reconstructible over long time horizons  

This layer exists for durability, not convenience.

---

## 14. Mental Model

Audit answers:

> “What happened?”

Specification identity answers:

> “Under which rules did it happen?”

Both are required for long-term integrity.

Nothing in this mechanism grants authority.  
It only reveals it.