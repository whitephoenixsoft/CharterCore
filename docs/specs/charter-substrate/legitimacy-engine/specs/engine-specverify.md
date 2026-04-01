# ENG-SPECVERIFY — Specification Identity & Verification

Status: REFACTORED (v3 – Structural Boundary Clarification & Provenance Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Engine rule identity, manifest embedding, spec-set hashing, and deterministic provenance verification  

Authority: Foundational authority for Engine rule identity and specification provenance.

Subordinate to: None  

Referenced by:

- ENG-DOMAIN  
- ENG-SESSION  
- ENG-DECISION  
- ENG-RECEIPT  
- ENG-INTEGRITY  
- ENG-API  
- ENG-CANON  
- ENG-INITIALIZATION  
- ENG-IMPORT  
- ENG-COMPILATION  
- ENG-STRUCTURE  
- ENG-USABILITY  

---

# 1. Purpose

ENG-SPECVERIFY defines how an Engine binary identifies and proves the exact rule set it enforces.

It is the authoritative specification for:

- specification manifest structure  
- deterministic specification hashing  
- spec_set_hash construction  
- rule provenance fields carried by Engine artifacts  
- verification outcomes for historical artifacts  
- prohibition of silent rule reinterpretation  

ENG-SPECVERIFY does not define:

- legitimacy  
- session mechanics  
- receipt structure  
- canonical byte encoding for domain artifacts  
- binary authenticity or supply-chain trust  
- governance semantics  
- structural graph semantics  
- usability or availability semantics  

Those are defined elsewhere.

Specification verification does not create legitimacy.  
It provides deterministic evidence of the rule system under which legitimacy was evaluated.

---

# 2. Core Principle

## ENG-SPECVERIFY-01 — The Engine Must Be Self-Describing

An Engine instance must be able to answer deterministically:

“What exact rule set does this binary enforce?”

That answer must be:

- machine-verifiable  
- immutable for the lifetime of the binary  
- independent of external repositories  
- deterministic across identical builds  
- explicitly representable in emitted artifacts  

The Engine must be self-describing with respect to rule identity.

---

# 3. Specification Manifest

## ENG-SPECVERIFY-02 — Immutable Embedded Manifest

Every Engine binary must embed a read-only Specification Manifest.

The manifest must include:

- engine_version  
- spec_manifest  
- spec_set_hash  
- hash_algorithm  

spec_manifest is a deterministic mapping of specification identifiers to specification hashes.

Each manifest entry must include:

- spec_name  
- spec_hash  

The manifest must be:

- embedded in the binary  
- immutable at runtime  
- deterministic across identical builds  
- independent of network access  
- sufficient to uniquely identify the rule set  

ENG-SPECVERIFY is the authority for the manifest concept itself.

---

# 4. Specification Hash Rules

## ENG-SPECVERIFY-03 — Deterministic Specification Hashing

Each specification hash must be computed using the declared hash_algorithm.

Requirements:

- hash computed over canonical specification text  
- canonical UTF-8 encoding  
- deterministic normalization rules applied consistently  
- no environment-dependent transformation  

If specification text changes in a rule-relevant way, the resulting spec_hash must change.

Specification hashes must never be mutated retroactively.

The declared hash_algorithm must be supported by ENG-CANON where canonical hashing behavior is required.

ENG-SPECVERIFY defines specification hash meaning.  
It does not define general canonical serialization of domain artifacts.

---

# 5. Specification Set Hash

## ENG-SPECVERIFY-04 — spec_set_hash Construction

spec_set_hash is the canonical identifier of the full rule set enforced by the Engine binary.

It must be computed deterministically from spec_manifest as follows:

1. Sort manifest entries lexicographically by spec_name  
2. For each entry produce the canonical string:

   spec_name + ":" + spec_hash  

3. Concatenate entries using newline separators  
4. Hash the resulting byte sequence with the declared hash_algorithm  

The resulting value is spec_set_hash.

Properties:

- deterministic  
- stable across identical implementations  
- unique to the manifest-defined rule set  
- independent of runtime environment  

---

# 6. Manifest Immutability

## ENG-SPECVERIFY-05 — No Runtime Rule Substitution

The specification manifest must be immutable.

The Engine must not allow:

- runtime modification of the manifest  
- dynamic rule loading  
- hidden rule extensions  
- specification replacement without manifest change  
- undocumented rule enforcement  

If a binary enforces rules not represented in its manifest, it is inconsistent.

That constitutes an Engine defect.

---

# 7. Provenance-Carrying Artifacts

## ENG-SPECVERIFY-06 — Rule Provenance Must Be Explicit

Artifacts that preserve Engine-evaluated outcomes or finalized decisions must carry explicit rule provenance.

Such artifacts include, where applicable:

- receipts  
- exported DAGs  
- snapshots  
- external verification bundles  
- other explicitly declared provenance-bearing structural exports  

At minimum, provenance fields must include:

- engine_version  
- spec_set_hash  
- hash_algorithm  
- export_schema_version where export-specific schema applies  

No consumer may infer rule identity implicitly.

---

## ENG-SPECVERIFY-07 — Receipts Must Carry Rule Identity

Receipts must include:

- engine_version  
- spec_set_hash  

These fields must participate in receipt integrity binding as required by ENG-RECEIPT and ENG-CANON.

This ensures that receipt integrity proves both:

- what happened  
- under which rules it happened  

---

## ENG-SPECVERIFY-08 — Resolutions May Carry Rule Identity as Structural Provenance

Where Resolutions carry:

- engine_version  
- spec_set_hash  

those fields are structural provenance fields.

ENG-DOMAIN governs whether those fields exist structurally.  
ENG-SPECVERIFY governs what they mean.

---

## ENG-SPECVERIFY-08A — EvaluationReport Is Not a Provenance Artifact

EvaluationReport, as defined in ENG-ERROR, is not a provenance-bearing artifact.

EvaluationReport must not be required to carry:

- spec_set_hash  
- engine_version as rule identity  

EvaluationReport represents runtime evaluation output, not finalized or historically persistent rule-bound artifacts.

---

# 8. Verification Semantics

## ENG-SPECVERIFY-09 — Verification Inputs

When the Engine loads or examines a provenance-bearing artifact, specification verification compares:

- artifact.spec_set_hash  
- engine.spec_set_hash  

Possible results:

- MATCH  
- LEGACY_MATCH  
- SPEC_SET_UNKNOWN  

ENG-SPECVERIFY is the authority for these verification outcomes.

---

## ENG-SPECVERIFY-10 — MATCH

MATCH means:

- the artifact references the same specification set enforced by the current Engine  

Full rule-identity verification may proceed.

---

## ENG-SPECVERIFY-11 — LEGACY_MATCH

LEGACY_MATCH means:

- the artifact references a historical spec_set_hash explicitly recognized by the Engine as supported for legacy verification  

Verification may proceed in historical compatibility mode.

Legacy support must be explicit and deterministic.

---

## ENG-SPECVERIFY-12 — SPEC_SET_UNKNOWN

SPEC_SET_UNKNOWN means:

- the artifact references a specification set unknown to the Engine  

In this case the Engine may:

- store the artifact  
- export the artifact  
- report provenance  
- preserve the artifact as historical data  

The Engine must not:

- reinterpret the artifact under different rules  
- silently assume compatibility  
- recompute legitimacy as though rules matched  
- claim full equivalence  

This is the core anti-reinterpretation guarantee.

---

## ENG-SPECVERIFY-12A — Runtime Consequences Are External

The runtime consequences of verification outcomes are not defined in this specification.

They are defined in:

- ENG-INTEGRITY (runtime safety and trust decisions)  
- ENG-INITIALIZATION (runtime entry outcomes)  

ENG-SPECVERIFY defines only the verification result, not how the Engine reacts to it.

---

# 9. Legacy Verification Support

## ENG-SPECVERIFY-13 — Optional Historical Support

An Engine may optionally support historical specification sets.

If implemented, supported legacy rule sets must be declared as a deterministic mapping:

- spec_set_hash → manifest definition  

Legacy verification may support:

- historical receipt validation  
- historical export provenance verification  
- cross-version legitimacy analysis  

Legacy support must never mutate historical artifacts.

---

# 10. Relationship to Legitimacy

## ENG-SPECVERIFY-14 — Rule Identity Is Not Legitimacy

Specification verification does not determine legitimacy.

Legitimacy is determined only by the governing behavioral specifications, including:

- session mechanics  
- authority evaluation  
- deterministic acceptance  
- receipt emission  

Specification verification provides rule provenance only.

It answers:

- under which rule system the artifact was produced  

It does not answer:

- whether that artifact is legitimate  

---

## ENG-SPECVERIFY-14A — No Structural or Usability Interpretation

Specification verification must not interpret:

- structural graph semantics  
- supersession relationships  
- ACTIVE derivation  
- usability states such as ON_HOLD or RETIRED  

These concerns belong exclusively to:

- ENG-STRUCTURE  
- ENG-USABILITY  

ENG-SPECVERIFY must treat all artifacts as opaque with respect to those semantics.

---

# 11. Fork Transparency

## ENG-SPECVERIFY-15 — Rule Divergence Must Be Visible

Forking and alternate implementations are permitted.

However:

- a modified binary must not claim the identity of a different rule set  
- changed rule text must produce changed spec_hash values  
- changed manifest composition must produce changed spec_set_hash  

This ensures transparent fork detection.

ENG-SPECVERIFY does not prohibit divergence.  
It prohibits hidden divergence.

---

# 12. Binary Authenticity Is Out of Scope

## ENG-SPECVERIFY-16 — Rule Identity Is Not Binary Authenticity

Specification identity proves the declared rule set of a binary.

It does not prove:

- binary authenticity  
- distribution integrity  
- supply-chain safety  
- publisher trustworthiness  

Those concerns belong outside the Engine specifications.

This separation must remain explicit.

---

# 13. Stability Guarantees

## ENG-SPECVERIFY-17 — Rule Identity Must Remain Stable Per Binary

The specification verification mechanism must remain:

- deterministic  
- immutable for a given binary  
- backward-auditable  
- independent of runtime configuration  

Future versions may:

- add specifications to the manifest  
- expand manifest detail  
- support more historical rule sets  

Future versions must not:

- make rule identity mutable  
- allow hidden runtime rule substitution  
- weaken provenance guarantees  
- permit silent reinterpretation  

---

# 14. Long-Term Durability

## ENG-SPECVERIFY-18 — Historical Artifacts Must Remain Interpretable

Artifacts carrying engine_version and spec_set_hash must remain interpretable independently of the original runtime context.

If the referenced specification manifest remains available, future systems must be able to determine:

- which rule set governed the decision  
- which Engine version produced the artifact  
- whether the artifact matches, legacy-matches, or diverges from the current rule set  

This supports long-term institutional auditability.

---

# 15. Engine Invariants

- every Engine binary embeds a deterministic specification manifest  
- every Engine binary computes spec_set_hash deterministically  
- rule identity is immutable for the lifetime of the binary  
- provenance-bearing artifacts must record rule identity explicitly  
- the Engine must expose rule identity through the public interface  
- silent rule reinterpretation is forbidden  
- forked rule systems must remain distinguishable  
- historical compatibility, if supported, must be explicit and deterministic  
- specification verification must not interpret structural graph semantics  
- specification verification must not interpret usability semantics  

Violation of these invariants constitutes an Engine integrity defect.

---

# 16. Possible Future Extensions

Future versions may define additional mechanisms for:

- standalone artifact verification  
- self-contained receipt verification bundles  
- portable rule-manifest packaging  
- richer historical rule-compatibility models  

Any such extension must preserve:

- explicit rule identity  
- deterministic verification  
- prohibition of silent reinterpretation  

---

# 17. Mental Model

Receipts answer:

“What was finalized?”

Specification verification answers:

“Under which rules was it finalized?”

Structure answers:

“What is the graph truth?”

Usability answers:

“What can be used forward?”

Decision answers:

“What can be accepted?”

Initialization answers:

“Can the Engine safely start?”

Integrity answers:

“Can the Engine safely operate?”

ENG-SPECVERIFY governs only rule identity.

It grants no authority.  
It creates no legitimacy.  
It prevents silent rule drift.