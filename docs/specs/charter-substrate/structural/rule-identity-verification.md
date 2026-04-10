# Charter Platform — Rule Identity & Verification

Status: FOUNDATIONAL (Architectural)  
Applies to: All Charter Libraries (Engine, Runtime, Guidance, VLS, VDS, Relay)  
Does NOT define: legitimacy semantics, session mechanics, storage formats, or transport protocols  

---

## I. Purpose

This document defines how Charter systems:

- identify the rule set they enforce  
- attach rule provenance to historical artifacts  
- verify compatibility across versions and systems  
- prevent silent reinterpretation of history  

This mechanism exists to preserve **meaning over time**, not to enforce trust or authority.

---

## II. Core Principle

### CI-RULE-01 — Systems Must Be Self-Describing

Any Charter-compatible system must be able to answer:

“What rule set does this system enforce?”

This answer must be:

- deterministic  
- machine-readable  
- immutable for a given build  
- independent of external repositories  
- stable across identical builds  

This identity is called **Rule Identity**.

---

## III. Rule Identity

Rule Identity is a deterministic fingerprint of the rule set enforced by a system.

It typically includes:

- system version identifier  
- rule set identifier (e.g., spec_set_hash)  
- declared hashing or normalization scheme (if applicable)  

Rule Identity:

- does not define legitimacy  
- does not imply correctness  
- does not imply trust  
- only identifies the rule system used  

---

### CI-RULE-02 — Rule Identity Is Immutable Per Build

For any compiled library or binary:

- Rule Identity must not change at runtime  
- Rule Identity must not be influenced by configuration  
- Rule Identity must not depend on external state  

If rule behavior changes, Rule Identity must change.

---

### CI-RULE-03 — No Hidden Rule Mutation

A system must not:

- apply rules not represented in its Rule Identity  
- load rules dynamically without updating identity  
- silently alter rule behavior  

Violation of this constraint constitutes a system integrity failure.

---

## IV. Provenance

### CI-RULE-04 — Artifacts Must Be Able to Carry Rule Provenance

Artifacts that preserve historical or structural truth may include Rule Identity as provenance.

Examples:

- receipts  
- exported graphs  
- snapshots  
- verification bundles  

At minimum, provenance should allow consumers to determine:

- which system produced the artifact  
- which rule set governed it  

---

### CI-RULE-05 — Provenance Does Not Create Legitimacy

Rule provenance:

- does not determine legitimacy  
- does not validate correctness  
- does not imply authority  

It only answers:

“Under which rules was this produced?”

---

## V. Verification

Verification compares:

- the Rule Identity of an artifact  
- the Rule Identity of the current system  

### CI-RULE-06 — Deterministic Verification

Verification must be:

- deterministic  
- side-effect free  
- read-only  
- reproducible  

It must not:

- mutate state  
- repair inconsistencies  
- reinterpret data  

---

### CI-RULE-07 — Verification Outcomes

Verification produces structural outcomes such as:

- MATCH  
- LEGACY_MATCH (explicitly supported historical rules)  
- UNKNOWN  

These outcomes describe **rule compatibility**, not legitimacy.

---

### CI-RULE-08 — Unknown Rule Sets

If a system encounters an artifact with unknown Rule Identity:

It may:

- store the artifact  
- display the artifact  
- preserve the artifact  
- export the artifact  

It must not:

- reinterpret it under different rules  
- assume compatibility  
- recompute legitimacy as equivalent  

This enforces the **anti-reinterpretation guarantee**.

---

## VI. Compatibility & Historical Interpretation

### CI-RULE-09 — Historical Rules Are Preserved

Artifacts must be interpreted under the rules that existed when they were produced.

New rule sets must not retroactively alter historical meaning.

---

### CI-RULE-10 — Explicit Compatibility Only

If a system supports historical rule sets:

- support must be explicitly declared  
- compatibility must be deterministic  
- mappings must not be inferred  

---

## VII. Relationship to Legitimacy

### CI-RULE-11 — Rule Identity Is Not Legitimacy

Rule Identity and Verification:

- do not create legitimacy  
- do not validate decisions  
- do not evaluate authority  

Legitimacy remains defined by:

- sessions  
- authority rules  
- acceptance semantics  
- resolution formation  

Verification provides context, not judgment.

---

## VIII. Forking and Divergence

### CI-RULE-12 — Divergence Must Be Visible

Forks and alternate implementations are allowed.

However:

- rule changes must change Rule Identity  
- identical identities must imply identical rules  
- divergence must be detectable  

This ensures transparency without restricting evolution.

---

## IX. Binary Identity (Out of Scope)

Rule Identity does not prove:

- binary authenticity  
- build provenance  
- supply chain integrity  

These concerns may be handled separately (e.g., signatures), but must remain independent.

---

## X. System Boundaries

Rule Identity and Verification apply across:

- Engine (legitimacy rules)  
- Runtime (workflow orchestration rules where applicable)  
- Guidance / Exegesis layers (interpretation rules)  
- VLS / VDS (alignment and signal rules)  

Each system may expose its own Rule Identity.

No system may infer another system’s rules implicitly.

---

## XI. Invariants

- Rule Identity must be deterministic  
- Rule Identity must be immutable per build  
- Verification must be side-effect free  
- Provenance must be explicit where used  
- Unknown rules must never be reinterpreted  
- Compatibility must be explicit, never inferred  
- Rule Identity must not be conflated with legitimacy  

Violation of these invariants compromises historical integrity.

---

## XII. Mental Model

Receipts answer:
“What was decided?”

Rule Identity answers:
“Under which rules was it decided?”

Verification answers:
“Do these rules match or differ?”

Legitimacy answers:
“Was the decision valid under those rules?”

These must remain separate.

---

## XIII. Closing Principle

Charter systems must preserve not only history, but the **meaning of that history**.

Rule Identity and Verification exist to ensure that:

- rules remain explicit  
- differences remain visible  
- history is never silently rewritten  

The system may evolve.

The past must remain interpretable.